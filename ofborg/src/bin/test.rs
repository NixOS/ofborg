extern crate ofborg;
extern crate amqp;
extern crate env_logger;

#[macro_use]
extern crate log;

use std::env;
use std::{thread, time};
use std::path::Path;
use amqp::Basic;
use amqp::Session;
use amqp::Table;

use ofborg::config;
use ofborg::checkout;
use ofborg::worker;
use ofborg::notifyworker;
use ofborg::tasks;


use std::collections::LinkedList;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use ofborg::message::buildjob;
use ofborg::nix;
use ofborg::commentparser;

use amqp::protocol::basic::{Deliver,BasicProperties};


pub struct TestWorker {
    system: String,
    identity: String,
}

impl TestWorker {
    pub fn new(system: String, identity: String) -> TestWorker {
        return TestWorker{
            system: system,
            identity: identity,
        };
    }
}

impl notifyworker::SimpleNotifyWorker for TestWorker {
    type J = String;

    fn msg_to_job(&self, _: &Deliver, _: &BasicProperties,
                  body: &Vec<u8>) -> Result<Self::J, String> {
        println!("lmao I got a job?");
        return Ok(String::from_utf8(body.to_vec()).unwrap())
    }

    fn consumer(&self, job: &String, notifier: &mut notifyworker::NotificationReceiver) {
        info!("Working on {}", job);

        for i in 1..100 {
            notifier.tell(worker::Action::Publish(worker::QueueMsg{
                exchange: None,
                routing_key: Some(String::from("test-notify-worker")),
                content: String::from("hi").into_bytes(),
                immediate: false,
                mandatory: false,
                properties: None,
            }));
        }

        notifier.tell(worker::Action::Ack);
    }
}


fn main() {
    let cfg = config::load(env::args().nth(1).unwrap().as_ref());

    if let Err(_) = env::var("RUST_LOG") {
        env::set_var("RUST_LOG", "info");
        env_logger::init().unwrap();
        info!("Defaulting RUST_LOG environment variable to info");
    } else {
        env_logger::init().unwrap();
    }

    println!("Hello, world!");

    let mut session = Session::open_url(&cfg.rabbitmq.as_uri()).unwrap();
    println!("Connected to rabbitmq");
    {
        println!("About to open channel #1");
        let hbchan = session.open_channel(1).unwrap();

        println!("Opened channel #1");

        tasks::heartbeat::start_on_channel(hbchan, cfg.whoami());
    }

    let mut channel = session.open_channel(2).unwrap();

    channel.queue_declare(
        "test-notify-worker",
        false, // passive
        false, // durable
        true, // exclusive
        true, // auto_delete
        false, //nowait
        Table::new()
    )
        .expect("Failed to declare queue!");


    channel.basic_prefetch(1).unwrap();
    channel.basic_consume(
        notifyworker::new(TestWorker::new(
            cfg.nix.system.clone(),
            cfg.runner.identity.clone()
        )),
        "test-notify-worker".as_ref(),
        format!("{}-test", cfg.whoami()).as_ref(),
        false,
        false,
        false,
        false,
        Table::new()
    ).unwrap();

    channel.start_consuming();

    println!("Finished consuming?");

    channel.close(200, "Bye").unwrap();
    println!("Closed the channel");
    session.close(200, "Good Bye");
    println!("Closed the session... EOF");
}
