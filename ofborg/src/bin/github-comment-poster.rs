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

    let Some(poster_cfg) = config::load(arg.as_ref()).github_comment_poster else {
        error!("No comment poster configuration found!");
        panic!();
    };

    let conn = easylapin::from_config(&poster_cfg.rabbitmq).await?;
    let mut chan = conn.create_channel().await?;

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

    chan.declare_queue(easyamqp::QueueConfig {
        queue: "build-results".to_owned(),
        passive: false,
        durable: true,
        exclusive: false,
        auto_delete: false,
        no_wait: false,
    })
    .await?;

    chan.bind_queue(easyamqp::BindQueueConfig {
        queue: "build-results".to_owned(),
        exchange: "build-results".to_owned(),
        routing_key: None,
        no_wait: false,
    })
    .await?;

    let handle = easylapin::WorkerChannel(chan)
        .consume(
            tasks::githubcommentposter::GitHubCommentPoster::new(cfg.github_app_vendingmachine()),
            easyamqp::ConsumeConfig {
                queue: "build-results".to_owned(),
                consumer_tag: format!("{}-github-comment-poster", cfg.whoami()),
                no_local: false,
                no_ack: false,
                no_wait: false,
                exclusive: false,
            },
        )
        .await?;

    handle.await;

    drop(conn); // Close connection.
    info!("Closed the session... EOF");
    Ok(())
}
