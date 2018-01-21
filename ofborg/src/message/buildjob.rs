use ofborg::message::{Pr, Repo};
use ofborg::commentparser::Subset;
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct BuildJob {
    pub repo: Repo,
    pub pr: Pr,
    pub subset: Option<Subset>,
    pub attrs: Vec<String>,
    pub logs: Option<(Option<String>, Option<String>)>, // (Exchange, Routing Key)
}

pub fn from(data: &Vec<u8>) -> Result<BuildJob, serde_json::error::Error> {
    return serde_json::from_slice(&data);
}

pub struct Actions {
    pub system: String,
}
