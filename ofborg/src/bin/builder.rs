extern crate ofborg;
extern crate amqp;
extern crate env_logger;

#[macro_use]
extern crate log;

use std::env;

use std::path::Path;
use amqp::Basic;
use ofborg::config;
use ofborg::checkout;
use ofborg::notifyworker;
use ofborg::tasks;
use ofborg::easyamqp;
use ofborg::easyamqp::TypedWrappers;


fn main() {
    let cfg = config::load(env::args().nth(1).unwrap().as_ref());

    ofborg::setup_log();

    let cloner = checkout::cached_cloner(Path::new(&cfg.checkout.root));
    let nix = cfg.nix();

    let full_logs: bool = match &cfg.feedback {
        &Some(ref feedback) => {
            feedback.full_logs
        }
        &None => {
            warn!("Please define feedback.full_logs in your configuration to true or false!");
            warn!("feedback.full_logs when true will cause the full build log to be sent back");
            warn!("to the server, and be viewable by everyone.");
            warn!("I strongly encourage everybody turn this on!");
            false
        }
    };


    let mut session = easyamqp::session_from_config(&cfg.rabbitmq).unwrap();
    let mut channel = session.open_channel(1).unwrap();
    channel.basic_prefetch(1).unwrap();

    channel
        .consume(
            notifyworker::new(tasks::build::BuildWorker::new(
                cloner,
                nix,
                cfg.nix.system.clone(),
                cfg.runner.identity.clone(),
                full_logs,
            )),
            easyamqp::ConsumeConfig {
                queue: format!("build-inputs-{}", cfg.nix.system.clone()),
                consumer_tag: format!("{}-builder", cfg.whoami()),
                no_local: false,
                no_ack: false,
                no_wait: false,
                exclusive: false,
                arguments: None
            },
        )
        .unwrap();

    channel.start_consuming();
    channel.close(200, "Bye").unwrap();
    println!("Closed the channel");
    session.close(200, "Good Bye");
    println!("Closed the session... EOF");
}
