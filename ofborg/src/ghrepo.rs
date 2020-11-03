use crate::commitstatus;
use crate::message;

use hubcaps::checks::{CheckRun, CheckRunOptions};
use hubcaps::issues::IssueRef;
use hubcaps::repositories::Repository;
use hubcaps::statuses::{Status, StatusOptions};
use hubcaps::Github;

pub struct Client<'a> {
    repo: Repository<'a>,
}

impl<'a> Client<'a> {
    pub fn new(github: &'a Github, repo: &message::Repo) -> Self {
        let repo = github.repo(repo.owner.clone(), repo.name.clone());
        Client { repo }
    }

    pub fn get_repo(&self) -> &Repository<'a> {
        &self.repo
    }

    pub fn get_issue_ref(&self, number: u64) -> IssueRef {
        self.repo.issue(number)
    }

    pub fn create_status(&self, sha: &str, status: &StatusOptions) -> hubcaps::Result<Status> {
        self.repo.statuses().create(sha, status)
    }

    pub fn create_checkrun(&self, check: &CheckRunOptions) -> hubcaps::Result<CheckRun> {
        self.repo.checkruns().create(&check)
    }

    pub fn create_commitstatus(
        &self,
        pr: &message::Pr,
        context: String,
        description: String,
    ) -> commitstatus::CommitStatus {
        commitstatus::CommitStatus::new(
            self.repo.statuses(),
            pr.head_sha.clone(),
            context,
            description,
            None,
        )
    }
}
