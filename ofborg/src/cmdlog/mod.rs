
pub mod nulllogger;
pub use cmdlog::nulllogger::NullLogger;

pub mod rabbitmqlogger;
pub use cmdlog::rabbitmqlogger::RabbitMQLogger;

pub mod lastn;
pub use cmdlog::lastn::LastNLogger;

pub trait Logger {
    fn build_output(&mut self, line: &str);
}
