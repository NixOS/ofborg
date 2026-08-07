use crate::message::{Pr, Repo};

/// A derivation the evaluator resolved from a single attribute.
///
/// The attribute name is carried alongside the store path because it is what
/// ends up in the GitHub check run's name; a bare drv path only gives us the
/// derivation name, which is not the same thing (`hello` vs `hello-2.12.1`).
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct HydraEvalDrv {
    pub attr: String,
    pub drv_path: String,
}

fn unknown_system() -> String {
    "unknown".to_owned()
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct HydraEvalJob {
    pub repo: Repo,
    pub pr: Pr,
    /// The system these derivations were instantiated for. The evaluator pins
    /// `--argstr system` to a single value, so one job covers exactly one system.
    #[serde(default = "unknown_system")]
    pub system: String,
    #[serde(default)]
    pub drvs: Vec<HydraEvalDrv>,
    /// Pre-`drvs` wire format. Never written by current producers, only read so
    /// that jobs already sitting in `hydra-eval-jobs` at deploy time still
    /// deserialize. Safe to delete once the queue has drained.
    #[serde(default, skip_serializing)]
    pub drv_paths: Vec<String>,
    pub request_id: String,
    pub jobset_id: i32,
}

impl HydraEvalJob {
    /// The derivations to build, tolerating the legacy `drv_paths` encoding.
    pub fn all_drvs(&self) -> Vec<HydraEvalDrv> {
        if !self.drvs.is_empty() {
            return self.drvs.clone();
        }

        self.drv_paths
            .iter()
            .map(|drv_path| HydraEvalDrv {
                attr: drv_name(drv_path),
                drv_path: drv_path.clone(),
            })
            .collect()
    }
}

/// `/nix/store/<hash>-hello-2.12.1.drv` -> `hello-2.12.1`
fn drv_name(drv_path: &str) -> String {
    let base = drv_path.rsplit('/').next().unwrap_or(drv_path);
    let base = base.strip_suffix(".drv").unwrap_or(base);

    // Store path basenames are `<32 char hash>-<name>`.
    match base.split_once('-') {
        Some((_hash, name)) => name.to_owned(),
        None => base.to_owned(),
    }
}

pub fn from(data: &[u8]) -> Result<HydraEvalJob, serde_json::error::Error> {
    serde_json::from_slice(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn job(json: &str) -> HydraEvalJob {
        from(json.as_bytes()).expect("should deserialize")
    }

    const REPO_AND_PR: &str = r#""repo":{"owner":"NixOS","name":"nixpkgs","full_name":"NixOS/nixpkgs","clone_url":"https://github.com/nixos/nixpkgs.git"},"pr":{"target_branch":"master","number":42,"head_sha":"abc123"},"system":"x86_64-linux""#;

    #[test]
    fn reads_the_current_format() {
        let job = job(&format!(
            r#"{{{REPO_AND_PR},"drvs":[{{"attr":"hello","drv_path":"/nix/store/aaa-hello-2.12.1.drv"}}],"request_id":"r","jobset_id":1}}"#
        ));

        let drvs = job.all_drvs();
        assert_eq!(drvs.len(), 1);
        assert_eq!(drvs[0].attr, "hello");
        assert_eq!(drvs[0].drv_path, "/nix/store/aaa-hello-2.12.1.drv");
    }

    #[test]
    fn reads_the_legacy_drv_paths_format() {
        let job = job(&format!(
            r#"{{{REPO_AND_PR},"drv_paths":["/nix/store/aaa-hello-2.12.1.drv"],"request_id":"r","jobset_id":1}}"#
        ));

        let drvs = job.all_drvs();
        assert_eq!(drvs.len(), 1);
        assert_eq!(drvs[0].attr, "hello-2.12.1");
        assert_eq!(drvs[0].drv_path, "/nix/store/aaa-hello-2.12.1.drv");
    }

    #[test]
    fn legacy_field_is_not_written_back_out() {
        let job = job(&format!(
            r#"{{{REPO_AND_PR},"drv_paths":["/nix/store/aaa-hello.drv"],"request_id":"r","jobset_id":1}}"#
        ));

        let out = serde_json::to_string(&job).expect("should serialize");
        assert!(!out.contains("drv_paths"), "serialized as: {out}");
    }

    #[test]
    fn system_defaults_when_absent() {
        let job = job(
            r#"{"repo":{"owner":"NixOS","name":"nixpkgs","full_name":"NixOS/nixpkgs","clone_url":"https://github.com/nixos/nixpkgs.git"},"pr":{"target_branch":"master","number":42,"head_sha":"abc123"},"drv_paths":["/nix/store/aaa-hello.drv"],"request_id":"r","jobset_id":1}"#,
        );

        assert_eq!(job.system, "unknown");
    }

    #[test]
    fn drv_name_strips_hash_and_suffix() {
        assert_eq!(drv_name("/nix/store/aaa-hello-2.12.1.drv"), "hello-2.12.1");
        assert_eq!(drv_name("aaa-hello.drv"), "hello");
        assert_eq!(drv_name("weird"), "weird");
    }
}
