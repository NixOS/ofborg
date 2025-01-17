use crate::ghevent::{Comment, Issue, Repository};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct IssueComment {
    pub action: IssueCommentAction,
    pub comment: Comment,
    pub repository: Repository,
    pub issue: Issue,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum IssueCommentAction {
    Created,
    Edited,
    Deleted,
}
