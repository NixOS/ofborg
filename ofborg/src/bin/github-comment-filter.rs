use ofborg::systems::System;
use tracing::{error, info};

use ofborg::easyamqp::{self, ChannelExt, ConsumerExt};
use ofborg::{config, easylapin, tasks};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    ofborg::setup_log();

    let arg = std::env::args()
        .nth(1)
        .unwrap_or_else(|| panic!("usage: {} <config>", std::env::args().next().unwrap()));
    let cfg = config::load(arg.as_ref());

    let Some(filter_cfg) = config::load(arg.as_ref()).github_comment_filter else {
        error!("No comment filter configuration found!");
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

    let queue_name = "build-inputs";
    chan.declare_queue(easyamqp::QueueConfig {
        queue: queue_name.to_owned(),
        passive: false,
        durable: true,
        exclusive: false,
        auto_delete: false,
        no_wait: false,
    })
    .await?;

    chan.bind_queue(easyamqp::BindQueueConfig {
        queue: "build-inputs".to_owned(),
        exchange: "github-events".to_owned(),
        routing_key: Some("issue_comment.*".to_owned()),
        no_wait: false,
    })
    .await?;

    chan.declare_exchange(easyamqp::ExchangeConfig {
        exchange: "build-results".to_owned(),
        exchange_type: easyamqp::ExchangeType::Fanout,
        passive: false,
        durable: true,
        auto_delete: false,
        no_wait: false,
        internal: false,
    })
    .await?;

    // Create build job queues
    for sys in System::all_known_systems().iter().map(System::to_string) {
        chan.declare_queue(easyamqp::QueueConfig {
            queue: format!("build-inputs-{sys}"),
            passive: false,
            durable: true,
            exclusive: false,
            auto_delete: false,
            no_wait: false,
        })
        .await?;
    }

    let handle = easylapin::WorkerChannel(chan)
        .consume(
            tasks::githubcommentfilter::GitHubCommentWorker::new(cfg.acl(), cfg.github()),
            easyamqp::ConsumeConfig {
                queue: "build-inputs".to_owned(),
                consumer_tag: format!("{}-github-comment-filter", cfg.whoami()),
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
