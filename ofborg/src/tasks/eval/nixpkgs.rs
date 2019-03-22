use hubcaps::issues::IssueRef;
use ofborg::checkout::CachedProjectCo;
use ofborg::commitstatus::CommitStatus;
use ofborg::evalchecker::EvalChecker;
use ofborg::message::buildjob::BuildJob;
use ofborg::nix::Nix;
use ofborg::tagger::StdenvTagger;
use ofborg::tasks::eval::{stdenvs::Stdenvs, EvaluationStrategy, StepResult};
use ofborg::tasks::evaluate::update_labels;
use std::path::Path;

pub struct NixpkgsStrategy<'a> {
    issue_ref: &'a IssueRef<'a>,
    nix: Nix,
    stdenv_diff: Option<Stdenvs>,
}
impl<'a> NixpkgsStrategy<'a> {
    pub fn new(issue_ref: &'a IssueRef, nix: Nix) -> NixpkgsStrategy<'a> {
        Self {
            issue_ref,
            nix,
            stdenv_diff: None,
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

        Ok(())
    }

    fn after_fetch(&mut self, _co: &CachedProjectCo) -> StepResult<()> {
        Ok(())
    }

    fn merge_conflict(&mut self) {}

    fn after_merge(&mut self, status: &mut CommitStatus) -> StepResult<()> {
        status.set_with_description("Checking new stdenvs", hubcaps::statuses::State::Pending);
        self.check_stdenvs_after();

        Ok(())
    }

    fn evaluation_checks(&self) -> Vec<EvalChecker> {
        vec![]
    }

    fn all_evaluations_passed(
        &mut self,
        _co: &Path,
        _status: &mut CommitStatus,
    ) -> StepResult<Vec<BuildJob>> {
        self.update_stdenv_labels();

        Ok(vec![])
    }
}
