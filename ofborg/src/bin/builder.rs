use std::env;
use std::error::Error;
use std::path::Path;

use async_std::task::{JoinHandle, spawn};
use futures_util::future;
use ofborg::block_on;
use tracing::{error, info, warn};

use ofborg::easyamqp::{self, ChannelExt, ConsumerExt};
use ofborg::easylapin;
use ofborg::{checkout, config, tasks};

fn main() -> Result<(), Box<dyn Error>> {
    ofborg::setup_log();

    let arg = env::args()
        .nth(1)
        .unwrap_or_else(|| panic!("usage: {} <config>", std::env::args().next().unwrap()));
    let cfg = config::load(arg.as_ref());

    let Some(builder_cfg) = config::load(arg.as_ref()).builder else {
        error!("No builder configuration found!");
        panic!();
    };

    let conn = easylapin::from_config(&builder_cfg.rabbitmq)?;
    let mut handles = Vec::new();

    for system in &cfg.nix.system {
        let handle_ext = self::create_handle(&conn, &cfg, system.to_string())?;
        handles.push(handle_ext);
    }

    block_on(future::join_all(handles));

    drop(conn); // Close connection.
    info!("Closed the session... EOF");
    Ok(())
}

fn create_handle(
    conn: &lapin::Connection,
    cfg: &config::Config,
    system: String,
) -> Result<JoinHandle<()>, Box<dyn Error>> {
    let mut chan = block_on(conn.create_channel())?;

    let cloner = checkout::cached_cloner(Path::new(&cfg.checkout.root));
    let nix = cfg.nix().with_system(system.clone());

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
        let queue_name = format!("build-inputs-{system}");
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
        tasks::build::BuildWorker::new(cloner, nix, system, cfg.runner.identity.clone()),
        easyamqp::ConsumeConfig {
            queue: queue_name.clone(),
            consumer_tag: format!("{}-builder", cfg.whoami()),
            no_local: false,
            no_ack: false,
            no_wait: false,
            exclusive: false,
        },
    )?;

    info!("Fetching jobs from {}", &queue_name);
    Ok(spawn(handle))
}
