/// This is what evaluates every pull-requests
extern crate amqp;
extern crate env_logger;
extern crate uuid;

use tasks::eval;
use uuid::Uuid;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use ofborg::checkout;
use ofborg::message::{massrebuildjob, buildjob};
use std::time::Instant;
use ofborg::files::file_to_str;
use ofborg::nix;
use ofborg::acl::ACL;
use ofborg::stats;
use ofborg::stats::Event;
use ofborg::worker;
use ofborg::tagger::{StdenvTagger, RebuildTagger, PathsTagger, PkgsAddedRemovedTagger};
use ofborg::outpathdiff::{OutPaths, OutPathDiff};
use ofborg::evalchecker::EvalChecker;
use ofborg::commitstatus::CommitStatus;
use ofborg::commentparser::Subset;
use amqp::protocol::basic::{Deliver, BasicProperties};
use hubcaps;

pub struct MassRebuildWorker<E> {
    cloner: checkout::CachedCloner,
    nix: nix::Nix,
    github: hubcaps::Github,
    acl: ACL,
    identity: String,
    events: E,
    tag_paths: HashMap<String, Vec<String>>,
}

impl<E: stats::SysEvents> MassRebuildWorker<E> {
    pub fn new(
        cloner: checkout::CachedCloner,
        nix: &nix::Nix,
        github: hubcaps::Github,
        acl: ACL,
        identity: String,
        events: E,
        tag_paths: HashMap<String, Vec<String>>,
    ) -> MassRebuildWorker<E> {
        MassRebuildWorker {
            cloner,
            nix: nix.without_limited_supported_systems(),
            github,
            acl,
            identity,
            events,
            tag_paths,
        }
    }

    fn actions(&self) -> massrebuildjob::Actions {
        massrebuildjob::Actions {}
    }

    fn tag_from_title(&self, issue: &hubcaps::issues::IssueRef) {
        let darwin = issue.get()
            .map(|iss| {
                iss.title.to_lowercase().contains("darwin") ||
                iss.title.to_lowercase().contains("macos")
            })
            .unwrap_or(false);

        if darwin {
            update_labels(
                &issue,
                &vec![String::from("6.topic: darwin")],
                &vec![],
            );
        }
    }

    fn tag_from_paths(&self, issue: &hubcaps::issues::IssueRef, paths: Vec<String>) {
        let mut tagger = PathsTagger::new(self.tag_paths.clone());

        for path in paths {
            tagger.path_changed(&path);
        }

        update_labels(
            &issue,
            &tagger.tags_to_add(),
            &tagger.tags_to_remove(),
        );
    }
}

impl<E: stats::SysEvents + 'static> worker::SimpleWorker for MassRebuildWorker<E> {
    type J = massrebuildjob::MassRebuildJob;

    fn msg_to_job(
        &mut self,
        _: &Deliver,
        _: &BasicProperties,
        body: &Vec<u8>,
    ) -> Result<Self::J, String> {
        self.events.notify(Event::JobReceived);
        match massrebuildjob::from(body) {
            Ok(e) => {
                self.events.notify(Event::JobDecodeSuccess);
                Ok(e)
            }
            Err(e) => {
                self.events.notify(Event::JobDecodeFailure);
                error!(
                    "Failed to decode message: {:?}, Err: {:?}",
                    String::from_utf8(body.clone()),
                    e
                );
                Err("Failed to decode message".to_owned())
            }
        }
    }

    fn consumer(&mut self, job: &massrebuildjob::MassRebuildJob) -> worker::Actions {
        let repo = self.github.repo(
            job.repo.owner.clone(),
            job.repo.name.clone(),
        );
        let gists = self.github.gists();
        let issue = repo.issue(job.pr.number);

        let auto_schedule_build_archs: Vec<buildjob::ExchangeQueue>;

        match issue.get() {
            Ok(iss) => {
                if iss.state == "closed" {
                    self.events.notify(Event::IssueAlreadyClosed);
                    info!("Skipping {} because it is closed", job.pr.number);
                    return self.actions().skip(&job);
                }

                if issue_is_wip(&iss) {
                    auto_schedule_build_archs = vec![];
                } else {
                    auto_schedule_build_archs = self.acl.build_job_destinations_for_user_repo(
                        &iss.user.login,
                        &job.repo.full_name,
                    );
                }
            }
            Err(e) => {
                self.events.notify(Event::IssueFetchFailed);
                info!("Error fetching {}!", job.pr.number);
                info!("E: {:?}", e);
                return self.actions().skip(&job);
            }
        }

        self.tag_from_title(&issue);

        let mut overall_status = CommitStatus::new(
            repo.statuses(),
            job.pr.head_sha.clone(),
            "grahamcofborg-eval".to_owned(),
            "Starting".to_owned(),
            None,
        );

        overall_status.set_with_description("Starting", hubcaps::statuses::State::Pending);

        let project = self.cloner.project(
            &job.repo.full_name,
            job.repo.clone_url.clone(),
        );

        overall_status.set_with_description("Cloning project", hubcaps::statuses::State::Pending);

        info!("Working on {}", job.pr.number);
        let co = project
            .clone_for("mr-est".to_string(), self.identity.clone())
            .unwrap();

        let target_branch = match job.pr.target_branch.clone() {
            Some(x) => x,
            None => String::from("master"),
        };

        overall_status.set_with_description(
            format!("Checking out {}", &target_branch).as_ref(),
            hubcaps::statuses::State::Pending,
        );
        info!("Checking out target branch {}", &target_branch);
        let refpath = co.checkout_origin_ref(target_branch.as_ref()).unwrap();

        overall_status.set_with_description(
            "Checking original stdenvs",
            hubcaps::statuses::State::Pending,
        );


        let mut stdenvs = eval::Stdenvs::new(self.nix.clone(), PathBuf::from(&refpath));
        stdenvs.identify_before();

        let mut rebuildsniff = OutPathDiff::new(self.nix.clone(), PathBuf::from(&refpath));

        overall_status.set_with_description(
            "Checking original out paths",
            hubcaps::statuses::State::Pending,
        );

        let target_branch_rebuild_sniff_start = Instant::now();

        if let Err(mut output) = rebuildsniff.find_before() {
            overall_status.set_url(make_gist(
                &gists,
                "Output path comparison",
                Some("".to_owned()),
                file_to_str(&mut output),
            ));

            self.events.notify(Event::TargetBranchFailsEvaluation(target_branch.clone()));
            overall_status.set_with_description(
                format!("Target branch {} doesn't evaluate!", &target_branch).as_ref(),
                hubcaps::statuses::State::Failure,
            );

            return self.actions().skip(&job);
        }
        self.events.notify(
            Event::EvaluationDuration(
                target_branch.clone(),
                target_branch_rebuild_sniff_start.elapsed().as_secs(),
            )
        );
        self.events.notify(
            Event::EvaluationDurationCount(
                target_branch.clone()
            )
        );

        overall_status.set_with_description("Fetching PR", hubcaps::statuses::State::Pending);

        co.fetch_pr(job.pr.number).unwrap();

        if !co.commit_exists(job.pr.head_sha.as_ref()) {
            overall_status.set_with_description(
                "Commit not found",
                hubcaps::statuses::State::Error,
            );

            info!("Commit {} doesn't exist", job.pr.head_sha);
            return self.actions().skip(&job);
        }

        let possibly_touched_packages =
            parse_commit_messages(&co.commit_messages_from_head(&job.pr.head_sha).unwrap_or(
                vec!["".to_owned()],
            ));

        self.tag_from_paths(
            &issue,
            co.files_changed_from_head(&job.pr.head_sha).unwrap_or(vec![])
        );

        overall_status.set_with_description("Merging PR", hubcaps::statuses::State::Pending);

        if let Err(_) = co.merge_commit(job.pr.head_sha.as_ref()) {
            overall_status.set_with_description(
                "Failed to merge",
                hubcaps::statuses::State::Failure,
            );

            info!("Failed to merge {}", job.pr.head_sha);

            update_labels(
                &issue,
                &vec!["2.status: merge conflict".to_owned()],
                &vec![],
            );

            return self.actions().skip(&job);
        } else {
            update_labels(
                &issue,
                &vec![],
                &vec!["2.status: merge conflict".to_owned()],
            );
        }

        overall_status.set_with_description(
            "Checking new stdenvs",
            hubcaps::statuses::State::Pending,
        );

        stdenvs.identify_after();

        overall_status.set_with_description(
            "Checking new out paths",
            hubcaps::statuses::State::Pending,
        );

        if let Err(mut output) = rebuildsniff.find_after() {
            overall_status.set_url(make_gist(
                &gists,
                "Output path comparison",
                Some("".to_owned()),
                file_to_str(&mut output),
            ));
            overall_status.set_with_description(
                format!(
                    "Failed to enumerate outputs after merging to {}",
                    &target_branch
                ).as_ref(),
                hubcaps::statuses::State::Failure,
            );
            return self.actions().skip(&job);
        }

        println!("Got path: {:?}, building", refpath);
        overall_status.set_with_description(
            "Beginning Evaluations",
            hubcaps::statuses::State::Pending,
        );

        let eval_checks = vec![
            EvalChecker::new(
                "package-list",
                nix::Operation::QueryPackagesJSON,
                vec![
                    String::from("--file"),
                    String::from("."),
                ],
                self.nix.clone()
            ),

            EvalChecker::new(
                "package-list-no-aliases",
                nix::Operation::QueryPackagesJSON,
                vec![
                    String::from("--file"),
                    String::from("."),
                    String::from("--arg"),
                    String::from("config"),
                    String::from("{ allowAliases = false; }"),
                ],
                self.nix.clone()
            ),

            EvalChecker::new(
                "nixos-options",
                nix::Operation::Instantiate,
                vec![
                    String::from("--arg"),
                    String::from("nixpkgs"),
                    String::from("{ outPath=./.; revCount=999999; shortRev=\"ofborg\"; }"),
                    String::from("./nixos/release.nix"),
                    String::from("-A"),
                    String::from("options"),
                ],
                self.nix.clone()
            ),

            EvalChecker::new(
                "nixos-manual",
                nix::Operation::Instantiate,
                vec![
                    String::from("--arg"),
                    String::from("nixpkgs"),
                    String::from("{ outPath=./.; revCount=999999; shortRev=\"ofborg\"; }"),
                    String::from("./nixos/release.nix"),
                    String::from("-A"),
                    String::from("manual"),
                ],
                self.nix.clone()
            ),

            EvalChecker::new(
                "nixpkgs-manual",
                nix::Operation::Instantiate,
                vec![
                    String::from("--arg"),
                    String::from("nixpkgs"),
                    String::from("{ outPath=./.; revCount=999999; shortRev=\"ofborg\"; }"),
                    String::from("./pkgs/top-level/release.nix"),
                    String::from("-A"),
                    String::from("manual"),
                ],
                self.nix.clone()
            ),

            EvalChecker::new(
                "nixpkgs-tarball",
                nix::Operation::Instantiate,
                vec![
                    String::from("--arg"),
                    String::from("nixpkgs"),
                    String::from("{ outPath=./.; revCount=999999; shortRev=\"ofborg\"; }"),
                    String::from("./pkgs/top-level/release.nix"),
                    String::from("-A"),
                    String::from("tarball"),
                ],
                self.nix.clone()
            ),

            EvalChecker::new(
                "nixpkgs-unstable-jobset",
                nix::Operation::Instantiate,
                vec![
                    String::from("--arg"),
                    String::from("nixpkgs"),
                    String::from("{ outPath=./.; revCount=999999; shortRev=\"ofborg\"; }"),
                    String::from("./pkgs/top-level/release.nix"),
                    String::from("-A"),
                    String::from("unstable"),
                ],
                self.nix.clone()
            ),
        ];

        let mut eval_results: bool = eval_checks
            .into_iter()
            .map(|check| {
                let mut status = CommitStatus::new(
                    repo.statuses(),
                    job.pr.head_sha.clone(),
                    check.name(),
                    check.cli_cmd(),
                    None,
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
                            &check.name(),
                            Some(format!("{:?}", state)),
                            file_to_str(&mut out),
                        );
                    }
                }

                status.set_url(gist_url);
                status.set(state.clone());

                if state == hubcaps::statuses::State::Success {
                    Ok(())
                } else {
                    Err(())
                }
            })
            .all(|status| status == Ok(()));


        let mut response: worker::Actions = vec![];

        if eval_results {
            let mut status = CommitStatus::new(
                repo.statuses(),
                job.pr.head_sha.clone(),
                String::from("grahamcofborg-eval-check-meta"),
                String::from("config.nix: checkMeta = true"),
                None,
            );

            status.set(hubcaps::statuses::State::Pending);

            let state: hubcaps::statuses::State;
            let gist_url: Option<String>;

            let checker = OutPaths::new(self.nix.clone(), PathBuf::from(&refpath), true);
            match checker.find() {
                Ok(pkgs) => {
                    state = hubcaps::statuses::State::Success;
                    gist_url = None;

                    let mut try_build: Vec<String> = pkgs.keys()
                        .map(|pkgarch| pkgarch.package.clone())
                        .filter(|pkg| possibly_touched_packages.contains(&pkg))
                        .collect();
                    try_build.sort();
                    try_build.dedup();

                    if try_build.len() > 0 && try_build.len() <= 10 {
                        // In the case of trying to merge master in to
                        // a stable branch, we don't want to do this.
                        // Therefore, only schedule builds if there
                        // less than or exactly 10
                        let msg = buildjob::BuildJob::new(
                            job.repo.clone(),
                            job.pr.clone(),
                            Subset::Nixpkgs,
                            try_build,
                            None,
                            None,
                            format!("{}", Uuid::new_v4()),
                        );
                        for (dest, rk) in auto_schedule_build_archs {
                            response.push(worker::publish_serde_action(dest, rk, &msg));
                        }
                    }
                }
                Err(mut out) => {
                    eval_results = false;
                    state = hubcaps::statuses::State::Failure;
                    gist_url = make_gist(
                        &gists,
                        "Meta Check",
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
                hubcaps::statuses::State::Pending,
            );

            let mut stdenvtagger = StdenvTagger::new();
            if !stdenvs.are_same() {
                stdenvtagger.changed(stdenvs.changed());
            }
            update_labels(
                &issue,
                &stdenvtagger.tags_to_add(),
                &stdenvtagger.tags_to_remove(),
            );

            if let Some((removed, added)) = rebuildsniff.package_diff() {
            let mut addremovetagger = PkgsAddedRemovedTagger::new();
                addremovetagger.changed(&removed, &added);
                update_labels(
                    &issue,
                    &addremovetagger.tags_to_add(),
                    &addremovetagger.tags_to_remove(),
                );
            }

            let mut rebuild_tags = RebuildTagger::new();
            if let Some(attrs) = rebuildsniff.calculate_rebuild() {
                if attrs.len() > 0 {
                    let gist_url = make_gist(
                        &gists,
                        "Changed Paths",
                        Some("".to_owned()),
                        attrs
                            .iter()
                            .map(|attr| format!("{}\t{}", &attr.architecture, &attr.package))
                            .collect::<Vec<String>>()
                            .join("\n"),
                    );

                    overall_status.set_url(gist_url);
                }

                rebuild_tags.parse_attrs(attrs);
            }

            update_labels(
                &issue,
                &rebuild_tags.tags_to_add(),
                &rebuild_tags.tags_to_remove(),
            );

            overall_status.set_with_description("^.^!", hubcaps::statuses::State::Success);

        } else {
            overall_status.set_with_description(
                "Complete, with errors",
                hubcaps::statuses::State::Failure,
            );
        }

        self.events.notify(Event::TaskEvaluationCheckComplete);

        self.actions().done(&job, response)
    }
}



fn make_gist<'a>(
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

pub fn update_labels(issue: &hubcaps::issues::IssueRef, add: &Vec<String>, remove: &Vec<String>) {
    let l = issue.labels();

    let existing: Vec<String> = issue
        .get()
        .unwrap()
        .labels
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

fn parse_commit_messages(messages: &[String]) -> Vec<String> {
    messages
        .iter()
        .filter_map(|line| {
            // Convert "foo: some notes" in to "foo"
            let parts: Vec<&str> = line.splitn(2, ":").collect();
            if parts.len() == 2 {
                Some(parts[0])
            } else {
                None
            }
        })
        .flat_map(|line| {
            let pkgs: Vec<&str> = line.split(",").collect();
            pkgs
        })
        .map(|line| line.trim().to_owned())
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_commit_messages() {
        let expect: Vec<&str> = vec![
            "firefox{-esr", // don't support such fancy syntax
            "}", // Don't support such fancy syntax
            "firefox",
            "buildkite-agent",
            "python.pkgs.ptyprocess",
            "python.pkgs.ptyprocess",
            "android-studio-preview",
            "foo",
            "bar",
        ];
        assert_eq!(
            parse_commit_messages(
                &"
              firefox{-esr,}: fix failing build due to the google-api-key
              Merge pull request #34483 from andir/dovecot-cve-2017-15132
              firefox: enable official branding
              Merge pull request #34442 from rnhmjoj/virtual
              buildkite-agent: enable building on darwin
              python.pkgs.ptyprocess: 0.5 -> 0.5.2
              python.pkgs.ptyprocess: move expression
              Merge pull request #34465 from steveeJ/steveej-attempt-qtile-bump-0.10.7
              android-studio-preview: 3.1.0.8 -> 3.1.0.9
              Merge pull request #34188 from dotlambda/home-assistant
              Merge pull request #34414 from dotlambda/postfix
              foo,bar: something here: yeah
            "
                    .lines()
                    .map(|l| l.to_owned())
                    .collect::<Vec<String>>(),
            ),
            expect
        );
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
