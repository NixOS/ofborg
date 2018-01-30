extern crate amqp;
extern crate uuid;
extern crate env_logger;

use uuid::Uuid;

use std::collections::VecDeque;
use ofborg::asynccmd::AsyncCmd;
use ofborg::checkout;
use ofborg::message::buildjob;
use ofborg::message::buildresult;
use ofborg::message::buildlogmsg;
use ofborg::nix;
use ofborg::commentparser;

use ofborg::worker;
use ofborg::notifyworker;
use amqp::protocol::basic::{Deliver, BasicProperties};


pub struct BuildWorker {
    cloner: checkout::CachedCloner,
    nix: nix::Nix,
    system: String,
    identity: String,
    full_logs: bool,
}

impl BuildWorker {
    pub fn new(
        cloner: checkout::CachedCloner,
        nix: nix::Nix,
        system: String,
        identity: String,
        full_logs: bool,
    ) -> BuildWorker {
        return BuildWorker {
            cloner: cloner,
            nix: nix,
            system: system,
            identity: identity,
            full_logs: full_logs,
        };
    }

    fn actions<'a, 'b>(
        &self,
        job: &'b buildjob::BuildJob,
        receiver: &'a mut notifyworker::NotificationReceiver,
    ) -> JobActions<'a, 'b> {
        JobActions::new(&self.system, &self.identity, job, receiver)
    }
}

pub struct JobActions<'a, 'b> {
    system: String,
    identity: String,
    receiver: &'a mut notifyworker::NotificationReceiver,
    job: &'b buildjob::BuildJob,
    line_counter: u64,
    attempt_id: String,
    log_exchange: Option<String>,
    log_routing_key: Option<String>,
    result_exchange: Option<String>,
    result_routing_key: Option<String>,
}

impl<'a, 'b> JobActions<'a, 'b> {
    pub fn new(
        system: &str,
        identity: &str,
        job: &'b buildjob::BuildJob,
        receiver: &'a mut notifyworker::NotificationReceiver,
    ) -> JobActions<'a, 'b> {
        let (log_exchange, log_routing_key) = job.logs.clone().unwrap_or((
            Some(String::from("logs")),
            Some(String::from("build.log")),
        ));

        let (result_exchange, result_routing_key) =
            job.statusreport.clone().unwrap_or((
                Some(String::from("build-results")),
                None,
            ));

        return JobActions {
            system: system.to_owned(),
            identity: identity.to_owned(),
            receiver: receiver,
            job: job,
            line_counter: 0,
            attempt_id: format!("{}", Uuid::new_v4()),
            log_exchange: log_exchange,
            log_routing_key: log_routing_key,
            result_exchange: result_exchange,
            result_routing_key: result_routing_key,
        };
    }

    pub fn commit_missing(&mut self) {
        self.tell(worker::Action::Ack);
    }

    pub fn nasty_hack_linux_only(&mut self) {
        self.tell(worker::Action::Ack);
    }

    pub fn merge_failed(&mut self) {
        let msg = buildresult::BuildResult {
            repo: self.job.repo.clone(),
            pr: self.job.pr.clone(),
            system: self.system.clone(),
            output: vec![String::from("Merge failed")],

            success: false,
        };

        let result_exchange = self.result_exchange.clone();
        let result_routing_key = self.result_routing_key.clone();


        self.tell(worker::publish_serde_action(
            result_exchange,
            result_routing_key,
            &msg,
        ));
        self.tell(worker::Action::Ack);
    }

    pub fn log_started(&mut self) {
        let msg = buildlogmsg::BuildLogStart {
            identity: self.identity.clone(),
            system: self.system.clone(),
            attempt_id: self.attempt_id.clone(),
        };

        let log_exchange = self.log_exchange.clone();
        let log_routing_key = self.log_routing_key.clone();

        self.tell(worker::publish_serde_action(
            log_exchange,
            log_routing_key,
            &msg,
        ));
    }

    pub fn log_line(&mut self, line: &str) {
        self.line_counter += 1;

        let msg = buildlogmsg::BuildLogMsg {
            identity: self.identity.clone(),
            system: self.system.clone(),
            attempt_id: self.attempt_id.clone(),
            line_number: self.line_counter,
            output: line.to_owned(),
        };

        let log_exchange = self.log_exchange.clone();
        let log_routing_key = self.log_routing_key.clone();

        self.tell(worker::publish_serde_action(
            log_exchange,
            log_routing_key,
            &msg,
        ));
    }

    pub fn build_finished(&mut self, success: bool, lines: Vec<String>) {
        let msg = buildresult::BuildResult {
            repo: self.job.repo.clone(),
            pr: self.job.pr.clone(),
            system: self.system.clone(),
            output: lines,
            success: success,
        };

        let result_exchange = self.result_exchange.clone();
        let result_routing_key = self.result_routing_key.clone();


        self.tell(worker::publish_serde_action(
            result_exchange,
            result_routing_key,
            &msg,
        ));
        self.tell(worker::Action::Ack);
    }

    fn tell(&mut self, action: worker::Action) {
        self.receiver.tell(action);
    }
}

fn strip_x8664linux_arch_suffix(attr: &str) -> &str {
    if !attr.starts_with("tests.") {
        return attr;
    }

    if !attr.ends_with(".x86_64-linux") {
        return attr;
    }

    let trim_at = attr.len() - 13;

    return &attr[0..trim_at];
}

impl notifyworker::SimpleNotifyWorker for BuildWorker {
    type J = buildjob::BuildJob;

    fn msg_to_job(
        &self,
        _: &Deliver,
        _: &BasicProperties,
        body: &Vec<u8>,
    ) -> Result<Self::J, String> {
        println!("lmao I got a job?");
        return match buildjob::from(body) {
            Ok(e) => Ok(e),
            Err(e) => {
                println!("{:?}", String::from_utf8(body.clone()));
                panic!("{:?}", e);
            }
        };
    }

    fn consumer(
        &self,
        job: &buildjob::BuildJob,
        notifier: &mut notifyworker::NotificationReceiver,
    ) {
        let mut actions = self.actions(&job, notifier);

        info!("Working on {}", job.pr.number);
        let project = self.cloner.project(
            job.repo.full_name.clone(),
            job.repo.clone_url.clone(),
        );
        let co = project
            .clone_for("builder".to_string(), self.identity.clone())
            .unwrap();

        let target_branch = match job.pr.target_branch.clone() {
            Some(x) => x,
            None => String::from("origin/master"),
        };

        let buildfile = match job.subset {
            Some(commentparser::Subset::NixOS) => "./nixos/release.nix",
            _ => "./default.nix",
        };

        let attrs = match job.subset {
            Some(commentparser::Subset::NixOS) => {
                job.attrs
                    .clone()
                    .into_iter()
                    .map(|attr| strip_x8664linux_arch_suffix(&attr).to_owned())
                    .collect()
            }
            _ => job.attrs.clone(),
        };

        if buildfile == "./nixos/release.nix" && self.system == "x86_64-darwin" {
            actions.nasty_hack_linux_only();
            return;
        }

        let refpath = co.checkout_origin_ref(target_branch.as_ref()).unwrap();
        co.fetch_pr(job.pr.number).unwrap();

        if !co.commit_exists(job.pr.head_sha.as_ref()) {
            info!("Commit {} doesn't exist", job.pr.head_sha);
            actions.commit_missing();
            return;
        }

        if let Err(_) = co.merge_commit(job.pr.head_sha.as_ref()) {
            info!("Failed to merge {}", job.pr.head_sha);
            actions.merge_failed();
            return;
        }

        println!("Got path: {:?}, building", refpath);


        let cmd = self.nix.safely_build_attrs_cmd(
            refpath.as_ref(),
            buildfile,
            attrs,
        );

        actions.log_started();
        let acmd = AsyncCmd::new(cmd);
        let mut spawned = acmd.spawn();

        let mut snippet_log = VecDeque::with_capacity(10);


        if !self.full_logs {
            actions.log_line("Full logs are disabled on this builder.");
        }

        for line in spawned.lines().iter() {
            if self.full_logs {
                actions.log_line(&line);
            }

            if snippet_log.len() >= 10 {
                snippet_log.pop_front();
            }

            snippet_log.push_back(line.to_owned());
        }

        let success = match spawned.wait() {
            Ok(Some(Ok(status))) => status.success(),
            e => {
                println!("Failed on the interior command: {:?}", e);
                false
            }
        };

        println!("ok built ({:?}), building", success);
        println!("Lines: {:?}", snippet_log);

        let last10lines: Vec<String> = snippet_log.into_iter().collect::<Vec<String>>();

        actions.build_finished(success, last10lines.clone());
        println!("Done!");
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};
    use ofborg::message::{Pr, Repo};
    use notifyworker::SimpleNotifyWorker;
    use std::process::{Command, Stdio};
    use std::vec::IntoIter;
    use ofborg::test_scratch::TestScratch;

    fn nix() -> nix::Nix {
        nix::Nix::new("x86_64-linux".to_owned(), "daemon".to_owned(), 1800)
    }

    fn tpath(component: &str) -> PathBuf {
        return Path::new(env!("CARGO_MANIFEST_DIR")).join(component);
    }

    fn make_worker(path: &Path) -> BuildWorker {
        let cloner = checkout::cached_cloner(path);
        let nix = nix();
        let worker = BuildWorker::new(
            cloner,
            nix,
            "x86_64-linux".to_owned(),
            "cargo-test-build".to_owned(),
            true,
        );

        return worker;
    }

    fn make_pr_repo(bare: &Path, co: &Path) -> String {
        let output = Command::new("./make-pr.sh")
            .current_dir(tpath("./test-srcs"))
            .arg(bare)
            .arg(co)
            .stderr(Stdio::null())
            .stdout(Stdio::piped())
            .output()
            .expect("building the test PR failed");
        let hash = String::from_utf8(output.stdout).expect("Should just be a hash");
        return hash.trim().to_owned();
    }

    fn assert_contains_job(actions: &mut IntoIter<worker::Action>, text_to_match: &str) {
        println!("\n\nSearching for {:?}", text_to_match);
        actions
            .position(|job| match job {
                worker::Action::Publish(ref body) => {
                    let mystr = String::from_utf8(body.content.clone()).unwrap();
                    if mystr.contains(text_to_match) {
                        println!("    Matched: {:?}", mystr);
                        return true;
                    } else {
                        println!("    miss: {:?}", mystr);
                        return false;
                    }
                }
                e => {
                    println!("    notPublish: {:?}", e);
                    return false;
                }
            })
            .expect(&format!(
                "Actions should contain a job matching {:?}, after the previous check",
                text_to_match
            ));
    }

    #[test]
    pub fn test_simple_build() {
        let p = TestScratch::new_dir("build-simple-build-working");
        let bare_repo = TestScratch::new_dir("build-simple-build-bare");
        let co_repo = TestScratch::new_dir("build-simple-build-co");

        let head_sha = make_pr_repo(&bare_repo.path(), &co_repo.path());
        let worker = make_worker(&p.path());


        let job = buildjob::BuildJob {
            attrs: vec!["success".to_owned()],
            pr: Pr {
                head_sha: head_sha,
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
            logs: Some((
                Some(String::from("logs")),
                Some(String::from("build.log")),
            )),
            statusreport: Some((Some(String::from("build-results")), None)),
        };

        let mut dummyreceiver = notifyworker::DummyNotificationReceiver::new();

        worker.consumer(&job, &mut dummyreceiver);

        println!("Total actions: {:?}", dummyreceiver.actions.len());
        let mut actions = dummyreceiver.actions.into_iter();

        assert_contains_job(&mut actions, "output\":\"hi");
        assert_contains_job(&mut actions, "output\":\"1");
        assert_contains_job(&mut actions, "output\":\"2");
        assert_contains_job(&mut actions, "output\":\"3");
        assert_contains_job(&mut actions, "output\":\"4");
        assert_contains_job(&mut actions, "success\":true");
        assert_eq!(actions.next(), Some(worker::Action::Ack));
    }

    #[test]
    fn test_strip_x8664linux_arch_suffix() {
        assert_eq!(strip_x8664linux_arch_suffix(""), "");
        assert_eq!(
            strip_x8664linux_arch_suffix("tests.foo.bar"),
            "tests.foo.bar"
        );
        assert_eq!(
            strip_x8664linux_arch_suffix("foo.bar.x86_64-linux"),
            "foo.bar.x86_64-linux"
        );
        assert_eq!(
            strip_x8664linux_arch_suffix("tests.foo.bar.x86_64-linux"),
            "tests.foo.bar"
        );
    }
}
