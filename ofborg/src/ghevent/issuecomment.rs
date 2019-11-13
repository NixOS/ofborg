use crate::ghevent::{Comment, Issue, Repository};

#[derive(Serialize, Deserialize, Debug)]
pub struct IssueComment {
    pub action: IssueCommentAction,
    pub comment: Comment,
    pub repository: Repository,
    pub issue: Issue,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssueCommentAction {
    Created,
    Edited,
    Deleted,
}
