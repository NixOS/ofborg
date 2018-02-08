use serde_json;
use amqp::Channel;
use amqp::protocol::basic::BasicProperties;
use amqp::Basic;

pub trait SysEvents: Send {
    fn notify(&mut self, event: Event);
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="kebab-case")]
pub enum Event {
    StatCollectorLegacyEvent,
    StatCollectorBogusEvent,
    JobReceived,
    JobDecodeSuccess,
    JobDecodeFailure,
    IssueAlreadyClosed,
    IssueFetchFailed,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EventMessage {
    pub sender: String,
    pub events: Vec<Event>,
}

pub struct RabbitMQ {
    identity: String,
    channel: Channel,
}

impl RabbitMQ {
    pub fn new(identity: &str, channel: Channel) -> RabbitMQ {
        RabbitMQ { identity: identity.to_owned(), channel: channel }
    }
}

impl SysEvents for RabbitMQ {
    fn notify(&mut self, event: Event) {
        let props = BasicProperties { ..Default::default() };
        self.channel
            .basic_publish(
                String::from("stats"),
                "".to_owned(),
                false,
                false,
                props,
                serde_json::to_string(&EventMessage {
                    sender: self.identity.clone(),
                    events: vec![event],
                }).unwrap().into_bytes(),
            )
            .unwrap();
    }
}
