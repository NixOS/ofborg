mod nixpkgs;

pub use self::nixpkgs::NixpkgsStrategy;
use crate::checkout::CachedProjectCo;
use crate::commitstatus::{CommitStatus, CommitStatusError};
use crate::evalchecker::EvalChecker;
use crate::message::buildjob::BuildJob;

use std::path::Path;

pub trait EvaluationStrategy {
    fn pre_clone(&mut self) -> impl std::future::Future<Output = StepResult<()>>;

    fn on_target_branch(
        &mut self,
        co: &Path,
        status: &mut CommitStatus,
    ) -> impl std::future::Future<Output = StepResult<()>>;
    fn after_fetch(&mut self, co: &CachedProjectCo) -> StepResult<()>;
    fn after_merge(
        &mut self,
        status: &mut CommitStatus,
    ) -> impl std::future::Future<Output = StepResult<()>>;
    fn evaluation_checks(&self) -> Vec<EvalChecker>;
    fn all_evaluations_passed(
        &mut self,
        status: &mut CommitStatus,
    ) -> impl std::future::Future<Output = StepResult<EvaluationComplete>>;
}

pub type StepResult<T> = Result<T, Error>;

#[derive(Default)]
pub struct EvaluationComplete {
    pub builds: Vec<BuildJob>,
}

#[derive(Debug)]
pub enum Error {
    CommitStatusWrite(CommitStatusError),
    Fail(String),
}

impl From<CommitStatusError> for Error {
    fn from(e: CommitStatusError) -> Error {
        Error::CommitStatusWrite(e)
    }
}
