use crate::config::GithubAppVendingMachine;
use crate::message::buildjob::{BuildJob, QueuedBuildJobs};
use crate::message::buildresult::{BuildResult, BuildStatus, LegacyBuildResult};
use crate::message::Repo;
use crate::worker;

use chrono::{DateTime, Utc};
use hubcaps::checks::{CheckRunOptions, CheckRunState, Conclusion, Output};
use tracing::{debug, debug_span, info, warn};

pub struct GitHubCommentPoster {
    github_vend: GithubAppVendingMachine,
}

impl GitHubCommentPoster {
    pub fn new(github_vend: GithubAppVendingMachine) -> GitHubCommentPoster {
        GitHubCommentPoster { github_vend }
    }
}

pub enum PostableEvent {
    BuildQueued(QueuedBuildJobs),
    BuildFinished(BuildResult),
}

impl PostableEvent {
    fn from(bytes: &[u8]) -> Result<PostableEvent, String> {
        match serde_json::from_slice::<QueuedBuildJobs>(bytes) {
            Ok(e) => Ok(PostableEvent::BuildQueued(e)),
            Err(_) => match serde_json::from_slice::<BuildResult>(bytes) {
                Ok(e) => Ok(PostableEvent::BuildFinished(e)),
                Err(e) => Err(format!(
                    "Failed to deserialize PostableEvent: {:?}, err: {:}",
                    String::from_utf8_lossy(bytes),
                    e
                )),
            },
        }
    }
}

impl worker::SimpleWorker for GitHubCommentPoster {
    type J = PostableEvent;

    fn msg_to_job(&mut self, _: &str, _: &Option<String>, body: &[u8]) -> Result<Self::J, String> {
        PostableEvent::from(body)
    }

    fn consumer(&mut self, job: &PostableEvent) -> worker::Actions {
        let mut checks: Vec<CheckRunOptions> = vec![];
        let repo: Repo;

        let pr = match job {
            PostableEvent::BuildQueued(queued_job) => {
                repo = queued_job.job.repo.clone();
                for architecture in queued_job.architectures.iter() {
                    checks.push(job_to_check(&queued_job.job, architecture, Utc::now()));
                }
                queued_job.job.pr.to_owned()
            }
            PostableEvent::BuildFinished(finished_job) => {
                let result = finished_job.legacy();
                repo = result.repo.clone();
                checks.push(result_to_check(&result, Utc::now()));
                finished_job.pr()
            }
        };

        let span = debug_span!("job", pr = ?pr.number);
        let _enter = span.enter();

        for check in checks {
            info!(
                "check {:?} {} {}",
                check.status,
                check.name,
                check.details_url.as_ref().unwrap_or(&String::from("-"))
            );
            debug!("{:?}", check);

            let check_create_attempt = async_std::task::block_on(
                self.github_vend
                    .for_repo(&repo.owner, &repo.name)
                    .unwrap()
                    .repo(repo.owner.clone(), repo.name.clone())
                    .checkruns()
                    .create(&check),
            );

            match check_create_attempt {
                Ok(_) => info!("Successfully sent."),
                Err(err) => warn!("Failed to send check {:?}", err),
            }
        }

        vec![worker::Action::Ack]
    }
}

fn job_to_check(job: &BuildJob, architecture: &str, timestamp: DateTime<Utc>) -> CheckRunOptions {
    let mut all_attrs: Vec<String> = job.attrs.clone();
    all_attrs.sort();

    if all_attrs.is_empty() {
        all_attrs = vec![String::from("(unknown attributes)")];
    }

    CheckRunOptions {
        name: format!("{} on {architecture}", all_attrs.join(", ")),
        actions: None,
        completed_at: None,
        started_at: Some(timestamp.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)),
        conclusion: None,
        details_url: Some(format!(
            "https://logs.ofborg.org/?key={}/{}.{}",
            &job.repo.owner.to_lowercase(),
            &job.repo.name.to_lowercase(),
            job.pr.number,
        )),
        external_id: None,
        head_sha: job.pr.head_sha.clone(),
        output: None,
        status: Some(CheckRunState::Queued),
    }
}

fn result_to_check(result: &LegacyBuildResult, timestamp: DateTime<Utc>) -> CheckRunOptions {
    let mut all_attrs: Vec<String> =
        vec![result.attempted_attrs.clone(), result.skipped_attrs.clone()]
            .into_iter()
            .map(|opt| opt.unwrap_or_else(|| vec![]))
            .flat_map(|list| list.into_iter())
            .collect();
    all_attrs.sort();

    if all_attrs.is_empty() {
        all_attrs = vec![String::from("(unknown attributes)")];
    }

    let conclusion: Conclusion = result.status.clone().into();

    let mut summary: Vec<String> = vec![];
    if let Some(ref attempted) = result.attempted_attrs {
        summary.extend(list_segment("Attempted", attempted));
    }

    if result.status == BuildStatus::TimedOut {
        summary.push(String::from("Build timed out."));
    }

    if let Some(ref skipped) = result.skipped_attrs {
        summary.extend(list_segment(
            &format!(
                "The following builds were skipped because they don't evaluate on {}",
                result.system
            ),
            skipped,
        ));
    }

    // Allow the clippy violation for improved readability
    #[allow(clippy::vec_init_then_push)]
    let text: String = if !result.output.is_empty() {
        let mut reply: Vec<String> = vec![];

        reply.push("## Partial log".to_owned());
        reply.push("".to_owned());
        reply.push("```".to_owned());
        reply.extend(result.output.clone());
        reply.push("```".to_owned());

        reply.join("\n")
    } else {
        String::from("No partial log is available.")
    };

    CheckRunOptions {
        name: format!("{} on {}", all_attrs.join(", "), result.system),
        actions: None,
        completed_at: Some(timestamp.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)),
        started_at: None,
        conclusion: Some(conclusion),
        details_url: Some(format!(
            "https://logs.ofborg.org/?key={}/{}.{}&attempt_id={}",
            &result.repo.owner.to_lowercase(),
            &result.repo.name.to_lowercase(),
            result.pr.number,
            result.attempt_id,
        )),
        external_id: Some(result.attempt_id.clone()),
        head_sha: result.pr.head_sha.clone(),

        output: Some(Output {
            annotations: None,
            images: None,
            summary: summary.join("\n"),
            text: Some(text),
            title: result.status.clone().into(),
        }),
        status: Some(CheckRunState::Completed),
    }
}

fn list_segment(name: &str, things: &[String]) -> Vec<String> {
    let mut reply: Vec<String> = vec![];

    if !things.is_empty() {
        reply.push(format!("{name}: {}", things.join(", ")));
        reply.push("".to_owned());
    }

    reply
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::{Pr, Repo};
    use chrono::TimeZone;

    #[test]
    pub fn test_queued_build() {
        let job = BuildJob {
            repo: Repo {
                clone_url: "https://github.com/nixos/nixpkgs.git".to_owned(),
                full_name: "NixOS/nixpkgs".to_owned(),
                owner: "NixOS".to_owned(),
                name: "nixpkgs".to_owned(),
            },
            pr: Pr {
                head_sha: "abc123".to_owned(),
                number: 2345,
                target_branch: Some("master".to_owned()),
            },
            logs: None,
            statusreport: None,
            subset: None,

            request_id: "bogus-request-id".to_owned(),
            attrs: vec!["foo".to_owned(), "bar".to_owned()],
        };

        let timestamp = Utc.with_ymd_and_hms(2023, 4, 20, 13, 37, 42).unwrap();
        assert_eq!(
            job_to_check(&job, "x86_64-linux", timestamp),
            CheckRunOptions {
                name: "bar, foo on x86_64-linux".to_string(),
                actions: None,
                started_at: Some("2023-04-20T13:37:42Z".to_string()),
                completed_at: None,
                status: Some(CheckRunState::Queued),
                conclusion: None,
                details_url: Some("https://logs.ofborg.org/?key=nixos/nixpkgs.2345".to_string()),
                external_id: None,
                head_sha: "abc123".to_string(),
                output: None,
            }
        );
    }

    #[test]
    pub fn test_check_passing_build() {
        let result = LegacyBuildResult {
            repo: Repo {
                clone_url: "https://github.com/nixos/nixpkgs.git".to_owned(),
                full_name: "NixOS/nixpkgs".to_owned(),
                owner: "NixOS".to_owned(),
                name: "nixpkgs".to_owned(),
            },
            pr: Pr {
                head_sha: "abc123".to_owned(),
                number: 2345,
                target_branch: Some("master".to_owned()),
            },
            output: vec![
                "make[2]: Entering directory '/private/tmp/nix-build-gdb-8.1.drv-0/gdb-8.1/readline'".to_owned(),
                "make[2]: Nothing to be done for 'install'.".to_owned(),
                "make[2]: Leaving directory '/private/tmp/nix-build-gdb-8.1.drv-0/gdb-8.1/readline'".to_owned(),
                "make[1]: Nothing to be done for 'install-target'.".to_owned(),
                "make[1]: Leaving directory '/private/tmp/nix-build-gdb-8.1.drv-0/gdb-8.1'".to_owned(),
                "removed '/nix/store/pcja75y9isdvgz5i00pkrpif9rxzxc29-gdb-8.1/share/info/bfd.info'".to_owned(),
                "post-installation fixup".to_owned(),
                "strip is /nix/store/5a88zk3jgimdmzg8rfhvm93kxib3njf9-cctools-binutils-darwin/bin/strip".to_owned(),
                "patching script interpreter paths in /nix/store/pcja75y9isdvgz5i00pkrpif9rxzxc29-gdb-8.1".to_owned(),
                "/nix/store/pcja75y9isdvgz5i00pkrpif9rxzxc29-gdb-8.1".to_owned(),
            ],
            attempt_id: "neatattemptid".to_owned(),
            request_id: "bogus-request-id".to_owned(),
            system: "x86_64-linux".to_owned(),
            attempted_attrs: Some(vec!["foo".to_owned()]),
            skipped_attrs: Some(vec!["bar".to_owned()]),
            status: BuildStatus::Success,
        };

        let timestamp = Utc.with_ymd_and_hms(2023, 4, 20, 13, 37, 42).unwrap();

        assert_eq!(
            result_to_check(&result, timestamp),
            CheckRunOptions {
                name: "bar, foo on x86_64-linux".to_string(),
                actions: None,
                started_at: None,
                completed_at: Some("2023-04-20T13:37:42Z".to_string()),
                status: Some(CheckRunState::Completed),
                conclusion: Some(Conclusion::Success),
                details_url: Some(
                    "https://logs.ofborg.org/?key=nixos/nixpkgs.2345&attempt_id=neatattemptid"
                        .to_string()
                ),
                external_id: Some("neatattemptid".to_string()),
                head_sha: "abc123".to_string(),
                output: Some(Output {
                    title: "Success".to_string(),
                    summary: "Attempted: foo

The following builds were skipped because they don't evaluate on x86_64-linux: bar
"
                    .to_string(),
                    text: Some(
                        "## Partial log

```
make[2]: Entering directory '/private/tmp/nix-build-gdb-8.1.drv-0/gdb-8.1/readline'
make[2]: Nothing to be done for 'install'.
make[2]: Leaving directory '/private/tmp/nix-build-gdb-8.1.drv-0/gdb-8.1/readline'
make[1]: Nothing to be done for 'install-target'.
make[1]: Leaving directory '/private/tmp/nix-build-gdb-8.1.drv-0/gdb-8.1'
removed '/nix/store/pcja75y9isdvgz5i00pkrpif9rxzxc29-gdb-8.1/share/info/bfd.info'
post-installation fixup
strip is /nix/store/5a88zk3jgimdmzg8rfhvm93kxib3njf9-cctools-binutils-darwin/bin/strip
patching script interpreter paths in /nix/store/pcja75y9isdvgz5i00pkrpif9rxzxc29-gdb-8.1
/nix/store/pcja75y9isdvgz5i00pkrpif9rxzxc29-gdb-8.1
```"
                        .to_string()
                    ),
                    annotations: None,
                    images: None,
                })
            }
        );
    }

    #[test]
    pub fn test_check_failing_build() {
        let result = LegacyBuildResult {
            repo: Repo {
                clone_url: "https://github.com/nixos/nixpkgs.git".to_owned(),
                full_name: "NixOS/nixpkgs".to_owned(),
                owner: "NixOS".to_owned(),
                name: "nixpkgs".to_owned(),
            },
            pr: Pr {
                head_sha: "abc123".to_owned(),
                number: 2345,
                target_branch: Some("master".to_owned()),
            },
            output: vec![
                "make[2]: Entering directory '/private/tmp/nix-build-gdb-8.1.drv-0/gdb-8.1/readline'".to_owned(),
                "make[2]: Nothing to be done for 'install'.".to_owned(),
                "make[2]: Leaving directory '/private/tmp/nix-build-gdb-8.1.drv-0/gdb-8.1/readline'".to_owned(),
                "make[1]: Nothing to be done for 'install-target'.".to_owned(),
                "make[1]: Leaving directory '/private/tmp/nix-build-gdb-8.1.drv-0/gdb-8.1'".to_owned(),
                "removed '/nix/store/pcja75y9isdvgz5i00pkrpif9rxzxc29-gdb-8.1/share/info/bfd.info'".to_owned(),
                "post-installation fixup".to_owned(),
                "strip is /nix/store/5a88zk3jgimdmzg8rfhvm93kxib3njf9-cctools-binutils-darwin/bin/strip".to_owned(),
                "patching script interpreter paths in /nix/store/pcja75y9isdvgz5i00pkrpif9rxzxc29-gdb-8.1".to_owned(),
                "/nix/store/pcja75y9isdvgz5i00pkrpif9rxzxc29-gdb-8.1".to_owned(),
            ],
            attempt_id: "neatattemptid".to_owned(),
            request_id: "bogus-request-id".to_owned(),
            system: "x86_64-linux".to_owned(),
            attempted_attrs: Some(vec!["foo".to_owned()]),
            skipped_attrs: None,
            status: BuildStatus::Failure,
        };

        let timestamp = Utc.with_ymd_and_hms(2023, 4, 20, 13, 37, 42).unwrap();

        assert_eq!(
            result_to_check(&result, timestamp),
            CheckRunOptions {
                name: "foo on x86_64-linux".to_string(),
                actions: None,
                started_at: None,
                completed_at: Some("2023-04-20T13:37:42Z".to_string()),
                status: Some(CheckRunState::Completed),
                conclusion: Some(Conclusion::Neutral),
                details_url: Some(
                    "https://logs.ofborg.org/?key=nixos/nixpkgs.2345&attempt_id=neatattemptid"
                        .to_string()
                ),
                external_id: Some("neatattemptid".to_string()),
                head_sha: "abc123".to_string(),
                output: Some(Output {
                    title: "Failure".to_string(),
                    summary: "Attempted: foo
"
                    .to_string(),
                    text: Some(
                        "## Partial log

```
make[2]: Entering directory '/private/tmp/nix-build-gdb-8.1.drv-0/gdb-8.1/readline'
make[2]: Nothing to be done for 'install'.
make[2]: Leaving directory '/private/tmp/nix-build-gdb-8.1.drv-0/gdb-8.1/readline'
make[1]: Nothing to be done for 'install-target'.
make[1]: Leaving directory '/private/tmp/nix-build-gdb-8.1.drv-0/gdb-8.1'
removed '/nix/store/pcja75y9isdvgz5i00pkrpif9rxzxc29-gdb-8.1/share/info/bfd.info'
post-installation fixup
strip is /nix/store/5a88zk3jgimdmzg8rfhvm93kxib3njf9-cctools-binutils-darwin/bin/strip
patching script interpreter paths in /nix/store/pcja75y9isdvgz5i00pkrpif9rxzxc29-gdb-8.1
/nix/store/pcja75y9isdvgz5i00pkrpif9rxzxc29-gdb-8.1
```"
                        .to_string()
                    ),
                    annotations: None,
                    images: None,
                })
            }
        );
    }

    #[test]
    pub fn test_check_timedout_build() {
        let result = LegacyBuildResult {
            repo: Repo {
                clone_url: "https://github.com/nixos/nixpkgs.git".to_owned(),
                full_name: "NixOS/nixpkgs".to_owned(),
                owner: "NixOS".to_owned(),
                name: "nixpkgs".to_owned(),
            },
            pr: Pr {
                head_sha: "abc123".to_owned(),
                number: 2345,
                target_branch: Some("master".to_owned()),
            },
            output: vec![
                "make[2]: Entering directory '/private/tmp/nix-build-gdb-8.1.drv-0/gdb-8.1/readline'".to_owned(),
                "make[2]: Nothing to be done for 'install'.".to_owned(),
                "make[2]: Leaving directory '/private/tmp/nix-build-gdb-8.1.drv-0/gdb-8.1/readline'".to_owned(),
                "make[1]: Nothing to be done for 'install-target'.".to_owned(),
                "make[1]: Leaving directory '/private/tmp/nix-build-gdb-8.1.drv-0/gdb-8.1'".to_owned(),
                "removed '/nix/store/pcja75y9isdvgz5i00pkrpif9rxzxc29-gdb-8.1/share/info/bfd.info'".to_owned(),
                "post-installation fixup".to_owned(),
                "building of '/nix/store/l1limh50lx2cx45yb2gqpv7k8xl1mik2-gdb-8.1.drv' timed out after 1 seconds".to_owned(),
                "error: build of '/nix/store/l1limh50lx2cx45yb2gqpv7k8xl1mik2-gdb-8.1.drv' failed".to_owned(),
            ],
            attempt_id: "neatattemptid".to_owned(),
            request_id: "bogus-request-id".to_owned(),
            system: "x86_64-linux".to_owned(),
            attempted_attrs: Some(vec!["foo".to_owned()]),
            skipped_attrs: None,
            status: BuildStatus::TimedOut,
        };

        let timestamp = Utc.with_ymd_and_hms(2023, 4, 20, 13, 37, 42).unwrap();

        assert_eq!(
            result_to_check(&result, timestamp),
            CheckRunOptions {
                name: "foo on x86_64-linux".to_string(),
                actions: None,
                started_at: None,
                completed_at: Some("2023-04-20T13:37:42Z".to_string()),
                status: Some(CheckRunState::Completed),
                conclusion: Some(Conclusion::Neutral),
                details_url: Some(
                    "https://logs.ofborg.org/?key=nixos/nixpkgs.2345&attempt_id=neatattemptid"
                        .to_string()
                ),
                external_id: Some("neatattemptid".to_string()),
                head_sha: "abc123".to_string(),
                output: Some(Output {
                    title: "Timed out, unknown build status".to_string(),
                    summary: "Attempted: foo

Build timed out."
                        .to_string(),
                    text: Some(
                        "## Partial log

```
make[2]: Entering directory '/private/tmp/nix-build-gdb-8.1.drv-0/gdb-8.1/readline'
make[2]: Nothing to be done for 'install'.
make[2]: Leaving directory '/private/tmp/nix-build-gdb-8.1.drv-0/gdb-8.1/readline'
make[1]: Nothing to be done for 'install-target'.
make[1]: Leaving directory '/private/tmp/nix-build-gdb-8.1.drv-0/gdb-8.1'
removed '/nix/store/pcja75y9isdvgz5i00pkrpif9rxzxc29-gdb-8.1/share/info/bfd.info'
post-installation fixup
building of '/nix/store/l1limh50lx2cx45yb2gqpv7k8xl1mik2-gdb-8.1.drv' timed out after 1 seconds
error: build of '/nix/store/l1limh50lx2cx45yb2gqpv7k8xl1mik2-gdb-8.1.drv' failed
```"
                        .to_string()
                    ),
                    annotations: None,
                    images: None,
                })
            }
        );
    }

    #[test]
    pub fn test_check_passing_build_unspecified_attributes() {
        let result = LegacyBuildResult {
            repo: Repo {
                clone_url: "https://github.com/nixos/nixpkgs.git".to_owned(),
                full_name: "NixOS/nixpkgs".to_owned(),
                owner: "NixOS".to_owned(),
                name: "nixpkgs".to_owned(),
            },
            pr: Pr {
                head_sha: "abc123".to_owned(),
                number: 2345,
                target_branch: Some("master".to_owned()),
            },
            output: vec![
                "make[2]: Entering directory '/private/tmp/nix-build-gdb-8.1.drv-0/gdb-8.1/readline'".to_owned(),
                "make[2]: Nothing to be done for 'install'.".to_owned(),
                "make[2]: Leaving directory '/private/tmp/nix-build-gdb-8.1.drv-0/gdb-8.1/readline'".to_owned(),
                "make[1]: Nothing to be done for 'install-target'.".to_owned(),
                "make[1]: Leaving directory '/private/tmp/nix-build-gdb-8.1.drv-0/gdb-8.1'".to_owned(),
                "removed '/nix/store/pcja75y9isdvgz5i00pkrpif9rxzxc29-gdb-8.1/share/info/bfd.info'".to_owned(),
                "post-installation fixup".to_owned(),
                "strip is /nix/store/5a88zk3jgimdmzg8rfhvm93kxib3njf9-cctools-binutils-darwin/bin/strip".to_owned(),
                "patching script interpreter paths in /nix/store/pcja75y9isdvgz5i00pkrpif9rxzxc29-gdb-8.1".to_owned(),
                "/nix/store/pcja75y9isdvgz5i00pkrpif9rxzxc29-gdb-8.1".to_owned(),
            ],
            attempt_id: "neatattemptid".to_owned(),
            request_id: "bogus-request-id".to_owned(),
            system: "x86_64-linux".to_owned(),
            attempted_attrs: None,
            skipped_attrs: None,
            status: BuildStatus::Success,
        };

        let timestamp = Utc.with_ymd_and_hms(2023, 4, 20, 13, 37, 42).unwrap();

        assert_eq!(
            result_to_check(&result, timestamp),
            CheckRunOptions {
                name: "(unknown attributes) on x86_64-linux".to_string(),
                actions: None,
                started_at: None,
                completed_at: Some("2023-04-20T13:37:42Z".to_string()),
                status: Some(CheckRunState::Completed),
                conclusion: Some(Conclusion::Success),
                details_url: Some(
                    "https://logs.ofborg.org/?key=nixos/nixpkgs.2345&attempt_id=neatattemptid"
                        .to_string()
                ),
                external_id: Some("neatattemptid".to_string()),
                head_sha: "abc123".to_string(),
                output: Some(Output {
                    title: "Success".to_string(),
                    summary: "".to_string(),
                    text: Some(
                        "## Partial log

```
make[2]: Entering directory '/private/tmp/nix-build-gdb-8.1.drv-0/gdb-8.1/readline'
make[2]: Nothing to be done for 'install'.
make[2]: Leaving directory '/private/tmp/nix-build-gdb-8.1.drv-0/gdb-8.1/readline'
make[1]: Nothing to be done for 'install-target'.
make[1]: Leaving directory '/private/tmp/nix-build-gdb-8.1.drv-0/gdb-8.1'
removed '/nix/store/pcja75y9isdvgz5i00pkrpif9rxzxc29-gdb-8.1/share/info/bfd.info'
post-installation fixup
strip is /nix/store/5a88zk3jgimdmzg8rfhvm93kxib3njf9-cctools-binutils-darwin/bin/strip
patching script interpreter paths in /nix/store/pcja75y9isdvgz5i00pkrpif9rxzxc29-gdb-8.1
/nix/store/pcja75y9isdvgz5i00pkrpif9rxzxc29-gdb-8.1
```"
                        .to_string()
                    ),
                    annotations: None,
                    images: None,
                })
            }
        );
    }

    #[test]
    pub fn test_check_failing_build_unspecified_attributes() {
        let result = LegacyBuildResult {
            repo: Repo {
                clone_url: "https://github.com/nixos/nixpkgs.git".to_owned(),
                full_name: "NixOS/nixpkgs".to_owned(),
                owner: "NixOS".to_owned(),
                name: "nixpkgs".to_owned(),
            },
            pr: Pr {
                head_sha: "abc123".to_owned(),
                number: 2345,
                target_branch: Some("master".to_owned()),
            },
            output: vec![
                "make[2]: Entering directory '/private/tmp/nix-build-gdb-8.1.drv-0/gdb-8.1/readline'".to_owned(),
                "make[2]: Nothing to be done for 'install'.".to_owned(),
                "make[2]: Leaving directory '/private/tmp/nix-build-gdb-8.1.drv-0/gdb-8.1/readline'".to_owned(),
                "make[1]: Nothing to be done for 'install-target'.".to_owned(),
                "make[1]: Leaving directory '/private/tmp/nix-build-gdb-8.1.drv-0/gdb-8.1'".to_owned(),
                "removed '/nix/store/pcja75y9isdvgz5i00pkrpif9rxzxc29-gdb-8.1/share/info/bfd.info'".to_owned(),
                "post-installation fixup".to_owned(),
                "strip is /nix/store/5a88zk3jgimdmzg8rfhvm93kxib3njf9-cctools-binutils-darwin/bin/strip".to_owned(),
                "patching script interpreter paths in /nix/store/pcja75y9isdvgz5i00pkrpif9rxzxc29-gdb-8.1".to_owned(),
                "/nix/store/pcja75y9isdvgz5i00pkrpif9rxzxc29-gdb-8.1".to_owned(),
            ],
            attempt_id: "neatattemptid".to_owned(),
            request_id: "bogus-request-id".to_owned(),
            system: "x86_64-linux".to_owned(),
            attempted_attrs: None,
            skipped_attrs: None,
            status: BuildStatus::Failure,
        };

        let timestamp = Utc.with_ymd_and_hms(2023, 4, 20, 13, 37, 42).unwrap();

        assert_eq!(
            result_to_check(&result, timestamp),
            CheckRunOptions {
                name: "(unknown attributes) on x86_64-linux".to_string(),
                actions: None,
                started_at: None,
                completed_at: Some("2023-04-20T13:37:42Z".to_string()),
                status: Some(CheckRunState::Completed),
                conclusion: Some(Conclusion::Neutral),
                details_url: Some(
                    "https://logs.ofborg.org/?key=nixos/nixpkgs.2345&attempt_id=neatattemptid"
                        .to_string()
                ),
                external_id: Some("neatattemptid".to_string()),
                head_sha: "abc123".to_string(),
                output: Some(Output {
                    title: "Failure".to_string(),
                    summary: "".to_string(),
                    text: Some(
                        "## Partial log

```
make[2]: Entering directory '/private/tmp/nix-build-gdb-8.1.drv-0/gdb-8.1/readline'
make[2]: Nothing to be done for 'install'.
make[2]: Leaving directory '/private/tmp/nix-build-gdb-8.1.drv-0/gdb-8.1/readline'
make[1]: Nothing to be done for 'install-target'.
make[1]: Leaving directory '/private/tmp/nix-build-gdb-8.1.drv-0/gdb-8.1'
removed '/nix/store/pcja75y9isdvgz5i00pkrpif9rxzxc29-gdb-8.1/share/info/bfd.info'
post-installation fixup
strip is /nix/store/5a88zk3jgimdmzg8rfhvm93kxib3njf9-cctools-binutils-darwin/bin/strip
patching script interpreter paths in /nix/store/pcja75y9isdvgz5i00pkrpif9rxzxc29-gdb-8.1
/nix/store/pcja75y9isdvgz5i00pkrpif9rxzxc29-gdb-8.1
```"
                        .to_string()
                    ),
                    annotations: None,
                    images: None,
                })
            }
        );
    }

    #[test]
    pub fn test_check_no_attempt() {
        let result = LegacyBuildResult {
            repo: Repo {
                clone_url: "https://github.com/nixos/nixpkgs.git".to_owned(),
                full_name: "NixOS/nixpkgs".to_owned(),
                owner: "NixOS".to_owned(),
                name: "nixpkgs".to_owned(),
            },
            pr: Pr {
                head_sha: "abc123".to_owned(),
                number: 2345,
                target_branch: Some("master".to_owned()),
            },
            output: vec!["foo".to_owned()],
            attempt_id: "neatattemptid".to_owned(),
            request_id: "bogus-request-id".to_owned(),
            system: "x86_64-linux".to_owned(),
            attempted_attrs: None,
            skipped_attrs: Some(vec!["not-attempted".to_owned()]),
            status: BuildStatus::Skipped,
        };

        let timestamp = Utc.with_ymd_and_hms(2023, 4, 20, 13, 37, 42).unwrap();

        assert_eq!(
            result_to_check(&result, timestamp),
            CheckRunOptions {
                name: "not-attempted on x86_64-linux".to_string(),
                actions: None,
                started_at: None,
                completed_at: Some("2023-04-20T13:37:42Z".to_string()),
                status: Some(CheckRunState::Completed),
                conclusion: Some(Conclusion::Skipped),
                details_url: Some("https://logs.ofborg.org/?key=nixos/nixpkgs.2345&attempt_id=neatattemptid".to_string()),
                external_id: Some("neatattemptid".to_string()),
                head_sha: "abc123".to_string(),
                output: Some(Output {
                    title: "No attempt".to_string(),
                    summary: "The following builds were skipped because they don\'t evaluate on x86_64-linux: not-attempted
".to_string(),
                    text: Some("## Partial log

```
foo
```".to_string()),
                    annotations: None,
                    images: None,
                })
            }
        );
    }

    #[test]
    pub fn test_check_no_attempt_no_log() {
        let result = LegacyBuildResult {
            repo: Repo {
                clone_url: "https://github.com/nixos/nixpkgs.git".to_owned(),
                full_name: "NixOS/nixpkgs".to_owned(),
                owner: "NixOS".to_owned(),
                name: "nixpkgs".to_owned(),
            },
            pr: Pr {
                head_sha: "abc123".to_owned(),
                number: 2345,
                target_branch: Some("master".to_owned()),
            },
            output: vec![],
            attempt_id: "neatattemptid".to_owned(),
            request_id: "bogus-request-id".to_owned(),
            system: "x86_64-linux".to_owned(),
            attempted_attrs: None,
            skipped_attrs: Some(vec!["not-attempted".to_owned()]),
            status: BuildStatus::Skipped,
        };

        let timestamp = Utc.with_ymd_and_hms(2023, 4, 20, 13, 37, 42).unwrap();

        assert_eq!(
            result_to_check(&result, timestamp),
            CheckRunOptions {
                name: "not-attempted on x86_64-linux".to_string(),
                actions: None,
                started_at: None,
                completed_at: Some("2023-04-20T13:37:42Z".to_string()),
                status: Some(CheckRunState::Completed),
                conclusion: Some(Conclusion::Skipped),
                details_url: Some("https://logs.ofborg.org/?key=nixos/nixpkgs.2345&attempt_id=neatattemptid".to_string()),
                external_id: Some("neatattemptid".to_string()),
                head_sha: "abc123".to_string(),
                output: Some(Output {
                    title: "No attempt".to_string(),
                    summary: "The following builds were skipped because they don\'t evaluate on x86_64-linux: not-attempted
".to_string(),
                    text: Some("No partial log is available.".to_string()),
                    annotations: None,
                    images: None,
                })
            }
        );
    }
}
