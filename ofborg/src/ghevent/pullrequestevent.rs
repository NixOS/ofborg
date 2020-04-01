use crate::ghevent::Repository;

#[derive(Serialize, Deserialize)]
pub struct PullRequestEvent {
    pub action: PullRequestAction,
    pub number: u64,
    pub repository: Repository,
    pub pull_request: PullRequest,
    pub changes: Option<PullRequestChanges>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullRequestChanges {
    pub base: Option<BaseChange>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseChange {
    #[serde(rename = "ref")]
    pub git_ref: ChangeWas,
    pub sha: ChangeWas,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ChangeWas {
    pub from: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PullRequestState {
    Open,
    Closed,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PullRequestAction {
    Assigned,
    Unassigned,
    ReviewRequested,
    ReviewRequestRemoved,
    Labeled,
    Unlabeled,
    Opened,
    Edited,
    Closed,
    Reopened,
    Synchronize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullRequestRef {
    #[serde(rename = "ref")]
    pub git_ref: String,
    pub sha: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullRequest {
    pub state: PullRequestState,
    pub base: PullRequestRef,
    pub head: PullRequestRef,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_parse_pr_event() {
        let data = include_str!("../../test-srcs/events/pr-changed-base.json");

        let _p: PullRequestEvent =
            serde_json::from_str(&data.to_string()).expect("Should properly deserialize");
    }
}
