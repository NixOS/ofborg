extern crate amqp;
extern crate env_logger;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;
use ofborg::checkout;
use ofborg::message::massrebuildjob;
use ofborg::nix::Nix;

use ofborg::worker;
use ofborg::tagger::{StdenvTagger,RebuildTagger};
use ofborg::outpathdiff::{OutPaths, OutPathDiff};
use ofborg::evalchecker::EvalChecker;
use ofborg::commitstatus::CommitStatus;
use amqp::protocol::basic::{Deliver,BasicProperties};
use hubcaps;

pub struct MassRebuildWorker {
    cloner: checkout::CachedCloner,
    nix: Nix,
    github: hubcaps::Github,
    identity: String,
}

impl MassRebuildWorker {
    pub fn new(cloner: checkout::CachedCloner, nix: Nix, github: hubcaps::Github, identity: String) -> MassRebuildWorker {
        return MassRebuildWorker{
            cloner: cloner,
            nix: nix,
            github: github,
            identity: identity
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

    fn consumer(&mut self, job: &massrebuildjob::MassRebuildJob) -> worker::Actions {
        let repo = self.github
            .repo(job.repo.owner.clone(), job.repo.name.clone());
        let gists = self.github.gists();
        let issue = repo.issue(job.pr.number);

        match issue.get() {
            Ok(iss) => {
                if iss.state == "closed" {
                    info!("Skipping {} because it is closed", job.pr.number);
                    return self.actions().skip(&job);
                }
            }
            Err(e) => {
                info!("Error fetching {}!", job.pr.number);
                info!("E: {:?}", e);
                return self.actions().skip(&job);
            }
        }

        let mut overall_status = CommitStatus::new(
            repo.statuses(),
            job.pr.head_sha.clone(),
            "grahamcofborg-eval".to_owned(),
            "Starting".to_owned(),
            None
        );

        overall_status.set_with_description("Starting", hubcaps::statuses::State::Pending);

        let project = self.cloner.project(job.repo.full_name.clone(), job.repo.clone_url.clone());

        overall_status.set_with_description("Cloning project", hubcaps::statuses::State::Pending);

        info!("Working on {}", job.pr.number);
        let co = project.clone_for("mr-est".to_string(),
                                   self.identity.clone()).unwrap();

        let target_branch = match job.pr.target_branch.clone() {
            Some(x) => { x }
            None => { String::from("master") }
        };

        overall_status.set_with_description(
            format!("Checking out {}", &target_branch).as_ref(),
            hubcaps::statuses::State::Pending
        );
        info!("Checking out target branch {}", &target_branch);
        let refpath = co.checkout_origin_ref(target_branch.as_ref()).unwrap();

        overall_status.set_with_description(
            "Checking original stdenvs",
            hubcaps::statuses::State::Pending
        );


        let mut stdenvs = Stdenvs::new(self.nix.clone(), PathBuf::from(&refpath));
        stdenvs.identify_before();

        let mut rebuildsniff = OutPathDiff::new(
            self.nix.clone(),
            PathBuf::from(&refpath)
        );

        overall_status.set_with_description(
            "Checking original out paths",
            hubcaps::statuses::State::Pending
        );

        if let Err(mut output) = rebuildsniff.find_before() {
            overall_status.set_url(make_gist(
                &gists,
                "Output path comparison".to_owned(),
                Some("".to_owned()),
                file_to_str(&mut output),
            ));

            overall_status.set_with_description(
                format!("Target branch {} doesn't evaluate!", &target_branch).as_ref(),
                hubcaps::statuses::State::Failure
            );

            return self.actions().skip(&job);
        }

        overall_status.set_with_description(
            "Fetching PR",
            hubcaps::statuses::State::Pending
        );

        co.fetch_pr(job.pr.number).unwrap();

        if !co.commit_exists(job.pr.head_sha.as_ref()) {
            overall_status.set_with_description(
                "Commit not found",
                hubcaps::statuses::State::Error
            );

            info!("Commit {} doesn't exist", job.pr.head_sha);
            return self.actions().skip(&job);
        }

        overall_status.set_with_description(
            "Merging PR",
            hubcaps::statuses::State::Pending
        );

        if let Err(_) = co.merge_commit(job.pr.head_sha.as_ref()) {
            overall_status.set_with_description(
                "Failed to merge",
                hubcaps::statuses::State::Failure
            );

            info!("Failed to merge {}", job.pr.head_sha);
            return self.actions().skip(&job);
        }

        overall_status.set_with_description(
            "Checking new stdenvs",
            hubcaps::statuses::State::Pending
        );

        stdenvs.identify_after();

        overall_status.set_with_description(
            "Checking new out paths",
            hubcaps::statuses::State::Pending
        );

        if let Err(mut output) = rebuildsniff.find_after() {
            overall_status.set_url(make_gist(
                &gists,
                "Output path comparison".to_owned(),
                Some("".to_owned()),
                file_to_str(&mut output),
            ));
            overall_status.set_with_description(
                format!("Failed to enumerate outputs after merging to {}", &target_branch).as_ref(),
                hubcaps::statuses::State::Failure
            );
            return self.actions().skip(&job);
        }

        println!("Got path: {:?}, building", refpath);
        overall_status.set_with_description(
            "Beginning Evaluations",
            hubcaps::statuses::State::Pending
        );

        let eval_checks = vec![
            EvalChecker::new("package-list",
                             "nix-env",
                             vec![
                                 String::from("--file"),
                                 String::from("."),
                                 String::from("--query"),
                                 String::from("--available"),
                                 String::from("--json"),
                             ],
                             self.nix.clone()
            ),

            EvalChecker::new("nixos-options",
                             "nix-instantiate",
                             vec![
                                 String::from("./nixos/release.nix"),
                                 String::from("-A"),
                                 String::from("options"),
                             ],
                             self.nix.clone()
            ),

            EvalChecker::new("nixos-manual",
                             "nix-instantiate",
                             vec![
                                 String::from("./nixos/release.nix"),
                                 String::from("-A"),
                                 String::from("manual"),
                             ],
                             self.nix.clone()
            ),

            EvalChecker::new("nixpkgs-manual",
                             "nix-instantiate",
                             vec![
                                 String::from("./pkgs/top-level/release.nix"),
                                 String::from("-A"),
                                 String::from("manual"),
                             ],
                             self.nix.clone()
            ),

            EvalChecker::new("nixpkgs-tarball",
                             "nix-instantiate",
                             vec![
                                 String::from("./pkgs/top-level/release.nix"),
                                 String::from("-A"),
                                 String::from("tarball"),
                             ],
                             self.nix.clone()
            ),

            EvalChecker::new("nixpkgs-unstable-jobset",
                             "nix-instantiate",
                             vec![
                                 String::from("./pkgs/top-level/release.nix"),
                                 String::from("-A"),
                                 String::from("unstable"),
                             ],
                             self.nix.clone()
            ),
        ];

        let mut eval_results: bool = eval_checks.into_iter()
            .map(|check|
                 {
                     let mut status = CommitStatus::new(
                         repo.statuses(),
                         job.pr.head_sha.clone(),
                         check.name(),
                         check.cli_cmd(),
                         None
                     );

                     status.set(hubcaps::statuses::State::Pending);

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
                                 &gists,
                                 check.name(),
                                 Some(format!("{:?}", state)),
                                 file_to_str(&mut out),
                             );
                         }
                     }

                     status.set_url(gist_url);
                     status.set(state.clone());

                     if state == hubcaps::statuses::State::Success {
                         return Ok(())
                     } else {
                         return Err(())
                     }
                 }
            )
            .all(|status| status == Ok(()));

        if eval_results {
            let mut status = CommitStatus::new(
                repo.statuses(),
                job.pr.head_sha.clone(),
                String::from("grahamcofborg-eval-check-meta"),
                String::from("config.nix: checkMeta = true"),
                None
            );

            status.set(hubcaps::statuses::State::Pending);

            let state: hubcaps::statuses::State;
            let gist_url: Option<String>;

            let checker = OutPaths::new(
                self.nix.clone(),
                PathBuf::from(&refpath),
                true
            );
            match checker.find() {
                Ok(_) => {
                    state = hubcaps::statuses::State::Success;
                    gist_url = None;
                }
                Err(mut out) => {
                    eval_results = false;
                    state = hubcaps::statuses::State::Failure;
                    gist_url = make_gist(
                        &gists,
                        String::from("Meta Check"),
                        Some(format!("{:?}", state)),
                        file_to_str(&mut out),
                    );
                }
            }

            status.set_url(gist_url);
            status.set(state.clone());
        }

        if eval_results {
            overall_status.set_with_description(
                "Calculating Changed Outputs",
                hubcaps::statuses::State::Pending
            );

            let mut stdenvtagger = StdenvTagger::new();
            if !stdenvs.are_same() {
                stdenvtagger.changed(stdenvs.changed());
            }
            update_labels(&issue, stdenvtagger.tags_to_add(),
                          stdenvtagger.tags_to_remove());

            let mut rebuild_tags = RebuildTagger::new();
            if let Some(attrs) = rebuildsniff.calculate_rebuild() {
                rebuild_tags.parse_attrs(attrs);
            }
            update_labels(&issue, rebuild_tags.tags_to_add(),
                          rebuild_tags.tags_to_remove());

            overall_status.set_with_description(
                "^.^!",
                hubcaps::statuses::State::Success
            );

        } else {
            overall_status.set_with_description(
                "Complete, with errors",
                hubcaps::statuses::State::Failure
            );
        }

        return self.actions().done(&job);
    }
}

enum StdenvFrom {
    Before,
    After
}

#[derive(Debug)]
pub enum System {
    X8664Darwin,
    X8664Linux
}

#[derive(Debug, PartialEq)]
struct Stdenvs {
    nix: Nix,
    co: PathBuf,

    linux_stdenv_before: Option<String>,
    linux_stdenv_after: Option<String>,

    darwin_stdenv_before: Option<String>,
    darwin_stdenv_after: Option<String>,
}

impl Stdenvs {
    fn new(nix: Nix, co: PathBuf) -> Stdenvs {
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
            ],
            true
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

fn make_gist<'a>(gists: &hubcaps::gists::Gists<'a>, name: String, description: Option<String>, contents: String) -> Option<String> {
    let mut files = HashMap::new();
    files.insert(name.clone(),
                 hubcaps::gists::Content {
                     filename: Some(name.clone()),
                     content: contents,
                 }
    );

    return Some(gists.create(
        &hubcaps::gists::GistOptions {
            description: description,
            public: Some(true),
            files: files,
        }
    ).expect("Failed to create gist!").html_url);
}

pub fn update_labels(issue: &hubcaps::issues::IssueRef, add: Vec<String>, remove: Vec<String>) {
    let l = issue.labels();

    let existing: Vec<String> = issue.get().unwrap().labels
        .iter()
        .map(|l| l.name.clone())
        .collect();
    println!("Already: {:?}", existing);
    let to_add = add
        .iter()
        .filter(|l| !existing.contains(l)) // Remove labels already on the issue
        .map(|l| l.as_ref()).collect();
    info!("Adding labels: {:?}", to_add);

    let to_remove: Vec<String> = remove
        .iter()
        .filter(|l| existing.contains(l)) // Remove labels already on the issue
        .map(|l| l.clone())
        .collect();
    info!("Removing labels: {:?}", to_remove);

    l.add(to_add).expect("Failed to add tags");

    for label in to_remove {
        l.remove(&label).expect("Failed to remove tag");
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
        let nix = Nix::new(String::from("x86_64-linux"), String::from("daemon"), 1200);
        let mut stdenv = Stdenvs::new(nix.clone(), PathBuf::from("/nix/var/nix/profiles/per-user/root/channels/nixos/nixpkgs"));
        stdenv.identify(System::X8664Linux, StdenvFrom::Before);
        stdenv.identify(System::X8664Darwin, StdenvFrom::Before);

        stdenv.identify(System::X8664Linux, StdenvFrom::After);
        stdenv.identify(System::X8664Darwin, StdenvFrom::After);

        assert!(stdenv.are_same());
    }
}
