use crate::checkout;
use crate::commentparser;
use crate::message::buildresult::{BuildResult, BuildStatus, V1Tag};
use crate::message::{buildjob, buildlogmsg};
use crate::nix;
use crate::notifyworker;
use crate::worker;

use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use tracing::{debug, debug_span, error, info};
use uuid::Uuid;

pub struct BuildWorker {
    cloner: checkout::CachedCloner,
    nix: nix::Nix,
    system: String,
    identity: String,
}

impl BuildWorker {
    pub fn new(
        cloner: checkout::CachedCloner,
        nix: nix::Nix,
        system: String,
        identity: String,
    ) -> BuildWorker {
        BuildWorker {
            cloner,
            nix,
            system,
            identity,
        }
    }

    fn actions(
        &self,
        job: buildjob::BuildJob,
        receiver: Arc<
            dyn notifyworker::NotificationReceiver + std::marker::Send + std::marker::Sync,
        >,
    ) -> JobActions {
        JobActions::new(&self.system, &self.identity, job, receiver)
    }
}

pub struct JobActions {
    system: String,
    identity: String,
    receiver: Arc<dyn notifyworker::NotificationReceiver + std::marker::Send + std::marker::Sync>,
    job: buildjob::BuildJob,
    line_counter: AtomicU64,
    snippet_log: parking_lot::RwLock<VecDeque<String>>,
    attempt_id: String,
    log_exchange: Option<String>,
    log_routing_key: Option<String>,
    result_exchange: Option<String>,
    result_routing_key: Option<String>,
}

impl JobActions {
    pub fn new(
        system: &str,
        identity: &str,
        job: buildjob::BuildJob,
        receiver: Arc<
            dyn notifyworker::NotificationReceiver + std::marker::Send + std::marker::Sync,
        >,
    ) -> JobActions {
        let (log_exchange, log_routing_key) = job
            .logs
            .clone()
            .unwrap_or((Some(String::from("logs")), Some(String::from("build.log"))));

        let (result_exchange, result_routing_key) = job
            .statusreport
            .clone()
            .unwrap_or((Some(String::from("build-results")), None));

        JobActions {
            system: system.to_owned(),
            identity: identity.to_owned(),
            receiver,
            job,
            line_counter: 0.into(),
            snippet_log: parking_lot::RwLock::new(VecDeque::with_capacity(10)),
            attempt_id: Uuid::new_v4().to_string(),
            log_exchange,
            log_routing_key,
            result_exchange,
            result_routing_key,
        }
    }

    pub fn log_snippet(&self) -> Vec<String> {
        self.snippet_log.read().clone().into()
    }

    pub async fn pr_head_missing(&self) {
        self.tell(worker::Action::Ack).await;
    }

    pub async fn commit_missing(&self) {
        self.tell(worker::Action::Ack).await;
    }

    pub async fn nothing_to_do(&self) {
        self.tell(worker::Action::Ack).await;
    }

    pub async fn merge_failed(&self) {
        let msg = BuildResult::V1 {
            tag: V1Tag::V1,
            repo: self.job.repo.clone(),
            pr: self.job.pr.clone(),
            system: self.system.clone(),
            output: vec![String::from("Merge failed")],
            attempt_id: self.attempt_id.clone(),
            request_id: self.job.request_id.clone(),
            attempted_attrs: None,
            skipped_attrs: None,
            status: BuildStatus::Failure,
        };

        let result_exchange = self.result_exchange.clone();
        let result_routing_key = self.result_routing_key.clone();

        self.tell(worker::publish_serde_action(
            result_exchange,
            result_routing_key,
            &msg,
        ))
        .await;
        self.tell(worker::Action::Ack).await;
    }

    pub async fn log_started(&self, can_build: Vec<String>, cannot_build: Vec<String>) {
        let msg = buildlogmsg::BuildLogStart {
            identity: self.identity.clone(),
            system: self.system.clone(),
            attempt_id: self.attempt_id.clone(),
            attempted_attrs: Some(can_build),
            skipped_attrs: Some(cannot_build),
        };

        let log_exchange = self.log_exchange.clone();
        let log_routing_key = self.log_routing_key.clone();

        self.tell(worker::publish_serde_action(
            log_exchange,
            log_routing_key,
            &msg,
        ))
        .await;
    }

    pub async fn log_instantiation_errors(&self, cannot_build: Vec<(String, Vec<String>)>) {
        for (attr, log) in cannot_build {
            self.log_line(format!("Cannot nix-instantiate `{attr}` because:"))
                .await;

            for line in log {
                self.log_line(line).await;
            }
            self.log_line("".into()).await;
        }
    }

    pub async fn log_line(&self, line: String) {
        self.line_counter.fetch_add(1, Ordering::SeqCst);

        {
            let mut snippet_log = self.snippet_log.write();
            if snippet_log.len() >= 10 {
                snippet_log.pop_front();
            }
            snippet_log.push_back(line.clone());
        }

        let msg = buildlogmsg::BuildLogMsg {
            identity: self.identity.clone(),
            system: self.system.clone(),
            attempt_id: self.attempt_id.clone(),
            line_number: self.line_counter.load(Ordering::SeqCst),
            output: line,
        };

        let log_exchange = self.log_exchange.clone();
        let log_routing_key = self.log_routing_key.clone();

        self.tell(worker::publish_serde_action(
            log_exchange,
            log_routing_key,
            &msg,
        ))
        .await;
    }

    pub async fn build_not_attempted(&self, not_attempted_attrs: Vec<String>) {
        let msg = BuildResult::V1 {
            tag: V1Tag::V1,
            repo: self.job.repo.clone(),
            pr: self.job.pr.clone(),
            system: self.system.clone(),
            output: self.log_snippet(),
            attempt_id: self.attempt_id.clone(),
            request_id: self.job.request_id.clone(),
            skipped_attrs: Some(not_attempted_attrs),
            attempted_attrs: None,
            status: BuildStatus::Skipped,
        };

        let result_exchange = self.result_exchange.clone();
        let result_routing_key = self.result_routing_key.clone();
        self.tell(worker::publish_serde_action(
            result_exchange,
            result_routing_key,
            &msg,
        ))
        .await;

        let log_exchange = self.log_exchange.clone();
        let log_routing_key = self.log_routing_key.clone();
        self.tell(worker::publish_serde_action(
            log_exchange,
            log_routing_key,
            &msg,
        ))
        .await;

        self.tell(worker::Action::Ack).await;
    }

    pub async fn build_finished(
        &self,
        status: BuildStatus,
        attempted_attrs: Vec<String>,
        not_attempted_attrs: Vec<String>,
    ) {
        let msg = BuildResult::V1 {
            tag: V1Tag::V1,
            repo: self.job.repo.clone(),
            pr: self.job.pr.clone(),
            system: self.system.clone(),
            output: self.log_snippet(),
            attempt_id: self.attempt_id.clone(),
            request_id: self.job.request_id.clone(),
            status,
            attempted_attrs: Some(attempted_attrs),
            skipped_attrs: Some(not_attempted_attrs),
        };

        let result_exchange = self.result_exchange.clone();
        let result_routing_key = self.result_routing_key.clone();
        self.tell(worker::publish_serde_action(
            result_exchange,
            result_routing_key,
            &msg,
        ))
        .await;

        let log_exchange = self.log_exchange.clone();
        let log_routing_key = self.log_routing_key.clone();
        self.tell(worker::publish_serde_action(
            log_exchange,
            log_routing_key,
            &msg,
        ))
        .await;

        self.tell(worker::Action::Ack).await;
    }

    async fn tell(&self, action: worker::Action) {
        self.receiver.tell(action).await;
    }
}

#[async_trait::async_trait]
impl notifyworker::SimpleNotifyWorker for BuildWorker {
    type J = buildjob::BuildJob;

    fn msg_to_job(&self, _: &str, _: &Option<String>, body: &[u8]) -> Result<Self::J, String> {
        info!("lmao I got a job?");
        match buildjob::from(body) {
            Ok(job) => Ok(job),
            Err(err) => {
                error!("{:?}", std::str::from_utf8(body).unwrap_or("<not utf8>"));
                panic!("{err:?}");
            }
        }
    }

    // FIXME: remove with rust/cargo update
    #[allow(clippy::cognitive_complexity)]
    async fn consumer(
        &self,
        job: buildjob::BuildJob,
        notifier: Arc<
            dyn notifyworker::NotificationReceiver + std::marker::Send + std::marker::Sync,
        >,
    ) {
        let span = debug_span!("job", pr = ?job.pr.number);
        let _enter = span.enter();

        let actions = self.actions(job, notifier);

        if actions.job.attrs.is_empty() {
            debug!("No attrs to build");
            actions.nothing_to_do().await;
            return;
        }

        info!(
            "Working on https://github.com/{}/pull/{}",
            actions.job.repo.full_name, actions.job.pr.number
        );
        let project = self.cloner.project(
            &actions.job.repo.full_name,
            actions.job.repo.clone_url.clone(),
        );
        let co = project
            .clone_for("builder".to_string(), self.identity.clone())
            .unwrap();

        let target_branch = match actions.job.pr.target_branch.clone() {
            Some(x) => x,
            None => String::from("origin/master"),
        };

        let buildfile = match actions.job.subset {
            Some(commentparser::Subset::NixOS) => nix::File::ReleaseNixOS,
            _ => nix::File::DefaultNixpkgs,
        };

        let refpath = co.checkout_origin_ref(target_branch.as_ref()).unwrap();

        if co.fetch_pr(actions.job.pr.number).is_err() {
            info!("Failed to fetch {}", actions.job.pr.number);
            actions.pr_head_missing().await;
            return;
        }

        if !co.commit_exists(actions.job.pr.head_sha.as_ref()) {
            info!("Commit {} doesn't exist", actions.job.pr.head_sha);
            actions.commit_missing().await;
            return;
        }

        if co.merge_commit(actions.job.pr.head_sha.as_ref()).is_err() {
            info!("Failed to merge {}", actions.job.pr.head_sha);
            actions.merge_failed().await;
            return;
        }

        info!(
            "Got path: {:?}, determining which ones we can build ",
            refpath
        );
        let (can_build, cannot_build) = self.nix.safely_partition_instantiable_attrs(
            refpath.as_ref(),
            buildfile,
            actions.job.attrs.clone(),
        );

        let cannot_build_attrs: Vec<String> = cannot_build
            .clone()
            .into_iter()
            .map(|(attr, _)| attr)
            .collect();

        info!(
            "Can build: '{}', Cannot build: '{}'",
            can_build.join(", "),
            cannot_build_attrs.join(", ")
        );

        actions
            .log_started(can_build.clone(), cannot_build_attrs.clone())
            .await;
        actions.log_instantiation_errors(cannot_build).await;

        if can_build.is_empty() {
            actions.build_not_attempted(cannot_build_attrs).await;
            return;
        }

        let mut spawned =
            self.nix
                .safely_build_attrs_async(refpath.as_ref(), buildfile, can_build.clone());

        while let Ok(line) = spawned.get_next_line() {
            actions.log_line(line).await;
        }

        let status = nix::wait_for_build_status(spawned);

        info!("ok built ({:?}), building", status);
        info!("Lines:");
        info!("-----8<-----");
        actions
            .log_snippet()
            .iter()
            .inspect(|x| info!("{}", x))
            .next_back();
        info!("----->8-----");

        actions
            .build_finished(status, can_build, cannot_build_attrs)
            .await;
        info!("Build done!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::{Pr, Repo};
    use crate::notifyworker::SimpleNotifyWorker;
    use crate::test_scratch::TestScratch;
    use std::env;
    use std::path::{Path, PathBuf};
    use std::process::{Command, Stdio};
    use std::vec::IntoIter;

    #[cfg(target_os = "linux")]
    const SYSTEM: &str = "x86_64-linux";
    #[cfg(target_os = "macos")]
    const SYSTEM: &str = "x86_64-darwin";

    fn nix() -> nix::Nix {
        let remote = env::var("NIX_REMOTE").unwrap_or("".to_owned());
        nix::Nix::new("x86_64-linux".to_owned(), remote, 1800, None)
    }

    fn tpath(component: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join(component)
    }

    fn make_worker(path: &Path) -> BuildWorker {
        let cloner = checkout::cached_cloner(path);
        let nix = nix();

        BuildWorker::new(
            cloner,
            nix,
            SYSTEM.to_owned(),
            "cargo-test-build".to_owned(),
        )
    }

    fn make_pr_repo(bare: &Path, co: &Path) -> String {
        let output = Command::new("bash")
            .current_dir(tpath("./test-srcs"))
            .arg("make-pr.sh")
            .arg(bare)
            .arg(co)
            .stderr(Stdio::null())
            .stdout(Stdio::piped())
            .output()
            .expect("building the test PR failed");
        let hash = String::from_utf8(output.stdout).expect("Should just be a hash");

        hash.trim().to_owned()
    }

    fn strip_escaped_ansi(string: &str) -> String {
        string
            .replace(['‘', '’'], "'")
            .replace("\\u001b[31;1m", "") // red
            .replace("\\u001b[0m", "") // reset
    }

    fn assert_contains_job(actions: &mut IntoIter<worker::Action>, text_to_match: &str) {
        println!("\n\n   Searching: {text_to_match:?}");
        actions
            .position(|job| match job {
                worker::Action::Publish(ref body) => {
                    let content = std::str::from_utf8(&body.content).unwrap();
                    let text = strip_escaped_ansi(content);
                    eprintln!("{text}");
                    if text.contains(text_to_match) {
                        println!(" ok");
                        true
                    } else {
                        println!(" notContains: {text}");
                        false
                    }
                }
                other => {
                    println!(" notPublish: {other:?}");
                    false
                }
            })
            .unwrap_or_else(|| {
                panic!("Actions should contain a job matching {text_to_match}, after the previous check")
            });
    }

    #[tokio::test]
    pub async fn test_simple_build() {
        let p = TestScratch::new_dir("build-simple-build-working");
        let bare_repo = TestScratch::new_dir("build-simple-build-bare");
        let co_repo = TestScratch::new_dir("build-simple-build-co");

        let head_sha = make_pr_repo(&bare_repo.path(), &co_repo.path());
        let worker = make_worker(&p.path());

        let job = buildjob::BuildJob {
            attrs: vec!["success".to_owned()],
            pr: Pr {
                head_sha,
                number: 1,
                target_branch: Some("master".to_owned()),
            },
            repo: Repo {
                clone_url: bare_repo.path().to_str().unwrap().to_owned(),
                full_name: "test-git".to_owned(),
                name: "nixos".to_owned(),
                owner: "ofborg-test".to_owned(),
            },
            subset: None,
            logs: Some((Some(String::from("logs")), Some(String::from("build.log")))),
            statusreport: Some((Some(String::from("build-results")), None)),
            request_id: "bogus-request-id".to_owned(),
        };

        let dummyreceiver = Arc::new(notifyworker::DummyNotificationReceiver::new());

        worker.consumer(job, dummyreceiver.clone()).await;

        println!("Total actions: {:?}", dummyreceiver.actions.lock().len());
        let actions_vec = dummyreceiver.actions.lock().clone();
        let mut actions = actions_vec.into_iter();

        assert_contains_job(&mut actions, "output\":\"hi");
        assert_contains_job(&mut actions, "output\":\"1");
        assert_contains_job(&mut actions, "output\":\"2");
        assert_contains_job(&mut actions, "output\":\"3");
        assert_contains_job(&mut actions, "output\":\"4");
        assert_contains_job(&mut actions, "status\":\"Success\""); // First one to the github poster
        assert_contains_job(&mut actions, "status\":\"Success\""); // This one to the logs
        assert_eq!(actions.next(), Some(worker::Action::Ack));
    }

    #[tokio::test]
    pub async fn test_all_jobs_skipped() {
        let p = TestScratch::new_dir("no-attempt");
        let bare_repo = TestScratch::new_dir("no-attempt-bare");
        let co_repo = TestScratch::new_dir("no-attempt-co");

        let head_sha = make_pr_repo(&bare_repo.path(), &co_repo.path());
        let worker = make_worker(&p.path());

        let job = buildjob::BuildJob {
            attrs: vec!["not-real".to_owned()],
            pr: Pr {
                head_sha,
                number: 1,
                target_branch: Some("master".to_owned()),
            },
            repo: Repo {
                clone_url: bare_repo.path().to_str().unwrap().to_owned(),
                full_name: "test-git".to_owned(),
                name: "nixos".to_owned(),
                owner: "ofborg-test".to_owned(),
            },
            subset: None,
            logs: Some((Some(String::from("logs")), Some(String::from("build.log")))),
            statusreport: Some((Some(String::from("build-results")), None)),
            request_id: "bogus-request-id".to_owned(),
        };

        let dummyreceiver = Arc::new(notifyworker::DummyNotificationReceiver::new());

        worker.consumer(job, dummyreceiver.clone()).await;

        println!("Total actions: {:?}", dummyreceiver.actions.lock().len());
        let actions_vec = dummyreceiver.actions.lock().clone();
        let mut actions = actions_vec.into_iter();

        assert_contains_job(
            &mut actions,
            r#""line_number":1,"output":"Cannot nix-instantiate `not-real` because:""#,
        );
        assert_contains_job(
            &mut actions,
            "attribute 'not-real' in selection path 'not-real' not found\"}",
        );
        assert_contains_job(&mut actions, "skipped_attrs\":[\"not-real"); // First one to the github poster
        assert_contains_job(&mut actions, "skipped_attrs\":[\"not-real"); // This one to the logs
        assert_eq!(actions.next(), Some(worker::Action::Ack));
    }
}
