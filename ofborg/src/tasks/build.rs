extern crate amqp;
extern crate env_logger;

use std::collections::LinkedList;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use ofborg::checkout;
use ofborg::message::buildjob;
use ofborg::message::buildresult;
use ofborg::nix;
use ofborg::commentparser;

use ofborg::worker;
use ofborg::notifyworker;
use amqp::protocol::basic::{Deliver,BasicProperties};


pub struct BuildWorker {
    cloner: checkout::CachedCloner,
    nix: nix::Nix,
    system: String,
    identity: String,
}

impl BuildWorker {
    pub fn new(cloner: checkout::CachedCloner, nix: nix::Nix, system: String, identity: String) -> BuildWorker {
        return BuildWorker{
            cloner: cloner,
            nix: nix,
            system: system,
            identity: identity,
        };
    }

    fn actions<'a>(&self, receiver: &'a mut notifyworker::NotificationReceiver) -> JobActions<'a> {
        JobActions::new(&self.system, &self.identity, receiver)
    }
}

struct JobActions<'a> {
    system: String,
    identity: String,
    receiver: &'a mut notifyworker::NotificationReceiver
}

impl<'a> JobActions<'a> {
    fn new(system: &str, identity: &str, receiver: &'a mut notifyworker::NotificationReceiver) -> JobActions<'a> {
        return JobActions {
            system: system.to_owned(),
            identity: system.to_owned(),
            receiver: receiver,
        };
    }

    pub fn commit_missing(&mut self, _job: &buildjob::BuildJob) {
        self.tell(worker::Action::Ack);
    }

    pub fn nasty_hack_linux_only(&mut self, _job: &buildjob::BuildJob) {
        self.tell(worker::Action::Ack);
    }

    pub fn merge_failed(&mut self, job: &buildjob::BuildJob) {
        let msg = buildresult::BuildResult {
            repo: job.repo.clone(),
            pr: job.pr.clone(),
            system: self.system.clone(),
            output: vec![String::from("Merge failed")],

            success: false
        };

        self.tell(worker::publish_serde_action(
            Some("build-results".to_owned()),
            None,
            &msg
        ));
        self.tell(worker::Action::Ack);
    }

    pub fn build_finished(&mut self, job: &buildjob::BuildJob, success: bool, lines: Vec<String>) {
        let msg = buildresult::BuildResult {
            repo: job.repo.clone(),
            pr: job.pr.clone(),
            system: self.system.clone(),
            output: lines,
            success: success
        };

        self.tell(worker::publish_serde_action(
            Some("build-results".to_owned()),
            None,
            &msg
        ));
        self.tell(worker::Action::Ack);
    }

    fn tell(&mut self, action: worker::Action) {
        self.receiver.tell(action);
    }
}

impl notifyworker::SimpleNotifyWorker for BuildWorker {
    type J = buildjob::BuildJob;

    fn msg_to_job(&self, _: &Deliver, _: &BasicProperties,
                  body: &Vec<u8>) -> Result<Self::J, String> {
        println!("lmao I got a job?");
        return match buildjob::from(body) {
            Ok(e) => { Ok(e) }
            Err(e) => {
                println!("{:?}", String::from_utf8(body.clone()));
                panic!("{:?}", e);
            }
        }
    }

    fn consumer(&self, job: &buildjob::BuildJob, notifier: &mut notifyworker::NotificationReceiver) {
        let mut actions = self.actions(notifier);

        info!("Working on {}", job.pr.number);
        let project = self.cloner.project(job.repo.full_name.clone(), job.repo.clone_url.clone());
        let co = project.clone_for("builder".to_string(),
                                   self.identity.clone()).unwrap();

        let target_branch = match job.pr.target_branch.clone() {
            Some(x) => { x }
            None => { String::from("origin/master") }
        };

        let buildfile = match job.subset {
            Some(commentparser::Subset::NixOS) => "./nixos/release.nix",
            _ => "./default.nix"
        };

        // Note: Don't change the system limiter until the system isn't
        // hardcoded to x86_64-linux in the githubcommentfilter
        if buildfile == "./nixos/release.nix" && self.system != "x86_64-linux" {
            // NixOS jobs get routed to all builders, even though darwin
            // cannot build them.
            actions.nasty_hack_linux_only(&job);
            return;
        }

        let refpath = co.checkout_origin_ref(target_branch.as_ref()).unwrap();
        co.fetch_pr(job.pr.number).unwrap();

        if !co.commit_exists(job.pr.head_sha.as_ref()) {
            info!("Commit {} doesn't exist", job.pr.head_sha);
            actions.commit_missing(&job);
            return;
        }

        if let Err(_) = co.merge_commit(job.pr.head_sha.as_ref()) {
            info!("Failed to merge {}", job.pr.head_sha);
            actions.merge_failed(&job);
            return;
        }

        println!("Got path: {:?}, building", refpath);


        let success: bool;
        let reader: BufReader<File>;
        match self.nix.safely_build_attrs(refpath.as_ref(),
                                          buildfile,
                                          job.attrs.clone()) {
            Ok(r) => {
                success = true;
                reader = BufReader::new(r);
            }
            Err(r) => {
                success = false;
                reader = BufReader::new(r);
            }
        }
        println!("ok built ({:?}), building", success);

        let l10 = reader.lines().fold(LinkedList::new(),

                                      |mut coll, line|
                                      {
                                          match line {
                                              Ok(e) => { coll.push_back(e); }
                                              Err(wtf) => {
                                                  println!("Got err in lines: {:?}", wtf);
                                                  coll.push_back(String::from("<line omitted due to error>"));
                                              }
                                          }

                                          if coll.len() == 11 {
                                              coll.pop_front();
                                          }

                                          return coll
                                      }
        );
        println!("Lines: {:?}", l10);

        let last10lines = l10.into_iter().collect::<Vec<_>>();


        actions.build_finished(
            &job,
            success,
            last10lines.clone()
        );
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path,PathBuf};
    use ofborg::message::{Pr,Repo};
    use notifyworker::SimpleNotifyWorker;
    use std::process::{Command, Stdio};

    fn nix() -> nix::Nix {
        nix::Nix::new("x86_64-linux".to_owned(), "daemon".to_owned(), 1800)
    }

    fn tpath(component: &str)-> PathBuf {
        return Path::new(env!("CARGO_MANIFEST_DIR")).join(component);
    }

    fn make_pr_repo() -> String{
        let output = Command::new("./make-pr.sh")
            .current_dir(tpath("./test-srcs"))
            .stderr(Stdio::null())
            .stdout(Stdio::piped())
            .output()
            .expect("building the test PR failed");
        let hash = String::from_utf8(output.stdout).expect("Should just be a hash");
        return hash.trim().to_owned();
    }

    #[test]
    pub fn test_simple_build() {
        Command::new("rm")
            .arg("-rf")
            .arg(&tpath("./test-scratch"))
            .status()
            .expect("cleanup of test-scratch should work");

        // pub fn new(cloner: checkout::CachedCloner, nix: nix::Nix, system: String, identity: String) -> BuildWorker {
        let cloner = checkout::cached_cloner(&tpath("./test-scratch"));
        let nix = nix();
        let worker = BuildWorker::new(
            cloner,
            nix,
            "x86_64-linux".to_owned(),
            "cargo-test-build".to_owned()
        );

        let job = buildjob::BuildJob{
            attrs: vec!["success".to_owned()],
            pr: Pr{
                head_sha: make_pr_repo(),
                number: 1,
                target_branch: Some("master".to_owned()),
            },
            repo: Repo{
                clone_url: tpath("./test-srcs/bare-repo").to_str().unwrap().to_owned(),
                full_name: "test-git".to_owned(),
                name: "nixos".to_owned(),
                owner: "ofborg-test".to_owned(),
            },
            subset: None,
        };

        let mut dummyreceiver = notifyworker::DummyNotificationReceiver::new();

        worker.consumer(&job, &mut dummyreceiver);
        panic!("{:?}", dummyreceiver.actions);
    }
}
