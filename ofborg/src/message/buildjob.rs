use ofborg::message::{Pr,Repo};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct BuildJob {
    pub repo: Repo,
    pub pr: Pr,
}

pub fn from(data: &Vec<u8>) -> Result<BuildJob, serde_json::error::Error> {
    return serde_json::from_slice(&data);
}
