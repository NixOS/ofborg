use amqp::Basic;
use ofborg::config;
use ofborg::easyamqp::{self, ChannelExt, ConsumerExt};
use ofborg::tasks;
use ofborg::worker;

use std::env;

use tracing::info;

fn main() {
    let cfg = config::load(env::args().nth(1).unwrap().as_ref());
    ofborg::setup_log();

    info!("Hello, world!");

    let mut session = easyamqp::session_from_config(&cfg.rabbitmq).unwrap();
    info!("Connected to rabbitmq");

    let mut channel = session.open_channel(1).unwrap();

    channel
        .declare_exchange(easyamqp::ExchangeConfig {
            exchange: "github-events".to_owned(),
            exchange_type: easyamqp::ExchangeType::Topic,
            passive: false,
            durable: true,
            auto_delete: false,
            no_wait: false,
            internal: false,
        })
        .unwrap();

    channel
        .declare_queue(easyamqp::QueueConfig {
            queue: "mass-rebuild-check-jobs".to_owned(),
            passive: false,
            durable: true,
            exclusive: false,
            auto_delete: false,
            no_wait: false,
        })
        .unwrap();

    channel
        .declare_queue(easyamqp::QueueConfig {
            queue: "mass-rebuild-check-inputs".to_owned(),
            passive: false,
            durable: true,
            exclusive: false,
            auto_delete: false,
            no_wait: false,
        })
        .unwrap();

    channel
        .bind_queue(easyamqp::BindQueueConfig {
            queue: "mass-rebuild-check-inputs".to_owned(),
            exchange: "github-events".to_owned(),
            routing_key: Some("pull_request.nixos/nixpkgs".to_owned()),
            no_wait: false,
        })
        .unwrap();

    channel.basic_prefetch(1).unwrap();
    let mut channel = channel
        .consume(
            worker::new(tasks::evaluationfilter::EvaluationFilterWorker::new(
                cfg.acl(),
            )),
            easyamqp::ConsumeConfig {
                queue: "mass-rebuild-check-inputs".to_owned(),
                consumer_tag: format!("{}-evaluation-filter", cfg.whoami()),
                no_local: false,
                no_ack: false,
                no_wait: false,
                exclusive: false,
            },
        )
        .unwrap();

    channel.start_consuming();

    info!("Finished consuming?");

    channel.close(200, "Bye").unwrap();
    info!("Closed the channel");
    session.close(200, "Good Bye");
    info!("Closed the session... EOF");
}
