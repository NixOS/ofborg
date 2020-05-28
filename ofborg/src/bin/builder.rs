use std::env;
use std::error::Error;
use std::path::Path;

use async_std::task;
use tracing::{info, warn};

use ofborg::easyamqp::{self, ChannelExt, ConsumerExt};
use ofborg::{checkout, config, easylapin, metrics, tasks};

// FIXME: remove with rust/cargo update
#[allow(clippy::cognitive_complexity)]
fn main() -> Result<(), Box<dyn Error>> {
    ofborg::setup_log();

    let arg = env::args().nth(1).expect("usage: builder <config>");
    let cfg = config::load(arg.as_ref());

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

    let conn = easylapin::from_config(&cfg.rabbitmq)?;
    let mut chan = task::block_on(conn.create_channel())?;

    chan.declare_exchange(easyamqp::ExchangeConfig {
        exchange: "build-jobs".to_owned(),
        exchange_type: easyamqp::ExchangeType::Fanout,
        passive: false,
        durable: true,
        auto_delete: false,
        no_wait: false,
        internal: false,
    })?;

    let queue_name = if cfg.runner.build_all_jobs != Some(true) {
        let queue_name = format!("build-inputs-{}", cfg.nix.system.clone());
        chan.declare_queue(easyamqp::QueueConfig {
            queue: queue_name.clone(),
            passive: false,
            durable: true,
            exclusive: false,
            auto_delete: false,
            no_wait: false,
        })?;
        queue_name
    } else {
        warn!("Building all jobs, please don't use this unless you're");
        warn!("developing and have Graham's permission!");
        let queue_name = "".to_owned();
        chan.declare_queue(easyamqp::QueueConfig {
            queue: queue_name.clone(),
            passive: false,
            durable: false,
            exclusive: true,
            auto_delete: true,
            no_wait: false,
        })?;
        queue_name
    };

    chan.bind_queue(easyamqp::BindQueueConfig {
        queue: queue_name.clone(),
        exchange: "build-jobs".to_owned(),
        routing_key: None,
        no_wait: false,
    })?;

    let handle = easylapin::NotifyChannel(chan).consume(
        tasks::build::BuildWorker::new(
            cloner,
            nix,
            cfg.nix.system.clone(),
            cfg.runner.identity.clone(),
        ),
        easyamqp::ConsumeConfig {
            queue: queue_name.clone(),
            consumer_tag: format!("{}-builder", cfg.whoami()),
            no_local: false,
            no_ack: false,
            no_wait: false,
            exclusive: false,
        },
    )?;

    metrics::spawn_server();

    info!("Fetching jobs from {}", &queue_name);
    task::block_on(handle);

    drop(conn); // Close connection.
    info!("Closed the session... EOF");
    Ok(())
}
