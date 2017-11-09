use ofborg::message::{Pr,Repo};

#[derive(Serialize, Deserialize, Debug)]
pub struct BuildResult {
    pub repo: Repo,
    pub pr: Pr,
    pub system: String,
    pub output: Vec<String>,
    pub success: bool
}
