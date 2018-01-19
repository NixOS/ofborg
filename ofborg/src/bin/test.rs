extern crate ofborg;
extern crate amqp;
extern crate env_logger;

#[macro_use]
extern crate log;

use std::env;
use amqp::Basic;
use amqp::Session;
use amqp::Table;

use ofborg::config;
use ofborg::worker;
use ofborg::notifyworker;
use ofborg::tasks;

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

pub struct TestWorkerActions<'a> {
    receiver: &'a mut notifyworker::NotificationReceiver
}

impl<'a> TestWorkerActions<'a> {
    pub fn new(receiver: &'a mut notifyworker::NotificationReceiver) -> TestWorkerActions {
        TestWorkerActions {
            receiver: receiver,
        }
    }

    pub fn tick(&mut self, event: &str) {
        self.receiver.tell(worker::Action::Publish(worker::QueueMsg{
            exchange: Some(String::from("stats")),
            routing_key: None,
            content: String::from(event).into_bytes(),
            immediate: false,
            mandatory: false,
            properties: None,
        }));
    }

    pub fn say_hi(&mut self) {
        self.receiver.tell(worker::Action::Publish(worker::QueueMsg{
            exchange: None,
            routing_key: Some(String::from("test-notify-worker")),
            content: String::from("hi").into_bytes(),
            immediate: false,
            mandatory: false,
            properties: None,
        }));
    }

    pub fn ack(&mut self) {
        self.receiver.tell(worker::Action::Ack);
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
        let mut actions = TestWorkerActions::new(notifier);

        info!("Working on {}", job);

        actions.tick("started-work");
        for i in 1..100 {
            actions.say_hi();
        }
        actions.tick("finished-work-success");
        actions.tick("finished-work-failed");

        actions.ack();
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
