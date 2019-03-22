pub mod stdenvs;
pub use self::stdenvs::Stdenvs;
mod nixpkgs;
pub use self::nixpkgs::NixpkgsStrategy;
mod generic;
pub use self::generic::GenericStrategy;
use ofborg::checkout::CachedProjectCo;
use ofborg::commitstatus::CommitStatus;
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
    ) -> StepResult<Vec<BuildJob>>;
}

pub type StepResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Fail(String),
    FailWithGist(String, String, String),
}
