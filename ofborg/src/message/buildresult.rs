use ofborg::message::{Pr, Repo};

#[derive(Serialize, Deserialize, Debug)]
pub struct BuildResult {
    pub repo: Repo,
    pub pr: Pr,
    pub system: String,
    pub output: Vec<String>,
    pub attempt_id: Option<String>,
    pub success: Option<bool>,
    pub skipped_attrs: Option<Vec<String>>,
    pub attempted_attrs: Option<Vec<String>>,
}
