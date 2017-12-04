#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate irc;
extern crate amqp;
extern crate env_logger;
extern crate toml;


pub mod factoids;
pub mod config;
pub mod ircbot {
    pub use config;
    pub use factoids;
}
