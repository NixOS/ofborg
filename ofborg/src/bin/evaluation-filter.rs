use std::env;
use std::error::Error;

use tracing::{error, info};

use ofborg::easyamqp::{self, ChannelExt, ConsumerExt};
use ofborg::{config, easylapin, tasks};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    ofborg::setup_log();

    let arg = env::args()
        .nth(1)
        .unwrap_or_else(|| panic!("usage: {} <config>", std::env::args().next().unwrap()));
    let cfg = config::load(arg.as_ref());

    let Some(filter_cfg) = config::load(arg.as_ref()).evaluation_filter else {
        error!("No evaluation filter configuration found!");
        panic!();
    };

    let conn = easylapin::from_config(&filter_cfg.rabbitmq).await?;
    let mut chan = conn.create_channel().await?;

    chan.declare_exchange(easyamqp::ExchangeConfig {
        exchange: "github-events".to_owned(),
        exchange_type: easyamqp::ExchangeType::Topic,
        passive: false,
        durable: true,
        auto_delete: false,
        no_wait: false,
        internal: false,
    })
    .await?;

    chan.declare_queue(easyamqp::QueueConfig {
        queue: "mass-rebuild-check-jobs".to_owned(),
        passive: false,
        durable: true,
        exclusive: false,
        auto_delete: false,
        no_wait: false,
    })
    .await?;

    let queue_name = String::from("mass-rebuild-check-inputs");
    chan.declare_queue(easyamqp::QueueConfig {
        queue: queue_name.clone(),
        passive: false,
        durable: true,
        exclusive: false,
        auto_delete: false,
        no_wait: false,
    })
    .await?;

    chan.bind_queue(easyamqp::BindQueueConfig {
        queue: queue_name.clone(),
        exchange: "github-events".to_owned(),
        routing_key: Some("pull_request.nixos/*".to_owned()),
        no_wait: false,
    })
    .await?;

    let handle = easylapin::WorkerChannel(chan)
        .consume(
            tasks::evaluationfilter::EvaluationFilterWorker::new(cfg.acl()),
            easyamqp::ConsumeConfig {
                queue: queue_name.clone(),
                consumer_tag: format!("{}-evaluation-filter", cfg.whoami()),
                no_local: false,
                no_ack: false,
                no_wait: false,
                exclusive: false,
            },
        )
        .await?;

    info!("Fetching jobs from {}", &queue_name);
    handle.await;

    drop(conn); // Close connection.
    info!("Closed the session... EOF");
    Ok(())
}
