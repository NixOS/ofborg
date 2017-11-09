use ofborg::message::{Pr,Repo};
use ofborg::message::buildresult;
use serde_json;
use amqp::{Basic, Channel, protocol};

#[derive(Serialize, Deserialize, Debug)]
pub struct BuildJob {
    pub repo: Repo,
    pub pr: Pr,
    pub attrs: Vec<String>
}

pub fn from(data: &Vec<u8>) -> Result<BuildJob, serde_json::error::Error> {
    return serde_json::from_slice(&data);
}

pub struct Actions {
    pub system: String,
    pub job: BuildJob,
    pub channel: Channel
}

impl Actions {
    pub fn build_finished(&mut self, success: bool, lines: Vec<String>) {
        let msg = buildresult::BuildResult {
            repo: self.job.repo.clone(),
            pr: self.job.pr.clone(),
            system: self.system.clone(),
            output: lines,
            success: success
        };

        let props =  protocol::basic::BasicProperties {
            content_type: Some("application/json".to_owned()),
            ..Default::default()
        };

        self.channel.basic_publish("build-results", "", true, true,
                             props, serde_json::to_string(&msg).unwrap().into_bytes()).unwrap();
    }
}
