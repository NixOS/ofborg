use crate::ghevent::Repository;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PullRequestEvent {
    pub action: PullRequestAction,
    pub number: u64,
    pub repository: Repository,
    pub pull_request: PullRequest,
    pub changes: Option<PullRequestChanges>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct PullRequestChanges {
    pub base: Option<BaseChange>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct BaseChange {
    #[serde(rename = "ref")]
    pub git_ref: ChangeWas,
    pub sha: ChangeWas,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
pub struct ChangeWas {
    pub from: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PullRequestState {
    Open,
    Closed,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PullRequestAction {
    Edited,
    Opened,
    Reopened,
    Synchronize,
    #[serde(other)]
    Unknown,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct PullRequestRef {
    #[serde(rename = "ref")]
    pub git_ref: String,
    pub sha: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
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
    fn test_parse_changed_base() {
        let data = include_str!("../../test-srcs/events/pr-changed-base.json");

        let pr: PullRequestEvent = serde_json::from_str(data).expect("Should properly deserialize");
        assert_eq!(pr.action, PullRequestAction::Edited);
    }

    #[test]
    fn test_parse_unknown_action() {
        let data = include_str!("../../test-srcs/events/pr-converted-to-draft.json");

        let pr: PullRequestEvent = serde_json::from_str(data).expect("Should properly deserialize");
        assert_eq!(pr.action, PullRequestAction::Unknown);
    }
}
