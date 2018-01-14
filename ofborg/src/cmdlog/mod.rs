
pub mod nulllogger;
pub use cmdlog::nulllogger::NullLogger;

pub mod rabbitmqlogger;
pub use cmdlog::rabbitmqlogger::RabbitMQLogger;

pub trait Logger {

}
