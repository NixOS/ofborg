use crate::checkout::CachedProjectCo;
use crate::commitstatus::CommitStatus;
use crate::evalchecker::EvalChecker;
use crate::tasks::eval::{EvaluationComplete, EvaluationStrategy, StepResult};
use std::path::Path;

#[derive(Default)]
pub struct GenericStrategy {}
impl GenericStrategy {
    pub fn new() -> GenericStrategy {
        Self {}
    }
}

impl EvaluationStrategy for GenericStrategy {
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
    ) -> StepResult<EvaluationComplete> {
        Ok(Default::default())
    }
}
