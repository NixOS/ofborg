use crate::message;

use hubcaps::checks::{CheckRun, CheckRunOptions};
use hubcaps::issues::Issue;
use hubcaps::labels::Label;
use hubcaps::pulls::Pull;
use hubcaps::repositories::Repository;
use hubcaps::review_requests::ReviewRequestOptions;
use hubcaps::statuses::{Status, StatusOptions};
use hubcaps::Github;
use tracing::info;

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

    fn update_issue_labels(
        &self,
        number: u64,
        add: &[String],
        remove: &[String],
    ) -> hubcaps::Result<()> {
        let issue = self.get_issue(number)?;

        let existing: Vec<String> = issue.labels.iter().map(|l| l.name.clone()).collect();

        let to_add: Vec<&str> = add
            .iter()
            .filter(|l| !existing.contains(l)) // Remove labels already on the issue
            .map(|l| l.as_ref())
            .collect();

        let to_remove: Vec<String> = remove
            .iter()
            .filter(|l| existing.contains(l)) // Remove labels already on the issue
            .cloned()
            .collect();

        info!(
            "Labeling issue #{}: + {:?} , - {:?}, = {:?}",
            issue.number, to_add, to_remove, existing
        );

        self.add_labels(number, to_add.clone())?;
        for label in to_remove {
            self.remove_label(number, &label)?;
        }
        Ok(())
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use hubcaps::users::User;
    use std::cell::RefCell;

    struct RepoMock {
        issues: RefCell<Vec<hubcaps::Result<Issue>>>,
        add_labels: RefCell<Vec<Vec<Label>>>,
        remove_label: RefCell<Vec<Label>>,
    }

    impl Client for RepoMock {
        fn get_issue(&self, number: u64) -> hubcaps::Result<Issue> {
            let issue = self
                .issues
                .borrow_mut()
                .pop()
                .expect("RepoMock.get_issue called too many times")?;
            assert_eq!(number, issue.number);
            Ok(issue)
        }

        fn get_pull(&self, _number: u64) -> hubcaps::Result<Pull> {
            panic!("Not implemented");
        }

        fn add_labels(&self, _number: u64, add_labels: Vec<&str>) -> hubcaps::Result<Vec<Label>> {
            let labels = self
                .add_labels
                .borrow_mut()
                .pop()
                .expect("RepoMock.add_labels called too many times");
            assert_eq!(
                add_labels,
                labels.iter().map(|x| &x.name).collect::<Vec<_>>()
            );
            Ok(labels)
        }

        fn remove_label(&self, _number: u64, remove_label: &str) -> hubcaps::Result<()> {
            let label = self
                .remove_label
                .borrow_mut()
                .pop()
                .expect("RepoMock.add_labels called too many times");
            assert_eq!(remove_label, label.name,);
            Ok(())
        }

        fn create_review_request(
            &self,
            _number: u64,
            _review_request: &ReviewRequestOptions,
        ) -> hubcaps::Result<Pull> {
            panic!("Not implemented");
        }

        fn create_status(&self, _sha: &str, _status: &StatusOptions) -> hubcaps::Result<Status> {
            panic!("Not implemented");
        }

        fn create_checkrun(&self, _check: &CheckRunOptions) -> hubcaps::Result<CheckRun> {
            panic!("Not implemented");
        }
    }

    #[test]
    fn test_update_issue_labels() {
        let client = RepoMock {
            issues: RefCell::new(vec![Ok(Issue {
                id: 42,
                url: String::new(),
                labels_url: String::new(),
                comments_url: String::new(),
                events_url: String::new(),
                html_url: String::new(),
                number: 42,
                state: String::new(),
                title: String::new(),
                body: None,
                user: User {
                    login: String::from("johndoe"),
                    id: 42,
                    avatar_url: String::new(),
                    gravatar_id: String::new(),
                    url: String::new(),
                    html_url: String::new(),
                    followers_url: String::new(),
                    following_url: String::new(),
                    gists_url: String::new(),
                    starred_url: String::new(),
                    subscriptions_url: String::new(),
                    organizations_url: String::new(),
                    repos_url: String::new(),
                    events_url: String::new(),
                    received_events_url: String::new(),
                    site_admin: false,
                },
                labels: vec![
                    Label {
                        url: String::new(),
                        name: String::from("bar"),
                        color: String::new(),
                    },
                    Label {
                        url: String::new(),
                        name: String::from("baz"),
                        color: String::new(),
                    },
                    Label {
                        url: String::new(),
                        name: String::from("keep"),
                        color: String::new(),
                    },
                    Label {
                        url: String::new(),
                        name: String::from("ignore"),
                        color: String::new(),
                    },
                ],
                assignee: None,
                locked: false,
                comments: 0,
                closed_at: None,
                created_at: String::new(),
                updated_at: String::new(),
            })]),
            add_labels: RefCell::new(vec![vec![Label {
                url: String::new(),
                name: String::from("foo"),
                color: String::new(),
            }]]),
            remove_label: RefCell::new(vec![Label {
                url: String::new(),
                name: String::from("bar"),
                color: String::new(),
            }]),
        };

        let result = client.update_issue_labels(
            42,
            &[String::from("foo"), String::from("keep")],
            &[String::from("bar"), String::from("missing")],
        );
        assert!(result.is_ok());
    }
}
