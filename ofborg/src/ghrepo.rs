use crate::commitstatus;
use crate::message;

use hubcaps::checks::{CheckRun, CheckRunOptions};
use hubcaps::issues::IssueRef;
use hubcaps::pulls::Pull;
use hubcaps::repositories::Repository;
use hubcaps::review_requests::ReviewRequestOptions;
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

    pub fn get_pull(&self, number: u64) -> hubcaps::Result<Pull> {
        let pulls = self.repo.pulls();
        pulls.get(number).get()
    }

    pub fn create_review_request(
        &self,
        number: u64,
        review_request: &ReviewRequestOptions,
    ) -> hubcaps::Result<Pull> {
        let pulls = self.repo.pulls();
        pulls.get(number).review_requests().create(review_request)
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
