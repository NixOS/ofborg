use crate::message;

use hubcaps::checks::{CheckRun, CheckRunOptions};
use hubcaps::issues::Issue;
use hubcaps::labels::Label;
use hubcaps::pulls::Pull;
use hubcaps::repositories::Repository;
use hubcaps::review_requests::ReviewRequestOptions;
use hubcaps::statuses::{Status, StatusOptions};
use hubcaps::Github;

pub trait Client {
    fn get_issue(&self, number: u64) -> hubcaps::Result<Issue>;
    fn get_pull(&self, number: u64) -> hubcaps::Result<Pull>;
    fn add_labels(&self, number: u64, labels: Vec<&str>) -> hubcaps::Result<Vec<Label>>;
    fn remove_label(&self, number: u64, label: &str) -> hubcaps::Result<()>;
    fn create_review_request(
        &self,
        number: u64,
        review_request: &ReviewRequestOptions,
    ) -> hubcaps::Result<Pull>;
    fn create_status(&self, sha: &str, status: &StatusOptions) -> hubcaps::Result<Status>;
    fn create_checkrun(&self, check: &CheckRunOptions) -> hubcaps::Result<CheckRun>;
}

pub struct Hubcaps<'a> {
    repo: Repository<'a>,
}

impl<'a> Hubcaps<'a> {
    pub fn new(github: &'a Github, repo: &message::Repo) -> Self {
        let repo = github.repo(repo.owner.clone(), repo.name.clone());
        Hubcaps { repo }
    }
}

impl Client for Hubcaps<'_> {
    fn get_issue(&self, number: u64) -> hubcaps::Result<Issue> {
        self.repo.issue(number).get()
    }

    fn get_pull(&self, number: u64) -> hubcaps::Result<Pull> {
        let pulls = self.repo.pulls();
        pulls.get(number).get()
    }

    fn add_labels(&self, number: u64, labels: Vec<&str>) -> hubcaps::Result<Vec<Label>> {
        self.repo.issue(number).labels().add(labels)
    }

    fn remove_label(&self, number: u64, label: &str) -> hubcaps::Result<()> {
        self.repo.issue(number).labels().remove(label)
    }

    fn create_review_request(
        &self,
        number: u64,
        review_request: &ReviewRequestOptions,
    ) -> hubcaps::Result<Pull> {
        let pulls = self.repo.pulls();
        pulls.get(number).review_requests().create(review_request)
    }

    fn create_status(&self, sha: &str, status: &StatusOptions) -> hubcaps::Result<Status> {
        self.repo.statuses().create(sha, status)
    }

    fn create_checkrun(&self, check: &CheckRunOptions) -> hubcaps::Result<CheckRun> {
        self.repo.checkruns().create(&check)
    }
}
