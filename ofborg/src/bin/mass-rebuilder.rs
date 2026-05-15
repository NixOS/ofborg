use std::path::Path;

use tracing::{error, info};

use ofborg::checkout;
use ofborg::config;
use ofborg::easyamqp::{self, ChannelExt, ConsumerExt};
use ofborg::easylapin;
use ofborg::stats;
use ofborg::tasks;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    ofborg::setup_log();

    let arg = std::env::args()
        .nth(1)
        .unwrap_or_else(|| panic!("usage: {} <config>", std::env::args().next().unwrap()));
    let cfg = config::load(arg.as_ref());

    let Some(rebuilder_cfg) = config::load(arg.as_ref()).mass_rebuilder else {
        error!("No mass rebuilder configuration found!");
        panic!();
    };

    let conn = easylapin::from_config(&rebuilder_cfg.rabbitmq).await?;
    let mut chan = conn.create_channel().await?;

    let root = Path::new(&cfg.checkout.root);
    let cloner = checkout::cached_cloner(&root.join(cfg.runner.instance.to_string()));

    let events = stats::RabbitMq::from_lapin(&cfg.whoami(), conn.create_channel().await?);

    let queue_name = String::from("mass-rebuild-check-jobs");
    chan.declare_queue(easyamqp::QueueConfig {
        queue: queue_name.clone(),
        passive: false,
        durable: true,
        exclusive: false,
        auto_delete: false,
        no_wait: false,
    })
    .await?;

    let hydra_eval_cfg = cfg.hydra_evaluator.clone();
    let (hydra_eval_queue, hydra_eval_nix, hydra_eval_jobset_id) =
        if let Some(ref hec) = hydra_eval_cfg {
            chan.declare_queue(easyamqp::QueueConfig {
                queue: String::from("hydra-eval-jobs"),
                passive: false,
                durable: true,
                exclusive: false,
                auto_delete: false,
                no_wait: false,
            })
            .await?;
            (
                Some("hydra-eval-jobs".to_owned()),
                Some(cfg.nix()),
                Some(hec.jobset_id),
            )
        } else {
            (None, None, None)
        };

    let handle = easylapin::WorkerChannel(chan)
        .consume(
            tasks::evaluate::EvaluationWorker::new(
                cloner,
                cfg.github_app_vendingmachine(),
                cfg.runner.identity.clone(),
                events,
                hydra_eval_queue,
                hydra_eval_nix,
                hydra_eval_jobset_id,
            ),
            easyamqp::ConsumeConfig {
                queue: queue_name.clone(),
                consumer_tag: format!("{}-mass-rebuild-checker", cfg.whoami()),
                no_local: false,
                no_ack: false,
                no_wait: false,
                exclusive: false,
            },
        )
        .await?;

    info!("Fetching jobs from {}", queue_name);
    handle.await;

    drop(conn); // Close connection.
    info!("Closed the session... EOF");
    Ok(())
}
