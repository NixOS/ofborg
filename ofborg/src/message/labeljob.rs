use ofborg::message::{Pr, Repo};

#[derive(Serialize, Deserialize, Debug)]
pub struct LabelJob {
    pub repo: Repo,
    pub pr: Pr,
    pub add_labels: Vec<String>,
    pub remove_labels: Vec<String>,
}
