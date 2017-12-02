#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate irc;
extern crate amqp;
extern crate env_logger;

pub mod config;
pub mod ircbot {
    pub use config;
}
