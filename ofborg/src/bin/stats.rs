use std::env;
use std::error::Error;
use std::thread;

use async_std::task;
use hyper::server::{Request, Response, Server};
use tracing::info;

use ofborg::easyamqp::{ChannelExt, ConsumerExt};
use ofborg::{config, easyamqp, easylapin, stats, tasks};

fn main() -> Result<(), Box<dyn Error>> {
    ofborg::setup_log();

    let arg = env::args().nth(1).expect("usage: stats <config>");
    let cfg = config::load(arg.as_ref());

    let conn = easylapin::from_config(&cfg.rabbitmq)?;
    let mut chan = task::block_on(conn.create_channel())?;

    let events = stats::RabbitMQ::from_lapin(
        &format!("{}-{}", cfg.runner.identity, cfg.nix.system),
        task::block_on(conn.create_channel())?,
    );

    let metrics = stats::MetricCollector::new();
    let collector = tasks::statscollector::StatCollectorWorker::new(events, metrics.clone());

    chan.declare_exchange(easyamqp::ExchangeConfig {
        exchange: "stats".to_owned(),
        exchange_type: easyamqp::ExchangeType::Fanout,
        passive: false,
        durable: true,
        auto_delete: false,
        no_wait: false,
        internal: false,
    })?;

    let queue_name = String::from("stats-events");
    chan.declare_queue(easyamqp::QueueConfig {
        queue: queue_name.clone(),
        passive: false,
        durable: true,
        exclusive: false,
        auto_delete: false,
        no_wait: false,
    })?;

    chan.bind_queue(easyamqp::BindQueueConfig {
        queue: queue_name.clone(),
        exchange: "stats".to_owned(),
        routing_key: None,
        no_wait: false,
    })?;

    let handle = chan.consume(
        collector,
        easyamqp::ConsumeConfig {
            queue: "stats-events".to_owned(),
            consumer_tag: format!("{}-prometheus-stats-collector", cfg.whoami()),
            no_local: false,
            no_ack: false,
            no_wait: false,
            exclusive: false,
        },
    )?;

    thread::spawn(|| {
        let addr = "0.0.0.0:9898";
        info!("listening addr {:?}", addr);
        Server::http(addr)?.handle(move |_: Request, res: Response| {
            res.send(metrics.prometheus_output().as_bytes()).unwrap();
        })?;
        Ok::<_, Box<dyn Error + Sync + Send + '_>>(())
    });

    info!("Fetching jobs from {}", &queue_name);
    task::block_on(handle);

    drop(conn); // Close connection.
    info!("Closed the session... EOF");
    Ok(())
}
