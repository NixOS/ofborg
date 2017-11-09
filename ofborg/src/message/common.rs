
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Repo {
    pub full_name: String,
    pub clone_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pr {
    pub target_branch: Option<String>,
    pub number: i64,
    pub head_sha: String,
}
