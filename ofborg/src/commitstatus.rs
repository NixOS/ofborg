pub struct CommitStatus<'a> {
    api: hubcaps::statuses::Statuses<'a>,
    sha: String,
    context: String,
    description: String,
    url: String,
}

impl<'a> CommitStatus<'a> {
    pub fn new(
        api: hubcaps::statuses::Statuses<'a>,
        sha: String,
        context: String,
        description: String,
        url: Option<String>,
    ) -> CommitStatus<'a> {
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
            eprintln!(
                "Warning: description is over 140 char; truncating: {:?}",
                &self.description
            );
            self.description.chars().take(140).collect()
        } else {
            self.description.clone()
        };

        self.api
            .create(
                self.sha.as_ref(),
                &hubcaps::statuses::StatusOptions::builder(state)
                    .context(self.context.clone())
                    .description(desc)
                    .target_url(self.url.clone())
                    .build(),
            )
            .map(|_| ())
            .map_err(|e| CommitStatusError::HubcapsError(e))
    }
}

#[derive(Debug)]
pub enum CommitStatusError {
    HubcapsError(hubcaps::Error),
}
