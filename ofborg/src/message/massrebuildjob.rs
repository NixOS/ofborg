use ofborg::message::{Pr, Repo};
use ofborg::worker;
use serde_json;


pub fn from(data: &Vec<u8>) -> Result<MassRebuildJob, serde_json::error::Error> {
    return serde_json::from_slice(&data);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MassRebuildJob {
    pub repo: Repo,
    pub pr: Pr,
}

pub struct Actions {}

impl Actions {
    pub fn skip(&mut self, _job: &MassRebuildJob) -> worker::Actions {
        return vec![worker::Action::Ack];
    }

    pub fn done(&mut self, _job: &MassRebuildJob, mut response: worker::Actions) -> worker::Actions {
        response.push(worker::Action::Ack);
        return response;
    }
}
