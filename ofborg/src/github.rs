use octocrab::Octocrab;
use tracing::info;

use crate::commitstatus::CommitStatusError;

#[derive(Clone)]
pub struct GithubRepo {
    octocrab: Octocrab,
    owner: String,
    repo: String,
}

impl GithubRepo {
    pub fn new(octocrab: Octocrab, owner: impl Into<String>, repo: impl Into<String>) -> Self {
        Self {
            octocrab,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn repo(&self) -> &str {
        &self.repo
    }

    pub fn repos(&self) -> octocrab::repos::RepoHandler<'_> {
        self.octocrab.repos(&self.owner, &self.repo)
    }

    pub fn issues(&self) -> octocrab::issues::IssueHandler<'_> {
        self.octocrab.issues(&self.owner, &self.repo)
    }

    pub fn checks(&self) -> octocrab::checks::ChecksHandler<'_> {
        self.octocrab.checks(&self.owner, &self.repo)
    }

    pub fn pulls(&self) -> octocrab::pulls::PullRequestHandler<'_> {
        self.octocrab.pulls(&self.owner, &self.repo)
    }

    pub async fn update_labels(
        &self,
        issue_number: u64,
        add: &[String],
        remove: &[String],
    ) -> Result<(), CommitStatusError> {
        let issue = self.issues().get(issue_number).await?;

        let existing: Vec<String> = issue.labels.iter().map(|l| l.name.clone()).collect();

        let to_add: Vec<String> = add
            .iter()
            .filter(|l| !existing.contains(l))
            .cloned()
            .collect();

        let to_remove: Vec<String> = remove
            .iter()
            .filter(|l| existing.contains(l))
            .cloned()
            .collect();

        info!("Labeling issue #{issue_number}: + {to_add:?} , - {to_remove:?}, = {existing:?}");

        if !to_add.is_empty() {
            self.issues().add_labels(issue_number, &to_add).await?;
        }

        for label in to_remove {
            self.issues().remove_label(issue_number, &label).await?;
        }

        Ok(())
    }

    pub async fn get_prefix(&self, sha: &str) -> Result<&'static str, CommitStatusError> {
        let mut page: octocrab::Page<octocrab::models::Status> =
            self.repos().list_statuses(sha.to_string()).send().await?;

        loop {
            if page.items.iter().any(|s| {
                s.context
                    .as_ref()
                    .is_some_and(|c| c.starts_with("grahamcofborg-"))
            }) {
                return Ok("grahamcofborg");
            }

            match self.octocrab.get_page(&page.next).await? {
                Some(next_page) => page = next_page,
                None => return Ok("ofborg"),
            }
        }
    }
}
