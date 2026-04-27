/// This is what evaluates every pull-request
use std::path::Path;
use std::time::Instant;

use futures::stream::StreamExt;
use octocrab::{Octocrab, models::StatusState};
use tracing::{Instrument, debug_span, error, info, warn};

use crate::acl::Acl;
use crate::commitstatus::{CommitStatus, CommitStatusError};
use crate::config::GithubAppVendingMachine;
use crate::github::GithubRepo;
use crate::message::{buildjob, evaluationjob};
use crate::stats::{self, Event};
use crate::tasks::eval;
use crate::tasks::eval::EvaluationStrategy;
use crate::{checkout, systems, worker};

pub struct EvaluationWorker<E> {
    cloner: checkout::CachedCloner,
    github_vend: tokio::sync::RwLock<GithubAppVendingMachine>,
    acl: Acl,
    identity: String,
    events: E,
}

impl<E: stats::SysEvents> EvaluationWorker<E> {
    pub fn new(
        cloner: checkout::CachedCloner,
        github_vend: GithubAppVendingMachine,
        acl: Acl,
        identity: String,
        events: E,
    ) -> EvaluationWorker<E> {
        EvaluationWorker {
            cloner,
            github_vend: tokio::sync::RwLock::new(github_vend),
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
        async {
            let github_client = {
                let mut vending_machine = self.github_vend.write().await;
                match vending_machine
                    .for_repo(&job.repo.owner, &job.repo.name)
                    .await
                {
                    Some(client) => client.clone(),
                    None => {
                        error!(
                            "Failed to get a github client token for {}/{}",
                            job.repo.owner, job.repo.name
                        );
                        return vec![worker::Action::NackRequeue];
                    }
                }
            };

            OneEval::new(
                &github_client,
                &self.acl,
                &mut self.events,
                &self.identity,
                &self.cloner,
                job,
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
    acl: &'a Acl,
    events: &'a mut E,
    identity: &'a str,
    cloner: &'a checkout::CachedCloner,
    job: &'a evaluationjob::EvaluationJob,
    prefix: Option<&'static str>,
}

impl<'a, E: stats::SysEvents + 'static> OneEval<'a, E> {
    #[allow(clippy::too_many_arguments)]
    fn new(
        octocrab: &'a Octocrab,
        acl: &'a Acl,
        events: &'a mut E,
        identity: &'a str,
        cloner: &'a checkout::CachedCloner,
        job: &'a evaluationjob::EvaluationJob,
    ) -> OneEval<'a, E> {
        OneEval {
            repo: GithubRepo::new(
                octocrab.clone(),
                job.repo.owner.clone(),
                job.repo.name.clone(),
            ),
            acl,
            events,
            identity,
            cloner,
            job,
            prefix: None,
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
        self.prefix = Some(match self.repo.get_prefix(&self.job.pr.head_sha).await {
            Ok(p) => p,
            Err(e) => {
                error!("Failed to determine commit status prefix: {:?}", e);
                return self.actions().retry_later(self.job);
            }
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
                let issue = self.repo.issues().get(self.job.pr.number).await;
                if let Ok(issue) = issue
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

                let issue = self.repo.issues().get(self.job.pr.number).await;
                if let Ok(issue) = issue
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
        let auto_schedule_build_archs: Vec<systems::System>;

        let issue = self.repo.issues().get(job.pr.number).await;

        match issue {
            Ok(iss) => {
                if iss.state == octocrab::models::IssueState::Closed {
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

        let issue = self.repo.issues().get(job.pr.number).await.ok();

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

        overall_status
            .set_with_description("Starting", StatusState::Pending)
            .await?;

        evaluation_strategy.pre_clone(&self.repo).await?;

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

        let eval_results: bool = futures::stream::iter(evaluation_strategy.evaluation_checks())
            .map(|check| {
                let repo = self.repo.clone();
                let head_sha = job.pr.head_sha.clone();
                let refpath = refpath.clone();

                async move {
                    let status = CommitStatus::new(
                        repo,
                        head_sha,
                        format!("{prefix}-eval-{}", check.name()),
                        check.cli_cmd(),
                        None,
                    );

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

            response.extend(schedule_builds(complete.builds, auto_schedule_build_archs));

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

fn schedule_builds(
    builds: Vec<buildjob::BuildJob>,
    auto_schedule_build_archs: Vec<systems::System>,
) -> Vec<worker::Action> {
    let mut response = vec![];
    info!(
        "Scheduling build jobs {:?} on arches {:?}",
        builds, auto_schedule_build_archs
    );
    for buildjob in builds {
        for arch in auto_schedule_build_archs.iter() {
            let (exchange, routingkey) = arch.as_build_destination();
            response.push(worker::publish_serde_action(
                exchange, routingkey, &buildjob,
            ));
        }
        response.push(worker::publish_serde_action(
            Some("build-results".to_string()),
            None,
            &buildjob::QueuedBuildJobs {
                job: buildjob,
                architectures: auto_schedule_build_archs
                    .iter()
                    .map(|arch| arch.to_string())
                    .collect(),
            },
        ));
    }

    response
}

fn issue_is_wip(issue: &octocrab::models::issues::Issue) -> bool {
    issue.title.starts_with("WIP:") || issue.title.contains("[WIP]")
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
