use crate::commentparser::Subset;
use crate::message::{Pr, Repo};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct BuildJob {
    pub repo: Repo,
    pub pr: Pr,
    pub subset: Option<Subset>,
    pub attrs: Vec<String>,
    pub request_id: String,
    pub logs: Option<ExchangeQueue>, // (Exchange, Routing Key)
    pub statusreport: Option<ExchangeQueue>, // (Exchange, Routing Key)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueuedBuildJobs {
    pub job: BuildJob,
    pub architectures: Vec<String>,
}

pub type ExchangeQueue = (Option<Exchange>, Option<RoutingKey>);
type Exchange = String;
type RoutingKey = String;

impl BuildJob {
    pub fn new(
        repo: Repo,
        pr: Pr,
        subset: Subset,
        attrs: Vec<String>,
        logs: Option<ExchangeQueue>,
        statusreport: Option<ExchangeQueue>,
        request_id: String,
    ) -> BuildJob {
        let logbackrk = format!("{}.{}", repo.full_name.clone(), pr.number,).to_lowercase();

        BuildJob {
            repo,
            pr,
            subset: Some(subset),
            attrs,
            logs: Some(logs.unwrap_or((Some("logs".to_owned()), Some(logbackrk)))),
            statusreport: Some(statusreport.unwrap_or((Some("build-results".to_owned()), None))),
            request_id,
        }
    }
}

pub fn from(data: &[u8]) -> Result<BuildJob, serde_json::error::Error> {
    serde_json::from_slice(&data)
}

pub struct Actions {
    pub system: String,
}
