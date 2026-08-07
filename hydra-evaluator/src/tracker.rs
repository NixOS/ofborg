//! Joins Hydra build events back onto the pull requests that caused them.
//!
//! The tracker holds two streams open:
//!
//! 1. `hydra-build-tracking`, consumed **without acking**. Every unacked
//!    delivery is a build still in flight, so the queue doubles as the restart
//!    state: kill the tracker and RabbitMQ redelivers exactly the set of builds
//!    that had not finished yet.
//! 2. Build events from the queue-runner.
//!
//! Each event that matches a tracked build becomes a `HydraBuildUpdate` on the
//! `build-results` fanout, where `github-comment-poster` turns it into a check
//! run. A terminal event acks the tracking record and drops it.

use std::collections::HashMap;
use std::time::Duration;

use lapin::Channel;
use lapin::options::{BasicAckOptions, BasicConsumeOptions, BasicQosOptions};
use lapin::types::FieldTable;
use ofborg::message::buildresult::BuildStatus;
use ofborg::message::hydra_build::{
    HydraBuildState, HydraBuildTracking, HydraBuildUpdate, HydraV1Tag,
};
use tokio_stream::StreamExt as _;
use tracing::{debug, error, info, warn};

use crate::events::{BuildEvent, BuildEventKind, BuildEventStream};

/// Upper bound on how many in-flight builds we hold in memory at once. Beyond
/// this RabbitMQ stops handing out tracking records until some are acked, which
/// is backpressure rather than an out-of-memory crash.
const TRACKING_PREFETCH: u16 = 10_000;

const SWEEP_INTERVAL: Duration = Duration::from_secs(60);

#[derive(Debug)]
struct Pending {
    delivery_tag: u64,
    tracking: HydraBuildTracking,
    /// What we last told GitHub about this build. The queue-runner reports
    /// every step transition, most of which render identically, and each one we
    /// forward costs a GitHub API call.
    last_published: Option<(HydraBuildState, Option<String>)>,
}

pub struct Tracker {
    chan: Channel,
    hydra_base_url: String,
    stale_after_seconds: u64,
    pending: HashMap<i32, Pending>,
}

impl std::fmt::Debug for Tracker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tracker")
            .field("hydra_base_url", &self.hydra_base_url)
            .field("stale_after_seconds", &self.stale_after_seconds)
            .field("pending", &self.pending.len())
            .finish_non_exhaustive()
    }
}

impl Tracker {
    pub fn new(chan: Channel, hydra_base_url: String, stale_after_seconds: u64) -> Self {
        Self {
            chan,
            hydra_base_url,
            stale_after_seconds,
            pending: HashMap::new(),
        }
    }

    pub async fn run(&mut self, mut events: BuildEventStream) -> anyhow::Result<()> {
        crate::declare_durable_queue(&self.chan, crate::HYDRA_BUILD_TRACKING_QUEUE).await?;
        crate::declare_build_results_exchange(&self.chan).await?;

        self.chan
            .basic_qos(TRACKING_PREFETCH, BasicQosOptions::default())
            .await?;

        let mut tracking = self
            .chan
            .basic_consume(
                crate::HYDRA_BUILD_TRACKING_QUEUE.into(),
                "ofborg-hydra-build-tracker".into(),
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        let mut sweep = tokio::time::interval(SWEEP_INTERVAL);
        sweep.tick().await; // the first tick completes immediately

        info!("tracking Hydra builds");

        loop {
            tokio::select! {
                Some(delivery) = tracking.next() => {
                    match delivery {
                        Ok(delivery) => self.on_tracking(delivery).await?,
                        Err(e) => {
                            error!("hydra-build-tracking consumer failed: {e:?}");
                            return Err(e.into());
                        }
                    }
                }
                Some(event) = events.next() => {
                    match event {
                        Ok(event) => self.on_event(event).await?,
                        Err(e) => {
                            error!("build event stream failed: {e:?}");
                            return Err(e);
                        }
                    }
                }
                _ = sweep.tick() => self.sweep().await?,
                else => break,
            }
        }

        Ok(())
    }

    async fn on_tracking(&mut self, delivery: lapin::message::Delivery) -> anyhow::Result<()> {
        let tracking: HydraBuildTracking =
            match ofborg::message::hydra_build::tracking_from(&delivery.data) {
                Ok(t) => t,
                Err(e) => {
                    // Nothing will ever make this message parse, so ack it
                    // rather than let it block the prefetch window forever.
                    error!(
                        "Failed to deserialize HydraBuildTracking: {e}, body: {:?}",
                        std::str::from_utf8(&delivery.data)
                    );
                    self.ack(delivery.delivery_tag).await;
                    return Ok(());
                }
            };

        info!(
            "tracking build {} ({} on {}) for {}/{} PR #{}",
            tracking.build_id,
            tracking.attr,
            tracking.system,
            tracking.repo.owner,
            tracking.repo.name,
            tracking.pr.number,
        );

        let replaced = self.pending.insert(
            tracking.build_id,
            Pending {
                delivery_tag: delivery.delivery_tag,
                tracking,
                last_published: None,
            },
        );

        // A duplicate record for the same build would otherwise strand the
        // delivery it displaced, holding a prefetch slot until the process dies.
        if let Some(old) = replaced {
            warn!("duplicate tracking record, acking the one it replaced");
            self.ack(old.delivery_tag).await;
        }

        Ok(())
    }

    async fn on_event(&mut self, event: BuildEvent) -> anyhow::Result<()> {
        if let BuildEventKind::Lagged { dropped } = &event.kind {
            warn!(
                "fell behind the queue-runner event stream, {dropped} event(s) dropped; \
                 affected builds resolve on their next event or via the stale sweep"
            );
            return Ok(());
        }

        // Events arrive for every ofborg build the queue-runner knows about,
        // including ones whose tracking record has not reached us yet, so an
        // unknown id is normal rather than an error.
        let Some(pending) = self.pending.get(&event.build_id) else {
            debug!("ignoring event for untracked build {}", event.build_id);
            return Ok(());
        };

        let update = build_update(&pending.tracking, &event, &self.hydra_base_url);
        let rendered = (update.state.clone(), update.machine.clone());

        if pending.last_published.as_ref() == Some(&rendered) {
            debug!(
                "build {} is still {:?} on the same machine, not republishing",
                event.build_id, update.state
            );
            return Ok(());
        }

        crate::publish_json(&self.chan, crate::BUILD_RESULTS_EXCHANGE, "", &update).await?;

        if let Some(pending) = self.pending.get_mut(&event.build_id) {
            pending.last_published = Some(rendered);
        }

        if event.is_terminal() {
            info!("build {} finished, done tracking it", event.build_id);
            if let Some(pending) = self.pending.remove(&event.build_id) {
                self.ack(pending.delivery_tag).await;
            }
        }

        Ok(())
    }

    /// Complete the check run of any build Hydra never reported on, so a lost
    /// event cannot leave a pull request showing a build in progress forever.
    async fn sweep(&mut self) -> anyhow::Result<()> {
        let now = crate::unix_now();

        let stale: Vec<i32> = self
            .pending
            .iter()
            .filter(|(_, pending)| {
                let age = now.saturating_sub(pending.tracking.queued_at);
                u64::try_from(age).unwrap_or(0) >= self.stale_after_seconds
            })
            .map(|(build_id, _)| *build_id)
            .collect();

        for build_id in stale {
            let Some(pending) = self.pending.get(&build_id) else {
                continue;
            };

            warn!("no result from Hydra for build {build_id}, giving up on it");

            let update = build_update(
                &pending.tracking,
                &BuildEvent {
                    build_id,
                    machine: None,
                    kind: BuildEventKind::Finished {
                        status: BuildStatus::TimedOut,
                    },
                },
                &self.hydra_base_url,
            );

            // Publish before dropping the record. A failure here propagates and
            // ends the run — the connection is gone — and because nothing was
            // acked, RabbitMQ redelivers everything on restart.
            crate::publish_json(&self.chan, crate::BUILD_RESULTS_EXCHANGE, "", &update).await?;

            if let Some(pending) = self.pending.remove(&build_id) {
                self.ack(pending.delivery_tag).await;
            }
        }

        Ok(())
    }

    async fn ack(&self, delivery_tag: u64) {
        if let Err(e) = self
            .chan
            .basic_ack(delivery_tag, BasicAckOptions::default())
            .await
        {
            warn!("failed to ack delivery {delivery_tag}: {e:?}");
        }
    }
}

fn build_update(
    tracking: &HydraBuildTracking,
    event: &BuildEvent,
    hydra_base_url: &str,
) -> HydraBuildUpdate {
    let state = match &event.kind {
        BuildEventKind::Queued => HydraBuildState::Queued,
        BuildEventKind::Running { step } => HydraBuildState::Running { step: step.clone() },
        BuildEventKind::Finished { status } => HydraBuildState::Finished {
            status: status.clone(),
        },
        // Filtered out before we get here, but map it to something harmless
        // rather than leaving the match non-exhaustive.
        BuildEventKind::Lagged { .. } => HydraBuildState::Running { step: None },
    };

    HydraBuildUpdate {
        tag: HydraV1Tag::HydraV1,
        repo: tracking.repo.clone(),
        pr: tracking.pr.clone(),
        attr: tracking.attr.clone(),
        system: tracking.system.clone(),
        build_id: tracking.build_id,
        machine: event.machine.clone(),
        state,
        hydra_base_url: hydra_base_url.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ofborg::message::{Pr, Repo};

    fn tracking() -> HydraBuildTracking {
        HydraBuildTracking {
            repo: Repo {
                clone_url: "https://github.com/nixos/nixpkgs.git".to_owned(),
                full_name: "NixOS/nixpkgs".to_owned(),
                owner: "NixOS".to_owned(),
                name: "nixpkgs".to_owned(),
            },
            pr: Pr {
                head_sha: "abc123".to_owned(),
                number: 42,
                target_branch: Some("master".to_owned()),
            },
            attr: "hello".to_owned(),
            system: "x86_64-linux".to_owned(),
            drv_path: "/nix/store/aaa-hello.drv".to_owned(),
            build_id: 7,
            jobset_id: 1,
            request_id: "r".to_owned(),
            queued_at: 0,
        }
    }

    #[test]
    fn carries_the_machine_through_to_the_update() {
        let update = build_update(
            &tracking(),
            &BuildEvent {
                build_id: 7,
                machine: Some("builder-01".to_owned()),
                kind: BuildEventKind::Running {
                    step: Some("Building".to_owned()),
                },
            },
            "https://hydra.example.com",
        );

        assert_eq!(update.machine.as_deref(), Some("builder-01"));
        assert_eq!(
            update.state,
            HydraBuildState::Running {
                step: Some("Building".to_owned())
            }
        );
        assert_eq!(update.attr, "hello");
        assert_eq!(update.system, "x86_64-linux");
        assert_eq!(update.details_url(), "https://hydra.example.com/build/7");
    }

    #[test]
    fn maps_a_failed_build_to_a_finished_update() {
        let update = build_update(
            &tracking(),
            &BuildEvent {
                build_id: 7,
                machine: None,
                kind: BuildEventKind::Finished {
                    status: BuildStatus::Failure,
                },
            },
            "https://hydra.example.com",
        );

        assert_eq!(
            update.state,
            HydraBuildState::Finished {
                status: BuildStatus::Failure
            }
        );
    }
}
