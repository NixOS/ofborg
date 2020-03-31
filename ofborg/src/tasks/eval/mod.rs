pub mod stdenvs;
pub use self::stdenvs::Stdenvs;
mod nixpkgs;
pub use self::nixpkgs::NixpkgsStrategy;
mod generic;
pub use self::generic::GenericStrategy;
use hubcaps::checks::CheckRunOptions;
use ofborg::checkout::CachedProjectCo;
use ofborg::commitstatus::{CommitStatus, CommitStatusError};
use ofborg::evalchecker::EvalChecker;
use ofborg::message::buildjob::BuildJob;
use std::path::Path;

pub trait EvaluationStrategy {
    fn pre_clone(&mut self) -> StepResult<()>;

    fn on_target_branch(&mut self, co: &Path, status: &mut CommitStatus) -> StepResult<()>;
    fn after_fetch(&mut self, co: &CachedProjectCo) -> StepResult<()>;
    fn merge_conflict(&mut self);
    fn after_merge(&mut self, status: &mut CommitStatus) -> StepResult<()>;
    fn evaluation_checks(&self) -> Vec<EvalChecker>;
    fn all_evaluations_passed(
        &mut self,
        co: &Path,
        status: &mut CommitStatus,
    ) -> StepResult<EvaluationComplete>;
}

pub type StepResult<T> = Result<T, Error>;

#[derive(Default)]
pub struct EvaluationComplete {
    pub builds: Vec<BuildJob>,
    pub checks: Vec<CheckRunOptions>,
}

#[derive(Debug)]
pub enum Error {
    CommitStatusWrite(CommitStatusError),
    Fail(String),
    FailWithGist(String, String, String),
}

impl From<CommitStatusError> for Error {
    fn from(e: CommitStatusError) -> Error {
        Error::CommitStatusWrite(e)
    }
}
