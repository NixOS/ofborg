use ofborg::message::{Pr, Repo};


#[derive(Serialize, Deserialize, Debug)]
pub enum BuildStatus {
    Skipped,
    Success,
    Failure,
    TimedOut,
    UnexpectedError { err: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BuildResult {
    pub repo: Repo,
    pub pr: Pr,
    pub system: String,
    pub output: Vec<String>,
    pub attempt_id: String,
    pub request_id: String,
    pub success: Option<bool>, // replaced by status
    pub status: Option<BuildStatus>,
    pub skipped_attrs: Option<Vec<String>>,
    pub attempted_attrs: Option<Vec<String>>,
}
