/// This is what evaluates every pull-request
use crate::acl::Acl;
use crate::checkout;
use crate::commitstatus::{CommitStatus, CommitStatusError};
use crate::config::GithubAppVendingMachine;
use crate::message::{buildjob, evaluationjob};
use crate::nix;
use crate::stats::{self, Event};
use crate::systems;
use crate::tasks::eval;
use crate::tasks::eval::EvaluationStrategy;
use crate::worker;
use futures::stream::StreamExt;
use futures_util::TryFutureExt;

use std::path::Path;
use std::time::Instant;

use tracing::{debug_span, error, info, warn};

pub struct EvaluationWorker<E> {
    cloner: checkout::CachedCloner,
    github_vend: tokio::sync::RwLock<GithubAppVendingMachine>,
    nix: nix::Nix,
    acl: Acl,
    identity: String,
    events: E,
}

impl<E: stats::SysEvents> EvaluationWorker<E> {
    pub fn new(
        cloner: checkout::CachedCloner,
        github_vend: GithubAppVendingMachine,
        nix: nix::Nix,
        acl: Acl,
        identity: String,
        events: E,
    ) -> EvaluationWorker<E> {
        EvaluationWorker {
            cloner,
            github_vend: tokio::sync::RwLock::new(github_vend),
            nix,
            acl,
            identity,
            events,
        }
    }
}

impl<E: stats::SysEvents + 'static> worker::SimpleWorker for EvaluationWorker<E> {
    type J = evaluationjob::EvaluationJob;

    async fn msg_to_job(
        &mut self,
        _: &str,
        _: &Option<String>,
        body: &[u8],
    ) -> Result<Self::J, String> {
        self.events.notify(Event::JobReceived).await;
        match evaluationjob::from(body) {
            Ok(job) => {
                self.events.notify(Event::JobDecodeSuccess).await;
                Ok(job)
            }
            Err(err) => {
                self.events.notify(Event::JobDecodeFailure).await;
                error!(
                    "Failed to decode message: {}, Err: {err:?}",
                    std::str::from_utf8(body).unwrap_or("<message not utf8>")
                );
                Err("Failed to decode message".to_owned())
            }
        }
    }

    async fn consumer(&mut self, job: &evaluationjob::EvaluationJob) -> worker::Actions {
        let span = debug_span!("job", pr = ?job.pr.number);
        let _enter = span.enter();

        let mut vending_machine = self.github_vend.write().await;

        let github_client = vending_machine
            .for_repo(&job.repo.owner, &job.repo.name)
            .await
            .expect("Failed to get a github client token");

        OneEval::new(
            github_client,
            &self.nix,
            &self.acl,
            &mut self.events,
            &self.identity,
            &self.cloner,
            job,
        )
        .worker_actions()
        .await
    }
}

struct OneEval<'a, E> {
    client_app: &'a hubcaps::Github,
    repo: hubcaps::repositories::Repository,
    nix: &'a nix::Nix,
    acl: &'a Acl,
    events: &'a mut E,
    identity: &'a str,
    cloner: &'a checkout::CachedCloner,
    job: &'a evaluationjob::EvaluationJob,
}

impl<'a, E: stats::SysEvents + 'static> OneEval<'a, E> {
    #[allow(clippy::too_many_arguments)]
    fn new(
        client_app: &'a hubcaps::Github,
        nix: &'a nix::Nix,
        acl: &'a Acl,
        events: &'a mut E,
        identity: &'a str,
        cloner: &'a checkout::CachedCloner,
        job: &'a evaluationjob::EvaluationJob,
    ) -> OneEval<'a, E> {
        let repo = client_app.repo(job.repo.owner.clone(), job.repo.name.clone());
        OneEval {
            client_app,
            repo,
            nix,
            acl,
            events,
            identity,
            cloner,
            job,
        }
    }

    fn actions(&self) -> evaluationjob::Actions {
        evaluationjob::Actions {}
    }

    async fn update_status(
        &self,
        description: String,
        url: Option<String>,
        state: hubcaps::statuses::State,
    ) -> Result<(), CommitStatusError> {
        let description = if description.len() >= 140 {
            warn!(
                "description is over 140 char; truncating: {:?}",
                &description
            );
            description.chars().take(140).collect()
        } else {
            description
        };
        let repo = self
            .client_app
            .repo(self.job.repo.owner.clone(), self.job.repo.name.clone());
        let prefix = get_prefix(repo.statuses(), &self.job.pr.head_sha).await?;

        let mut builder = hubcaps::statuses::StatusOptions::builder(state);
        builder.context(format!("{prefix}-eval"));
        builder.description(description.clone());

        if let Some(url) = url {
            builder.target_url(url);
        }

        info!(
            "Updating status on {}:{} -> {}",
            &self.job.pr.number, &self.job.pr.head_sha, &description
        );

        self.repo
            .statuses()
            .create(&self.job.pr.head_sha, &builder.build())
            .map_ok(|_| ())
            .map_err(|e| CommitStatusError::from(e))
            .await
    }

    async fn worker_actions(&mut self) -> worker::Actions {
        let eval_result = match self.evaluate_job().await {
            Ok(v) => Ok(v),
            Err(eval_error) => match eval_error {
                // Handle error cases which expect us to post statuses
                // to github. Convert Eval Errors in to Result<_, CommitStatusWrite>
                EvalWorkerError::EvalError(eval::Error::Fail(msg)) => Err(self
                    .update_status(msg, None, hubcaps::statuses::State::Failure)
                    .await),
                EvalWorkerError::EvalError(eval::Error::CommitStatusWrite(e)) => Err(Err(e)),
                EvalWorkerError::CommitStatusWrite(e) => Err(Err(e)),
            },
        };

        match eval_result {
            Ok(eval_actions) => {
                let issue_ref = self.repo.issue(self.job.pr.number);
                update_labels(&issue_ref, &[], &[String::from("ofborg-internal-error")]).await;

                eval_actions
            }
            Err(Ok(())) => {
                // There was an error during eval, but we successfully
                // updated the PR.

                let issue_ref = self.repo.issue(self.job.pr.number);
                update_labels(&issue_ref, &[], &[String::from("ofborg-internal-error")]).await;

                self.actions().skip(self.job)
            }
            Err(Err(CommitStatusError::ExpiredCreds(e))) => {
                error!("Failed writing commit status: creds expired: {:?}", e);
                self.actions().retry_later(self.job)
            }
            Err(Err(CommitStatusError::InternalError(e))) => {
                error!("Failed writing commit status: internal error: {:?}", e);
                self.actions().retry_later(self.job)
            }
            Err(Err(CommitStatusError::MissingSha(e))) => {
                error!(
                    "Failed writing commit status: commit sha was force-pushed away: {:?}",
                    e
                );
                self.actions().skip(self.job)
            }

            Err(Err(CommitStatusError::Error(cswerr))) => {
                error!(
                    "Internal error writing commit status: {:?}, marking internal error",
                    cswerr
                );
                let issue_ref = self.repo.issue(self.job.pr.number);
                update_labels(&issue_ref, &[String::from("ofborg-internal-error")], &[]).await;

                self.actions().skip(self.job)
            }
        }
    }

    async fn evaluate_job(&mut self) -> Result<worker::Actions, EvalWorkerError> {
        let job = self.job;
        let repo = self
            .client_app
            .repo(self.job.repo.owner.clone(), self.job.repo.name.clone());
        let issue_ref = repo.issue(job.pr.number);
        let auto_schedule_build_archs: Vec<systems::System>;

        match issue_ref.get().await {
            Ok(iss) => {
                if iss.state == "closed" {
                    self.events.notify(Event::IssueAlreadyClosed).await;
                    info!("Skipping {} because it is closed", job.pr.number);
                    return Ok(self.actions().skip(job));
                }

                if issue_is_wip(&iss) {
                    auto_schedule_build_archs = vec![];
                } else {
                    auto_schedule_build_archs = self.acl.build_job_architectures_for_user_repo(
                        &iss.user.login,
                        &job.repo.full_name,
                    );
                }
            }

            Err(e) => {
                self.events.notify(Event::IssueFetchFailed).await;
                error!("Error fetching {}!", job.pr.number);
                error!("E: {:?}", e);
                return Ok(self.actions().skip(job));
            }
        };

        let mut evaluation_strategy = eval::NixpkgsStrategy::new(job, &issue_ref);

        let prefix = get_prefix(repo.statuses(), &job.pr.head_sha).await?;

        let mut overall_status = CommitStatus::new(
            repo.statuses(),
            job.pr.head_sha.clone(),
            format!("{prefix}-eval"),
            "Starting".to_owned(),
            None,
        );

        overall_status
            .set_with_description("Starting", hubcaps::statuses::State::Pending)
            .await?;

        evaluation_strategy.pre_clone().await?;

        let project = self
            .cloner
            .project(&job.repo.full_name, job.repo.clone_url.clone());

        overall_status
            .set_with_description("Cloning project", hubcaps::statuses::State::Pending)
            .await?;

        info!("Working on {}", job.pr.number);
        let co = project
            .clone_for("mr-est".to_string(), self.identity.to_string())
            .map_err(|e| {
                EvalWorkerError::CommitStatusWrite(CommitStatusError::InternalError(format!(
                    "Cloning failed: {e}"
                )))
            })?;

        let target_branch = match job.pr.target_branch.clone() {
            Some(x) => x,
            None => String::from("master"),
        };

        if target_branch.starts_with("nixos-") || target_branch.starts_with("nixpkgs-") {
            overall_status
                .set_with_description(
                    "The branch you have targeted is a read-only mirror for channels. \
                    Please target release-* or master.",
                    hubcaps::statuses::State::Error,
                )
                .await?;

            info!("PR targets a nixos-* or nixpkgs-* branch");
            return Ok(self.actions().skip(job));
        };

        overall_status
            .set_with_description(
                format!("Checking out {}", &target_branch).as_ref(),
                hubcaps::statuses::State::Pending,
            )
            .await?;
        info!("Checking out target branch {}", &target_branch);
        let refpath = co
            .checkout_origin_ref(target_branch.as_ref())
            .map_err(|e| {
                EvalWorkerError::CommitStatusWrite(CommitStatusError::InternalError(format!(
                    "Checking out target branch failed: {e}"
                )))
            })?;

        evaluation_strategy
            .on_target_branch(Path::new(&refpath), &mut overall_status)
            .await?;

        let target_branch_rebuild_sniff_start = Instant::now();

        self.events
            .notify(Event::EvaluationDuration(
                target_branch.clone(),
                target_branch_rebuild_sniff_start.elapsed().as_secs(),
            ))
            .await;
        self.events
            .notify(Event::EvaluationDurationCount(target_branch))
            .await;

        overall_status
            .set_with_description("Fetching PR", hubcaps::statuses::State::Pending)
            .await?;

        co.fetch_pr(job.pr.number).map_err(|e| {
            EvalWorkerError::CommitStatusWrite(CommitStatusError::InternalError(format!(
                "Fetching PR failed: {e}"
            )))
        })?;

        if !co.commit_exists(job.pr.head_sha.as_ref()) {
            overall_status
                .set_with_description("Commit not found", hubcaps::statuses::State::Error)
                .await?;

            info!("Commit {} doesn't exist", job.pr.head_sha);
            return Ok(self.actions().skip(job));
        }

        evaluation_strategy.after_fetch(&co)?;

        overall_status
            .set_with_description("Merging PR", hubcaps::statuses::State::Pending)
            .await?;

        if co.merge_commit(job.pr.head_sha.as_ref()).is_err() {
            overall_status
                .set_with_description("Failed to merge", hubcaps::statuses::State::Failure)
                .await?;

            info!("Failed to merge {}", job.pr.head_sha);

            return Ok(self.actions().skip(job));
        }

        evaluation_strategy.after_merge(&mut overall_status).await?;

        info!("Got path: {:?}, building", refpath);
        overall_status
            .set_with_description("Beginning Evaluations", hubcaps::statuses::State::Pending)
            .await?;

        let eval_results: bool = futures::stream::iter(evaluation_strategy.evaluation_checks())
            .map(|check| {
                // We need to clone or move variables into the async block
                let repo_statuses = repo.statuses();
                let head_sha = job.pr.head_sha.clone();
                let refpath = refpath.clone();

                async move {
                    let status = CommitStatus::new(
                        repo_statuses,
                        head_sha,
                        format!("{prefix}-eval-{}", check.name()),
                        check.cli_cmd(),
                        None,
                    );

                    status
                        .set(hubcaps::statuses::State::Pending)
                        .await
                        .expect("Failed to set status on eval strategy");

                    let state = match check.execute(Path::new(&refpath)) {
                        Ok(_) => hubcaps::statuses::State::Success,
                        Err(_) => hubcaps::statuses::State::Failure,
                    };

                    status
                        .set(state.clone())
                        .await
                        .expect("Failed to set status on eval strategy");

                    if state == hubcaps::statuses::State::Success {
                        Ok(())
                    } else {
                        Err(())
                    }
                }
            })
            .buffered(1)
            .all(|res| async move { res.is_ok() })
            .await;

        info!("Finished evaluations");
        let mut response: worker::Actions = vec![];

        if eval_results {
            let complete = evaluation_strategy
                .all_evaluations_passed(&mut overall_status)
                .await?;

            let builds = classify_builds(
                self.nix,
                Path::new(&refpath),
                complete.builds,
                auto_schedule_build_archs,
            )?;

            if builds.is_empty() {
                overall_status
                    .set_with_description("^.^!", hubcaps::statuses::State::Success)
                    .await?;
            } else {
                response.extend(schedule_builds(builds, format!("{prefix}-eval")));
            }
        } else {
            overall_status
                .set_with_description("Complete, with errors", hubcaps::statuses::State::Failure)
                .await?;
        }

        self.events.notify(Event::TaskEvaluationCheckComplete).await;

        info!("Evaluations done!");
        Ok(self.actions().done(job, response))
    }
}

struct ScheduledBuild {
    job: buildjob::BuildJob,
    architectures: Vec<systems::System>,
}

fn classify_builds(
    nix: &nix::Nix,
    nixpkgs: &Path,
    builds: Vec<buildjob::BuildJob>,
    architectures: Vec<systems::System>,
) -> Result<Vec<ScheduledBuild>, eval::Error> {
    let mut scheduled = vec![];

    for job in builds {
        let mut relevant_architectures = vec![];

        for architecture in &architectures {
            match nix
                .with_system(architecture.to_string())
                .any_attr_instantiable(nixpkgs, &job.attrs)
            {
                Ok(true) => relevant_architectures.push(architecture.clone()),
                Ok(false) => info!(
                    system = %architecture,
                    attrs = ?job.attrs,
                    "Skipping automatic build without instantiable attributes"
                ),
                Err(errors) => {
                    error!(
                        system = %architecture,
                        attrs = ?job.attrs,
                        errors = ?errors,
                        "Failed to determine automatic build relevance"
                    );
                    return Err(eval::Error::Fail(format!(
                        "Failed to determine build relevance on {architecture}"
                    )));
                }
            }
        }

        if !relevant_architectures.is_empty() {
            scheduled.push(ScheduledBuild {
                job,
                architectures: relevant_architectures,
            });
        }
    }

    Ok(scheduled)
}

fn schedule_builds(builds: Vec<ScheduledBuild>, evaluation_status: String) -> Vec<worker::Action> {
    let queued_builds = builds
        .iter()
        .map(|build| buildjob::QueuedBuildJobs {
            job: build.job.clone(),
            architectures: build
                .architectures
                .iter()
                .map(ToString::to_string)
                .collect(),
        })
        .collect();
    vec![worker::publish_serde_action_mandatory(
        Some("build-results".to_owned()),
        None,
        &buildjob::AutomaticBuildJobs {
            builds: queued_builds,
            evaluation_status,
        },
    )]
}

pub async fn update_labels(
    issueref: &hubcaps::issues::IssueRef,
    add: &[String],
    remove: &[String],
) {
    let l = issueref.labels();
    let issue = issueref.get().await.expect("Failed to get issue");

    let existing: Vec<String> = issue.labels.iter().map(|l| l.name.clone()).collect();

    let to_add: Vec<&str> = add
        .iter()
        .filter(|l| !existing.contains(l)) // Remove labels already on the issue
        .map(|l| l.as_ref())
        .collect();

    let to_remove: Vec<String> = remove
        .iter()
        .filter(|l| existing.contains(l)) // Remove labels already on the issue
        .cloned()
        .collect();

    let issue = issue.number;

    info!("Labeling issue #{issue}: + {to_add:?} , - {to_remove:?}, = {existing:?}");

    l.add(to_add.clone())
        .await
        .unwrap_or_else(|err| panic!("Failed to add labels {to_add:?} to issue #{issue}: {err:?}"));

    for label in to_remove {
        l.remove(&label).await.unwrap_or_else(|err| {
            panic!("Failed to remove label {label:?} from issue #{issue}: {err:?}")
        });
    }
}

fn issue_is_wip(issue: &hubcaps::issues::Issue) -> bool {
    issue.title.starts_with("WIP:") || issue.title.contains("[WIP]")
}

/// Determine whether or not to use the "old" status prefix, `grahamcofborg`, or
/// the new one, `ofborg`.
///
/// If the PR already has any `grahamcofborg`-prefixed statuses, continue to use
/// that (e.g. if someone used `@ofborg eval`, `@ofborg build`, `@ofborg test`).
/// Otherwise, if it's a new PR or was recently force-pushed (and therefore
/// doesn't have any old `grahamcofborg`-prefixed statuses), use the new prefix.
pub async fn get_prefix(
    statuses: hubcaps::statuses::Statuses,
    sha: &str,
) -> Result<&str, CommitStatusError> {
    if statuses
        .list(sha)
        .await?
        .iter()
        .any(|s| s.context.starts_with("grahamcofborg-"))
    {
        Ok("grahamcofborg")
    } else {
        Ok("ofborg")
    }
}

enum EvalWorkerError {
    EvalError(eval::Error),
    CommitStatusWrite(CommitStatusError),
}

impl From<eval::Error> for EvalWorkerError {
    fn from(e: eval::Error) -> EvalWorkerError {
        EvalWorkerError::EvalError(e)
    }
}

impl From<CommitStatusError> for EvalWorkerError {
    fn from(e: CommitStatusError) -> EvalWorkerError {
        EvalWorkerError::CommitStatusWrite(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commentparser::Subset;
    use crate::message::{Pr, Repo};
    use std::env;
    use std::path::PathBuf;

    fn nix() -> nix::Nix {
        nix::Nix::new(
            "x86_64-linux".to_owned(),
            env::var("NIX_REMOTE").unwrap_or_default(),
            1800,
            None,
        )
    }

    fn fixture(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("test-srcs")
            .join(name)
    }

    fn build(attrs: &[&str]) -> buildjob::BuildJob {
        buildjob::BuildJob::new(
            Repo {
                clone_url: "https://github.com/NixOS/nixpkgs.git".to_owned(),
                full_name: "NixOS/nixpkgs".to_owned(),
                owner: "NixOS".to_owned(),
                name: "nixpkgs".to_owned(),
            },
            Pr {
                head_sha: "abc123".to_owned(),
                number: 42,
                target_branch: Some("master".to_owned()),
            },
            Subset::Nixpkgs,
            attrs.iter().map(|attr| (*attr).to_owned()).collect(),
            None,
            None,
            "request-id".to_owned(),
        )
    }

    #[test]
    fn automatic_builds_only_target_relevant_systems() {
        let scheduled = match classify_builds(
            &nix(),
            &fixture("instantiable"),
            vec![build(&["linux-only"])],
            systems::System::all_known_systems().to_vec(),
        ) {
            Ok(scheduled) => scheduled,
            Err(_) => panic!("classification should succeed"),
        };

        assert_eq!(scheduled.len(), 1);
        assert_eq!(scheduled[0].architectures.len(), 2);
        assert_eq!(
            scheduled[0]
                .architectures
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
            vec!["x86_64-linux", "aarch64-linux"]
        );

        let actions = schedule_builds(scheduled, "ofborg-eval".to_owned());
        assert_eq!(actions.len(), 1);
        let worker::Action::Publish(queued_message) = &actions[0] else {
            panic!("automatic build plan should be published");
        };
        assert_eq!(queued_message.exchange, Some("build-results".to_owned()));
        assert!(queued_message.mandatory);
        let queued: buildjob::AutomaticBuildJobs =
            serde_json::from_slice(&queued_message.content).unwrap();
        assert_eq!(queued.evaluation_status, "ofborg-eval");
        assert_eq!(queued.builds.len(), 1);
        assert_eq!(
            queued.builds[0].architectures,
            vec!["x86_64-linux", "aarch64-linux"]
        );
    }

    #[test]
    fn automatic_builds_are_not_scheduled_without_relevant_systems() {
        let scheduled = match classify_builds(
            &nix(),
            &fixture("instantiable"),
            vec![build(&["missing", "unavailable"])],
            systems::System::all_known_systems().to_vec(),
        ) {
            Ok(scheduled) => scheduled,
            Err(_) => panic!("classification should succeed"),
        };

        assert!(scheduled.is_empty());
    }

    #[test]
    fn automatic_build_classification_errors_are_not_treated_as_irrelevant() {
        let result = classify_builds(
            &nix(),
            &fixture("instantiable-broken"),
            vec![build(&["package"])],
            vec![systems::System::X8664Linux],
        );

        assert!(matches!(result, Err(eval::Error::Fail(_))));
    }
}
