extern crate amqp;
extern crate env_logger;

use hubcaps;

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
            api: api,
            sha: sha,
            context: context,
            description: description,
            url: "".to_owned(),
        };

        stat.set_url(url);

        return stat;
    }

    pub fn set_url(&mut self, url: Option<String>) {
        self.url = url.unwrap_or(String::from(""))
    }

    pub fn set_with_description(&mut self, description: &str, state: hubcaps::statuses::State) {
        self.set_description(description.to_owned());
        self.set(state);
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn set(&self, state: hubcaps::statuses::State) {
        self.api
            .create(
                self.sha.as_ref(),
                &hubcaps::statuses::StatusOptions::builder(state)
                    .context(self.context.clone())
                    .description(self.description.clone())
                    .target_url(self.url.clone())
                    .build(),
            )
            .expect("Failed to mark final status on commit");
    }
}
