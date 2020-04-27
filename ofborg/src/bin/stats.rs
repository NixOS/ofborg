use amqp::Basic;
use hyper::server::{Request, Response, Server};
use log::{info, log};
use ofborg::easyamqp::{ChannelExt, ConsumerExt};
use ofborg::{config, easyamqp, stats, tasks, worker};

use std::env;
use std::thread;

fn main() {
    let cfg = config::load(env::args().nth(1).unwrap().as_ref());
    ofborg::setup_log();

    info!("Hello, world!");

    let mut session = easyamqp::session_from_config(&cfg.rabbitmq).unwrap();
    info!("Connected to rabbitmq");

    let events = stats::RabbitMQ::new(
        &format!("{}-{}", cfg.runner.identity.clone(), cfg.nix.system.clone()),
        session.open_channel(3).unwrap(),
    );

    let metrics = stats::MetricCollector::new();

    let collector = tasks::statscollector::StatCollectorWorker::new(events, metrics.clone());

    let mut channel = session.open_channel(1).unwrap();
    channel
        .declare_exchange(easyamqp::ExchangeConfig {
            exchange: "stats".to_owned(),
            exchange_type: easyamqp::ExchangeType::Fanout,
            passive: false,
            durable: true,
            auto_delete: false,
            no_wait: false,
            internal: false,
        })
        .unwrap();

    channel
        .declare_queue(easyamqp::QueueConfig {
            queue: "stats-events".to_owned(),
            passive: false,
            durable: true,
            exclusive: false,
            auto_delete: false,
            no_wait: false,
        })
        .unwrap();

    channel
        .bind_queue(easyamqp::BindQueueConfig {
            queue: "stats-events".to_owned(),
            exchange: "stats".to_owned(),
            routing_key: None,
            no_wait: false,
        })
        .unwrap();

    channel.basic_prefetch(1).unwrap();
    channel
        .consume(
            worker::new(collector),
            easyamqp::ConsumeConfig {
                queue: "stats-events".to_owned(),
                consumer_tag: format!("{}-prometheus-stats-collector", cfg.whoami()),
                no_local: false,
                no_ack: false,
                no_wait: false,
                exclusive: false,
            },
        )
        .unwrap();

    thread::spawn(|| {
        let addr = "0.0.0.0:9898";
        info!("listening addr {:?}", addr);
        Server::http(addr)
            .unwrap()
            .handle(move |_: Request, res: Response| {
                res.send(metrics.prometheus_output().as_bytes()).unwrap();
            })
            .unwrap();
    });

    channel.start_consuming();

    info!("Finished consuming?");

    channel.close(200, "Bye").unwrap();
    info!("Closed the channel");
    session.close(200, "Good Bye");
    info!("Closed the session... EOF");
}
