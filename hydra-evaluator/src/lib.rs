#![forbid(unsafe_code)]
#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::expect_used,
    clippy::unwrap_used,
    future_incompatible,
    missing_debug_implementations,
    nonstandard_style,
    missing_copy_implementations,
    unused_qualifications
)]
#![allow(clippy::missing_errors_doc, clippy::must_use_candidate)]
#![cfg_attr(test, allow(clippy::expect_used, clippy::unwrap_used))]

pub mod config;
pub mod events;
pub mod grpc;
pub mod tracker;

use lapin::options::{BasicPublishOptions, ExchangeDeclareOptions, QueueDeclareOptions};
use lapin::types::FieldTable;
use lapin::{BasicProperties, Channel, ExchangeKind};

/// Queue the mass-rebuilder publishes evaluated derivations to.
pub const HYDRA_EVAL_JOBS_QUEUE: &str = "hydra-eval-jobs";

/// Durable queue holding one record per in-flight Hydra build.
///
/// This queue is the only place the `build_id -> pull request` mapping is
/// persisted: `hydra-build-tracker` consumes without acking and only acks once
/// a build reached a terminal state, so a restart gets exactly the still-pending
/// records redelivered.
pub const HYDRA_BUILD_TRACKING_QUEUE: &str = "hydra-build-tracking";

/// Fanout exchange the GitHub comment poster consumes from.
pub const BUILD_RESULTS_EXCHANGE: &str = "build-results";

pub async fn declare_durable_queue(chan: &Channel, queue: &str) -> anyhow::Result<()> {
    chan.queue_declare(
        queue.into(),
        QueueDeclareOptions {
            passive: false,
            durable: true,
            exclusive: false,
            auto_delete: false,
            nowait: false,
        },
        FieldTable::default(),
    )
    .await?;
    Ok(())
}

/// Declared here as well as in the comment poster so that neither has to start
/// first.
pub async fn declare_build_results_exchange(chan: &Channel) -> anyhow::Result<()> {
    chan.exchange_declare(
        BUILD_RESULTS_EXCHANGE.into(),
        ExchangeKind::Fanout,
        ExchangeDeclareOptions {
            passive: false,
            durable: true,
            auto_delete: false,
            internal: false,
            nowait: false,
        },
        FieldTable::default(),
    )
    .await?;
    Ok(())
}

pub async fn publish_json<T: serde::Serialize + ?Sized>(
    chan: &Channel,
    exchange: &str,
    routing_key: &str,
    msg: &T,
) -> anyhow::Result<()> {
    let payload = serde_json::to_vec(msg)?;
    let props = BasicProperties::default()
        .with_content_type("application/json".into())
        .with_delivery_mode(2); // persistent

    chan.basic_publish(
        exchange.into(),
        routing_key.into(),
        BasicPublishOptions::default(),
        &payload,
        props,
    )
    .await?
    .await?;

    Ok(())
}

/// Seconds since the unix epoch, saturating rather than panicking on a clock
/// that is set before 1970.
pub fn unix_now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| i64::try_from(d.as_secs()).unwrap_or(i64::MAX))
}
