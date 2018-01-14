use cmdlog::Logger;
use amqp::Channel;

pub struct RabbitMQLogger {
    channel: Channel
}

impl RabbitMQLogger {
    pub fn new(channel: Channel) -> RabbitMQLogger {
        RabbitMQLogger{
            channel: channel
        }
    }
}

impl Logger for RabbitMQLogger {
}
