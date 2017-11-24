extern crate amqp;
extern crate env_logger;

use std::collections::LinkedList;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use ofborg::checkout;
use ofborg::message::buildjob;
use ofborg::nix;
use ofborg::commentparser;

use ofborg::worker;
use amqp::protocol::basic::{Deliver,BasicProperties};


pub struct BuildWorker {
    cloner: checkout::CachedCloner,
    nix: nix::Nix,
    system: String,
}

impl BuildWorker {
    pub fn new(cloner: checkout::CachedCloner, nix: nix::Nix, system: String) -> BuildWorker {
        return BuildWorker{
            cloner: cloner,
            nix: nix,
            system: system,
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
        let project = self.cloner.project(job.repo.full_name.clone(), job.repo.clone_url.clone());
        let co = project.clone_for("builder".to_string(),
                                   job.pr.number.to_string()).unwrap();

        let target_branch = match job.pr.target_branch.clone() {
            Some(x) => { x }
            None => { String::from("origin/master") }
        };

        let buildfile = match job.subset {
            Some(commentparser::Subset::NixOS) => "./nixos/default.nix",
            _ => "./default.nix"
        };

        if buildfile == "./nixos/default.nix" && self.system != "x86_64-linux" {
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


        return self.actions().build_finished(
            &job,
            success,
            last10lines.clone()
        );
    }
}
