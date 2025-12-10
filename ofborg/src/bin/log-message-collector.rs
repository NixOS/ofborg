use std::env;
use std::error::Error;
use std::path::PathBuf;

use ofborg::block_on;
use tracing::{error, info};

use ofborg::config;
use ofborg::easyamqp::{self, ChannelExt, ConsumerExt};
use ofborg::easylapin;
use ofborg::tasks;

fn main() -> Result<(), Box<dyn Error>> {
    ofborg::setup_log();

    let arg = env::args()
        .nth(1)
        .unwrap_or_else(|| panic!("usage: {} <config>", std::env::args().next().unwrap()));
    let cfg = config::load(arg.as_ref());

    let Some(collector_cfg) = config::load(arg.as_ref()).log_message_collector else {
        error!("No log message collector configuration found!");
        panic!();
    };

    let conn = easylapin::from_config(&collector_cfg.rabbitmq)?;
    let mut chan = block_on(conn.create_channel())?;

    chan.declare_exchange(easyamqp::ExchangeConfig {
        exchange: "logs".to_owned(),
        exchange_type: easyamqp::ExchangeType::Topic,
        passive: false,
        durable: true,
        auto_delete: false,
        no_wait: false,
        internal: false,
    })?;

    let queue_name = "logs".to_owned();
    chan.declare_queue(easyamqp::QueueConfig {
        queue: queue_name.clone(),
        passive: false,
        durable: false,
        exclusive: true,
        auto_delete: true,
        no_wait: false,
    })?;

    chan.bind_queue(easyamqp::BindQueueConfig {
        queue: queue_name.clone(),
        exchange: "logs".to_owned(),
        routing_key: Some("*.*".to_owned()),
        no_wait: false,
    })?;

    // Regular channel, we want prefetching here.
    let handle = chan.consume(
        tasks::log_message_collector::LogMessageCollector::new(
            PathBuf::from(collector_cfg.logs_path),
            100,
        ),
        easyamqp::ConsumeConfig {
            queue: queue_name.clone(),
            consumer_tag: format!("{}-log-collector", cfg.whoami()),
            no_local: false,
            no_ack: false,
            no_wait: false,
            exclusive: false,
        },
    )?;

    info!("Fetching jobs from {}", &queue_name);
    block_on(handle);

    drop(conn); // Close connection.
    info!("Closed the session... EOF");
    Ok(())
}
