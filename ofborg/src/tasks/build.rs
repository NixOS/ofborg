extern crate amqp;
extern crate env_logger;

use ofborg::checkout;
use ofborg::message::buildjob;
use ofborg::nix;
use ofborg::cmdlog;
use ofborg::commentparser;
use ofborg::asynccmd::AsyncCmd;
use cmdlog::Logger;
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

    fn consumer(&mut self, job: &buildjob::BuildJob) -> worker::Actions {
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


        let cmd = self.nix.safely_build_attrs_cmd(
            refpath.as_ref(),
            buildfile,
            job.attrs.clone()
        );
        let acmd = AsyncCmd::new(cmd);
        let mut spawned = acmd.spawn();

        let mut snippet_log = cmdlog::LastNLogger::new(10);
        for line in spawned.lines().iter() {
            self.build_logger.build_output(&line);
            snippet_log.build_output(&line);
        }

        let success = spawned.wait().success();

        let last10lines: Vec<String> = snippet_log.lines().into_iter().collect::<Vec<String>>();

        return self.actions().build_finished(
            &job,
            success,
            last10lines.clone()
        );
    }
}
