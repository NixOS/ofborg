use crate::acl;
use crate::ghevent;
use crate::message::{evaluationjob, Pr, Repo};
use crate::worker;

use tracing::{debug_span, info};

pub struct EvaluationFilterWorker {
    acl: acl::Acl,
}

impl EvaluationFilterWorker {
    pub fn new(acl: acl::Acl) -> EvaluationFilterWorker {
        EvaluationFilterWorker { acl }
    }
}

impl worker::SimpleWorker for EvaluationFilterWorker {
    type J = ghevent::PullRequestEvent;

    fn msg_to_job(&mut self, _: &str, _: &Option<String>, body: &[u8]) -> Result<Self::J, String> {
        match serde_json::from_slice(body) {
            Ok(event) => Ok(event),
            Err(err) => Err(format!(
                "Failed to deserialize job {err:?}: {:?}",
                String::from_utf8(body.to_vec())
            )),
        }
    }

    fn consumer(&mut self, job: &ghevent::PullRequestEvent) -> worker::Actions {
        let span = debug_span!("job", pr = ?job.number);
        let _enter = span.enter();

        if !self.acl.is_repo_eligible(&job.repository.full_name) {
            info!("Repo not authorized ({})", job.repository.full_name);
            return vec![worker::Action::Ack];
        }

        if job.pull_request.state != ghevent::PullRequestState::Open {
            info!(
                "PR is not open ({}#{})",
                job.repository.full_name, job.number
            );
            return vec![worker::Action::Ack];
        }

        let interesting: bool = match job.action {
            ghevent::PullRequestAction::Opened => true,
            ghevent::PullRequestAction::Synchronize => true,
            ghevent::PullRequestAction::Reopened => true,
            ghevent::PullRequestAction::Edited => {
                if let Some(ref changes) = job.changes {
                    changes.base.is_some()
                } else {
                    false
                }
            }
            _ => false,
        };

        if !interesting {
            info!(
                "Not interesting: {}#{} because of {:?}",
                job.repository.full_name, job.number, job.action
            );

            return vec![worker::Action::Ack];
        }

        info!(
            "Found {}#{} to be interesting because of {:?}",
            job.repository.full_name, job.number, job.action
        );
        let repo_msg = Repo {
            clone_url: job.repository.clone_url.clone(),
            full_name: job.repository.full_name.clone(),
            owner: job.repository.owner.login.clone(),
            name: job.repository.name.clone(),
        };

        let pr_msg = Pr {
            number: job.number,
            head_sha: job.pull_request.head.sha.clone(),
            target_branch: Some(job.pull_request.base.git_ref.clone()),
        };

        let msg = evaluationjob::EvaluationJob {
            repo: repo_msg,
            pr: pr_msg,
        };

        vec![
            worker::publish_serde_action(None, Some("mass-rebuild-check-jobs".to_owned()), &msg),
            worker::Action::Ack,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::worker::SimpleWorker;

    #[test]
    fn changed_base() {
        let data = include_str!("../../test-srcs/events/pr-changed-base.json");

        let job: ghevent::PullRequestEvent =
            serde_json::from_str(data).expect("Should properly deserialize");

        let mut worker = EvaluationFilterWorker::new(acl::Acl::new(
            vec!["nixos/nixpkgs".to_owned()],
            Some(vec![]),
        ));

        assert_eq!(
            worker.consumer(&job),
            vec![
                worker::publish_serde_action(
                    None,
                    Some("mass-rebuild-check-jobs".to_owned()),
                    &evaluationjob::EvaluationJob {
                        repo: Repo {
                            clone_url: String::from("https://github.com/NixOS/nixpkgs.git"),
                            full_name: String::from("NixOS/nixpkgs"),
                            owner: String::from("NixOS"),
                            name: String::from("nixpkgs"),
                        },
                        pr: Pr {
                            number: 33299,
                            head_sha: String::from("887e8b460a7d45ddb3bbdebe01447b251b3229e8"),
                            target_branch: Some(String::from("staging")),
                        },
                    }
                ),
                worker::Action::Ack,
            ]
        );
    }
}
