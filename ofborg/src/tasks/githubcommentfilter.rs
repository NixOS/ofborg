use crate::acl;
use crate::commentparser;
use crate::ghevent;
use crate::message::{buildjob, evaluationjob, Pr, Repo};
use crate::worker;

use tracing::{debug_span, error, info};
use uuid::Uuid;

pub struct GitHubCommentWorker {
    acl: acl::Acl,
    github: hubcaps::Github,
}

impl GitHubCommentWorker {
    pub fn new(acl: acl::Acl, github: hubcaps::Github) -> GitHubCommentWorker {
        GitHubCommentWorker { acl, github }
    }
}

impl worker::SimpleWorker for GitHubCommentWorker {
    type J = ghevent::IssueComment;

    fn msg_to_job(&mut self, _: &str, _: &Option<String>, body: &[u8]) -> Result<Self::J, String> {
        match serde_json::from_slice(body) {
            Ok(comment) => Ok(comment),
            Err(err) => {
                error!(
                    "Failed to deserialize IsssueComment: {:?}",
                    String::from_utf8(body.to_vec())
                );
                panic!("{err:?}");
            }
        }
    }

    // FIXME: remove with rust/cargo update
    #[allow(clippy::cognitive_complexity)]
    fn consumer(&mut self, job: &ghevent::IssueComment) -> worker::Actions {
        let span = debug_span!("job", pr = ?job.issue.number);
        let _enter = span.enter();

        if job.action == ghevent::IssueCommentAction::Deleted {
            return vec![worker::Action::Ack];
        }

        let instructions = commentparser::parse(&job.comment.body);
        if instructions.is_none() {
            return vec![worker::Action::Ack];
        }

        let build_destinations = self.acl.build_job_architectures_for_user_repo(
            &job.comment.user.login,
            &job.repository.full_name,
        );

        if build_destinations.is_empty() {
            info!("No build destinations for: {:?}", job);
            // Don't process comments if they can't build anything
            return vec![worker::Action::Ack];
        }

        info!("Got job: {:?}", job);

        let instructions = commentparser::parse(&job.comment.body);
        info!("Instructions: {:?}", instructions);

        let pr = async_std::task::block_on(
            self.github
                .repo(
                    job.repository.owner.login.clone(),
                    job.repository.name.clone(),
                )
                .pulls()
                .get(job.issue.number)
                .get(),
        );

        if let Err(x) = pr {
            info!(
                "fetching PR {}#{} from GitHub yielded error {}",
                job.repository.full_name, job.issue.number, x
            );
            return vec![worker::Action::Ack];
        }

        let pr = pr.unwrap();

        let repo_msg = Repo {
            clone_url: job.repository.clone_url.clone(),
            full_name: job.repository.full_name.clone(),
            owner: job.repository.owner.login.clone(),
            name: job.repository.name.clone(),
        };

        let pr_msg = Pr {
            number: job.issue.number,
            head_sha: pr.head.sha.clone(),
            target_branch: Some(pr.base.commit_ref),
        };

        let mut response: Vec<worker::Action> = vec![];
        if let Some(instructions) = instructions {
            for instruction in instructions {
                match instruction {
                    commentparser::Instruction::Build(subset, attrs) => {
                        let build_destinations = match subset {
                            commentparser::Subset::NixOS => build_destinations
                                .clone()
                                .into_iter()
                                .filter(|x| x.can_run_nixos_tests())
                                .collect(),
                            _ => build_destinations.clone(),
                        };

                        let msg = buildjob::BuildJob::new(
                            repo_msg.clone(),
                            pr_msg.clone(),
                            subset,
                            attrs,
                            None,
                            None,
                            Uuid::new_v4().to_string(),
                        );

                        for arch in build_destinations.iter() {
                            let (exchange, routingkey) = arch.as_build_destination();
                            response.push(worker::publish_serde_action(exchange, routingkey, &msg));
                        }

                        response.push(worker::publish_serde_action(
                            Some("build-results".to_string()),
                            None,
                            &buildjob::QueuedBuildJobs {
                                job: msg,
                                architectures: build_destinations
                                    .iter()
                                    .cloned()
                                    .map(|arch| arch.to_string())
                                    .collect(),
                            },
                        ));
                    }
                    commentparser::Instruction::Eval => {
                        let msg = evaluationjob::EvaluationJob {
                            repo: repo_msg.clone(),
                            pr: pr_msg.clone(),
                        };

                        response.push(worker::publish_serde_action(
                            None,
                            Some("mass-rebuild-check-jobs".to_owned()),
                            &msg,
                        ));
                    }
                }
            }
        }

        response.push(worker::Action::Ack);
        response
    }
}
