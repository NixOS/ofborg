extern crate amqp;
extern crate env_logger;
use std::process::Stdio;

use std::collections::LinkedList;

use std::sync::{Arc, Mutex, MutexGuard};
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use std::thread;
use std::sync::mpsc::channel;

use ofborg::checkout;
use ofborg::message::buildjob;
use ofborg::nix;
use ofborg::cmdlog;
use ofborg::commentparser;

use ofborg::worker;
use amqp::protocol::basic::{Deliver,BasicProperties};


pub struct BuildWorker {
    cloner: checkout::CachedCloner,
    nix: nix::Nix,
    system: String,
    identity: String,
    build_logger: Box<cmdlog::Logger + Send>,
}

impl BuildWorker {
    pub fn new(cloner: checkout::CachedCloner, nix: nix::Nix, system: String, identity: String, build_logger: Box<cmdlog::Logger + Send>) -> BuildWorker {
        return BuildWorker{
            cloner: cloner,
            nix: nix,
            system: system,
            identity: identity,
            build_logger: build_logger,
        };
    }

    fn actions(&self) -> buildjob::Actions {
        return buildjob::Actions{
            system: self.system.clone(),
        };
    }
}

impl worker::SimpleWorker for BuildWorker {
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

    fn consumer(&self, job: &buildjob::BuildJob) -> worker::Actions {
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
            return self.actions().nasty_hack_linux_only(&job);
        }

        let refpath = co.checkout_origin_ref(target_branch.as_ref()).unwrap();
        co.fetch_pr(job.pr.number).unwrap();

        if !co.commit_exists(job.pr.head_sha.as_ref()) {
            info!("Commit {} doesn't exist", job.pr.head_sha);
            return self.actions().commit_missing(&job);
        }

        if let Err(_) = co.merge_commit(job.pr.head_sha.as_ref()) {
            info!("Failed to merge {}", job.pr.head_sha);
            return self.actions().merge_failed(&job);
        }

        println!("Got path: {:?}, building", refpath);


        let mut child = self.nix.safely_build_attrs_cmd(refpath.as_ref(),
                                                        buildfile,
                                                        job.attrs.clone())
            .stdin(Stdio::null())
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let (tx, rx) = channel();

        let stderrh = {
            let stderrsrc = child.stderr.take().unwrap();
            let stderrlogdest = tx.clone();
            thread::spawn(move|| {
                let readfrom = BufReader::new(stderrsrc);
                for line in readfrom.lines() {
                    stderrlogdest.send(line).unwrap();
                }
            })
        };

        let stdouth = {
            let stdoutsrc = child.stdout.take().unwrap();
            let stdoutlogdest = tx.clone();
            thread::spawn(move|| {
                let readfrom = BufReader::new(stdoutsrc);
                for line in readfrom.lines() {
                    stdoutlogdest.send(line).unwrap();
                }
            })
        };

        let mut l10: Arc<Mutex<LinkedList<String>>> = Arc::new(Mutex::new(LinkedList::new()));

        let linehandler = {
            let mut l10 = l10.clone();
            thread::spawn(move|| {
                let l10x = l10.lock().unwrap();
                let mut l10writer = l10x;
                for line in rx.recv() {
                    if let Ok(line) = line {
                        println!("> {:?}", line);

                        if l10writer.len() >= 10 {
                            l10writer.pop_front();
                        }
                        l10writer.push_back(line);
                    } else {
                        break;
                    }
                }
            })
        };

        stderrh.join().unwrap();
        stdouth.join().unwrap();

        let success = child.wait()
            .expect("should have been run")
            .success();
        linehandler.join().unwrap();

        println!("ok built ({:?}), building", success);

        // println!("Lines: {:?}", l10);

        let l10x: Mutex<LinkedList<String>> = Arc::try_unwrap(l10).expect("all threads should be dead");
        /*
        let mut l10y: MutexGuard<Option<LinkedList<String>>> = l10x.lock().unwrap();
        let mut l10z: LinkedList<String> = l10y.take().unwrap();
        let last10lines: Vec<String> = l10z.into_iter().collect::<Vec<String>>();
         */
        let last10lines: Vec<String> = l10x.lock().unwrap().split_off(0).into_iter().collect::<Vec<String>>();

        return self.actions().build_finished(
            &job,
            success,
            last10lines.clone()
        );
    }
}
