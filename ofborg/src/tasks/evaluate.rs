/// This is what evaluates every pull-request
use crate::acl::ACL;
use crate::checkout;
use crate::commitstatus::CommitStatusError;
use crate::config::GithubAppVendingMachine;
use crate::files::file_to_str;
use crate::ghgist;
use crate::ghrepo;
use crate::message::{buildjob, evaluationjob, Pr};
use crate::nix;
use crate::stats::{self, Event};
use crate::systems;
use crate::tasks::eval;
use crate::worker;

use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;
use std::sync::RwLock;
use std::time::Instant;

use hubcaps::checks::CheckRunOptions;
use tracing::{debug, debug_span, error, info, warn};

pub struct EvaluationWorker<E> {
    cloner: checkout::CachedCloner,
    nix: nix::Nix,
    github: hubcaps::Github,
    github_vend: RwLock<GithubAppVendingMachine>,
    acl: ACL,
    identity: String,
    events: E,
    tag_paths: HashMap<String, Vec<String>>,
}

impl<E: stats::SysEvents> EvaluationWorker<E> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        cloner: checkout::CachedCloner,
        nix: &nix::Nix,
        github: hubcaps::Github,
        github_vend: GithubAppVendingMachine,
        acl: ACL,
        identity: String,
        events: E,
        tag_paths: HashMap<String, Vec<String>>,
    ) -> EvaluationWorker<E> {
        EvaluationWorker {
            cloner,
            nix: nix.without_limited_supported_systems(),
            github,
            github_vend: RwLock::new(github_vend),
            acl,
            identity,
            events,
            tag_paths,
        }
    }
}

impl<E: stats::SysEvents + 'static> worker::SimpleWorker for EvaluationWorker<E> {
    type J = evaluationjob::EvaluationJob;

    fn msg_to_job(&mut self, _: &str, _: &Option<String>, body: &[u8]) -> Result<Self::J, String> {
        self.events.notify(Event::JobReceived);
        match evaluationjob::from(body) {
            Ok(e) => {
                self.events.notify(Event::JobDecodeSuccess);
                Ok(e)
            }
            Err(e) => {
                self.events.notify(Event::JobDecodeFailure);
                error!(
                    "Failed to decode message: {:?}, Err: {:?}",
                    String::from_utf8(body.to_vec()),
                    e
                );
                Err("Failed to decode message".to_owned())
            }
        }
    }

    fn consumer(&mut self, job: &evaluationjob::EvaluationJob) -> worker::Actions {
        let span = debug_span!("job", pr = ?job.pr.number);
        let _enter = span.enter();

        let mut vending_machine = self
            .github_vend
            .write()
            .expect("Failed to get write lock on github vending machine");

        let github_client = vending_machine
            .for_repo(&job.repo.owner, &job.repo.name)
            .expect("Failed to get a github client token");

        let mut eval = OneEval::new(
            github_client,
            &self.github,
            &self.nix,
            &self.acl,
            &mut self.events,
            &self.identity,
            &self.tag_paths,
            &self.cloner,
            job,
        );
        eval.worker_actions()
    }
}

struct OneEval<'a, E> {
    repo_client: Rc<dyn ghrepo::Client + 'a>,
    gist_client: Rc<dyn ghgist::Client + 'a>,
    nix: &'a nix::Nix,
    acl: &'a ACL,
    events: &'a mut E,
    identity: &'a str,
    tag_paths: &'a HashMap<String, Vec<String>>,
    cloner: &'a checkout::CachedCloner,
    job: &'a evaluationjob::EvaluationJob,
}

impl<'a, E: stats::SysEvents + 'static> OneEval<'a, E> {
    #[allow(clippy::too_many_arguments)]
    fn new(
        client_app: &'a hubcaps::Github,
        client_legacy: &'a hubcaps::Github,
        nix: &'a nix::Nix,
        acl: &'a ACL,
        events: &'a mut E,
        identity: &'a str,
        tag_paths: &'a HashMap<String, Vec<String>>,
        cloner: &'a checkout::CachedCloner,
        job: &'a evaluationjob::EvaluationJob,
    ) -> OneEval<'a, E> {
        let gist_client = ghgist::Hubcaps::new(client_legacy);
        let repo_client = ghrepo::Hubcaps::new(client_app, &job.repo);
        OneEval {
            repo_client: Rc::new(repo_client),
            gist_client: Rc::new(gist_client),
            nix,
            acl,
            events,
            identity,
            tag_paths,
            cloner,
            job,
        }
    }

    fn actions(&self) -> evaluationjob::Actions {
        evaluationjob::Actions {}
    }

    fn update_status(
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

        let mut builder = hubcaps::statuses::StatusOptions::builder(state);
        builder.context("grahamcofborg-eval");
        builder.description(description.clone());

        if let Some(url) = url {
            builder.target_url(url);
        }

        info!(
            "Updating status on {}:{} -> {}",
            &self.job.pr.number, &self.job.pr.head_sha, &description
        );

        self.repo_client
            .create_status(&self.job.pr.head_sha, &builder.build())
            .map_err(|e| CommitStatusError::from(e))?;
        Ok(())
    }

    fn worker_actions(&mut self) -> worker::Actions {
        let eval_result = self.evaluate_job().map_err(|eval_error| match eval_error {
            // Handle error cases which expect us to post statuses
            // to github. Convert Eval Errors in to Result<_, CommitStatusWrite>
            EvalWorkerError::EvalError(eval::Error::Fail(msg)) => {
                self.update_status(msg, None, hubcaps::statuses::State::Failure)
            }
            EvalWorkerError::EvalError(eval::Error::FailWithGist(msg, filename, content)) => self
                .update_status(
                    msg,
                    make_gist(
                        self.gist_client.as_ref(),
                        &filename,
                        Some("".to_owned()),
                        content,
                    ),
                    hubcaps::statuses::State::Failure,
                ),
            EvalWorkerError::EvalError(eval::Error::CommitStatusWrite(e)) => Err(e),
            EvalWorkerError::CommitStatusWrite(e) => Err(e),
        });

        match eval_result {
            Ok(eval_actions) => eval_actions,
            Err(Ok(())) => {
                // There was an error during eval, but we successfully
                // updated the PR.

                self.actions().skip(&self.job)
            }
            Err(Err(CommitStatusError::ExpiredCreds(e))) => {
                error!("Failed writing commit status: creds expired: {:?}", e);
                self.actions().retry_later(&self.job)
            }
            Err(Err(CommitStatusError::MissingSHA(e))) => {
                error!(
                    "Failed writing commit status: commit sha was force-pushed away: {:?}",
                    e
                );
                self.actions().skip(&self.job)
            }

            Err(Err(CommitStatusError::Error(cswerr))) => {
                error!(
                    "Internal error writing commit status: {:?}, marking internal error",
                    cswerr
                );
                update_labels(
                    self.repo_client.as_ref(),
                    &self.job.pr,
                    &[String::from("ofborg-internal-error")],
                    &[],
                );

                self.actions().skip(&self.job)
            }
        }
    }

    // FIXME: remove with rust/cargo update
    #[allow(clippy::cognitive_complexity)]
    fn evaluate_job(&mut self) -> Result<worker::Actions, EvalWorkerError> {
        let job = self.job;
        let auto_schedule_build_archs: Vec<systems::System>;

        match self.repo_client.get_issue(job.pr.number) {
            Ok(issue) => {
                if issue.state == "closed" {
                    self.events.notify(Event::IssueAlreadyClosed);
                    info!("Skipping {} because it is closed", job.pr.number);
                    return Ok(self.actions().skip(&job));
                }

                if issue_is_wip(&issue) {
                    auto_schedule_build_archs = vec![];
                } else {
                    auto_schedule_build_archs = self.acl.build_job_architectures_for_user_repo(
                        &issue.user.login,
                        &job.repo.full_name,
                    );
                }
            }

            Err(e) => {
                self.events.notify(Event::IssueFetchFailed);
                info!("Error fetching {}!", job.pr.number);
                info!("E: {:?}", e);
                return Ok(self.actions().skip(&job));
            }
        };

        let mut evaluation_strategy: Box<dyn eval::EvaluationStrategy> = if job.is_nixpkgs() {
            Box::new(eval::NixpkgsStrategy::new(
                self.repo_client.clone(),
                self.gist_client.clone(),
                &job,
                self.nix.clone(),
                &self.tag_paths,
            ))
        } else {
            Box::new(eval::GenericStrategy::new())
        };

        let mut overall_status = self.repo_client.create_commitstatus(
            &job.pr,
            "grahamcofborg-eval".to_string(),
            "Starting".to_string(),
            None,
        );

        overall_status.set_with_description("Starting", hubcaps::statuses::State::Pending)?;

        evaluation_strategy.pre_clone()?;

        let project = self
            .cloner
            .project(&job.repo.full_name, job.repo.clone_url.clone());

        overall_status
            .set_with_description("Cloning project", hubcaps::statuses::State::Pending)?;

        info!("Working on {}", job.pr.number);
        let co = project
            .clone_for("mr-est".to_string(), self.identity.to_string())
            .unwrap();

        let target_branch = match job.pr.target_branch.clone() {
            Some(x) => x,
            None => String::from("master"),
        };

        overall_status.set_with_description(
            format!("Checking out {}", &target_branch).as_ref(),
            hubcaps::statuses::State::Pending,
        )?;
        info!("Checking out target branch {}", &target_branch);
        let refpath = co.checkout_origin_ref(target_branch.as_ref()).unwrap();

        evaluation_strategy.on_target_branch(&Path::new(&refpath), &mut overall_status)?;

        let target_branch_rebuild_sniff_start = Instant::now();

        self.events.notify(Event::EvaluationDuration(
            target_branch.clone(),
            target_branch_rebuild_sniff_start.elapsed().as_secs(),
        ));
        self.events
            .notify(Event::EvaluationDurationCount(target_branch));

        overall_status.set_with_description("Fetching PR", hubcaps::statuses::State::Pending)?;

        co.fetch_pr(job.pr.number).unwrap();

        if !co.commit_exists(job.pr.head_sha.as_ref()) {
            overall_status
                .set_with_description("Commit not found", hubcaps::statuses::State::Error)?;

            info!("Commit {} doesn't exist", job.pr.head_sha);
            return Ok(self.actions().skip(&job));
        }

        evaluation_strategy.after_fetch(&co)?;

        overall_status.set_with_description("Merging PR", hubcaps::statuses::State::Pending)?;

        if co.merge_commit(job.pr.head_sha.as_ref()).is_err() {
            overall_status
                .set_with_description("Failed to merge", hubcaps::statuses::State::Failure)?;

            info!("Failed to merge {}", job.pr.head_sha);

            evaluation_strategy.merge_conflict();

            return Ok(self.actions().skip(&job));
        }

        evaluation_strategy.after_merge(&mut overall_status)?;

        info!("Got path: {:?}, building", refpath);
        overall_status
            .set_with_description("Beginning Evaluations", hubcaps::statuses::State::Pending)?;

        let eval_results: bool = evaluation_strategy
            .evaluation_checks()
            .into_iter()
            .map(|check| {
                let mut status = self.repo_client.create_commitstatus(
                    &job.pr,
                    check.name(),
                    check.cli_cmd(),
                    None,
                );

                status
                    .set(hubcaps::statuses::State::Pending)
                    .expect("Failed to set status on eval strategy");

                let state: hubcaps::statuses::State;
                let gist_url: Option<String>;
                match check.execute(Path::new(&refpath)) {
                    Ok(_) => {
                        state = hubcaps::statuses::State::Success;
                        gist_url = None;
                    }
                    Err(mut out) => {
                        state = hubcaps::statuses::State::Failure;
                        gist_url = make_gist(
                            self.gist_client.as_ref(),
                            &check.name(),
                            Some(format!("{:?}", state)),
                            file_to_str(&mut out),
                        );
                    }
                }

                status.set_url(gist_url);
                status
                    .set(state.clone())
                    .expect("Failed to set status on eval strategy");

                if state == hubcaps::statuses::State::Success {
                    Ok(())
                } else {
                    Err(())
                }
            })
            .all(|status| status == Ok(()));

        info!("Finished evaluations");
        let mut response: worker::Actions = vec![];

        if eval_results {
            let complete = evaluation_strategy
                .all_evaluations_passed(&Path::new(&refpath), &mut overall_status)?;

            self.send_check_statuses(complete.checks);
            response.extend(schedule_builds(complete.builds, auto_schedule_build_archs));

            overall_status.set_with_description("^.^!", hubcaps::statuses::State::Success)?;
        } else {
            overall_status
                .set_with_description("Complete, with errors", hubcaps::statuses::State::Failure)?;
        }

        self.events.notify(Event::TaskEvaluationCheckComplete);

        info!("Evaluations done!");
        Ok(self.actions().done(&job, response))
    }

    fn send_check_statuses(&self, checks: Vec<CheckRunOptions>) {
        for check in checks {
            match self.repo_client.create_checkrun(&check) {
                Ok(_) => debug!("Sent check update"),
                Err(e) => warn!("Failed to send check update: {:?}", e),
            }
        }
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

pub fn make_gist(
    gist_client: &dyn ghgist::Client,
    name: &str,
    description: Option<String>,
    contents: String,
) -> Option<String> {
    let mut files: HashMap<String, hubcaps::gists::Content> = HashMap::new();
    files.insert(
        name.to_string(),
        hubcaps::gists::Content {
            filename: Some(name.to_string()),
            content: contents,
        },
    );

    Some(
        gist_client
            .create_gist(&hubcaps::gists::GistOptions {
                description,
                public: Some(true),
                files,
            })
            .expect("Failed to create gist!")
            .html_url,
    )
}

pub fn update_labels(repo_client: &dyn ghrepo::Client, pr: &Pr, add: &[String], remove: &[String]) {
    let issue = repo_client
        .get_issue(pr.number)
        .expect("Failed to get issue");

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

    info!(
        "Labeling issue #{}: + {:?} , - {:?}, = {:?}",
        issue.number, to_add, to_remove, existing
    );

    repo_client
        .add_labels(pr.number, to_add.clone())
        .unwrap_or_else(|e| {
            panic!(
                "Failed to add labels {:?} to issue #{}: {:?}",
                to_add, issue.number, e
            )
        });

    for label in to_remove {
        repo_client
            .remove_label(pr.number, &label)
            .unwrap_or_else(|e| {
                panic!(
                    "Failed to remove label {:?} from issue #{}: {:?}",
                    label, issue.number, e
                )
            });
    }
}

fn issue_is_wip(issue: &hubcaps::issues::Issue) -> bool {
    if issue.title.contains("[WIP]") {
        return true;
    }

    if issue.title.starts_with("WIP:") {
        return true;
    }

    issue.labels.iter().any(|label| indicates_wip(&label.name))
}

fn indicates_wip(text: &str) -> bool {
    let text = text.to_lowercase();

    if text.contains("work in progress") {
        return true;
    }

    if text.contains("work-in-progress") {
        return true;
    }

    false
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
