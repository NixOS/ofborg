#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Repo {
    pub owner: String,
    pub name: String,
    pub full_name: String,
    pub clone_url: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Pr {
    pub target_branch: Option<String>,
    pub number: u64,
    pub head_sha: String,
}
