use chrono::Utc;
use lru_cache::LruCache;
use octocrab::models::CheckRunId;
use octocrab::params::checks::{CheckRunConclusion, CheckRunOutput, CheckRunStatus};
use octocrab::params::repos::Commitish;
use tracing::{Instrument, debug_span, info, warn};

use crate::config::GithubAppVendingMachine;
use crate::github::GithubRepo;
use crate::message::Repo;
use crate::message::buildjob::{BuildJob, QueuedBuildJobs};
use crate::message::buildresult::{BuildResult, BuildStatus, LegacyBuildResult};
use crate::message::hydra_build::{HydraBuildState, HydraBuildUpdate};
use crate::worker;

/// How many `(repo, sha, check name) -> check run id` mappings to remember.
///
/// Only an optimisation: a miss costs one extra API call to re-resolve the id
/// from GitHub, it never costs correctness.
const CHECK_RUN_CACHE_SIZE: usize = 512;

type CheckRunKey = (String, String, String);

pub struct GitHubCommentPoster {
    github_vend: GithubAppVendingMachine,
    check_run_ids: LruCache<CheckRunKey, CheckRunId>,
}

impl GitHubCommentPoster {
    pub fn new(github_vend: GithubAppVendingMachine) -> GitHubCommentPoster {
        GitHubCommentPoster {
            github_vend,
            check_run_ids: LruCache::new(CHECK_RUN_CACHE_SIZE),
        }
    }
}

pub struct CheckRunInfo {
    pub name: String,
    pub details_url: String,
    pub output: CheckRunOutput,
    pub conclusion: Option<CheckRunConclusion>,
    pub status: Option<CheckRunStatus>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub enum PostableEvent {
    BuildQueued(QueuedBuildJobs),
    BuildFinished(BuildResult),
    HydraBuild(HydraBuildUpdate),
}

impl PostableEvent {
    fn from(bytes: &[u8]) -> Result<PostableEvent, String> {
        // `HydraBuildUpdate` is tried first, and carries an explicit tag, because
        // `BuildResult`'s untagged `Legacy` variant will happily accept loosely
        // shaped JSON that was never meant for it.
        if let Ok(e) = serde_json::from_slice::<HydraBuildUpdate>(bytes) {
            return Ok(PostableEvent::HydraBuild(e));
        }

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

    async fn msg_to_job(
        &mut self,
        _: &str,
        _: &Option<String>,
        body: &[u8],
    ) -> Result<Self::J, String> {
        PostableEvent::from(body)
    }

    async fn consumer(&mut self, job: &PostableEvent) -> worker::Actions {
        let mut check_runs: Vec<CheckRunInfo> = vec![];
        let repo: Repo;
        let pr_number: u64;
        let head_sha: String;
        // Hydra reports the same build several times (queued, running, finished),
        // so those updates have to advance one check run instead of piling up new
        // ones. The legacy messages are each posted once and keep the old
        // create-only behaviour.
        let update_in_place: bool;

        match job {
            PostableEvent::BuildQueued(queued_job) => {
                repo = queued_job.job.repo.clone();
                pr_number = queued_job.job.pr.number;
                head_sha = queued_job.job.pr.head_sha.clone();
                update_in_place = false;
                for architecture in queued_job.architectures.iter() {
                    check_runs.push(job_to_check_info(&queued_job.job, architecture));
                }
            }
            PostableEvent::BuildFinished(finished_job) => {
                let result = finished_job.legacy();
                repo = result.repo.clone();
                pr_number = result.pr.number;
                head_sha = result.pr.head_sha.clone();
                update_in_place = false;
                check_runs.push(result_to_check_info(&result));
            }
            PostableEvent::HydraBuild(update) => {
                repo = update.repo.clone();
                pr_number = update.pr.number;
                head_sha = update.pr.head_sha.clone();
                update_in_place = true;
                check_runs.push(hydra_update_to_check_info(update));
            }
        };

        let span = debug_span!("job", pr = ?pr_number);
        async move {
            let octocrab_ref = match self.github_vend.for_repo(&repo.owner, &repo.name).await {
                Some(client) => client.clone(),
                None => {
                    warn!(
                        "No GitHub installation found for {}/{}, skipping checks",
                        repo.owner, repo.name
                    );
                    return vec![worker::Action::Ack];
                }
            };

            let github_repo = GithubRepo::new(octocrab_ref, &repo.owner, &repo.name);

            for check in check_runs {
                info!(
                    "check {:?} {} {}",
                    check.status, check.name, check.details_url,
                );

                if update_in_place {
                    self.upsert_check_run(&github_repo, &repo.full_name, &head_sha, check)
                        .await;
                } else {
                    create_check_run(&github_repo, &head_sha, check).await;
                }
            }

            vec![worker::Action::Ack]
        }
        .instrument(span)
        .await
    }
}

impl GitHubCommentPoster {
    /// Advance an existing check run if we can find one, otherwise create it.
    async fn upsert_check_run(
        &mut self,
        github_repo: &GithubRepo,
        full_name: &str,
        head_sha: &str,
        check: CheckRunInfo,
    ) {
        let key: CheckRunKey = (
            full_name.to_owned(),
            head_sha.to_owned(),
            check.name.clone(),
        );

        let cached = self.check_run_ids.get_mut(&key).copied();
        let known = match cached {
            Some(id) => Some(id),
            // Cache miss (first update for this build, or the poster restarted):
            // ask GitHub which check run already carries this name.
            None => find_check_run(github_repo, head_sha, &check.name).await,
        };

        if let Some(id) = known {
            let checks_handler = github_repo.checks();
            let mut builder = checks_handler
                .update_check_run(id)
                .details_url(check.details_url.clone())
                .output(clone_output(&check.output));

            if let Some(completed_at) = check.completed_at {
                builder = builder.completed_at(completed_at);
            }
            if let Some(conclusion) = check.conclusion {
                builder = builder.conclusion(conclusion);
            }
            if let Some(status) = check.status {
                builder = builder.status(status);
            }

            match builder.send().await {
                Ok(_) => {
                    info!("Successfully updated check run {id}.");
                    self.check_run_ids.insert(key, id);
                    return;
                }
                // Most likely the run belongs to another app, or was deleted.
                // Falling through to create is always safe.
                Err(err) => warn!("Failed to update check run {id}, creating a new one: {err:?}"),
            }
        }

        if let Some(id) = create_check_run(github_repo, head_sha, check).await {
            self.check_run_ids.insert(key, id);
        }
    }
}

async fn create_check_run(
    github_repo: &GithubRepo,
    head_sha: &str,
    check: CheckRunInfo,
) -> Option<CheckRunId> {
    let checks_handler = github_repo.checks();
    let mut builder = checks_handler
        .create_check_run(check.name, head_sha.to_owned())
        .details_url(check.details_url)
        .output(check.output);

    if let Some(completed_at) = check.completed_at {
        builder = builder.completed_at(completed_at);
    }
    if let Some(conclusion) = check.conclusion {
        builder = builder.conclusion(conclusion);
    }
    if let Some(status) = check.status {
        builder = builder.status(status);
    }

    match builder.send().await {
        Ok(run) => {
            info!("Successfully sent check.");
            Some(run.id)
        }
        Err(err) => {
            warn!("Failed to send check {:?}", err);
            None
        }
    }
}

/// The list endpoint returns every app's check runs for the commit, and offers
/// no name filter, so match on the name here. A collision with another app's
/// run is harmless: the update then fails and we fall back to creating our own.
async fn find_check_run(
    github_repo: &GithubRepo,
    head_sha: &str,
    name: &str,
) -> Option<CheckRunId> {
    match github_repo
        .checks()
        .list_check_runs_for_git_ref(Commitish(head_sha.to_owned()))
        .per_page(100u8)
        .send()
        .await
    {
        Ok(list) => list
            .check_runs
            .into_iter()
            .find(|run| run.name == name)
            .map(|run| run.id),
        Err(err) => {
            warn!("Failed to list check runs for {head_sha}: {err:?}");
            None
        }
    }
}

/// `CheckRunOutput` is not `Clone`, and the update path needs one copy for the
/// update attempt and one for the create it may fall back to. Annotations and
/// images are not carried over — nothing in ofborg sets them.
fn clone_output(output: &CheckRunOutput) -> CheckRunOutput {
    CheckRunOutput {
        title: output.title.clone(),
        summary: output.summary.clone(),
        text: output.text.clone(),
        annotations: vec![],
        images: vec![],
    }
}

fn job_to_check_info(job: &BuildJob, architecture: &str) -> CheckRunInfo {
    let mut all_attrs: Vec<String> = job.attrs.clone();
    all_attrs.sort();

    if all_attrs.is_empty() {
        all_attrs = vec![String::from("(unknown attributes)")];
    }

    let name = format!("{} on {architecture}", all_attrs.join(", "));
    let details_url = format!(
        "https://logs.ofborg.org/?key={}/{}.{}",
        &job.repo.owner.to_lowercase(),
        &job.repo.name.to_lowercase(),
        job.pr.number,
    );

    CheckRunInfo {
        name,
        details_url,
        output: CheckRunOutput {
            title: "Queued".to_string(),
            summary: String::new(),
            text: None,
            annotations: vec![],
            images: vec![],
        },
        conclusion: None,
        status: Some(CheckRunStatus::Queued),
        completed_at: None,
    }
}

fn result_to_check_info(result: &LegacyBuildResult) -> CheckRunInfo {
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

    let conclusion = conclusion_for(&result.status);

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

    let name = format!("{} on {}", all_attrs.join(", "), result.system);
    let details_url = format!(
        "https://logs.ofborg.org/?key={}/{}.{}&attempt_id={}",
        &result.repo.owner.to_lowercase(),
        &result.repo.name.to_lowercase(),
        result.pr.number,
        result.attempt_id,
    );

    CheckRunInfo {
        name,
        details_url,
        output: CheckRunOutput {
            title: result.status.clone().into(),
            summary: summary.join("\n"),
            text: Some(text),
            annotations: vec![],
            images: vec![],
        },
        conclusion: Some(conclusion),
        status: Some(CheckRunStatus::Completed),
        completed_at: Some(Utc::now()),
    }
}

fn conclusion_for(status: &BuildStatus) -> CheckRunConclusion {
    match status {
        BuildStatus::Skipped => CheckRunConclusion::Skipped,
        BuildStatus::Success => CheckRunConclusion::Success,
        BuildStatus::Failure => CheckRunConclusion::Neutral,
        BuildStatus::TimedOut => CheckRunConclusion::Neutral,
        BuildStatus::UnexpectedError { .. } => CheckRunConclusion::Neutral,
        BuildStatus::HashMismatch => CheckRunConclusion::Failure,
    }
}

fn hydra_update_to_check_info(update: &HydraBuildUpdate) -> CheckRunInfo {
    let details_url = update.details_url();
    let name = format!("{} on {}", update.attr, update.system);

    let (title, status, conclusion, completed_at, mut summary) = match &update.state {
        HydraBuildState::Queued => (
            "Queued".to_owned(),
            CheckRunStatus::Queued,
            None,
            None,
            vec![format!("Queued in Hydra as build {}.", update.build_id)],
        ),
        HydraBuildState::Running { step } => {
            // The queue-runner reports which step the builder is on; fall back
            // to the generic verb when it has not said yet.
            let step = step.clone().unwrap_or_else(|| "Building".to_owned());
            (step, CheckRunStatus::InProgress, None, None, vec![])
        }
        HydraBuildState::Finished { status } => (
            String::from(status.clone()),
            CheckRunStatus::Completed,
            Some(conclusion_for(status)),
            Some(Utc::now()),
            vec![],
        ),
    };

    // The machine only becomes known once a step has been scheduled, so it can
    // show up on the running update, the finished one, or neither.
    if let Some(machine) = &update.machine {
        summary.push(match &update.state {
            HydraBuildState::Finished { .. } => format!("Built on {machine}."),
            _ => format!("{title} on {machine}."),
        });
    }

    CheckRunInfo {
        name,
        details_url: details_url.clone(),
        output: CheckRunOutput {
            title,
            summary: summary.join("\n"),
            text: Some(format!("[Hydra build {}]({details_url})", update.build_id)),
            annotations: vec![],
            images: vec![],
        },
        conclusion,
        status: Some(status),
        completed_at,
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
    use crate::message::buildjob::BuildJob;
    use crate::message::{Pr, Repo};

    fn check_passing_log() -> Vec<String> {
        vec![
            "make[2]: Entering directory '/private/tmp/nix-build-gdb-8.1.drv-0/gdb-8.1/readline'"
                .to_owned(),
            "make[2]: Nothing to be done for 'install'.".to_owned(),
            "make[2]: Leaving directory '/private/tmp/nix-build-gdb-8.1.drv-0/gdb-8.1/readline'"
                .to_owned(),
            "make[1]: Nothing to be done for 'install-target'.".to_owned(),
            "make[1]: Leaving directory '/private/tmp/nix-build-gdb-8.1.drv-0/gdb-8.1'".to_owned(),
            "removed '/nix/store/pcja75y9isdvgz5i00pkrpif9rxzxc29-gdb-8.1/share/info/bfd.info'"
                .to_owned(),
            "post-installation fixup".to_owned(),
            "strip is /nix/store/5a88zk3jgimdmzg8rfhvm93kxib3njf9-cctools-binutils-darwin/bin/strip"
                .to_owned(),
            "patching script interpreter paths in /nix/store/pcja75y9isdvgz5i00pkrpif9rxzxc29-gdb-8.1"
                .to_owned(),
            "/nix/store/pcja75y9isdvgz5i00pkrpif9rxzxc29-gdb-8.1".to_owned(),
        ]
    }

    fn partial_log_text(lines: &[String]) -> String {
        let mut reply: Vec<String> = vec![];
        reply.push("## Partial log".to_owned());
        reply.push("".to_owned());
        reply.push("```".to_owned());
        reply.extend(lines.iter().cloned());
        reply.push("```".to_owned());
        reply.join("\n")
    }

    fn base_repo() -> Repo {
        Repo {
            clone_url: "https://github.com/nixos/nixpkgs.git".to_owned(),
            full_name: "NixOS/nixpkgs".to_owned(),
            owner: "NixOS".to_owned(),
            name: "nixpkgs".to_owned(),
        }
    }

    fn base_pr() -> Pr {
        Pr {
            head_sha: "abc123".to_owned(),
            number: 2345,
            target_branch: Some("master".to_owned()),
        }
    }

    fn base_result() -> LegacyBuildResult {
        LegacyBuildResult {
            repo: base_repo(),
            pr: base_pr(),
            output: vec![],
            attempt_id: "neatattemptid".to_owned(),
            request_id: "bogus-request-id".to_owned(),
            system: "x86_64-linux".to_owned(),
            attempted_attrs: None,
            skipped_attrs: None,
            status: BuildStatus::Skipped,
        }
    }

    #[test]
    pub fn test_queued_build() {
        let job = BuildJob {
            repo: base_repo(),
            pr: base_pr(),
            logs: None,
            statusreport: None,
            subset: None,
            request_id: "bogus-request-id".to_owned(),
            attrs: vec!["foo".to_owned(), "bar".to_owned()],
        };

        let result = job_to_check_info(&job, "x86_64-linux");
        assert_eq!(result.name, "bar, foo on x86_64-linux");
        assert_eq!(
            result.details_url,
            "https://logs.ofborg.org/?key=nixos/nixpkgs.2345"
        );
        assert_eq!(result.output.title, "Queued");
        assert_eq!(result.output.summary, "");
        assert_eq!(result.output.text, None);
        assert!(result.output.annotations.is_empty());
        assert!(result.output.images.is_empty());
        assert!(result.conclusion.is_none());
        assert!(matches!(result.status, Some(CheckRunStatus::Queued)));
    }

    #[test]
    pub fn test_check_passing_build() {
        let result = LegacyBuildResult {
            output: check_passing_log(),
            attempted_attrs: Some(vec!["foo".to_owned()]),
            skipped_attrs: Some(vec!["bar".to_owned()]),
            status: BuildStatus::Success,
            ..base_result()
        };

        let check = result_to_check_info(&result);
        assert_eq!(check.name, "bar, foo on x86_64-linux");
        assert_eq!(
            check.details_url,
            "https://logs.ofborg.org/?key=nixos/nixpkgs.2345&attempt_id=neatattemptid"
        );
        assert!(matches!(
            check.conclusion,
            Some(CheckRunConclusion::Success)
        ));
        assert!(matches!(check.status, Some(CheckRunStatus::Completed)));
        assert_eq!(check.output.title, "Success");
        assert_eq!(
            check.output.summary,
            concat!(
                "Attempted: foo",
                "\n",
                "\n",
                "The following builds were skipped because they don't evaluate on x86_64-linux: bar",
                "\n",
            )
        );
        assert_eq!(
            check.output.text,
            Some(partial_log_text(&check_passing_log()))
        );
    }

    #[test]
    pub fn test_check_failing_build() {
        let result = LegacyBuildResult {
            output: check_passing_log(),
            attempted_attrs: Some(vec!["foo".to_owned()]),
            skipped_attrs: None,
            status: BuildStatus::Failure,
            ..base_result()
        };

        let check = result_to_check_info(&result);
        assert_eq!(check.name, "foo on x86_64-linux");
        assert_eq!(
            check.details_url,
            "https://logs.ofborg.org/?key=nixos/nixpkgs.2345&attempt_id=neatattemptid"
        );
        assert!(matches!(
            check.conclusion,
            Some(CheckRunConclusion::Neutral)
        ));
        assert!(matches!(check.status, Some(CheckRunStatus::Completed)));
        assert_eq!(check.output.title, "Failure");
        assert_eq!(check.output.summary, "Attempted: foo\n");
        assert_eq!(
            check.output.text,
            Some(partial_log_text(&check_passing_log()))
        );
    }

    #[test]
    pub fn test_check_timedout_build() {
        let mut log = check_passing_log();
        log.push(
            "building of '/nix/store/l1limh50lx2cx45yb2gqpv7k8xl1mik2-gdb-8.1.drv' \
             timed out after 1 seconds"
                .to_owned(),
        );
        log.push(
            "error: build of '/nix/store/l1limh50lx2cx45yb2gqpv7k8xl1mik2-gdb-8.1.drv' failed"
                .to_owned(),
        );

        let result = LegacyBuildResult {
            output: log.clone(),
            attempted_attrs: Some(vec!["foo".to_owned()]),
            skipped_attrs: None,
            status: BuildStatus::TimedOut,
            ..base_result()
        };

        let check = result_to_check_info(&result);
        assert_eq!(check.name, "foo on x86_64-linux");
        assert_eq!(
            check.details_url,
            "https://logs.ofborg.org/?key=nixos/nixpkgs.2345&attempt_id=neatattemptid"
        );
        assert!(matches!(
            check.conclusion,
            Some(CheckRunConclusion::Neutral)
        ));
        assert!(matches!(check.status, Some(CheckRunStatus::Completed)));
        assert_eq!(check.output.title, "Timed out, unknown build status");
        assert_eq!(
            check.output.summary,
            concat!("Attempted: foo", "\n", "\n", "Build timed out.")
        );
        assert_eq!(check.output.text, Some(partial_log_text(&log)));
    }

    #[test]
    pub fn test_check_passing_build_unspecified_attributes() {
        let result = LegacyBuildResult {
            output: check_passing_log(),
            attempted_attrs: None,
            skipped_attrs: None,
            status: BuildStatus::Success,
            ..base_result()
        };

        let check = result_to_check_info(&result);
        assert_eq!(check.name, "(unknown attributes) on x86_64-linux");
        assert_eq!(
            check.details_url,
            "https://logs.ofborg.org/?key=nixos/nixpkgs.2345&attempt_id=neatattemptid"
        );
        assert!(matches!(
            check.conclusion,
            Some(CheckRunConclusion::Success)
        ));
        assert!(matches!(check.status, Some(CheckRunStatus::Completed)));
        assert_eq!(check.output.title, "Success");
        assert_eq!(check.output.summary, "");
        assert_eq!(
            check.output.text,
            Some(partial_log_text(&check_passing_log()))
        );
    }

    #[test]
    pub fn test_check_failing_build_unspecified_attributes() {
        let result = LegacyBuildResult {
            output: check_passing_log(),
            attempted_attrs: None,
            skipped_attrs: None,
            status: BuildStatus::Failure,
            ..base_result()
        };

        let check = result_to_check_info(&result);
        assert_eq!(check.name, "(unknown attributes) on x86_64-linux");
        assert_eq!(
            check.details_url,
            "https://logs.ofborg.org/?key=nixos/nixpkgs.2345&attempt_id=neatattemptid"
        );
        assert!(matches!(
            check.conclusion,
            Some(CheckRunConclusion::Neutral)
        ));
        assert!(matches!(check.status, Some(CheckRunStatus::Completed)));
        assert_eq!(check.output.title, "Failure");
        assert_eq!(check.output.summary, "");
        assert_eq!(
            check.output.text,
            Some(partial_log_text(&check_passing_log()))
        );
    }

    #[test]
    pub fn test_check_no_attempt() {
        let result = LegacyBuildResult {
            output: vec!["foo".to_owned()],
            attempted_attrs: None,
            skipped_attrs: Some(vec!["not-attempted".to_owned()]),
            status: BuildStatus::Skipped,
            ..base_result()
        };

        let check = result_to_check_info(&result);
        assert_eq!(check.name, "not-attempted on x86_64-linux");
        assert_eq!(
            check.details_url,
            "https://logs.ofborg.org/?key=nixos/nixpkgs.2345&attempt_id=neatattemptid"
        );
        assert!(matches!(
            check.conclusion,
            Some(CheckRunConclusion::Skipped)
        ));
        assert!(matches!(check.status, Some(CheckRunStatus::Completed)));
        assert_eq!(check.output.title, "No attempt");
        assert_eq!(
            check.output.summary,
            concat!(
                "The following builds were skipped because they don't evaluate on x86_64-linux: \
                 not-attempted",
                "\n",
            )
        );
        assert_eq!(
            check.output.text,
            Some(
                concat!(
                    "## Partial log",
                    "\n",
                    "\n",
                    "```",
                    "\n",
                    "foo",
                    "\n",
                    "```"
                )
                .to_string()
            )
        );
    }

    fn hydra_update(state: HydraBuildState, machine: Option<&str>) -> HydraBuildUpdate {
        HydraBuildUpdate {
            tag: crate::message::hydra_build::HydraV1Tag::HydraV1,
            repo: base_repo(),
            pr: base_pr(),
            attr: "hello".to_owned(),
            system: "x86_64-linux".to_owned(),
            build_id: 4711,
            machine: machine.map(String::from),
            state,
            hydra_base_url: "https://hydra.example.com".to_owned(),
        }
    }

    #[test]
    pub fn test_hydra_queued() {
        let check = hydra_update_to_check_info(&hydra_update(HydraBuildState::Queued, None));

        assert_eq!(check.name, "hello on x86_64-linux");
        assert_eq!(check.details_url, "https://hydra.example.com/build/4711");
        assert_eq!(check.output.title, "Queued");
        assert_eq!(check.output.summary, "Queued in Hydra as build 4711.");
        assert!(matches!(check.status, Some(CheckRunStatus::Queued)));
        assert!(check.conclusion.is_none());
        assert!(check.completed_at.is_none());
    }

    #[test]
    pub fn test_hydra_running_names_the_machine() {
        let check = hydra_update_to_check_info(&hydra_update(
            HydraBuildState::Running { step: None },
            Some("builder-01.example.com"),
        ));

        assert_eq!(check.output.title, "Building");
        assert_eq!(check.output.summary, "Building on builder-01.example.com.");
        assert!(matches!(check.status, Some(CheckRunStatus::InProgress)));
        assert!(check.conclusion.is_none());
        assert_eq!(
            check.output.text,
            Some("[Hydra build 4711](https://hydra.example.com/build/4711)".to_owned())
        );
    }

    #[test]
    pub fn test_hydra_running_reports_the_step() {
        let check = hydra_update_to_check_info(&hydra_update(
            HydraBuildState::Running {
                step: Some("Sending inputs".to_owned()),
            },
            Some("builder-01.example.com"),
        ));

        assert_eq!(check.output.title, "Sending inputs");
        assert_eq!(
            check.output.summary,
            "Sending inputs on builder-01.example.com."
        );
        assert!(matches!(check.status, Some(CheckRunStatus::InProgress)));
    }

    #[test]
    pub fn test_hydra_finished_success() {
        let check = hydra_update_to_check_info(&hydra_update(
            HydraBuildState::Finished {
                status: BuildStatus::Success,
            },
            Some("builder-01.example.com"),
        ));

        assert_eq!(check.output.title, "Success");
        assert_eq!(check.output.summary, "Built on builder-01.example.com.");
        assert!(matches!(check.status, Some(CheckRunStatus::Completed)));
        assert!(matches!(
            check.conclusion,
            Some(CheckRunConclusion::Success)
        ));
        assert!(check.completed_at.is_some());
    }

    #[test]
    pub fn test_hydra_finished_failure() {
        let check = hydra_update_to_check_info(&hydra_update(
            HydraBuildState::Finished {
                status: BuildStatus::Failure,
            },
            None,
        ));

        assert_eq!(check.output.title, "Failure");
        assert_eq!(check.output.summary, "");
        assert!(matches!(
            check.conclusion,
            Some(CheckRunConclusion::Neutral)
        ));
    }

    #[test]
    pub fn test_hydra_update_wins_over_build_result() {
        let bytes =
            serde_json::to_vec(&hydra_update(HydraBuildState::Running { step: None }, None))
                .expect("json required");

        match PostableEvent::from(&bytes) {
            Ok(PostableEvent::HydraBuild(update)) => {
                assert_eq!(update.build_id, 4711);
            }
            Ok(_) => panic!("a tagged HydraBuildUpdate was decoded as another event"),
            Err(e) => panic!("failed to decode: {e}"),
        }
    }

    #[test]
    pub fn test_legacy_build_result_still_decodes() {
        let bytes = br#"{"repo":{"owner":"NixOS","name":"nixpkgs","full_name":"NixOS/nixpkgs","clone_url":"https://github.com/nixos/nixpkgs.git"},"pr":{"target_branch":"master","number":42,"head_sha":"0000000000000000000000000000000000000000"},"system":"x86_64-linux","output":[],"attempt_id":"a","request_id":"r","success":true,"status":"Success","skipped_attrs":null,"attempted_attrs":["hello"]}"#;

        assert!(matches!(
            PostableEvent::from(bytes),
            Ok(PostableEvent::BuildFinished(_))
        ));
    }

    #[test]
    pub fn test_check_no_attempt_no_log() {
        let result = LegacyBuildResult {
            output: vec![],
            attempted_attrs: None,
            skipped_attrs: Some(vec!["not-attempted".to_owned()]),
            status: BuildStatus::Skipped,
            ..base_result()
        };

        let check = result_to_check_info(&result);
        assert_eq!(check.name, "not-attempted on x86_64-linux");
        assert_eq!(
            check.details_url,
            "https://logs.ofborg.org/?key=nixos/nixpkgs.2345&attempt_id=neatattemptid"
        );
        assert!(matches!(
            check.conclusion,
            Some(CheckRunConclusion::Skipped)
        ));
        assert!(matches!(check.status, Some(CheckRunStatus::Completed)));
        assert_eq!(check.output.title, "No attempt");
        assert_eq!(
            check.output.summary,
            concat!(
                "The following builds were skipped because they don't evaluate on x86_64-linux: \
                 not-attempted",
                "\n",
            )
        );
        assert_eq!(
            check.output.text,
            Some("No partial log is available.".to_string())
        );
    }
}
