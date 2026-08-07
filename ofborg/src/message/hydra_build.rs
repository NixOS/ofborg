use crate::message::buildresult::BuildStatus;
use crate::message::{Pr, Repo};

/// Everything needed to map a Hydra build back to the pull request that caused it.
///
/// One of these is published per build to the durable `hydra-build-tracking`
/// queue right after `CreateBuild` returns. That queue *is* the store for this
/// mapping: `hydra-build-tracker` consumes without acking and only acks once the
/// build reached a terminal state, so a restart simply gets the still-pending
/// records redelivered.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct HydraBuildTracking {
    pub repo: Repo,
    pub pr: Pr,
    pub attr: String,
    pub system: String,
    pub drv_path: String,
    pub build_id: i32,
    pub jobset_id: i32,
    pub request_id: String,
    /// Unix seconds. Drives the stale sweep, so that a build Hydra never reports
    /// on cannot pin its tracking record in the queue forever.
    pub queued_at: i64,
}

pub fn tracking_from(data: &[u8]) -> Result<HydraBuildTracking, serde_json::error::Error> {
    serde_json::from_slice(data)
}

/// Discriminator for [`HydraBuildUpdate`].
///
/// `BuildResult` has an `untagged` `Legacy` variant that happily swallows
/// loosely shaped JSON, so this message needs a tag of its own to stay
/// distinguishable on the shared `build-results` exchange.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum HydraV1Tag {
    HydraV1,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "state")]
pub enum HydraBuildState {
    Queued,
    Running {
        /// What the builder is currently doing, e.g. `Sending inputs`.
        #[serde(default)]
        step: Option<String>,
    },
    Finished {
        status: BuildStatus,
    },
}

/// A build lifecycle update on its way to a GitHub check run.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct HydraBuildUpdate {
    pub tag: HydraV1Tag,
    pub repo: Repo,
    pub pr: Pr,
    pub attr: String,
    pub system: String,
    pub build_id: i32,
    /// Hostname of the builder the step landed on, once one has been assigned.
    pub machine: Option<String>,
    pub state: HydraBuildState,
    /// Base URL of the Hydra web UI, e.g. `https://hydra.nixos.org`.
    ///
    /// Travels with the message rather than living in the comment poster's
    /// config so that the deployment only has to configure it once, next to the
    /// gateway endpoint it belongs to.
    pub hydra_base_url: String,
}

impl HydraBuildUpdate {
    /// Link to the Hydra build page, which is where the per-step machine
    /// assignment and the build log live.
    pub fn details_url(&self) -> String {
        format!(
            "{}/build/{}",
            self.hydra_base_url.trim_end_matches('/'),
            self.build_id
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo() -> Repo {
        Repo {
            clone_url: "https://github.com/nixos/nixpkgs.git".to_owned(),
            full_name: "NixOS/nixpkgs".to_owned(),
            owner: "NixOS".to_owned(),
            name: "nixpkgs".to_owned(),
        }
    }

    fn pr() -> Pr {
        Pr {
            head_sha: "abc123".to_owned(),
            number: 2345,
            target_branch: Some("master".to_owned()),
        }
    }

    fn update(state: HydraBuildState) -> HydraBuildUpdate {
        HydraBuildUpdate {
            tag: HydraV1Tag::HydraV1,
            repo: repo(),
            pr: pr(),
            attr: "hello".to_owned(),
            system: "x86_64-linux".to_owned(),
            build_id: 7,
            machine: None,
            state,
            hydra_base_url: "https://hydra.example.com".to_owned(),
        }
    }

    #[test]
    fn round_trips() {
        for state in [
            HydraBuildState::Queued,
            HydraBuildState::Running { step: None },
            HydraBuildState::Running {
                step: Some("Sending inputs".to_owned()),
            },
            HydraBuildState::Finished {
                status: BuildStatus::Success,
            },
            HydraBuildState::Finished {
                status: BuildStatus::UnexpectedError {
                    err: "boom".to_owned(),
                },
            },
        ] {
            let original = update(state);
            let json = serde_json::to_vec(&original).expect("should serialize");
            let parsed: HydraBuildUpdate =
                serde_json::from_slice(&json).expect("should deserialize");
            assert_eq!(original, parsed);
        }
    }

    #[test]
    fn details_url_tolerates_a_trailing_slash() {
        let mut u = update(HydraBuildState::Queued);
        u.hydra_base_url = "https://hydra.example.com/".to_owned();
        assert_eq!(u.details_url(), "https://hydra.example.com/build/7");
    }

    #[test]
    fn rejects_an_untagged_payload() {
        let mut json: serde_json::Value =
            serde_json::to_value(update(HydraBuildState::Queued)).expect("should serialize");
        json.as_object_mut().expect("object").remove("tag");

        assert!(serde_json::from_value::<HydraBuildUpdate>(json).is_err());
    }
}
