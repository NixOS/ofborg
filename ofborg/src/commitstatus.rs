use octocrab::{self, models::StatusState};
use tracing::warn;

use crate::github::GithubRepo;

pub struct CommitStatus {
    repo: GithubRepo,
    sha: String,
    context: String,
    description: String,
    url: String,
    enable_publish: bool,
}

impl CommitStatus {
    pub fn new(
        repo: GithubRepo,
        sha: String,
        context: String,
        description: String,
        url: Option<String>,
    ) -> CommitStatus {
        CommitStatus {
            repo,
            sha,
            context,
            description,
            url: url.unwrap_or_else(|| String::from("")),
            enable_publish: true,
        }
    }

    pub fn set_enable_publish(&mut self, enable_publish: bool) {
        self.enable_publish = enable_publish;
    }

    pub fn set_url(&mut self, url: Option<String>) {
        self.url = url.unwrap_or_else(|| String::from(""))
    }

    pub async fn set_with_description(
        &mut self,
        description: &str,
        state: StatusState,
    ) -> Result<(), CommitStatusError> {
        self.set_description(description.to_owned());
        self.set(state).await
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub async fn set(&self, state: StatusState) -> Result<(), CommitStatusError> {
        if !self.enable_publish {
            return Ok(());
        }

        let desc = if self.description.len() >= 140 {
            warn!(
                "description is over 140 char; truncating: {:?}",
                &self.description
            );
            self.description.chars().take(140).collect()
        } else {
            self.description.clone()
        };

        self.repo
            .repos()
            .create_status(self.sha.clone(), state)
            .context(self.context.clone())
            .description(desc)
            .target(self.url.clone())
            .send()
            .await?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum CommitStatusError {
    OctocrabError(octocrab::Error),
    InternalError(String),
}

impl From<octocrab::Error> for CommitStatusError {
    fn from(e: octocrab::Error) -> CommitStatusError {
        CommitStatusError::OctocrabError(e)
    }
}
