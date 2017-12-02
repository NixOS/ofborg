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

use amqp::protocol::basic::Deliver;
use amqp::protocol::basic::BasicProperties;
use amqp::Basic;
use amqp::Channel;
use amqp::Session;
use amqp::Table;


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

    let mut session = Session::open_url(&cfg.rabbitmq.as_uri()).unwrap();
    println!("Connected to rabbitmq");
    println!("About to open channel #1");
    let mut writechan = session.open_channel(1).unwrap();

    let writeexch = writechan.exchange_declare(
        "exchange-messages",  // exchange name
        "fanout", //  exchange type
        false, //
        true, //
        false, //
        false,
        false,
        Table::new()
    );


    let mut readchan = session.open_channel(2).unwrap();
    //queue: &str, passive: bool, durable: bool, exclusive: bool, auto_delete: bool, nowait: bool, arguments: Table
    let readqueue = readchan.queue_declare("queue-publish", false, true, false, false, false, Table::new());

    let server = IrcServer::from_config(cfg.irc_config()).unwrap();
    server.identify().unwrap();
    let reader = server.clone();

    thread::spawn(move || {
        let consumer_name = readchan.basic_consume(
            move |_channel: &mut Channel, _deliver: Deliver, _headers: BasicProperties, body: Vec<u8>| {
                let msg: Result<Message, serde_json::Error> = serde_json::from_slice(&body);
                if let Ok(msg) = msg {
                    server.send_privmsg(&msg.target, &msg.body).unwrap();
                }
            },
            "queue-publish", "", false, true, false, false, Table::new());
        println!("Starting consumer {:?}", consumer_name);
        // server.stream().map(|m| print!("{}", m)).wait().count();

        readchan.start_consuming();
        readchan.close(200, "Bye").unwrap();
        panic!("Lost the consumer!");
    });


    reader.for_each_incoming(|message| {
        match message.command {
            Command::PRIVMSG(ref target, ref msg) => {
                let msg = serde_json::to_string(&Message{
                    target: target.clone(),
                    body: msg.clone(),
                }).unwrap();

                writechan.basic_publish(
                    "exchange-messages".to_owned(),
                    "".to_owned(),
                    false,
                    false,
                    BasicProperties {
                        content_type: Some("application/json".to_owned()),
                        ..Default::default()
                    },
                    msg.into_bytes()
                ).expect("Failed to publish message");
            }
            _ => {
                print!("{:?}\n", message.command);
            },
        }
    }).unwrap();
}
