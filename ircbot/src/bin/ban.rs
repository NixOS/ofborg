#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate ircbot;
extern crate irc;
extern crate amqp;
extern crate env_logger;

#[macro_use]
extern crate log;
use irc::client::server::Server;
use irc::client::prelude::Command;
use irc::client::prelude::ServerExt;
use irc::client::prelude::IrcServer;
use irc::proto::mode::Mode;
use irc::proto::mode::ChannelMode;

use ircbot::config;

use std::thread;
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    target: String,
    body: String
}

fn main() {
    if let Err(_) = env::var("RUST_LOG") {
        env::set_var("RUST_LOG", "info");
        env_logger::init().unwrap();
        info!("Defaulting RUST_LOG environment variable to info");
    } else {
        env_logger::init().unwrap();
    }

    let cfg = config::load(env::args().nth(1).unwrap().as_ref());


    let server = IrcServer::from_config(cfg.irc_config()).unwrap();
    server.identify().unwrap();
    let reader = server.clone();

    reader.for_each_incoming(|message| {
        match message.command {
            Command::PRIVMSG(ref _target, ref msg) => {
                let badwords: Vec<&str> = vec![
                    "▄",
                    "companieshouse.gov.uk",
                    "christel sold",
                    "freenode to private",
                    "snoonet",
                    "nigger",
                    "chan freenode",
                    "supernets.org",
                    "flooding challenge",
                    "details!!",
                    "supernets.org",
                ];

                if let Some(from) = message.source_nickname() {
                    if let Some(inchan) = message.response_target() {
                        for word in badwords {
                            if msg.to_lowercase().contains(word) {
                                server.send_mode(
                                    inchan,
                                    &[
                                        Mode::Plus(
                                            ChannelMode::Ban,
                                            Some(from.to_owned())
                                        )
                                    ]
                                );
                                server.send_kick(
                                    inchan,
                                    from,
                                    "PM gchristensen to dispute this"
                                );
                                println!("Banning: {:?}, {:?}", from, msg);
                            }
                        }
                    }
                }
            }
            _ => {
                print!("{:?}\n", message.command);
            },
        }
    }).unwrap();
}
