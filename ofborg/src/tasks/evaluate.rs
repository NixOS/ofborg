/// This is what evaluates every pull-request
use crate::acl::ACL;
use crate::checkout;
use crate::commitstatus::{CommitStatus, CommitStatusError};
use crate::config::GithubAppVendingMachine;
use crate::files::file_to_str;
use crate::message::{buildjob, evaluationjob};
use crate::nix;
use crate::stats::{self, Event};
use crate::systems;
use crate::tasks::eval;
use crate::worker;

use std::collections::HashMap;
use std::path::Path;
use std::sync::RwLock;
use std::time::Instant;

use hubcaps::checks::CheckRunOptions;
use hubcaps::gists::Gists;
use hubcaps::issues::Issue;
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

        OneEval::new(
            github_client,
            &self.github,
            &self.nix,
            &self.acl,
            &mut self.events,
            &self.identity,
            &self.tag_paths,
            &self.cloner,
            job,
        )
        .worker_actions()
    }
}

struct OneEval<'a, E> {
    client_app: &'a hubcaps::Github,
    repo: hubcaps::repositories::Repository<'a>,
    gists: Gists<'a>,
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
        let gists = client_legacy.gists();

        let repo = client_app.repo(job.repo.owner.clone(), job.repo.name.clone());
        OneEval {
            client_app,
            repo,
            gists,
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
        builder.context("ofborg-eval");
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
            .map(|_| ())
            .map_err(|e| CommitStatusError::from(e))
    }

    fn make_gist(
        &self,
        filename: &str,
        description: Option<String>,
        content: String,
    ) -> Option<String> {
        make_gist(&self.gists, filename, description, content)
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
                    self.make_gist(&filename, Some("".to_owned()), content),
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
                let issue_ref = self.repo.issue(self.job.pr.number);
                update_labels(&issue_ref, &[String::from("ofborg-internal-error")], &[]);

                self.actions().skip(&self.job)
            }
        }
    }

    // FIXME: remove with rust/cargo update
    #[allow(clippy::cognitive_complexity)]
    fn evaluate_job(&mut self) -> Result<worker::Actions, EvalWorkerError> {
        let job = self.job;
        let repo = self
            .client_app
            .repo(self.job.repo.owner.clone(), self.job.repo.name.clone());
        let pulls = repo.pulls();
        let pull = pulls.get(job.pr.number);
        let issue_ref = repo.issue(job.pr.number);
        let issue: Issue;
        let auto_schedule_build_archs: Vec<systems::System>;

        match issue_ref.get() {
            Ok(iss) => {
                if iss.state == "closed" {
                    self.events.notify(Event::IssueAlreadyClosed);
                    info!("Skipping {} because it is closed", job.pr.number);
                    return Ok(self.actions().skip(&job));
                }

                if issue_is_wip(&iss) {
                    auto_schedule_build_archs = vec![];
                } else {
                    auto_schedule_build_archs = self.acl.build_job_architectures_for_user_repo(
                        &iss.user.login,
                        &job.repo.full_name,
                    );
                }

                issue = iss;
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
                &job,
                &pull,
                &issue,
                &issue_ref,
                &repo,
                &self.gists,
                self.nix.clone(),
                &self.tag_paths,
            ))
        } else {
            Box::new(eval::GenericStrategy::new())
        };

        let mut overall_status = CommitStatus::new(
            repo.statuses(),
            job.pr.head_sha.clone(),
            "ofborg-eval".to_owned(),
            "Starting".to_owned(),
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
                let mut status = CommitStatus::new(
                    repo.statuses(),
                    job.pr.head_sha.clone(),
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
                        gist_url = self.make_gist(
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

            send_check_statuses(complete.checks, &repo);
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
}

fn send_check_statuses(checks: Vec<CheckRunOptions>, repo: &hubcaps::repositories::Repository) {
    for check in checks {
        match repo.checkruns().create(&check) {
            Ok(_) => debug!("Sent check update"),
            Err(e) => warn!("Failed to send check update: {:?}", e),
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

pub fn make_gist<'a>(
    gists: &hubcaps::gists::Gists<'a>,
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
        gists
            .create(&hubcaps::gists::GistOptions {
                description,
                public: Some(true),
                files,
            })
            .expect("Failed to create gist!")
            .html_url,
    )
}

pub fn update_labels(issueref: &hubcaps::issues::IssueRef, add: &[String], remove: &[String]) {
    let l = issueref.labels();
    let issue = issueref.get().expect("Failed to get issue");

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

    l.add(to_add.clone()).unwrap_or_else(|e| {
        panic!(
            "Failed to add labels {:?} to issue #{}: {:?}",
            to_add, issue.number, e
        )
    });

    for label in to_remove {
        l.remove(&label).unwrap_or_else(|e| {
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
