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

struct JobActions<'a, 'b> {
    system: String,
    identity: String,
    receiver: &'a mut notifyworker::NotificationReceiver,
    job: &'b buildjob::BuildJob,
    line_counter: u64,
    attempt_id: String,
    log_exchange: Option<String>,
    log_routing_key: Option<String>,
}

impl<'a, 'b> JobActions<'a, 'b> {
    fn new(
        system: &str,
        identity: &str,
        job: &'b buildjob::BuildJob,
        receiver: &'a mut notifyworker::NotificationReceiver,
    ) -> JobActions<'a, 'b> {
        let (log_exchange, log_routing_key) = job.logs.clone().unwrap_or((
            Some(String::from("logs")),
            Some(String::from("build.log")),
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

        self.tell(worker::publish_serde_action(
            Some("build-results".to_owned()),
            None,
            &msg,
        ));
        self.tell(worker::Action::Ack);
    }

    pub fn log_started(&mut self) {
        self.line_counter += 1;

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

        self.tell(worker::publish_serde_action(
            Some("build-results".to_owned()),
            None,
            &msg,
        ));
        self.tell(worker::Action::Ack);
    }

    fn tell(&mut self, action: worker::Action) {
        self.receiver.tell(action);
    }
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

        // Note: Don't change the system limiter until the system isn't
        // hardcoded to x86_64-linux in the githubcommentfilter
        if buildfile == "./nixos/release.nix" && self.system != "x86_64-linux" {
            // NixOS jobs get routed to all builders, even though darwin
            // cannot build them.
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
            job.attrs.clone(),
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

        let success = spawned.wait().success();

        println!("ok built ({:?}), building", success);
        println!("Lines: {:?}", snippet_log);

        let last10lines: Vec<String> = snippet_log.into_iter().collect::<Vec<String>>();

        actions.build_finished(success, last10lines.clone());
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

    fn nix() -> nix::Nix {
        nix::Nix::new("x86_64-linux".to_owned(), "daemon".to_owned(), 1800)
    }

    fn tpath(component: &str) -> PathBuf {
        return Path::new(env!("CARGO_MANIFEST_DIR")).join(component);
    }

    fn scratch_dir() -> PathBuf {
        tpath("./test-scratch")
    }

    fn cleanup_scratch() {
        Command::new("rm")
            .arg("-rf")
            .arg(&scratch_dir())
            .status()
            .expect("cleanup of test-scratch should work");
    }

    fn make_worker() -> BuildWorker {
        cleanup_scratch();

        let cloner = checkout::cached_cloner(&scratch_dir());
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

    fn make_pr_repo() -> String {
        let output = Command::new("./make-pr.sh")
            .current_dir(tpath("./test-srcs"))
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
        let worker = make_worker();

        let job = buildjob::BuildJob {
            attrs: vec!["success".to_owned()],
            pr: Pr {
                head_sha: make_pr_repo(),
                number: 1,
                target_branch: Some("master".to_owned()),
            },
            repo: Repo {
                clone_url: tpath("./test-srcs/bare-repo").to_str().unwrap().to_owned(),
                full_name: "test-git".to_owned(),
                name: "nixos".to_owned(),
                owner: "ofborg-test".to_owned(),
            },
            subset: None,
            logs: Some((String::from("logs"), String::from("build.log"))),
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
}
