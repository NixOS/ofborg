/// This is what evaluates every pull-request
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

use futures::stream::StreamExt;
use octocrab::{Octocrab, models::StatusState};
use tracing::{Instrument, debug_span, error, info, warn};

use crate::commitstatus::{CommitStatus, CommitStatusError};
use crate::config::GithubAppVendingMachine;
use crate::github::GithubRepo;
use crate::message::{buildjob, evaluationjob, hydra_eval_job};
use crate::nix;
use crate::stats::{self, Event};
use crate::tasks::eval;
use crate::tasks::eval::EvaluationStrategy;
use crate::{checkout, worker};
use uuid::Uuid;

pub struct EvaluationWorker<E> {
    cloner: checkout::CachedCloner,
    github_vend: Option<tokio::sync::RwLock<GithubAppVendingMachine>>,
    identity: String,
    events: E,
    hydra_eval_queue: Option<String>,
    hydra_eval_nix: Option<nix::Nix>,
    hydra_eval_jobset_id: Option<i32>,
}

impl<E: stats::SysEvents> EvaluationWorker<E> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        cloner: checkout::CachedCloner,
        github_vend: Option<GithubAppVendingMachine>,
        identity: String,
        events: E,
        hydra_eval_queue: Option<String>,
        hydra_eval_nix: Option<nix::Nix>,
        hydra_eval_jobset_id: Option<i32>,
    ) -> EvaluationWorker<E> {
        EvaluationWorker {
            cloner,
            github_vend: github_vend.map(tokio::sync::RwLock::new),
            identity,
            events,
            hydra_eval_queue,
            hydra_eval_nix,
            hydra_eval_jobset_id,
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
        async {
            let github_client = if let Some(github_vend) = self.github_vend.as_ref() {
                let mut vending_machine = github_vend.write().await;
                match vending_machine
                    .for_repo(&job.repo.owner, &job.repo.name)
                    .await
                {
                    Some(client) => Some(client.clone()),
                    None => {
                        error!(
                            "Failed to get a github client token for {}/{}",
                            job.repo.owner, job.repo.name
                        );
                        return vec![worker::Action::NackRequeue];
                    }
                }
            } else {
                None
            };

            OneEval::new(
                github_client,
                &mut self.events,
                &self.identity,
                &self.cloner,
                job,
                self.hydra_eval_queue.clone(),
                self.hydra_eval_nix.clone(),
                self.hydra_eval_jobset_id,
            )
            .worker_actions()
            .await
        }
        .instrument(span)
        .await
    }
}

struct OneEval<'a, E> {
    repo: GithubRepo,
    enable_publish: bool,
    events: &'a mut E,
    identity: &'a str,
    cloner: &'a checkout::CachedCloner,
    job: &'a evaluationjob::EvaluationJob,
    prefix: Option<&'static str>,
    hydra_eval_queue: Option<String>,
    hydra_eval_nix: Option<nix::Nix>,
    hydra_eval_jobset_id: Option<i32>,
}

impl<'a, E: stats::SysEvents + 'static> OneEval<'a, E> {
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::borrow_as_ptr)]
    fn new(
        octocrab: Option<Octocrab>,
        events: &'a mut E,
        identity: &'a str,
        cloner: &'a checkout::CachedCloner,
        job: &'a evaluationjob::EvaluationJob,
        hydra_eval_queue: Option<String>,
        hydra_eval_nix: Option<nix::Nix>,
        hydra_eval_jobset_id: Option<i32>,
    ) -> OneEval<'a, E> {
        let (repo, enable_publish) = if let Some(octocrab) = octocrab {
            (
                GithubRepo::new(octocrab, job.repo.owner.clone(), job.repo.name.clone()),
                true,
            )
        } else {
            let octocrab = Octocrab::builder().build().unwrap();
            (
                GithubRepo::new(octocrab, job.repo.owner.clone(), job.repo.name.clone()),
                false,
            )
        };
        OneEval {
            repo,
            enable_publish,
            events,
            identity,
            cloner,
            job,
            prefix: None,
            hydra_eval_queue,
            hydra_eval_nix,
            hydra_eval_jobset_id,
        }
    }

    fn actions(&self) -> evaluationjob::Actions {
        evaluationjob::Actions {}
    }

    async fn update_status(
        &self,
        description: String,
        url: Option<String>,
        state: StatusState,
    ) -> Result<(), CommitStatusError> {
        if !self.enable_publish {
            return Ok(());
        }

        let prefix = self
            .prefix
            .expect("prefix should have been set in worker_actions");

        info!(
            "Updating status on {}:{} -> {}",
            &self.job.pr.number, &self.job.pr.head_sha, &description
        );

        let status = CommitStatus::new(
            self.repo.clone(),
            self.job.pr.head_sha.clone(),
            format!("{prefix}-eval"),
            description,
            url,
        );

        status.set(state).await
    }

    async fn worker_actions(&mut self) -> worker::Actions {
        self.prefix = Some(if self.enable_publish {
            match self.repo.get_prefix(&self.job.pr.head_sha).await {
                Ok(p) => p,
                Err(e) => {
                    error!("Failed to determine commit status prefix: {:?}", e);
                    return self.actions().retry_later(self.job);
                }
            }
        } else {
            "ofborg"
        });

        let eval_result = match self.evaluate_job().await {
            Ok(v) => Ok(v),
            Err(eval_error) => match eval_error {
                // Handle error cases which expect us to post statuses
                // to github. Convert Eval Errors in to Result<_, CommitStatusWrite>
                EvalWorkerError::EvalError(eval::Error::Fail(msg)) => {
                    Err(self.update_status(msg, None, StatusState::Failure).await)
                }
                EvalWorkerError::EvalError(eval::Error::CommitStatusWrite(e)) => Err(Err(e)),
                EvalWorkerError::CommitStatusWrite(e) => Err(Err(e)),
            },
        };

        match eval_result {
            Ok(eval_actions) => {
                if self.enable_publish
                    && let Ok(issue) = self.repo.issues().get(self.job.pr.number).await
                    && let Err(e) = self
                        .repo
                        .update_labels(issue.number, &[], &[String::from("ofborg-internal-error")])
                        .await
                {
                    warn!("Failed to update labels: {e:?}");
                }

                eval_actions
            }
            Err(Ok(())) => {
                // There was an error during eval, but we successfully
                // updated the PR.

                if self.enable_publish
                    && let Ok(issue) = self.repo.issues().get(self.job.pr.number).await
                    && let Err(e) = self
                        .repo
                        .update_labels(issue.number, &[], &[String::from("ofborg-internal-error")])
                        .await
                {
                    warn!("Failed to update labels: {e:?}");
                }

                self.actions().skip(self.job)
            }
            Err(Err(CommitStatusError::OctocrabError(e))) => {
                error!("Failed writing commit status: {:?}", e);
                self.actions().retry_later(self.job)
            }
            Err(Err(CommitStatusError::InternalError(e))) => {
                error!("Failed writing commit status: internal error: {:?}", e);
                self.actions().retry_later(self.job)
            }
        }
    }

    async fn evaluate_job(&mut self) -> Result<worker::Actions, EvalWorkerError> {
        let job = self.job;
        let issue = if self.enable_publish {
            let issue_result = self.repo.issues().get(job.pr.number).await;

            match &issue_result {
                Ok(iss) => {
                    if iss.state == octocrab::models::IssueState::Closed {
                        self.events.notify(Event::IssueAlreadyClosed).await;
                        info!("Skipping {} because it is closed", job.pr.number);
                        return Ok(self.actions().skip(job));
                    }
                }

                Err(e) => {
                    self.events.notify(Event::IssueFetchFailed).await;
                    error!("Error fetching {}!", job.pr.number);
                    error!("E: {:?}", e);
                    return Ok(self.actions().skip(job));
                }
            };

            issue_result.ok()
        } else {
            None
        };
        let mut evaluation_strategy = eval::NixpkgsStrategy::new(job, issue.as_ref());

        let prefix = self
            .prefix
            .expect("prefix should have been set in worker_actions");

        let mut overall_status = CommitStatus::new(
            self.repo.clone(),
            job.pr.head_sha.clone(),
            format!("{prefix}-eval"),
            "Starting".to_owned(),
            None,
        );
        overall_status.set_enable_publish(self.enable_publish);

        overall_status
            .set_with_description("Starting", StatusState::Pending)
            .await?;

        if self.enable_publish {
            evaluation_strategy.pre_clone(&self.repo).await?;
        }

        let project = self
            .cloner
            .project(&job.repo.full_name, job.repo.clone_url.clone());

        overall_status
            .set_with_description("Cloning project", StatusState::Pending)
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
                    StatusState::Error,
                )
                .await?;

            info!("PR targets a nixos-* or nixpkgs-* branch");
            return Ok(self.actions().skip(job));
        };

        overall_status
            .set_with_description(
                format!("Checking out {}", &target_branch).as_ref(),
                StatusState::Pending,
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

        let target_branch_rebuild_sniff_start = Instant::now();

        evaluation_strategy
            .on_target_branch(Path::new(&refpath), &mut overall_status)
            .await?;

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
            .set_with_description("Fetching PR", StatusState::Pending)
            .await?;

        co.fetch_pr(job.pr.number).map_err(|e| {
            EvalWorkerError::CommitStatusWrite(CommitStatusError::InternalError(format!(
                "Fetching PR failed: {e}"
            )))
        })?;

        if !co.commit_exists(job.pr.head_sha.as_ref()) {
            overall_status
                .set_with_description("Commit not found", StatusState::Error)
                .await?;

            info!("Commit {} doesn't exist", job.pr.head_sha);
            return Ok(self.actions().skip(job));
        }

        evaluation_strategy.after_fetch(&co)?;

        overall_status
            .set_with_description("Merging PR", StatusState::Pending)
            .await?;

        if co.merge_commit(job.pr.head_sha.as_ref()).is_err() {
            overall_status
                .set_with_description("Failed to merge", StatusState::Failure)
                .await?;

            info!("Failed to merge {}", job.pr.head_sha);

            return Ok(self.actions().skip(job));
        }

        evaluation_strategy.after_merge(&mut overall_status).await?;

        info!("Got path: {:?}, building", refpath);
        overall_status
            .set_with_description("Beginning Evaluations", StatusState::Pending)
            .await?;

        let enable_publish = self.enable_publish;
        let eval_results: bool = futures::stream::iter(evaluation_strategy.evaluation_checks())
            .map(|check| {
                let repo = self.repo.clone();
                let head_sha = job.pr.head_sha.clone();
                let refpath = refpath.clone();

                async move {
                    let mut status = CommitStatus::new(
                        repo,
                        head_sha,
                        format!("{prefix}-eval-{}", check.name()),
                        check.cli_cmd(),
                        None,
                    );
                    status.set_enable_publish(enable_publish);

                    if let Err(e) = status.set(StatusState::Pending).await {
                        warn!("Failed to set pending status on eval strategy: {e:?}");
                    }

                    let state = match check.execute(Path::new(&refpath)) {
                        Ok(_) => StatusState::Success,
                        Err(_) => StatusState::Failure,
                    };

                    if let Err(e) = status.set(state).await {
                        warn!("Failed to set status on eval strategy: {e:?}");
                    }

                    if state == StatusState::Success {
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

            if let (Some(ref queue), Some(ref nix), Some(jobset_id)) = (
                self.hydra_eval_queue.clone(),
                self.hydra_eval_nix.clone(),
                self.hydra_eval_jobset_id,
            ) {
                let drv_paths = resolve_attrs_to_drv_paths(
                    nix,
                    std::path::Path::new(&refpath),
                    &complete.builds,
                );
                if !drv_paths.is_empty() {
                    info!(
                        "Publishing {} drv paths to hydra-eval-jobs for PR #{}",
                        drv_paths.len(),
                        job.pr.number
                    );
                    response.push(worker::publish_serde_action(
                        None,
                        Some(queue.clone()),
                        &hydra_eval_job::HydraEvalJob {
                            repo: job.repo.clone(),
                            pr: job.pr.clone(),
                            drv_paths,
                            request_id: Uuid::new_v4().to_string(),
                            jobset_id,
                        },
                    ));
                }
            }

            overall_status
                .set_with_description("^.^!", StatusState::Success)
                .await?;
        } else {
            overall_status
                .set_with_description("Complete, with errors", StatusState::Failure)
                .await?;
        }

        self.events.notify(Event::TaskEvaluationCheckComplete).await;

        info!("Evaluations done!");
        Ok(self.actions().done(job, response))
    }
}

fn resolve_attrs_to_drv_paths(
    nix: &nix::Nix,
    nixpkgs: &std::path::Path,
    builds: &[buildjob::BuildJob],
) -> Vec<String> {
    let mut all_attrs: Vec<String> = builds.iter().flat_map(|b| b.attrs.clone()).collect();
    all_attrs.sort();
    all_attrs.dedup();

    if all_attrs.is_empty() {
        return vec![];
    }

    let file = builds
        .first()
        .and_then(|b| b.subset.clone())
        .map(|s| match s {
            crate::commentparser::Subset::NixOS => nix::File::ReleaseNixOS,
            crate::commentparser::Subset::Nixpkgs => nix::File::DefaultNixpkgs,
        })
        .unwrap_or(nix::File::DefaultNixpkgs);

    // Try batch instantiation first for performance (single nix-instantiate process)
    match nix.safely_instantiate_attrs(nixpkgs, file, all_attrs.clone()) {
        Ok(f) => {
            return BufReader::new(f)
                .lines()
                .map_while(Result::ok)
                .filter(|line| line.trim().ends_with(".drv"))
                .map(|line| line.trim().to_owned())
                .collect();
        }
        Err(_) => warn!("Batch instantiation failed, falling back to per-attr fallback"),
    }

    // Fallback: try each attr individually
    all_attrs
        .into_iter()
        .flat_map(
            |attr| match nix.safely_instantiate_attrs(nixpkgs, file, vec![attr.clone()]) {
                Ok(f) => BufReader::new(f)
                    .lines()
                    .map_while(Result::ok)
                    .filter(|line| line.trim().ends_with(".drv"))
                    .map(|line| line.trim().to_owned())
                    .collect::<Vec<String>>(),
                Err(f) => {
                    let stderr: Vec<String> =
                        BufReader::new(f).lines().map_while(Result::ok).collect();
                    warn!(
                        "nix-instantiate failed for attr '{attr}': {:?}",
                        stderr.join("\n")
                    );
                    vec![]
                }
            },
        )
        .collect()
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
