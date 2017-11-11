use ofborg::message::{Pr,Repo};
use ofborg::message::buildresult;
use ofborg::worker;
use serde_json;
use amqp::protocol;

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
}

impl Actions {
    pub fn commit_missing(&mut self, _job: &BuildJob) -> worker::Actions {
        return vec![
            worker::Action::Ack
        ];
    }

    pub fn merge_failed(&mut self, job: &BuildJob) -> worker::Actions {
        let msg = buildresult::BuildResult {
            repo: job.repo.clone(),
            pr: job.pr.clone(),
            system: self.system.clone(),
            output: vec![String::from("Merge failed")],
            success: false
        };

        let props =  protocol::basic::BasicProperties {
            content_type: Some("application/json".to_owned()),
            ..Default::default()
        };


        return vec![
            worker::Action::Publish(worker::QueueMsg{
                exchange: Some("build-results".to_owned()),
                routing_key: None,
                mandatory: true,
                immediate: false,
                properties: Some(props),
                content: serde_json::to_string(&msg).unwrap().into_bytes()
            }),
            worker::Action::Ack
        ];
    }

    pub fn build_finished(&mut self, job: &BuildJob, success: bool, lines: Vec<String>) -> worker::Actions {
        let msg = buildresult::BuildResult {
            repo: job.repo.clone(),
            pr: job.pr.clone(),
            system: self.system.clone(),
            output: lines,
            success: success
        };

        let props =  protocol::basic::BasicProperties {
            content_type: Some("application/json".to_owned()),
            ..Default::default()
        };


        return vec![
            worker::Action::Publish(worker::QueueMsg{
                exchange: Some("build-results".to_owned()),
                routing_key: None,
                mandatory: true,
                immediate: false,
                properties: Some(props),
                content: serde_json::to_string(&msg).unwrap().into_bytes()
            }),
            worker::Action::Ack
        ];
    }
}
