use ofborg::checkout::CachedProjectCo;
use ofborg::commitstatus::CommitStatus;
use ofborg::evalchecker::EvalChecker;
use ofborg::message::buildjob::BuildJob;
use std::path::Path;
use tasks::eval::{EvaluationStrategy, StepResult};

pub struct NixpkgsStrategy {}
impl NixpkgsStrategy {
    pub fn new() -> NixpkgsStrategy {
        Self {}
    }
}

impl EvaluationStrategy for NixpkgsStrategy {
    fn pre_clone(&mut self) -> StepResult<()> {
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
