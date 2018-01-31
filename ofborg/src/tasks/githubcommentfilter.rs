extern crate amqp;
extern crate env_logger;

use ofborg::ghevent;
use ofborg::acl;
use serde_json;

use hubcaps;
use ofborg::message::{Repo, Pr, buildjob, massrebuildjob};
use ofborg::worker;
use ofborg::commentparser;
use amqp::protocol::basic::{Deliver, BasicProperties};


pub struct GitHubCommentWorker {
    acl: acl::ACL,
    github: hubcaps::Github,
}

impl GitHubCommentWorker {
    pub fn new(acl: acl::ACL, github: hubcaps::Github) -> GitHubCommentWorker {
        return GitHubCommentWorker {
            acl: acl,
            github: github,
        };
    }
}

impl worker::SimpleWorker for GitHubCommentWorker {
    type J = ghevent::IssueComment;

    fn msg_to_job(
        &mut self,
        _: &Deliver,
        _: &BasicProperties,
        body: &Vec<u8>,
    ) -> Result<Self::J, String> {
        return match serde_json::from_slice(body) {
            Ok(e) => Ok(e),
            Err(e) => {
                println!(
                    "Failed to deserialize IsssueComment: {:?}",
                    String::from_utf8(body.clone())
                );
                panic!("{:?}", e);
            }
        };
    }

    fn consumer(&mut self, job: &ghevent::IssueComment) -> worker::Actions {
        let instructions = commentparser::parse(&job.comment.body);
        if instructions == None {
            return vec![worker::Action::Ack];
        }

        let build_destinations: Vec<(Option<String>, Option<String>)>;

        if self.acl.can_build_unrestricted(
            &job.comment.user.login,
            &job.repository.full_name,
        )
        {
            build_destinations = vec![(Some("build-jobs".to_owned()), None)];
        } else if self.acl.can_build_restricted(
            &job.comment.user.login,
            &job.repository.full_name,
        )
        {
            build_destinations = vec![
                (None, Some("build-inputs-x86_64-linux".to_owned())),
                (None, Some("build-inputs-aarch64-linux".to_owned())),
            ];
        } else {
            println!(
                "ACL prohibits {} from building {:?} for {}",
                job.comment.user.login,
                instructions,
                job.repository.full_name
            );
            return vec![worker::Action::Ack];
        }

        println!("Got job: {:?}", job);

        let instructions = commentparser::parse(&job.comment.body);
        println!("Instructions: {:?}", instructions);

        let pr = self.github
            .repo(
                job.repository.owner.login.clone(),
                job.repository.name.clone(),
            )
            .pulls()
            .get(job.issue.number)
            .get();

        if let Err(x) = pr {
            info!(
                "fetching PR {}#{} from GitHub yielded error {}",
                job.repository.full_name,
                job.issue.number,
                x
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
            number: job.issue.number.clone(),
            head_sha: pr.head.sha.clone(),
            target_branch: Some(pr.base.commit_ref.clone()),
        };

        let mut response: Vec<worker::Action> = vec![];
        if let Some(instructions) = instructions {
            for instruction in instructions {
                match instruction {
                    commentparser::Instruction::Build(subset, attrs) => {
                        let logbackrk = format!(
                            "{}.{}",
                            job.repository.full_name.clone(),
                            job.issue.number.clone()
                        );
                        let msg = buildjob::BuildJob {
                            repo: repo_msg.clone(),
                            pr: pr_msg.clone(),
                            subset: Some(subset),
                            attrs: attrs,
                            logs: Some((Some("logs".to_owned()), Some(logbackrk.to_lowercase()))),
                            statusreport: Some((Some("build-results".to_owned()), None)),
                        };

                        for (exch, rk) in build_destinations.clone() {
                            response.push(worker::publish_serde_action(exch, rk, &msg));
                        }
                    }
                    commentparser::Instruction::Eval => {
                        let msg = massrebuildjob::MassRebuildJob {
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
        return response;
    }
}
