#![forbid(unsafe_code)]
#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::expect_used,
    clippy::unwrap_used,
    future_incompatible,
    nonstandard_style,
    unused_qualifications
)]
#![allow(clippy::missing_errors_doc)]

use hydra_evaluator::config::TrackerCli;
use hydra_evaluator::events::{self, BuildEventStream};
use hydra_evaluator::tracker::Tracker;

/// Open the queue-runner's build event stream.
///
/// Requires the `SubscribeBuildEvents` RPC, which is being added to
/// helsinki-systems/hydra alongside this component. Until the `hydra-proto`
/// pin in `Cargo.toml` moves to a revision that has it, build the tracker
/// without this feature and drive it with `--replay`.
#[cfg(feature = "queue-runner-events")]
async fn subscribe(cli: &TrackerCli) -> anyhow::Result<BuildEventStream> {
    hydra_evaluator::grpc::subscribe_build_events(&cli.grpc).await
}

#[cfg(not(feature = "queue-runner-events"))]
#[allow(clippy::unused_async)]
async fn subscribe(_cli: &TrackerCli) -> anyhow::Result<BuildEventStream> {
    anyhow::bail!(
        "this build has no queue-runner event subscription (feature \
         \"queue-runner-events\" is off); pass --replay <file> to drive the \
         tracker from a JSON-lines file instead"
    )
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    hydra_tracing::init()?;

    let cli = TrackerCli::new();
    let Some(cfg) = ofborg::config::load(&cli.config_path).hydra_evaluator else {
        tracing::error!("No ofborg/hydra evaluator configuration found!");
        panic!();
    };

    let conn = ofborg::easylapin::from_config(&cfg.rabbitmq).await?;
    let chan = conn.create_channel().await?;

    let events: BuildEventStream = if let Some(path) = &cli.replay {
        tracing::info!("replaying build events from {}", path.display());
        events::replay_from_file(path, std::time::Duration::from_secs(cli.replay_delay_secs))?
    } else {
        tracing::info!(
            "subscribing to queue-runner build events at {}",
            cli.grpc.gateway_endpoint
        );
        subscribe(&cli).await?
    };

    let mut tracker = Tracker::new(chan, cfg.hydra_base_url.clone(), cfg.stale_after_seconds);
    tracker.run(events).await?;

    drop(conn); // Close connection.
    tracing::info!("Closed the session... EOF");
    Ok(())
}
