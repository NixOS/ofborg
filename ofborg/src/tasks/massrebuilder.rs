extern crate amqp;
extern crate env_logger;

use std::fs::File;
use std::io::Read;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;
use ofborg::checkout;
use ofborg::message::massrebuildjob;
use ofborg::nix;

use ofborg::worker;
use amqp::protocol::basic::{Deliver,BasicProperties};

pub struct MassRebuildWorker {
    cloner: checkout::CachedCloner,
    nix: nix::Nix,
}

impl MassRebuildWorker {
    pub fn new(cloner: checkout::CachedCloner, nix: nix::Nix) -> MassRebuildWorker {
        return MassRebuildWorker{
            cloner: cloner,
            nix: nix,
        };
    }

    fn actions(&self) -> massrebuildjob::Actions {
        return massrebuildjob::Actions{
        };
    }
}

impl worker::SimpleWorker for MassRebuildWorker {
    type J = massrebuildjob::MassRebuildJob;

    fn msg_to_job(&self, _: &Deliver, _: &BasicProperties,
                  body: &Vec<u8>) -> Result<Self::J, String> {
        return match massrebuildjob::from(body) {
            Ok(e) => { Ok(e) }
            Err(e) => {
                println!("{:?}", String::from_utf8(body.clone()));
                panic!("{:?}", e);
            }
        }
    }

    fn consumer(&self, job: &massrebuildjob::MassRebuildJob) -> worker::Actions {
        let project = self.cloner.project(job.repo.full_name.clone(), job.repo.clone_url.clone());
        let co = project.clone_for("mr-est".to_string(),
                                   job.pr.number.to_string()).unwrap();

        let target_branch = match job.pr.target_branch.clone() {
            Some(x) => { x }
            None => { String::from("origin/master") }
        };

        let refpath = co.checkout_ref(target_branch.as_ref()).unwrap();


        let mut stdenvs = Stdenvs::new(self.nix.clone(), PathBuf::from(&refpath));
        stdenvs.identify_before();

        co.fetch_pr(job.pr.number).unwrap();

        if !co.commit_exists(job.pr.head_sha.as_ref()) {
            info!("Commit {} doesn't exist", job.pr.head_sha);
            return self.actions().skip(&job);
        }

        if let Err(_) = co.merge_commit(job.pr.head_sha.as_ref()) {
            info!("Failed to merge {}", job.pr.head_sha);
            return self.actions().skip(&job);
        }

        stdenvs.identify_after();

        println!("Got path: {:?}, building", refpath);
        if !stdenvs.are_same() {
            println!("Stdenvs changed? {:?}", stdenvs.changed());
        }

        return vec![];
    }
}

enum StdenvFrom {
    Before,
    After
}

#[derive(Debug)]
enum System {
    X8664Darwin,
    X8664Linux
}

#[derive(Debug, PartialEq)]
struct Stdenvs {
    nix: nix::Nix,
    co: PathBuf,

    linux_stdenv_before: Option<String>,
    linux_stdenv_after: Option<String>,

    darwin_stdenv_before: Option<String>,
    darwin_stdenv_after: Option<String>,
}

impl Stdenvs {
    fn new(nix: nix::Nix, co: PathBuf) -> Stdenvs {
        return Stdenvs {
            nix: nix,
            co: co,

            linux_stdenv_before: None,
            linux_stdenv_after: None,

            darwin_stdenv_before: None,
            darwin_stdenv_after: None,
        }
    }

    fn identify_before(&mut self) {
        self.identify(System::X8664Linux, StdenvFrom::Before);
        self.identify(System::X8664Darwin, StdenvFrom::Before);
    }

    fn identify_after(&mut self) {
        self.identify(System::X8664Linux, StdenvFrom::After);
        self.identify(System::X8664Darwin, StdenvFrom::After);
    }

    fn are_same(&self) -> bool {
        return self.changed().len() == 0;
    }

    fn changed(&self) -> Vec<System> {
        let mut changed: Vec<System> = vec![];

        if self.linux_stdenv_before != self.linux_stdenv_after {
            changed.push(System::X8664Linux);
        }

        if self.darwin_stdenv_before != self.darwin_stdenv_after {
            changed.push(System::X8664Darwin);
        }


        return changed
    }

    fn identify(&mut self, system: System, from: StdenvFrom) {
        match (system, from) {
            (System::X8664Linux, StdenvFrom::Before) => {
                self.linux_stdenv_before = self.evalstdenv("x86_64-linux");
            }
            (System::X8664Linux, StdenvFrom::After) => {
                self.linux_stdenv_after = self.evalstdenv("x86_64-linux");
            }

            (System::X8664Darwin, StdenvFrom::Before) => {
                self.darwin_stdenv_before = self.evalstdenv("x86_64-darwin");
            }
            (System::X8664Darwin, StdenvFrom::After) => {
                self.darwin_stdenv_after = self.evalstdenv("x86_64-darwin");
            }
        }
    }

    fn evalstdenv(&self, system: &str) -> Option<String> {
        let result = self.nix.with_system(system.to_owned()).safely(
            "nix-instantiate", &self.co, vec![
                String::from("."),
                String::from("-A"),
                String::from("stdenv"),
            ]
        );

        println!("{:?}", result);

        return match result {
            Ok(mut out) => {
                file_to_drv(&mut out)
            }
            Err(mut out) => {
                println!("{:?}", file_to_str(&mut out));
                None
            }
        }
    }
}

fn file_to_drv(f: &mut File) -> Option<String> {
    let r = BufReader::new(f);
    let matches: Vec<String>;
    matches = r.lines().filter_map(|x|
                     match x {
                         Ok(line) => {
                             if !line.starts_with("/nix/store/") {
                                 debug!("Skipping line, not /nix/store: {}", line);
                                 return None
                             }

                             if !line.ends_with(".drv") {
                                 debug!("Skipping line, not .drv: {}", line);
                                 return None
                             }

                             return Some(line)
                         }
                         Err(_) => None
                     }).collect();

    if matches.len() == 1 {
        return Some(matches.first().unwrap().clone());
    } else {
        info!("Got wrong number of matches: {}", matches.len());
        info!("Matches: {:?}", matches);
        return None
    }
}

fn file_to_str(f: &mut File) -> String {
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("Reading eval output");
    return String::from(String::from_utf8_lossy(&buffer));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn stdenv_checking() {
        let nix = nix::new(String::from("x86_64-linux"), String::from("daemon"));
        let mut stdenv = Stdenvs::new(nix.clone(), PathBuf::from("/nix/var/nix/profiles/per-user/root/channels/nixos/nixpkgs"));
        stdenv.identify(System::X8664Linux, StdenvFrom::Before);
        stdenv.identify(System::X8664Darwin, StdenvFrom::Before);

        stdenv.identify(System::X8664Linux, StdenvFrom::After);
        stdenv.identify(System::X8664Darwin, StdenvFrom::After);

        assert!(stdenv.are_same());
    }
}
