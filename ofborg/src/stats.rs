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

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct EventMessage {
    pub sender: String,
    pub events: Vec<Event>,
}

pub struct RabbitMq<C> {
    identity: String,
    channel: C,
}

impl RabbitMq<lapin::Channel> {
    pub fn from_lapin(identity: &str, channel: lapin::Channel) -> Self {
        RabbitMq {
            identity: identity.to_owned(),
            channel,
        }
    }
}

impl SysEvents for RabbitMq<lapin::Channel> {
    fn notify(&mut self, event: Event) {
        let props = lapin::BasicProperties::default().with_content_type("application/json".into());
        task::block_on(async {
            let _confirmaton = self
                .channel
                .basic_publish(
                    &String::from("stats"),
                    "",
                    BasicPublishOptions::default(),
                    &serde_json::to_string(&EventMessage {
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
