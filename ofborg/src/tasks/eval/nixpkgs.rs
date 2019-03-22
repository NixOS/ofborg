use hubcaps::issues::IssueRef;
use ofborg::checkout::CachedProjectCo;
use ofborg::commitstatus::CommitStatus;
use ofborg::evalchecker::EvalChecker;
use ofborg::message::buildjob::BuildJob;
use ofborg::tasks::eval::{EvaluationStrategy, StepResult};
use ofborg::tasks::evaluate::update_labels;
use std::path::Path;

pub struct NixpkgsStrategy<'a> {
    issue_ref: &'a IssueRef<'a>,
}
impl<'a> NixpkgsStrategy<'a> {
    pub fn new(issue_ref: &'a IssueRef) -> NixpkgsStrategy<'a> {
        Self { issue_ref }
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
}

impl<'a> EvaluationStrategy for NixpkgsStrategy<'a> {
    fn pre_clone(&mut self) -> StepResult<()> {
        self.tag_from_title();
        Ok(())
    }

    fn on_target_branch(&mut self, _co: &Path, _status: &mut CommitStatus) -> StepResult<()> {
        Ok(())
    }

    fn after_fetch(&mut self, _co: &CachedProjectCo) -> StepResult<()> {
        Ok(())
    }

    fn merge_conflict(&mut self) {}

    fn after_merge(&mut self, _status: &mut CommitStatus) -> StepResult<()> {
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
        Ok(vec![])
    }
}
