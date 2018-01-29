extern crate ofborg;
extern crate amqp;
extern crate env_logger;

#[macro_use]
extern crate log;

use std::env;

use std::path::Path;
use amqp::Basic;
use amqp::Session;
use amqp::Table;

use ofborg::config;
use ofborg::checkout;
use ofborg::notifyworker;
use ofborg::tasks;


fn main() {
    let cfg = config::load(env::args().nth(1).unwrap().as_ref());

    ofborg::setup_log();

    println!("Hello, world!");


    let mut session = Session::open_url(&cfg.rabbitmq.as_uri()).unwrap();
    println!("Connected to rabbitmq");

    let mut channel = session.open_channel(1).unwrap();

    let cloner = checkout::cached_cloner(Path::new(&cfg.checkout.root));
    let nix = cfg.nix();



    let full_logs: bool;
    match &cfg.feedback {
        &Some(ref feedback) => {
            full_logs = feedback.full_logs;
        }
        &None => {
            warn!("Please define feedback.full_logs in your configuration to true or false!");
            warn!("feedback.full_logs when true will cause the full build log to be sent back");
            warn!("to the server, and be viewable by everyone.");
            warn!("I strongly encourage everybody turn this on!");
            full_logs = false;
        }
    }

    channel.basic_prefetch(1).unwrap();
    channel
        .basic_consume(
            notifyworker::new(tasks::build::BuildWorker::new(
                cloner,
                nix,
                cfg.nix.system.clone(),
                cfg.runner.identity.clone(),
                full_logs,
            )),
            format!("build-inputs-{}", cfg.nix.system.clone()).as_ref(),
            format!("{}-builder", cfg.whoami()).as_ref(),
            false,
            false,
            false,
            false,
            Table::new(),
        )
        .unwrap();

    channel.start_consuming();

    println!("Finished consuming?");

    channel.close(200, "Bye").unwrap();
    println!("Closed the channel");
    session.close(200, "Good Bye");
    println!("Closed the session... EOF");
}
