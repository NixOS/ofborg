extern crate amqp;
extern crate env_logger;

use serde_json;

use hubcaps;
use hubcaps::checks::{CheckRunOptions, Output, Conclusion, CheckRunState};
use ofborg::message::buildresult::{BuildStatus, BuildResult, LegacyBuildResult};
use ofborg::worker;
use amqp::protocol::basic::{Deliver, BasicProperties};
use chrono::{DateTime, Utc};


pub struct GitHubCommentPoster {
    github: hubcaps::Github,
}

impl GitHubCommentPoster {
    pub fn new(github: hubcaps::Github) -> GitHubCommentPoster {
        GitHubCommentPoster { github }
    }
}

impl worker::SimpleWorker for GitHubCommentPoster {
    type J = BuildResult;

    fn msg_to_job(
        &mut self,
        _: &Deliver,
        _: &BasicProperties,
        body: &Vec<u8>,
    ) -> Result<Self::J, String> {
        match serde_json::from_slice(body) {
            Ok(e) => Ok(e),
            Err(e) => {
                Err(format!(
                    "Failed to deserialize BuildResult: {:?}, err: {:}",
                    String::from_utf8_lossy(&body.clone()),
                    e
                ))
            }
        }
    }

    fn consumer(&mut self, job: &BuildResult) -> worker::Actions {
        let result = job.legacy();
        let comment = hubcaps::comments::CommentOptions { body: result_to_comment(&result) };
        let check = result_to_check(&result, Utc::now());
        println!(":{:?}", check);
        println!(":{:?}", comment);

        let check_create_attempt = self.github
            .repo(result.repo.owner.clone(), result.repo.name.clone())
            .checkruns()
            .create(&check);

        match check_create_attempt {
            Ok(comment) => {
                info!("Successfully sent {:?} to {}",
                comment,
                result.pr.number,
            )
            }
            Err(err) => {
                info!(
                "Failed to send check {:?} to {}",
                err,
                result.pr.number,
            )
            }
        }

        vec![worker::Action::Ack]
    }
}

fn result_to_check(result: &LegacyBuildResult, timestamp: DateTime<Utc>) -> CheckRunOptions {
    let mut all_attrs: Vec<String> = vec![
        result.attempted_attrs.clone(),
        result.skipped_attrs.clone()
    ]
        .into_iter()
        .map(|opt| opt.unwrap_or(vec![]))
        .flat_map(|list| list.into_iter().map(|attr| format!("-A {}", attr)))
        .collect();
    all_attrs.sort();

    if all_attrs.len() == 0{
        all_attrs = vec![String::from("(unknown attributes)")];
    }

    let conclusion = match result.status {
        BuildStatus::Skipped => Conclusion::Neutral,
        BuildStatus::Success => Conclusion::Success,
        BuildStatus::Failure => Conclusion::Neutral,
        BuildStatus::TimedOut => Conclusion::Neutral,
        BuildStatus::UnexpectedError { .. } => Conclusion::Neutral,
    };

    let mut summary: Vec<String> = vec![];
    if let Some(ref attempted) = result.attempted_attrs {
        summary.extend(list_segment("Attempted", &attempted));
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
            &skipped));
    }

    let text: String;

    if result.output.len() > 0 {
        let mut reply: Vec<String> = vec![];

        reply.push("## Partial log".to_owned());
        reply.push("".to_owned());
        reply.push("```".to_owned());
        reply.extend(result.output.clone());
        reply.push("```".to_owned());

        text = reply.join("\n");
    } else {
        text = String::from("No partial log is available.");
    }


    CheckRunOptions{
        name: format!(
            "nix-build {} --argstr system {}",
            all_attrs.join(" "),
            result.system
        ),
        actions: None,
        completed_at: Some(timestamp.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)),
        started_at: None,
        conclusion: Some(conclusion),
        details_url: Some(format!(
            "https://logs.nix.ci/?key={}/{}.{}&attempt_id={}",
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
            title: "Build Results".to_string()
        }),
        status: Some(CheckRunState::Completed),
    }
}

fn result_to_comment(result: &LegacyBuildResult) -> String {
    let mut reply: Vec<String> = vec![];

    let log_link = if result.output.len() > 0 {
        format!(
            " [(full log)](https://logs.nix.ci/?key={}/{}.{}&attempt_id={})",
            &result.repo.owner.to_lowercase(),
            &result.repo.name.to_lowercase(),
            result.pr.number,
            result.attempt_id,
        )
    } else {
        "".to_owned()
    };

    reply.push(format!("<!--REQUEST_ID={}-->", result.request_id));
    reply.push(format!("{} on {}{}",
        (match result.status {
            BuildStatus::Skipped => "No attempt".into(),
            BuildStatus::Success => "Success".into(),
            BuildStatus::Failure => "Failure".into(),
            BuildStatus::TimedOut => "Timed out, unknown build status".into(),
            BuildStatus::UnexpectedError { ref err } => format!("Unexpected error: {}", err),
        }),
        result.system,
        log_link
    ));
    reply.push("".to_owned());

    if let Some(ref attempted) = result.attempted_attrs {
        reply.extend(list_segment("Attempted", &attempted));
    }

    if let Some(ref skipped) = result.skipped_attrs {
        reply.extend(list_segment(
            &format!(
                "The following builds were skipped because they don't evaluate on {}",
                result.system
            ),
            &skipped));
    }

    if result.output.len() > 0 {
        reply.extend(partial_log_segment(&result.output));
        reply.push("".to_owned());
        reply.push("".to_owned());
    } else {
        reply.push("No partial log is available.".to_owned());
        reply.push("".to_owned());
    }

    reply.join("\n")
}

fn list_segment(name: &str, things: &[String]) -> Vec<String> {
    let mut reply: Vec<String> = vec![];

    if things.len() > 0 {
        reply.push(format!("{}: {}", name, things.join(", ")));
        reply.push("".to_owned());
    }

    reply
}

fn partial_log_segment(output: &Vec<String>) -> Vec<String> {
    let mut reply: Vec<String> = vec![];

    reply.push(
        "<details><summary>Partial log (click to expand)</summary><p>".to_owned(),
    );
    reply.push("".to_owned());
    reply.push("```".to_owned());
    reply.extend(output.clone());
    reply.push("```".to_owned());
    reply.push("</p></details>".to_owned());

    reply
}

#[cfg(test)]
mod tests {
    use super::*;
    use message::{Pr, Repo};
    use chrono::TimeZone;

    #[test]
    pub fn test_passing_build() {
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

        assert_eq!(
            &result_to_comment(&result),
            "<!--REQUEST_ID=bogus-request-id-->
Success on x86_64-linux [(full log)](https://logs.nix.ci/?key=nixos/nixpkgs.2345&attempt_id=neatattemptid)

Attempted: foo

The following builds were skipped because they don't evaluate on x86_64-linux: bar

<details><summary>Partial log (click to expand)</summary><p>

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
```
</p></details>

"
        );
    }

    #[test]
    pub fn test_failing_build() {
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

        assert_eq!(
            &result_to_comment(&result),
            "<!--REQUEST_ID=bogus-request-id-->
Failure on x86_64-linux [(full log)](https://logs.nix.ci/?key=nixos/nixpkgs.2345&attempt_id=neatattemptid)

Attempted: foo

<details><summary>Partial log (click to expand)</summary><p>

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
```
</p></details>

"
        );
    }

    #[test]
    pub fn test_timedout_build() {
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

        assert_eq!(
            &result_to_comment(&result),
            "<!--REQUEST_ID=bogus-request-id-->
Timed out, unknown build status on x86_64-linux [(full log)](https://logs.nix.ci/?key=nixos/nixpkgs.2345&attempt_id=neatattemptid)

Attempted: foo

<details><summary>Partial log (click to expand)</summary><p>

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
```
</p></details>

"
        );
    }

    #[test]
    pub fn test_passing_build_unspecified_attributes() {
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

        assert_eq!(
            &result_to_comment(&result),
            "<!--REQUEST_ID=bogus-request-id-->
Success on x86_64-linux [(full log)](https://logs.nix.ci/?key=nixos/nixpkgs.2345&attempt_id=neatattemptid)

<details><summary>Partial log (click to expand)</summary><p>

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
```
</p></details>

"
        );
    }

    #[test]
    pub fn test_failing_build_unspecified_attributes() {
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

        assert_eq!(
            &result_to_comment(&result),
            "<!--REQUEST_ID=bogus-request-id-->
Failure on x86_64-linux [(full log)](https://logs.nix.ci/?key=nixos/nixpkgs.2345&attempt_id=neatattemptid)

<details><summary>Partial log (click to expand)</summary><p>

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
```
</p></details>

"
        );
    }

    #[test]
    pub fn test_no_attempt() {
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
            attempt_id: "foo".to_owned(),
            request_id: "bogus-request-id".to_owned(),
            system: "x86_64-linux".to_owned(),
            attempted_attrs: None,
            skipped_attrs: Some(vec!["not-attempted".to_owned()]),
            status: BuildStatus::Skipped,
        };

        assert_eq!(
            &result_to_comment(&result),
            "<!--REQUEST_ID=bogus-request-id-->
No attempt on x86_64-linux [(full log)](https://logs.nix.ci/?key=nixos/nixpkgs.2345&attempt_id=foo)

The following builds were skipped because they don't evaluate on x86_64-linux: not-attempted

<details><summary>Partial log (click to expand)</summary><p>

```
foo
```
</p></details>

"
        );
    }

    #[test]
    pub fn test_no_attempt_no_log() {
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
            attempt_id: "foo".to_owned(),
            request_id: "bogus-request-id".to_owned(),
            system: "x86_64-linux".to_owned(),
            attempted_attrs: None,
            skipped_attrs: Some(vec!["not-attempted".to_owned()]),
            status: BuildStatus::Skipped,
        };

        assert_eq!(
            &result_to_comment(&result),
            "<!--REQUEST_ID=bogus-request-id-->
No attempt on x86_64-linux

The following builds were skipped because they don't evaluate on x86_64-linux: not-attempted

No partial log is available.
"
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

        let timestamp = Utc.ymd(2023, 4, 20).and_hms(13, 37, 42);

        assert_eq!(
            result_to_check(&result, timestamp),
            CheckRunOptions {
                name: "nix-build -A bar -A foo --argstr system x86_64-linux".to_string(),
                actions: None,
                started_at: None,
                completed_at: Some("2023-04-20T13:37:42Z".to_string()),
                status: Some(CheckRunState::Completed),
                conclusion: Some(Conclusion::Success),
                details_url: Some("https://logs.nix.ci/?key=nixos/nixpkgs.2345&attempt_id=neatattemptid".to_string()),
                external_id: Some("neatattemptid".to_string()),
                head_sha: "abc123".to_string(),
                output: Some(Output {
                    title: "Build Results".to_string(),
                    summary: "Attempted: foo

The following builds were skipped because they don't evaluate on x86_64-linux: bar
".to_string(),
                    text: Some("## Partial log

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
```".to_string()),
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

        let timestamp = Utc.ymd(2023, 4, 20).and_hms(13, 37, 42);

        assert_eq!(
            result_to_check(&result, timestamp),
            CheckRunOptions {
                name: "nix-build -A foo --argstr system x86_64-linux".to_string(),
                actions: None,
                started_at: None,
                completed_at: Some("2023-04-20T13:37:42Z".to_string()),
                status: Some(CheckRunState::Completed),
                conclusion: Some(Conclusion::Neutral),
                details_url: Some("https://logs.nix.ci/?key=nixos/nixpkgs.2345&attempt_id=neatattemptid".to_string()),
                external_id: Some("neatattemptid".to_string()),
                head_sha: "abc123".to_string(),
                output: Some(Output {
                    title: "Build Results".to_string(),
                    summary: "Attempted: foo
".to_string(),
                    text: Some("## Partial log

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
```".to_string()),
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

        let timestamp = Utc.ymd(2023, 4, 20).and_hms(13, 37, 42);

        assert_eq!(
            result_to_check(&result, timestamp),
            CheckRunOptions {
                name: "nix-build -A foo --argstr system x86_64-linux".to_string(),
                actions: None,
                started_at: None,
                completed_at: Some("2023-04-20T13:37:42Z".to_string()),
                status: Some(CheckRunState::Completed),
                conclusion: Some(Conclusion::Neutral),
                details_url: Some("https://logs.nix.ci/?key=nixos/nixpkgs.2345&attempt_id=neatattemptid".to_string()),
                external_id: Some("neatattemptid".to_string()),
                head_sha: "abc123".to_string(),
                output: Some(Output {
                    title: "Build Results".to_string(),
                    summary: "Attempted: foo

Build timed out.".to_string(),
                    text: Some("## Partial log

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
```".to_string()),
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

        let timestamp = Utc.ymd(2023, 4, 20).and_hms(13, 37, 42);

        assert_eq!(
            result_to_check(&result, timestamp),
            CheckRunOptions {
                name: "nix-build (unknown attributes) --argstr system x86_64-linux".to_string(),
                actions: None,
                started_at: None,
                completed_at: Some("2023-04-20T13:37:42Z".to_string()),
                status: Some(CheckRunState::Completed),
                conclusion: Some(Conclusion::Success),
                details_url: Some("https://logs.nix.ci/?key=nixos/nixpkgs.2345&attempt_id=neatattemptid".to_string()),
                external_id: Some("neatattemptid".to_string()),
                head_sha: "abc123".to_string(),
                output: Some(Output {
                    title: "Build Results".to_string(),
                    summary: "".to_string(),
                    text: Some("## Partial log

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
```".to_string()),
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

        let timestamp = Utc.ymd(2023, 4, 20).and_hms(13, 37, 42);

        assert_eq!(
            result_to_check(&result, timestamp),
            CheckRunOptions {
                name: "nix-build (unknown attributes) --argstr system x86_64-linux".to_string(),
                actions: None,
                started_at: None,
                completed_at: Some("2023-04-20T13:37:42Z".to_string()),
                status: Some(CheckRunState::Completed),
                conclusion: Some(Conclusion::Neutral),
                details_url: Some("https://logs.nix.ci/?key=nixos/nixpkgs.2345&attempt_id=neatattemptid".to_string()),
                external_id: Some("neatattemptid".to_string()),
                head_sha: "abc123".to_string(),
                output: Some(Output {
                    title: "Build Results".to_string(),
                    summary: "".to_string(),
                    text: Some("## Partial log

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
```".to_string()),
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

        let timestamp = Utc.ymd(2023, 4, 20).and_hms(13, 37, 42);

        assert_eq!(
            result_to_check(&result, timestamp),
            CheckRunOptions {
                name: "nix-build -A not-attempted --argstr system x86_64-linux".to_string(),
                actions: None,
                started_at: None,
                completed_at: Some("2023-04-20T13:37:42Z".to_string()),
                status: Some(CheckRunState::Completed),
                conclusion: Some(Conclusion::Neutral),
                details_url: Some("https://logs.nix.ci/?key=nixos/nixpkgs.2345&attempt_id=neatattemptid".to_string()),
                external_id: Some("neatattemptid".to_string()),
                head_sha: "abc123".to_string(),
                output: Some(Output {
                    title: "Build Results".to_string(),
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

        let timestamp = Utc.ymd(2023, 4, 20).and_hms(13, 37, 42);

        assert_eq!(
            result_to_check(&result, timestamp),
            CheckRunOptions {
                name: "nix-build -A not-attempted --argstr system x86_64-linux".to_string(),
                actions: None,
                started_at: None,
                completed_at: Some("2023-04-20T13:37:42Z".to_string()),
                status: Some(CheckRunState::Completed),
                conclusion: Some(Conclusion::Neutral),
                details_url: Some("https://logs.nix.ci/?key=nixos/nixpkgs.2345&attempt_id=neatattemptid".to_string()),
                external_id: Some("neatattemptid".to_string()),
                head_sha: "abc123".to_string(),
                output: Some(Output {
                    title: "Build Results".to_string(),
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
