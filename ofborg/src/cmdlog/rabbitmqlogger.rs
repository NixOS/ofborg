use cmdlog::Logger;
use amqp::Channel;
use amqp::protocol::basic::BasicProperties;
use amqp::{Basic,Table};

pub struct RabbitMQLogger {
    channel: Channel,
    exchange: Option<String>,
    routing_key: Option<String>,
}

impl RabbitMQLogger {
    pub fn new(channel: Channel) -> RabbitMQLogger {
        RabbitMQLogger{
            channel: channel,
            exchange: Some("logs".to_owned()),
            routing_key: None,
        }
    }

    pub fn setup(&mut self) {
        self.channel.exchange_declare(
            "logs".to_owned(), // exchange: S,
            "topic".to_owned(), // _type: S,
            true, // passive: bool,
            true, // durable: bool,
            false, // auto_delete: bool,
            false, // internal: bool,
            false, // nowait: bool,
            Table::new() // arguments: Table
        ).expect("logs exch setup wrong");
    }
}

impl Logger for RabbitMQLogger {
    fn build_output(&mut self, line: &str) {
        self.channel.basic_publish(
            self.exchange.clone().unwrap_or("".to_owned()),
            self.routing_key.clone().unwrap_or("build.log".to_owned()),
            false,
            false,
            BasicProperties{..BasicProperties::default()},
            Vec::from(line)
        );
    }
}
