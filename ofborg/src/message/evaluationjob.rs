use crate::message::{Pr, Repo};
use crate::worker;
use serde_json;

pub fn from(data: &[u8]) -> Result<EvaluationJob, serde_json::error::Error> {
    serde_json::from_slice(&data)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EvaluationJob {
    pub repo: Repo,
    pub pr: Pr,
}

impl EvaluationJob {
    pub fn is_nixpkgs(&self) -> bool {
        self.repo.name == "nixpkgs"
    }
}

pub struct Actions {}

impl Actions {
    pub fn skip(&mut self, _job: &EvaluationJob) -> worker::Actions {
        vec![worker::Action::Ack]
    }

    pub fn done(&mut self, _job: &EvaluationJob, mut response: worker::Actions) -> worker::Actions {
        response.push(worker::Action::Ack);
        response
    }
}
