#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Comment {
    pub body: String,
    pub user: User,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct User {
    pub login: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Repository {
    pub owner: User,
    pub name: String,
    pub full_name: String,
    pub clone_url: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Issue {
    pub number: u64,
}

/// A generic webhook that we received with minimal verification, only for handling in the GitHub
/// webhook receiver.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GenericWebhook {
    /// The repository the event originated
    pub repository: Repository,
}
