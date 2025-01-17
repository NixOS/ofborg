use crate::message::{Pr, Repo};
use crate::worker;

pub fn from(data: &[u8]) -> Result<EvaluationJob, serde_json::error::Error> {
    serde_json::from_slice(data)
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct EvaluationJob {
    pub repo: Repo,
    pub pr: Pr,
}

pub struct Actions {}

impl Actions {
    pub fn retry_later(&mut self, _job: &EvaluationJob) -> worker::Actions {
        vec![worker::Action::NackRequeue]
    }

    pub fn skip(&mut self, _job: &EvaluationJob) -> worker::Actions {
        vec![worker::Action::Ack]
    }

    pub fn done(&mut self, _job: &EvaluationJob, mut response: worker::Actions) -> worker::Actions {
        response.push(worker::Action::Ack);
        response
    }
}
