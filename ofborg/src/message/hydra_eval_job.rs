use crate::message::{Pr, Repo};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct HydraEvalJob {
    pub repo: Repo,
    pub pr: Pr,
    pub drv_paths: Vec<String>,
    pub request_id: String,
    pub jobset_id: i32,
}

pub fn from(data: &[u8]) -> Result<HydraEvalJob, serde_json::error::Error> {
    serde_json::from_slice(data)
}
