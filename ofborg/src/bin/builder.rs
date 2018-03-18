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

    if &cfg.feedback.full_logs != Some(true) {
        warn!("Please define feedback.full_logs in your configuration to true!");
        warn!("feedback.full_logs when true will cause the full build log to be sent back");
        warn!("to the server, and be viewable by everyone.");
        warn!("");
        warn!("Builders are no longer allowed to operate with this off");
        warn!("so your builder will no longer start.");
        panic!();
    };

    let mut session = easyamqp::session_from_config(&cfg.rabbitmq).unwrap();
    let mut channel = session.open_channel(1).unwrap();
    channel.basic_prefetch(1).unwrap();
    channel
        .declare_exchange(easyamqp::ExchangeConfig {
            exchange: "build-jobs".to_owned(),
            exchange_type: easyamqp::ExchangeType::Fanout,
            passive: false,
            durable: true,
            auto_delete: false,
            no_wait: false,
            internal: false,
            arguments: None,
        })
        .unwrap();

    channel
        .declare_queue(easyamqp::QueueConfig {
            queue: format!("build-inputs-{}", cfg.nix.system.clone()),
            passive: false,
            durable: true,
            exclusive: false,
            auto_delete: false,
            no_wait: false,
            arguments: None,
        })
        .unwrap();

    channel
        .bind_queue(easyamqp::BindQueueConfig {
            queue: format!("build-inputs-{}", cfg.nix.system.clone()),
            exchange: "build-jobs".to_owned(),
            routing_key: None,
            no_wait: false,
            arguments: None,
        })
        .unwrap();

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
                arguments: None,
            },
        )
        .unwrap();

    channel.start_consuming();
    channel.close(200, "Bye").unwrap();
    println!("Closed the channel");
    session.close(200, "Good Bye");
    println!("Closed the session... EOF");
}
