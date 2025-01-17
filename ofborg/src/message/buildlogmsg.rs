#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct BuildLogMsg {
    pub system: String,
    pub identity: String,
    pub attempt_id: String,
    pub line_number: u64,
    pub output: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct BuildLogStart {
    pub system: String,
    pub identity: String,
    pub attempt_id: String,
    pub attempted_attrs: Option<Vec<String>>,
    pub skipped_attrs: Option<Vec<String>>,
}
