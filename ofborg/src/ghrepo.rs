use crate::commitstatus;
use crate::message;

use hubcaps::checks::{CheckRun, CheckRunOptions};
use hubcaps::issues::Issue;
use hubcaps::labels::Label;
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

    pub fn get_issue(&self, number: u64) -> hubcaps::Result<Issue> {
        self.repo.issue(number).get()
    }

    pub fn get_pull(&self, number: u64) -> hubcaps::Result<Pull> {
        let pulls = self.repo.pulls();
        pulls.get(number).get()
    }

    pub fn add_labels(&self, number: u64, labels: Vec<&str>) -> hubcaps::Result<Vec<Label>> {
        self.repo.issue(number).labels().add(labels)
    }

    pub fn remove_label(&self, number: u64, label: &str) -> hubcaps::Result<()> {
        self.repo.issue(number).labels().remove(label)
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
        gist_url: Option<String>,
    ) -> commitstatus::CommitStatus {
        commitstatus::CommitStatus::new(
            self.repo.statuses(),
            pr.head_sha.clone(),
            context,
            description,
            gist_url,
        )
    }
}
