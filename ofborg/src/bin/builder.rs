use amqp::Basic;
use log::{info, log, warn};
use ofborg::checkout;
use ofborg::config;
use ofborg::easyamqp::{self, TypedWrappers};
use ofborg::notifyworker;
use ofborg::tasks;

use std::env;
use std::path::Path;

fn main() {
    let cfg = config::load(env::args().nth(1).unwrap().as_ref());

    ofborg::setup_log();

    let cloner = checkout::cached_cloner(Path::new(&cfg.checkout.root));
    let nix = cfg.nix();

    if !cfg.feedback.full_logs {
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

    let queue_name: String = if cfg.runner.build_all_jobs != Some(true) {
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
            .unwrap()
            .queue
    } else {
        warn!("Building all jobs, please don't use this unless you're");
        warn!("developing and have Graham's permission!");
        channel
            .declare_queue(easyamqp::QueueConfig {
                queue: "".to_owned(),
                passive: false,
                durable: false,
                exclusive: true,
                auto_delete: true,
                no_wait: false,
                arguments: None,
            })
            .unwrap()
            .queue
    };

    channel
        .bind_queue(easyamqp::BindQueueConfig {
            queue: queue_name.clone(),
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
            )),
            easyamqp::ConsumeConfig {
                queue: queue_name.clone(),
                consumer_tag: format!("{}-builder", cfg.whoami()),
                no_local: false,
                no_ack: false,
                no_wait: false,
                exclusive: false,
                arguments: None,
            },
        )
        .unwrap();

    info!("Fetching jobs from {}", &queue_name);
    channel.start_consuming();
    channel.close(200, "Bye").unwrap();
    info!("Closed the channel");
    session.close(200, "Good Bye");
    info!("Closed the session... EOF");
}
