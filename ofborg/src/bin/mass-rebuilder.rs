extern crate ofborg;
extern crate amqp;
extern crate env_logger;

use std::env;
use std::path::Path;
use ofborg::tasks;
use ofborg::config;
use ofborg::checkout;

use ofborg::stats;
use ofborg::worker;
use amqp::Basic;
use ofborg::easyamqp;
use ofborg::easyamqp::TypedWrappers;

fn main() {
    let cfg = config::load(env::args().nth(1).unwrap().as_ref());

    ofborg::setup_log();

    println!("Hello, world!");


    let mut session = easyamqp::session_from_config(&cfg.rabbitmq).unwrap();
    println!("Connected to rabbitmq");

    let mut channel = session.open_channel(1).unwrap();

    let cloner = checkout::cached_cloner(Path::new(&cfg.checkout.root));
    let nix = cfg.nix();

    let events = stats::RabbitMQ::new(
        &format!("{}-{}", cfg.runner.identity.clone(), cfg.nix.system.clone()),
        session.open_channel(3).unwrap()
    );

    let mrw = tasks::massrebuilder::MassRebuildWorker::new(
        cloner,
        nix,
        cfg.github(),
        cfg.acl(),
        cfg.runner.identity.clone(),
        events,
        cfg.tag_paths.clone().unwrap(),
    );

    channel
        .declare_queue(easyamqp::QueueConfig {
            queue: "mass-rebuild-check-jobs".to_owned(),
            passive: false,
            durable: true,
            exclusive: false,
            auto_delete: false,
            no_wait: false,
            arguments: None,
        })
        .unwrap();

    channel.basic_prefetch(1).unwrap();
    channel
        .consume(
            worker::new(mrw),
            easyamqp::ConsumeConfig {
                queue: "mass-rebuild-check-jobs".to_owned(),
                consumer_tag: format!("{}-mass-rebuild-checker", cfg.whoami()),
                no_local: false,
                no_ack: false,
                no_wait: false,
                exclusive: false,
                arguments: None,
            },
        )
        .unwrap();

    channel.start_consuming();

    println!("Finished consuming?");

    channel.close(200, "Bye").unwrap();
    println!("Closed the channel");
    session.close(200, "Good Bye");
    println!("Closed the session... EOF");
}
