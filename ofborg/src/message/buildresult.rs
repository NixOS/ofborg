use crate::message::{Pr, Repo};
use hubcaps::checks::Conclusion;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum BuildStatus {
    Skipped,
    Success,
    Failure,
    TimedOut,
    UnexpectedError { err: String },
}

impl From<BuildStatus> for String {
    fn from(status: BuildStatus) -> String {
        match status {
            BuildStatus::Skipped => "No attempt".into(),
            BuildStatus::Success => "Success".into(),
            BuildStatus::Failure => "Failure".into(),
            BuildStatus::TimedOut => "Timed out, unknown build status".into(),
            BuildStatus::UnexpectedError { ref err } => format!("Unexpected error: {}", err),
        }
    }
}

impl From<BuildStatus> for Conclusion {
    fn from(status: BuildStatus) -> Conclusion {
        match status {
            BuildStatus::Skipped => Conclusion::Neutral,
            BuildStatus::Success => Conclusion::Success,
            BuildStatus::Failure => Conclusion::Neutral,
            BuildStatus::TimedOut => Conclusion::Neutral,
            BuildStatus::UnexpectedError { .. } => Conclusion::Neutral,
        }
    }
}

pub struct LegacyBuildResult {
    pub repo: Repo,
    pub pr: Pr,
    pub system: String,
    pub output: Vec<String>,
    pub attempt_id: String,
    pub request_id: String,
    pub status: BuildStatus,
    pub skipped_attrs: Option<Vec<String>>,
    pub attempted_attrs: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum V1Tag {
    V1,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum BuildResult {
    V1 {
        tag: V1Tag, // use serde once all enum variants have a tag
        repo: Repo,
        pr: Pr,
        system: String,
        output: Vec<String>,
        attempt_id: String,
        request_id: String,
        // removed success
        status: BuildStatus,
        skipped_attrs: Option<Vec<String>>,
        attempted_attrs: Option<Vec<String>>,
    },
    Legacy {
        repo: Repo,
        pr: Pr,
        system: String,
        output: Vec<String>,
        attempt_id: String,
        request_id: String,
        success: Option<bool>, // replaced by status
        status: Option<BuildStatus>,
        skipped_attrs: Option<Vec<String>>,
        attempted_attrs: Option<Vec<String>>,
    },
}

impl BuildResult {
    pub fn legacy(&self) -> LegacyBuildResult {
        // TODO: replace this with simpler structs for specific usecases, since
        // it's decouples the structs from serialization.  These can be changed
        // as long as we can translate all enum variants.
        match *self {
            BuildResult::Legacy {
                ref repo,
                ref pr,
                ref system,
                ref output,
                ref attempt_id,
                ref request_id,
                ref attempted_attrs,
                ref skipped_attrs,
                ..
            } => LegacyBuildResult {
                repo: repo.to_owned(),
                pr: pr.to_owned(),
                system: system.to_owned(),
                output: output.to_owned(),
                attempt_id: attempt_id.to_owned(),
                request_id: request_id.to_owned(),
                status: self.status(),
                attempted_attrs: attempted_attrs.to_owned(),
                skipped_attrs: skipped_attrs.to_owned(),
            },
            BuildResult::V1 {
                ref repo,
                ref pr,
                ref system,
                ref output,
                ref attempt_id,
                ref request_id,
                ref attempted_attrs,
                ref skipped_attrs,
                ..
            } => LegacyBuildResult {
                repo: repo.to_owned(),
                pr: pr.to_owned(),
                system: system.to_owned(),
                output: output.to_owned(),
                attempt_id: attempt_id.to_owned(),
                request_id: request_id.to_owned(),
                status: self.status(),
                attempted_attrs: attempted_attrs.to_owned(),
                skipped_attrs: skipped_attrs.to_owned(),
            },
        }
    }

    pub fn status(&self) -> BuildStatus {
        match *self {
            BuildResult::Legacy {
                ref status,
                ref success,
                ..
            } => status.to_owned().unwrap_or_else(|| {
                // Fallback for old format.
                match *success {
                    None => BuildStatus::Skipped,
                    Some(true) => BuildStatus::Success,
                    Some(false) => BuildStatus::Failure,
                }
            }),
            BuildResult::V1 { ref status, .. } => status.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn v1_serialization() {
        let input = r#"{"tag":"V1","repo":{"owner":"NixOS","name":"nixpkgs","full_name":"NixOS/nixpkgs","clone_url":"https://github.com/nixos/nixpkgs.git"},"pr":{"target_branch":"master","number":42,"head_sha":"0000000000000000000000000000000000000000"},"system":"x86_64-linux","output":["unpacking sources"],"attempt_id":"attempt-id-foo","request_id":"bogus-request-id","status":"Success","skipped_attrs":["AAAAAASomeThingsFailToEvaluate"],"attempted_attrs":["hello"]}"#;
        let result: BuildResult = serde_json::from_str(input).expect("result required");
        assert_eq!(result.status(), BuildStatus::Success);
        let output = serde_json::to_string(&result).expect("json required");
        assert_eq!(output, r#"{"tag":"V1","repo":{"owner":"NixOS","name":"nixpkgs","full_name":"NixOS/nixpkgs","clone_url":"https://github.com/nixos/nixpkgs.git"},"pr":{"target_branch":"master","number":42,"head_sha":"0000000000000000000000000000000000000000"},"system":"x86_64-linux","output":["unpacking sources"],"attempt_id":"attempt-id-foo","request_id":"bogus-request-id","status":"Success","skipped_attrs":["AAAAAASomeThingsFailToEvaluate"],"attempted_attrs":["hello"]}"#, "json of: {:?}", result);
    }

    #[test]
    fn legacy_serialization() {
        let input = r#"{"repo":{"owner":"NixOS","name":"nixpkgs","full_name":"NixOS/nixpkgs","clone_url":"https://github.com/nixos/nixpkgs.git"},"pr":{"target_branch":"master","number":42,"head_sha":"0000000000000000000000000000000000000000"},"system":"x86_64-linux","output":["unpacking sources"],"attempt_id":"attempt-id-foo","request_id":"bogus-request-id","success":true,"status":"Success","skipped_attrs":["AAAAAASomeThingsFailToEvaluate"],"attempted_attrs":["hello"]}"#;
        let result: BuildResult = serde_json::from_str(input).expect("result required");
        assert_eq!(result.status(), BuildStatus::Success);
        let output = serde_json::to_string(&result).expect("json required");
        assert_eq!(output, r#"{"repo":{"owner":"NixOS","name":"nixpkgs","full_name":"NixOS/nixpkgs","clone_url":"https://github.com/nixos/nixpkgs.git"},"pr":{"target_branch":"master","number":42,"head_sha":"0000000000000000000000000000000000000000"},"system":"x86_64-linux","output":["unpacking sources"],"attempt_id":"attempt-id-foo","request_id":"bogus-request-id","success":true,"status":"Success","skipped_attrs":["AAAAAASomeThingsFailToEvaluate"],"attempted_attrs":["hello"]}"#, "json of: {:?}", result);
    }

    #[test]
    fn legacy_none_serialization() {
        let input = r#"{"repo":{"owner":"NixOS","name":"nixpkgs","full_name":"NixOS/nixpkgs","clone_url":"https://github.com/nixos/nixpkgs.git"},"pr":{"target_branch":"master","number":42,"head_sha":"0000000000000000000000000000000000000000"},"system":"x86_64-linux","output":[],"attempt_id":"attempt-id-foo","request_id":"bogus-request-id"}"#;
        let result: BuildResult = serde_json::from_str(input).expect("result required");
        assert_eq!(result.status(), BuildStatus::Skipped);
        let output = serde_json::to_string(&result).expect("json required");
        assert_eq!(output, r#"{"repo":{"owner":"NixOS","name":"nixpkgs","full_name":"NixOS/nixpkgs","clone_url":"https://github.com/nixos/nixpkgs.git"},"pr":{"target_branch":"master","number":42,"head_sha":"0000000000000000000000000000000000000000"},"system":"x86_64-linux","output":[],"attempt_id":"attempt-id-foo","request_id":"bogus-request-id","success":null,"status":null,"skipped_attrs":null,"attempted_attrs":null}"#, "json of: {:?}", result);
    }

    #[test]
    fn legacy_no_status_serialization() {
        let input = r#"{"repo":{"owner":"NixOS","name":"nixpkgs","full_name":"NixOS/nixpkgs","clone_url":"https://github.com/nixos/nixpkgs.git"},"pr":{"target_branch":"master","number":42,"head_sha":"0000000000000000000000000000000000000000"},"system":"x86_64-linux","output":["unpacking sources"],"attempt_id":"attempt-id-foo","request_id":"bogus-request-id","success":true,"status":null,"skipped_attrs":["AAAAAASomeThingsFailToEvaluate"],"attempted_attrs":["hello"]}"#;
        let result: BuildResult = serde_json::from_str(input).expect("result required");
        assert_eq!(result.status(), BuildStatus::Success);
        let output = serde_json::to_string(&result).expect("json required");
        assert_eq!(output, r#"{"repo":{"owner":"NixOS","name":"nixpkgs","full_name":"NixOS/nixpkgs","clone_url":"https://github.com/nixos/nixpkgs.git"},"pr":{"target_branch":"master","number":42,"head_sha":"0000000000000000000000000000000000000000"},"system":"x86_64-linux","output":["unpacking sources"],"attempt_id":"attempt-id-foo","request_id":"bogus-request-id","success":true,"status":null,"skipped_attrs":["AAAAAASomeThingsFailToEvaluate"],"attempted_attrs":["hello"]}"#, "json of: {:?}", result);
    }
}
