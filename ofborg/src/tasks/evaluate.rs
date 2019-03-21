/// This is what evaluates every pull-request
extern crate amqp;
extern crate env_logger;
extern crate uuid;

use crate::maintainers;
use crate::maintainers::ImpactedMaintainers;
use amqp::protocol::basic::{BasicProperties, Deliver};
use hubcaps;
use hubcaps::issues::Issue;
use ofborg::acl::ACL;
use ofborg::checkout;
use ofborg::commentparser::Subset;
use ofborg::commitstatus::CommitStatus;
use ofborg::evalchecker::EvalChecker;
use ofborg::files::file_to_str;
use ofborg::message::{buildjob, evaluationjob};
use ofborg::nix;
use ofborg::outpathdiff::{OutPathDiff, OutPaths};
use ofborg::stats;
use ofborg::stats::Event;
use ofborg::systems;
use ofborg::tagger::{
    MaintainerPRTagger, PathsTagger, PkgsAddedRemovedTagger, RebuildTagger, StdenvTagger,
};
use ofborg::worker;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::time::Instant;
use tasks::eval;
use uuid::Uuid;

pub struct EvaluationWorker<E> {
    cloner: checkout::CachedCloner,
    nix: nix::Nix,
    github: hubcaps::Github,
    acl: ACL,
    identity: String,
    events: E,
    tag_paths: HashMap<String, Vec<String>>,
}

impl<E: stats::SysEvents> EvaluationWorker<E> {
    pub fn new(
        cloner: checkout::CachedCloner,
        nix: &nix::Nix,
        github: hubcaps::Github,
        acl: ACL,
        identity: String,
        events: E,
        tag_paths: HashMap<String, Vec<String>>,
    ) -> EvaluationWorker<E> {
        EvaluationWorker {
            cloner,
            nix: nix.without_limited_supported_systems(),
            github,
            acl,
            identity,
            events,
            tag_paths,
        }
    }

    fn actions(&self) -> evaluationjob::Actions {
        evaluationjob::Actions {}
    }

    fn tag_from_title(&self, issue: &hubcaps::issues::IssueRef) {
        let darwin = issue
            .get()
            .map(|iss| {
                iss.title.to_lowercase().contains("darwin")
                    || iss.title.to_lowercase().contains("macos")
            })
            .unwrap_or(false);

        if darwin {
            update_labels(&issue, &[String::from("6.topic: darwin")], &[]);
        }
    }

    fn tag_from_paths(&self, issue: &hubcaps::issues::IssueRef, paths: &[String]) {
        let mut tagger = PathsTagger::new(self.tag_paths.clone());

        for path in paths {
            tagger.path_changed(&path);
        }

        update_labels(&issue, &tagger.tags_to_add(), &tagger.tags_to_remove());
    }
}

impl<E: stats::SysEvents + 'static> worker::SimpleWorker for EvaluationWorker<E> {
    type J = evaluationjob::EvaluationJob;

    fn msg_to_job(
        &mut self,
        _: &Deliver,
        _: &BasicProperties,
        body: &[u8],
    ) -> Result<Self::J, String> {
        self.events.notify(Event::JobReceived);
        match evaluationjob::from(body) {
            Ok(e) => {
                self.events.notify(Event::JobDecodeSuccess);
                Ok(e)
            }
            Err(e) => {
                self.events.notify(Event::JobDecodeFailure);
                error!(
                    "Failed to decode message: {:?}, Err: {:?}",
                    String::from_utf8(body.to_vec()),
                    e
                );
                Err("Failed to decode message".to_owned())
            }
        }
    }

    fn consumer(&mut self, job: &evaluationjob::EvaluationJob) -> worker::Actions {
        let repo = self
            .github
            .repo(job.repo.owner.clone(), job.repo.name.clone());
        let gists = self.github.gists();
        let pulls = repo.pulls();
        let pull = pulls.get(job.pr.number);
        let issue_ref = repo.issue(job.pr.number);
        let issue: Issue;
        let auto_schedule_build_archs: Vec<systems::System>;

        match issue_ref.get() {
            Ok(iss) => {
                if iss.state == "closed" {
                    self.events.notify(Event::IssueAlreadyClosed);
                    info!("Skipping {} because it is closed", job.pr.number);
                    return self.actions().skip(&job);
                }

                if issue_is_wip(&iss) {
                    auto_schedule_build_archs = vec![];
                } else {
                    auto_schedule_build_archs = self.acl.build_job_architectures_for_user_repo(
                        &iss.user.login,
                        &job.repo.full_name,
                    );
                }

                issue = iss;
            }

            Err(e) => {
                self.events.notify(Event::IssueFetchFailed);
                info!("Error fetching {}!", job.pr.number);
                info!("E: {:?}", e);
                return self.actions().skip(&job);
            }
        };

        self.tag_from_title(&issue_ref);

        let mut overall_status = CommitStatus::new(
            repo.statuses(),
            job.pr.head_sha.clone(),
            "grahamcofborg-eval".to_owned(),
            "Starting".to_owned(),
            None,
        );

        overall_status.set_with_description("Starting", hubcaps::statuses::State::Pending);

        let project = self
            .cloner
            .project(&job.repo.full_name, job.repo.clone_url.clone());

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

            self.events
                .notify(Event::TargetBranchFailsEvaluation(target_branch.clone()));
            overall_status.set_with_description(
                format!("Target branch {} doesn't evaluate!", &target_branch).as_ref(),
                hubcaps::statuses::State::Failure,
            );

            return self.actions().skip(&job);
        }
        self.events.notify(Event::EvaluationDuration(
            target_branch.clone(),
            target_branch_rebuild_sniff_start.elapsed().as_secs(),
        ));
        self.events
            .notify(Event::EvaluationDurationCount(target_branch.clone()));

        overall_status.set_with_description("Fetching PR", hubcaps::statuses::State::Pending);

        co.fetch_pr(job.pr.number).unwrap();

        if !co.commit_exists(job.pr.head_sha.as_ref()) {
            overall_status
                .set_with_description("Commit not found", hubcaps::statuses::State::Error);

            info!("Commit {} doesn't exist", job.pr.head_sha);
            return self.actions().skip(&job);
        }

        let possibly_touched_packages = parse_commit_messages(
            &co.commit_messages_from_head(&job.pr.head_sha)
                .unwrap_or_else(|_| vec!["".to_owned()]),
        );

        let changed_paths = co
            .files_changed_from_head(&job.pr.head_sha)
            .unwrap_or_else(|_| vec![]);
        self.tag_from_paths(&issue_ref, &changed_paths);

        overall_status.set_with_description("Merging PR", hubcaps::statuses::State::Pending);

        if co.merge_commit(job.pr.head_sha.as_ref()).is_err() {
            overall_status
                .set_with_description("Failed to merge", hubcaps::statuses::State::Failure);

            info!("Failed to merge {}", job.pr.head_sha);

            update_labels(&issue_ref, &["2.status: merge conflict".to_owned()], &[]);

            return self.actions().skip(&job);
        } else {
            update_labels(&issue_ref, &[], &["2.status: merge conflict".to_owned()]);
        }

        overall_status
            .set_with_description("Checking new stdenvs", hubcaps::statuses::State::Pending);

        stdenvs.identify_after();

        overall_status
            .set_with_description("Checking new out paths", hubcaps::statuses::State::Pending);

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
                )
                .as_ref(),
                hubcaps::statuses::State::Failure,
            );
            return self.actions().skip(&job);
        }

        println!("Got path: {:?}, building", refpath);
        overall_status
            .set_with_description("Beginning Evaluations", hubcaps::statuses::State::Pending);

        let eval_checks = vec![
            EvalChecker::new(
                "package-list",
                nix::Operation::QueryPackagesJSON,
                vec![String::from("--file"), String::from(".")],
                self.nix.clone(),
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
                self.nix.clone(),
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
                self.nix.clone(),
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
                self.nix.clone(),
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
                self.nix.clone(),
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
                self.nix.clone(),
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
                self.nix.clone(),
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

                    let mut try_build: Vec<String> = pkgs
                        .keys()
                        .map(|pkgarch| pkgarch.package.clone())
                        .filter(|pkg| possibly_touched_packages.contains(&pkg))
                        .collect();
                    try_build.sort();
                    try_build.dedup();

                    if !try_build.is_empty() && try_build.len() <= 10 {
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
                        for arch in auto_schedule_build_archs.iter() {
                            let (exchange, routingkey) = arch.as_build_destination();
                            response.push(worker::publish_serde_action(exchange, routingkey, &msg));
                        }
                        response.push(worker::publish_serde_action(
                            Some("build-results".to_string()),
                            None,
                            &buildjob::QueuedBuildJobs {
                                job: msg,
                                architectures: auto_schedule_build_archs
                                    .into_iter()
                                    .map(|arch| arch.to_string())
                                    .collect(),
                            },
                        ));
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
                &issue_ref,
                &stdenvtagger.tags_to_add(),
                &stdenvtagger.tags_to_remove(),
            );

            if let Some((removed, added)) = rebuildsniff.package_diff() {
                let mut addremovetagger = PkgsAddedRemovedTagger::new();
                addremovetagger.changed(&removed, &added);
                update_labels(
                    &issue_ref,
                    &addremovetagger.tags_to_add(),
                    &addremovetagger.tags_to_remove(),
                );
            }

            let mut rebuild_tags = RebuildTagger::new();
            if let Some(attrs) = rebuildsniff.calculate_rebuild() {
                if !attrs.is_empty() {
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

                    let changed_attributes = attrs
                        .iter()
                        .map(|attr| attr.package.split('.').collect::<Vec<&str>>())
                        .collect::<Vec<Vec<&str>>>();

                    let m = ImpactedMaintainers::calculate(
                        &self.nix,
                        &PathBuf::from(&refpath),
                        &changed_paths,
                        &changed_attributes,
                    );

                    let gist_url = make_gist(
                        &gists,
                        "Potential Maintainers",
                        Some("".to_owned()),
                        match m {
                            Ok(ref maintainers) => format!("Maintainers:\n{}", maintainers),
                            Err(ref e) => format!("Ignorable calculation error:\n{:?}", e),
                        },
                    );

                    if let Ok(ref maint) = m {
                        request_reviews(&maint, &pull);
                        let mut maint_tagger = MaintainerPRTagger::new();
                        maint_tagger
                            .record_maintainer(&issue.user.login, &maint.maintainers_by_package());
                        update_labels(
                            &issue_ref,
                            &maint_tagger.tags_to_add(),
                            &maint_tagger.tags_to_remove(),
                        );
                    }

                    let mut status = CommitStatus::new(
                        repo.statuses(),
                        job.pr.head_sha.clone(),
                        String::from("grahamcofborg-eval-check-maintainers"),
                        String::from("matching changed paths to changed attrs..."),
                        gist_url,
                    );

                    status.set(hubcaps::statuses::State::Success);
                }

                rebuild_tags.parse_attrs(attrs);
            }

            update_labels(
                &issue_ref,
                &rebuild_tags.tags_to_add(),
                &rebuild_tags.tags_to_remove(),
            );

            overall_status.set_with_description("^.^!", hubcaps::statuses::State::Success);
        } else {
            overall_status
                .set_with_description("Complete, with errors", hubcaps::statuses::State::Failure);
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

pub fn update_labels(issue: &hubcaps::issues::IssueRef, add: &[String], remove: &[String]) {
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
        .map(|l| l.as_ref())
        .collect();
    info!("Adding labels: {:?}", to_add);

    let to_remove: Vec<String> = remove
        .iter()
        .filter(|l| existing.contains(l)) // Remove labels already on the issue
        .cloned()
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
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() == 2 {
                Some(parts[0])
            } else {
                None
            }
        })
        .flat_map(|line| {
            let pkgs: Vec<&str> = line.split(',').collect();
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
            "}",            // Don't support such fancy syntax
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

fn request_reviews(maint: &maintainers::ImpactedMaintainers, pull: &hubcaps::pulls::PullRequest) {
    if maint.maintainers().len() < 10 {
        for maintainer in maint.maintainers() {
            if let Err(e) =
                pull.review_requests()
                    .create(&hubcaps::review_requests::ReviewRequestOptions {
                        reviewers: vec![maintainer.clone()],
                        team_reviewers: vec![],
                    })
            {
                println!("Failure requesting a review from {}: {:#?}", maintainer, e,);
            }
        }
    }
}
