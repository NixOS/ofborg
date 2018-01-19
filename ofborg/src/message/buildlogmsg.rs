use ofborg::message::{Pr,Repo};

#[derive(Serialize, Deserialize, Debug)]
pub struct BuildLogMsg {
    pub line_number: u64,
    pub output: String,
    pub identity: String,
    pub system: String,
}
