use futures_util::future::TryFutureExt;
use tracing::warn;

pub struct CommitStatus {
    api: hubcaps::statuses::Statuses,
    sha: String,
    context: String,
    description: String,
    url: String,
}

impl CommitStatus {
    pub fn new(
        api: hubcaps::statuses::Statuses,
        sha: String,
        context: String,
        description: String,
        url: Option<String>,
    ) -> CommitStatus {
        let mut stat = CommitStatus {
            api,
            sha,
            context,
            description,
            url: "".to_owned(),
        };

        stat.set_url(url);

        stat
    }

    pub fn set_url(&mut self, url: Option<String>) {
        self.url = url.unwrap_or_else(|| String::from(""))
    }

    pub fn set_with_description(
        &mut self,
        description: &str,
        state: hubcaps::statuses::State,
    ) -> Result<(), CommitStatusError> {
        self.set_description(description.to_owned());
        self.set(state)
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn set(&self, state: hubcaps::statuses::State) -> Result<(), CommitStatusError> {
        let desc = if self.description.len() >= 140 {
            warn!(
                "description is over 140 char; truncating: {:?}",
                &self.description
            );
            self.description.chars().take(140).collect()
        } else {
            self.description.clone()
        };
        async_std::task::block_on(
            self.api
                .create(
                    self.sha.as_ref(),
                    &hubcaps::statuses::StatusOptions::builder(state)
                        .context(self.context.clone())
                        .description(desc)
                        .target_url(self.url.clone())
                        .build(),
                )
                .map_ok(|_| ())
                .map_err(|e| CommitStatusError::from(e)),
        )
    }
}

#[derive(Debug)]
pub enum CommitStatusError {
    ExpiredCreds(hubcaps::Error),
    MissingSha(hubcaps::Error),
    Error(hubcaps::Error),
}

impl From<hubcaps::Error> for CommitStatusError {
    fn from(e: hubcaps::Error) -> CommitStatusError {
        use http::status::StatusCode;
        use hubcaps::Error;
        match &e {
            Error::Fault { code, error }
                if code == &StatusCode::UNAUTHORIZED && error.message == "Bad credentials" =>
            {
                CommitStatusError::ExpiredCreds(e)
            }
            Error::Fault { code, error }
                if code == &StatusCode::UNPROCESSABLE_ENTITY
                    && error.message.starts_with("No commit found for SHA:") =>
            {
                CommitStatusError::MissingSha(e)
            }
            _otherwise => CommitStatusError::Error(e),
        }
    }
}
