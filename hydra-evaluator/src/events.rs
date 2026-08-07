//! Build lifecycle events coming out of the Hydra queue-runner.
//!
//! The queue-runner is the authority on where a build runs and how it ends, but
//! it knows nothing about pull requests. These events carry the queue-runner's
//! view; [`crate::tracker`] joins them against the tracking records ofborg kept
//! when it created the builds.

use std::pin::Pin;

use futures::Stream;
use ofborg::message::buildresult::BuildStatus;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "kind")]
pub enum BuildEventKind {
    Queued,
    Running {
        /// What the builder is currently doing, e.g. `Sending inputs`.
        #[serde(default)]
        step: Option<String>,
    },
    Finished {
        status: BuildStatus,
    },
    /// The queue-runner produced events faster than we consumed them and some
    /// were dropped. Nothing is lost permanently — the stale sweep and the
    /// queue-runner's status endpoint still resolve anything we missed — but a
    /// build may sit at `Running` for longer than it really did.
    Lagged {
        dropped: u64,
    },
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct BuildEvent {
    pub build_id: i32,
    /// Hostname of the builder the step landed on, once one was assigned.
    #[serde(default)]
    pub machine: Option<String>,
    #[serde(flatten)]
    pub kind: BuildEventKind,
}

impl BuildEvent {
    pub fn is_terminal(&self) -> bool {
        matches!(self.kind, BuildEventKind::Finished { .. })
    }
}

pub type BuildEventStream = Pin<Box<dyn Stream<Item = anyhow::Result<BuildEvent>> + Send>>;

/// Read events from a JSON-lines file instead of the queue-runner.
///
/// This is what makes the whole RabbitMQ-to-check-run path testable without a
/// running Hydra: write the sequence you want to see, point `--replay` at it.
pub fn replay_from_file(
    path: &std::path::Path,
    delay: std::time::Duration,
) -> anyhow::Result<BuildEventStream> {
    let contents = fs_err::read_to_string(path)?;

    let events = contents
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| serde_json::from_str::<BuildEvent>(line).map_err(anyhow::Error::from))
        .collect::<Vec<_>>();

    Ok(Box::pin(async_stream_of(events, delay)))
}

fn async_stream_of(
    events: Vec<anyhow::Result<BuildEvent>>,
    delay: std::time::Duration,
) -> impl Stream<Item = anyhow::Result<BuildEvent>> + Send {
    futures::stream::unfold(events.into_iter(), move |mut iter| async move {
        let next = iter.next()?;
        tokio::time::sleep(delay).await;
        Some((next, iter))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_replay_line() {
        let event: BuildEvent = serde_json::from_str(
            r#"{"build_id":7,"machine":"builder-01","kind":"Running","step":"Building"}"#,
        )
        .expect("should parse");

        assert_eq!(event.build_id, 7);
        assert_eq!(event.machine.as_deref(), Some("builder-01"));
        assert_eq!(
            event.kind,
            BuildEventKind::Running {
                step: Some("Building".to_owned())
            }
        );
        assert!(!event.is_terminal());
    }

    #[test]
    fn step_is_optional() {
        let event: BuildEvent =
            serde_json::from_str(r#"{"build_id":7,"kind":"Running"}"#).expect("should parse");

        assert_eq!(event.kind, BuildEventKind::Running { step: None });
    }

    #[test]
    fn parses_a_finished_line() {
        let event: BuildEvent =
            serde_json::from_str(r#"{"build_id":7,"kind":"Finished","status":"Success"}"#)
                .expect("should parse");

        assert!(event.is_terminal());
        assert_eq!(
            event.kind,
            BuildEventKind::Finished {
                status: BuildStatus::Success
            }
        );
    }
}
