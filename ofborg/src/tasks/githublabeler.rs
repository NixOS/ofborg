extern crate amqp;
extern crate env_logger;

use std::collections::HashMap;

use serde_json;

use hubcaps;
use ofborg::message::labeljob::LabelJob;
use ofborg::tasks::massrebuilder::update_labels;
use ofborg::worker;
use amqp::protocol::basic::{Deliver, BasicProperties};


pub struct GitHubLabeler {
    github: hubcaps::Github,
    labels: HashMap<String, String>,
}

impl GitHubLabeler {
    pub fn new(github: hubcaps::Github, labels: HashMap<String, String>) -> GitHubLabeler {
        return GitHubLabeler { github, labels };
    }
}

impl worker::SimpleWorker for GitHubLabeler {
    type J = LabelJob;

    fn msg_to_job(
        &mut self,
        _: &Deliver,
        _: &BasicProperties,
        body: &Vec<u8>,
    ) -> Result<Self::J, String> {
        return match serde_json::from_slice(body) {
            Ok(e) => Ok(e),
            Err(e) => {
                Err(format!(
                    "Failed to deserialize LabelJob: {:?}, err: {:}",
                    String::from_utf8_lossy(&body.clone()),
                    e
                ))
            }
        };
    }

    fn consumer(&mut self, job: &LabelJob) -> worker::Actions {
        let add_labels = job.add_labels
            .iter()
            .filter_map(|x| self.labels.get(x).cloned())
            .collect::<Vec<String>>();
        let remove_labels = job.remove_labels
            .iter()
            .filter_map(|x| self.labels.get(x).cloned())
            .collect::<Vec<String>>();

        let repo = self.github.repo(job.repo.owner.clone(), job.repo.name.clone());
        let issue = repo.issue(job.pr.number);

        update_labels(&issue, add_labels, remove_labels);

        return vec![worker::Action::Ack];
    }
}
