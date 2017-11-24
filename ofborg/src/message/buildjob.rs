use ofborg::message::{Pr,Repo};
use ofborg::message::buildresult;
use ofborg::commentparser::Subset;
use ofborg::worker;
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct BuildJob {
    pub repo: Repo,
    pub pr: Pr,
    pub subset: Option<Subset>,
    pub attrs: Vec<String>,
}

pub fn from(data: &Vec<u8>) -> Result<BuildJob, serde_json::error::Error> {
    return serde_json::from_slice(&data);
}

pub struct Actions {
    pub system: String,
}

impl Actions {
    pub fn commit_missing(&mut self, _job: &BuildJob) -> worker::Actions {
        return vec![
            worker::Action::Ack
        ];
    }

    pub fn nasty_hack_linux_only(&mut self, _job: &BuildJob) -> worker::Actions {
        return vec![
            worker::Action::Ack
        ];
    }

    pub fn merge_failed(&mut self, job: &BuildJob) -> worker::Actions {
        let msg = buildresult::BuildResult {
            repo: job.repo.clone(),
            pr: job.pr.clone(),
            system: self.system.clone(),
            output: vec![String::from("Merge failed")],
            success: false
        };

        return vec![
            worker::publish_serde_action(
                Some("build-results".to_owned()),
                None,
                &msg
            ),
            worker::Action::Ack
        ];
    }

    pub fn build_finished(&mut self, job: &BuildJob, success: bool, lines: Vec<String>) -> worker::Actions {
        let msg = buildresult::BuildResult {
            repo: job.repo.clone(),
            pr: job.pr.clone(),
            system: self.system.clone(),
            output: lines,
            success: success
        };

        return vec![
            worker::publish_serde_action(
                Some("build-results".to_owned()),
                None,
                &msg
            ),
            worker::Action::Ack
        ];
    }
}
