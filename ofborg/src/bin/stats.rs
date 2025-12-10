use std::env;
use std::error::Error;
use std::sync::Arc;
use std::thread;

use hyper::server::{Request, Response, Server};
use ofborg::block_on;
use tracing::{error, info};

use ofborg::easyamqp::{ChannelExt, ConsumerExt};
use ofborg::{config, easyamqp, easylapin, stats, tasks};

fn run_http_server(metrics: Arc<stats::MetricCollector>) {
    let addr = "0.0.0.0:9898";
    info!("HTTP server listening on {}", addr);
    Server::http(addr)
        .expect("Failed to bind HTTP server")
        .handle(move |_: Request, res: Response| {
            res.send(metrics.prometheus_output().as_bytes()).unwrap();
        })
        .expect("Failed to start HTTP server");
}

fn main() -> Result<(), Box<dyn Error>> {
    ofborg::setup_log();

    let arg = env::args()
        .nth(1)
        .unwrap_or_else(|| panic!("usage: {} <config>", std::env::args().next().unwrap()));
    let cfg = config::load(arg.as_ref());

    let Some(stats_cfg) = config::load(arg.as_ref()).stats else {
        error!("No stats configuration found!");
        panic!();
    };

    let conn = easylapin::from_config(&stats_cfg.rabbitmq)?;

    let mut chan = block_on(conn.create_channel())?;

    let events = stats::RabbitMq::from_lapin(&cfg.whoami(), block_on(conn.create_channel())?);

    let metrics = Arc::new(stats::MetricCollector::new());
    let collector = tasks::statscollector::StatCollectorWorker::new(events, (*metrics).clone());

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

    // Spawn HTTP server in a separate thread
    let metrics_clone = metrics.clone();
    thread::spawn(move || {
        run_http_server(metrics_clone);
    });

    info!("Fetching jobs from {}", &queue_name);
    block_on(handle);

    drop(conn); // Close connection.
    info!("Closed the session... EOF");
    Ok(())
}
