use amqp::Channel;
use amqp::protocol::basic::BasicProperties;
use amqp::Basic;

pub trait SysEvents {
    fn tick(&mut self, name: &str);
}

pub struct RabbitMQ {
    channel: Channel,
}

impl RabbitMQ {
    pub fn new(channel: Channel) -> RabbitMQ {
        RabbitMQ { channel: channel }
    }
}

impl SysEvents for RabbitMQ {
    fn tick(&mut self, name: &str) {
        let props = BasicProperties { ..Default::default() };
        self.channel
            .basic_publish(
                String::from("stats"),
                "".to_owned(),
                false,
                false,
                props,
                String::from(name).into_bytes(),
            )
            .unwrap();
    }
}
