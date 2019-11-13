use crate::checkout::CachedProjectCo;
use crate::commentparser::Subset;
use crate::commitstatus::CommitStatus;
use crate::evalchecker::EvalChecker;
use crate::maintainers;
use crate::maintainers::ImpactedMaintainers;
use crate::message::buildjob::BuildJob;
use crate::message::evaluationjob::EvaluationJob;
use crate::nix;
use crate::nix::Nix;
use crate::nixenv::HydraNixEnv;
use crate::outpathdiff::{OutPathDiff, PackageArch};
use crate::tagger::{MaintainerPRTagger, PathsTagger, RebuildTagger};
use crate::tagger::{PkgsAddedRemovedTagger, StdenvTagger};
use crate::tasks::eval::{
    stdenvs::Stdenvs, Error, EvaluationComplete, EvaluationStrategy, StepResult,
};
use crate::tasks::evaluate::make_gist;
use crate::tasks::evaluate::update_labels;
use chrono::Utc;
use hubcaps::checks::{CheckRunOptions, CheckRunState, Conclusion, Output};
use hubcaps::gists::Gists;
use hubcaps::issues::{Issue, IssueRef};
use hubcaps::repositories::Repository;
use std::collections::HashMap;
use std::path::Path;
use uuid::Uuid;

pub struct NixpkgsStrategy<'a> {
    job: &'a EvaluationJob,
    pull: &'a hubcaps::pulls::PullRequest<'a>,
    issue: &'a Issue,
    issue_ref: &'a IssueRef<'a>,
    repo: &'a Repository<'a>,
    gists: &'a Gists<'a>,
    nix: Nix,
    tag_paths: &'a HashMap<String, Vec<String>>,
    stdenv_diff: Option<Stdenvs>,
    outpath_diff: Option<OutPathDiff>,
    changed_paths: Option<Vec<String>>,
    touched_packages: Option<Vec<String>>,
}

impl<'a> NixpkgsStrategy<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        job: &'a EvaluationJob,
        pull: &'a hubcaps::pulls::PullRequest,
        issue: &'a Issue,
        issue_ref: &'a IssueRef,
        repo: &'a Repository,
        gists: &'a Gists,
        nix: Nix,
        tag_paths: &'a HashMap<String, Vec<String>>,
    ) -> NixpkgsStrategy<'a> {
        Self {
            job,
            pull,
            issue,
            issue_ref,
            repo,
            gists,
            nix,
            tag_paths,
            stdenv_diff: None,
            outpath_diff: None,
            changed_paths: None,
            touched_packages: None,
        }
    }

    fn tag_from_title(&self) {
        let darwin = self
            .issue_ref
            .get()
            .map(|iss| {
                iss.title.to_lowercase().contains("darwin")
                    || iss.title.to_lowercase().contains("macos")
            })
            .unwrap_or(false);

        if darwin {
            update_labels(&self.issue_ref, &[String::from("6.topic: darwin")], &[]);
        }
    }

    fn tag_from_paths(&self) {
        if let Some(ref changed_paths) = self.changed_paths {
            let mut tagger = PathsTagger::new(self.tag_paths.clone());

            for path in changed_paths {
                tagger.path_changed(&path);
            }

            update_labels(
                &self.issue_ref,
                &tagger.tags_to_add(),
                &tagger.tags_to_remove(),
            );
        }
    }

    fn check_stdenvs_before(&mut self, dir: &Path) {
        let mut stdenvs = Stdenvs::new(self.nix.clone(), dir.to_path_buf());
        stdenvs.identify_before();
        self.stdenv_diff = Some(stdenvs);
    }

    fn check_stdenvs_after(&mut self) {
        if let Some(ref mut stdenvs) = self.stdenv_diff {
            stdenvs.identify_after();
        }
    }

    fn update_stdenv_labels(&self) {
        if let Some(ref stdenvs) = self.stdenv_diff {
            let mut stdenvtagger = StdenvTagger::new();
            if !stdenvs.are_same() {
                stdenvtagger.changed(stdenvs.changed());
            }
            update_labels(
                &self.issue_ref,
                &stdenvtagger.tags_to_add(),
                &stdenvtagger.tags_to_remove(),
            );
        }
    }

    fn check_outpaths_before(&mut self, dir: &Path) -> StepResult<()> {
        let mut rebuildsniff = OutPathDiff::new(self.nix.clone(), dir.to_path_buf());

        if let Err(err) = rebuildsniff.find_before() {
            /*
            self.events
                .notify(Event::TargetBranchFailsEvaluation(target_branch.clone()));
             */

            Err(Error::FailWithGist(
                String::from("The branch this PR will merge in to does not cleanly evaluate, and so this PR cannot be checked."),
                String::from("Output path comparison"),
                err.display(),
            ))
        } else {
            self.outpath_diff = Some(rebuildsniff);
            Ok(())
        }
    }

    fn check_outpaths_after(&mut self) -> StepResult<()> {
        if let Some(ref mut rebuildsniff) = self.outpath_diff {
            if let Err(err) = rebuildsniff.find_after() {
                Err(Error::FailWithGist(
                    String::from("This PR does not cleanly list of package outputs after merging."),
                    String::from("Output path comparison"),
                    err.display(),
                ))
            } else {
                Ok(())
            }
        } else {
            Err(Error::Fail(String::from(
                "Ofborg BUG: No outpath diff! Please report!",
            )))
        }
    }

    fn performance_stats(&self) -> Vec<CheckRunOptions> {
        if let Some(ref rebuildsniff) = self.outpath_diff {
            if let Some(report) = rebuildsniff.performance_diff() {
                return vec![CheckRunOptions {
                    name: "Evaluation Performance Report".to_owned(),
                    actions: None,
                    completed_at: Some(
                        Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
                    ),
                    started_at: None,
                    conclusion: Some(Conclusion::Success),
                    status: Some(CheckRunState::Completed),
                    details_url: None,
                    external_id: None,
                    head_sha: self.job.pr.head_sha.clone(),
                    output: Some(Output {
                        title: "Evaluator Performance Report".to_string(),
                        summary: "".to_string(),
                        text: Some(report.markdown()),
                        annotations: None,
                        images: None,
                    }),
                }];
            }
        }
        vec![]
    }

    fn update_new_package_labels(&self) {
        if let Some(ref rebuildsniff) = self.outpath_diff {
            if let Some((removed, added)) = rebuildsniff.package_diff() {
                let mut addremovetagger = PkgsAddedRemovedTagger::new();
                addremovetagger.changed(&removed, &added);
                update_labels(
                    &self.issue_ref,
                    &addremovetagger.tags_to_add(),
                    &addremovetagger.tags_to_remove(),
                );
            }
        }
    }

    fn update_rebuild_labels(&self, dir: &Path, overall_status: &mut CommitStatus) {
        if let Some(ref rebuildsniff) = self.outpath_diff {
            let mut rebuild_tags = RebuildTagger::new();

            if let Some(attrs) = rebuildsniff.calculate_rebuild() {
                if !attrs.is_empty() {
                    overall_status.set_url(self.gist_changed_paths(&attrs));
                    self.record_impacted_maintainers(&dir, &attrs);
                }

                rebuild_tags.parse_attrs(attrs.clone());
            }

            update_labels(
                &self.issue_ref,
                &rebuild_tags.tags_to_add(),
                &rebuild_tags.tags_to_remove(),
            );
        }
    }

    fn gist_changed_paths(&self, attrs: &[PackageArch]) -> Option<String> {
        make_gist(
            &self.gists,
            "Changed Paths",
            Some("".to_owned()),
            attrs
                .iter()
                .map(|attr| format!("{}\t{}", &attr.architecture, &attr.package))
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }

    fn record_impacted_maintainers(&self, dir: &Path, attrs: &[PackageArch]) {
        let changed_attributes = attrs
            .iter()
            .map(|attr| attr.package.split('.').collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>();

        if let Some(ref changed_paths) = self.changed_paths {
            let m = ImpactedMaintainers::calculate(
                &self.nix,
                &dir.to_path_buf(),
                &changed_paths,
                &changed_attributes,
            );

            let gist_url = make_gist(
                &self.gists,
                "Potential Maintainers",
                Some("".to_owned()),
                match m {
                    Ok(ref maintainers) => format!("Maintainers:\n{}", maintainers),
                    Err(ref e) => format!("Ignorable calculation error:\n{:?}", e),
                },
            );

            let status = CommitStatus::new(
                self.repo.statuses(),
                self.job.pr.head_sha.clone(),
                String::from("grahamcofborg-eval-check-maintainers"),
                String::from("matching changed paths to changed attrs..."),
                gist_url,
            );
            status.set(hubcaps::statuses::State::Success);

            if let Ok(ref maint) = m {
                request_reviews(&maint, &self.pull);
                let mut maint_tagger = MaintainerPRTagger::new();
                maint_tagger
                    .record_maintainer(&self.issue.user.login, &maint.maintainers_by_package());
                update_labels(
                    &self.issue_ref,
                    &maint_tagger.tags_to_add(),
                    &maint_tagger.tags_to_remove(),
                );
            }
        }
    }

    fn check_meta_queue_builds(&self, dir: &Path) -> StepResult<Vec<BuildJob>> {
        if let Some(ref possibly_touched_packages) = self.touched_packages {
            let mut status = CommitStatus::new(
                self.repo.statuses(),
                self.job.pr.head_sha.clone(),
                String::from("grahamcofborg-eval-check-meta"),
                String::from("config.nix: checkMeta = true"),
                None,
            );
            status.set(hubcaps::statuses::State::Pending);

            let nixenv = HydraNixEnv::new(self.nix.clone(), dir.to_path_buf(), true);
            match nixenv.execute_with_stats() {
                Ok((pkgs, _stats)) => {
                    let mut try_build: Vec<String> = pkgs
                        .keys()
                        .map(|pkgarch| pkgarch.package.clone())
                        .filter(|pkg| possibly_touched_packages.contains(&pkg))
                        .collect();
                    try_build.sort();
                    try_build.dedup();

                    status.set_url(None);
                    status.set(hubcaps::statuses::State::Success);

                    if !try_build.is_empty() && try_build.len() <= 10 {
                        // In the case of trying to merge master in to
                        // a stable branch, we don't want to do this.
                        // Therefore, only schedule builds if there
                        // less than or exactly 10
                        Ok(vec![BuildJob::new(
                            self.job.repo.clone(),
                            self.job.pr.clone(),
                            Subset::Nixpkgs,
                            try_build,
                            None,
                            None,
                            format!("{}", Uuid::new_v4()),
                        )])
                    } else {
                        Ok(vec![])
                    }
                }
                Err(out) => {
                    status.set_url(make_gist(&self.gists, "Meta Check", None, out.display()));
                    status.set(hubcaps::statuses::State::Failure);
                    Err(Error::Fail(String::from(
                        "Failed to validate package metadata.",
                    )))
                }
            }
        } else {
            Ok(vec![])
        }
    }
}

impl<'a> EvaluationStrategy for NixpkgsStrategy<'a> {
    fn pre_clone(&mut self) -> StepResult<()> {
        self.tag_from_title();
        Ok(())
    }

    fn on_target_branch(&mut self, dir: &Path, status: &mut CommitStatus) -> StepResult<()> {
        status.set_with_description(
            "Checking original stdenvs",
            hubcaps::statuses::State::Pending,
        );
        self.check_stdenvs_before(dir);

        status.set_with_description(
            "Checking original out paths",
            hubcaps::statuses::State::Pending,
        );
        self.check_outpaths_before(dir)?;

        Ok(())
    }

    fn after_fetch(&mut self, co: &CachedProjectCo) -> StepResult<()> {
        let changed_paths = co
            .files_changed_from_head(&self.job.pr.head_sha)
            .unwrap_or_else(|_| vec![]);
        self.changed_paths = Some(changed_paths);
        self.tag_from_paths();

        self.touched_packages = Some(parse_commit_messages(
            &co.commit_messages_from_head(&self.job.pr.head_sha)
                .unwrap_or_else(|_| vec!["".to_owned()]),
        ));

        Ok(())
    }

    fn merge_conflict(&mut self) {
        update_labels(
            &self.issue_ref,
            &["2.status: merge conflict".to_owned()],
            &[],
        );
    }

    fn after_merge(&mut self, status: &mut CommitStatus) -> StepResult<()> {
        update_labels(
            &self.issue_ref,
            &[],
            &["2.status: merge conflict".to_owned()],
        );

        status.set_with_description("Checking new stdenvs", hubcaps::statuses::State::Pending);
        self.check_stdenvs_after();

        status.set_with_description("Checking new out paths", hubcaps::statuses::State::Pending);
        self.check_outpaths_after()?;

        Ok(())
    }

    fn evaluation_checks(&self) -> Vec<EvalChecker> {
        vec![
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
                "nixos",
                nix::Operation::Instantiate,
                vec![
                    String::from("--arg"),
                    String::from("nixpkgs"),
                    String::from("{ outPath=./.; revCount=999999; shortRev=\"ofborg\"; }"),
                    String::from("./nixos/release-combined.nix"),
                    String::from("-A"),
                    String::from("tested"),
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
            EvalChecker::new(
                "darwin",
                nix::Operation::Instantiate,
                vec![
                    String::from("--arg"),
                    String::from("nixpkgs"),
                    String::from("{ outPath=./.; revCount=999999; shortRev=\"ofborg\"; }"),
                    String::from("./pkgs/top-level/release.nix"),
                    String::from("-A"),
                    String::from("darwin-tested"),
                ],
                self.nix.clone(),
            ),
        ]
    }

    fn all_evaluations_passed(
        &mut self,
        dir: &Path,
        status: &mut CommitStatus,
    ) -> StepResult<EvaluationComplete> {
        self.update_stdenv_labels();

        status.set_with_description(
            "Calculating Changed Outputs",
            hubcaps::statuses::State::Pending,
        );

        self.update_new_package_labels();
        self.update_rebuild_labels(&dir, status);
        let checks = self.performance_stats();

        let builds = self.check_meta_queue_builds(&dir)?;
        Ok(EvaluationComplete { builds, checks })
    }
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
