use std::env;
use std::error::Error;
use std::future::Future;
use std::path::Path;
use std::pin::Pin;

use futures_util::future;
use tracing::{error, info, warn};

use ofborg::easyamqp::{self, ChannelExt, ConsumerExt};
use ofborg::easylapin;
use ofborg::{checkout, config, tasks};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    ofborg::setup_log();

    let arg = env::args()
        .nth(1)
        .unwrap_or_else(|| panic!("usage: {} <config>", std::env::args().next().unwrap()));
    let cfg = config::load(arg.as_ref());

    let Some(builder_cfg) = config::load(arg.as_ref()).builder else {
        error!("No builder configuration found!");
        panic!();
    };

    let conn = easylapin::from_config(&builder_cfg.rabbitmq).await?;
    let mut handles: Vec<Pin<Box<dyn Future<Output = ()> + Send>>> = Vec::new();

    for system in &cfg.nix.system {
        handles.push(self::create_handle(&conn, &cfg, system.to_string()).await?);
    }

    future::join_all(handles).await;

    drop(conn); // Close connection.
    info!("Closed the session... EOF");
    Ok(())
}

#[allow(clippy::type_complexity)]
async fn create_handle(
    conn: &lapin::Connection,
    cfg: &config::Config,
    system: String,
) -> Result<Pin<Box<dyn Future<Output = ()> + Send>>, Box<dyn Error>> {
    let mut chan = conn.create_channel().await?;

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
    })
    .await?;

    let queue_name = if cfg.runner.build_all_jobs != Some(true) {
        let queue_name = format!("build-inputs-{system}");
        chan.declare_queue(easyamqp::QueueConfig {
            queue: queue_name.clone(),
            passive: false,
            durable: true,
            exclusive: false,
            auto_delete: false,
            no_wait: false,
        })
        .await?;
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
        })
        .await?;
        queue_name
    };

    chan.bind_queue(easyamqp::BindQueueConfig {
        queue: queue_name.clone(),
        exchange: "build-jobs".to_owned(),
        routing_key: None,
        no_wait: false,
    })
    .await?;

    let handle = easylapin::NotifyChannel(chan)
        .consume(
            tasks::build::BuildWorker::new(cloner, nix, system, cfg.runner.identity.clone()),
            easyamqp::ConsumeConfig {
                queue: queue_name.clone(),
                consumer_tag: format!("{}-builder", cfg.whoami()),
                no_local: false,
                no_ack: false,
                no_wait: false,
                exclusive: false,
            },
        )
        .await?;

    info!("Fetching jobs from {}", &queue_name);
    Ok(handle)
}
