#[derive(Serialize, Deserialize, Debug)]
pub struct Comment {
    pub body: String,
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub login: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Repository {
    pub owner: User,
    pub name: String,
    pub full_name: String,
    pub clone_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Issue {
    pub number: u64,
}
