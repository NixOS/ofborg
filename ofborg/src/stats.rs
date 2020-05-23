use amqp::protocol::basic;
use amqp::Basic;
use async_std::task;
use lapin::options::BasicPublishOptions;

include!(concat!(env!("OUT_DIR"), "/events.rs"));

#[macro_use]
mod macros {
    #[macro_export]
    macro_rules! my_macro(() => (FooBar));
}

pub trait SysEvents: Send {
    fn notify(&mut self, event: Event);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EventMessage {
    pub sender: String,
    pub events: Vec<Event>,
}

pub struct RabbitMQ<C> {
    identity: String,
    channel: C,
}

impl RabbitMQ<amqp::Channel> {
    pub fn from_amqp(identity: &str, channel: amqp::Channel) -> Self {
        RabbitMQ {
            identity: identity.to_owned(),
            channel,
        }
    }
}

impl SysEvents for RabbitMQ<amqp::Channel> {
    fn notify(&mut self, event: Event) {
        let props = basic::BasicProperties {
            ..Default::default()
        };
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
                })
                .unwrap()
                .into_bytes(),
            )
            .unwrap();
    }
}

impl RabbitMQ<lapin::Channel> {
    pub fn from_lapin(identity: &str, channel: lapin::Channel) -> Self {
        RabbitMQ {
            identity: identity.to_owned(),
            channel,
        }
    }
}

impl SysEvents for RabbitMQ<lapin::Channel> {
    fn notify(&mut self, event: Event) {
        let props = lapin::BasicProperties::default().with_content_type("application/json".into());
        task::block_on(async {
            let _confirmaton = self
                .channel
                .basic_publish(
                    &String::from("stats"),
                    &"".to_owned(),
                    BasicPublishOptions::default(),
                    serde_json::to_string(&EventMessage {
                        sender: self.identity.clone(),
                        events: vec![event],
                    })
                    .unwrap()
                    .into_bytes(),
                    props,
                )
                .await
                .unwrap()
                .await
                .unwrap();
        });
    }
}
