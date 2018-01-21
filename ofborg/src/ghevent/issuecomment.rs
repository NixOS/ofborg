use ofborg::ghevent::{Comment, Repository, Issue};

#[derive(Serialize, Deserialize, Debug)]
pub struct IssueComment {
    pub comment: Comment,
    pub repository: Repository,
    pub issue: Issue,
}
