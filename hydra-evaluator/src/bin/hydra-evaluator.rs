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

use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Context as _;
use futures::TryFutureExt as _;
use harmonia_store_path::{FromStoreDirStr, StorePath};
use hydra_evaluator::config;
use hydra_evaluator::grpc::{self, OfborgClient};
use hydra_proto::{CreateBuildRequest, ProtoStorePath};
use lapin::options::{BasicAckOptions, BasicConsumeOptions};
use lapin::types::FieldTable;
use nix_utils::BaseStore as _;
use ofborg::message::hydra_build::{
    HydraBuildState, HydraBuildTracking, HydraBuildUpdate, HydraV1Tag,
};
use ofborg::message::hydra_eval_job::HydraEvalDrv;
use tokio::sync::mpsc;
use tokio_stream::StreamExt as _;

#[tracing::instrument(skip(client, drv_paths), err)]
async fn import_drvs(client: &mut OfborgClient, drv_paths: &[StorePath]) -> anyhow::Result<()> {
    let (tx, rx) =
        mpsc::unbounded_channel::<Result<hydra_proto::AddToStoreRequest, tonic::Status>>();

    let store = nix_utils::LocalStore::init();
    let drv_paths = drv_paths.to_vec();
    let sender = tokio::task::spawn_blocking(move || {
        let rt = tokio::runtime::Runtime::new()?;
        let infos = rt.block_on(store.query_path_infos(&drv_paths.iter().collect::<Vec<_>>()))?;
        store_transfer::export::export(&store, &drv_paths, &infos, &tx);
        Ok::<(), anyhow::Error>(())
    });

    let upload = client
        .build_result(tokio_stream::StreamExt::filter_map(
            tokio_stream::wrappers::UnboundedReceiverStream::new(rx),
            Result::ok,
        ))
        .map_err(Into::<anyhow::Error>::into);

    let (upload_result, sender_result) = futures::future::join(upload, sender).await;
    upload_result?;
    sender_result??;

    Ok(())
}

#[tracing::instrument(skip(client), err)]
async fn create_builds(
    client: &mut OfborgClient,
    jobset_id: i32,
    drv_paths: &[StorePath],
) -> anyhow::Result<HashMap<String, i32>> {
    let response = client
        .create_build(CreateBuildRequest {
            jobset_id,
            drv_paths: drv_paths.iter().map(ProtoStorePath::from).collect(),
        })
        .await
        .context("Failed to call CreateBuild")?;

    Ok(response.into_inner().build_ids)
}

/// `CreateBuild` answers with a map keyed by store path. Whether that is the
/// full `/nix/store/...` path or just the basename is up to the queue-runner's
/// `ProtoStorePath` encoding, so accept either rather than silently dropping
/// every build.
fn lookup_build_id(build_ids: &HashMap<String, i32>, drv_path: &str) -> Option<i32> {
    if let Some(build_id) = build_ids.get(drv_path) {
        return Some(*build_id);
    }

    let base = drv_path.rsplit('/').next().unwrap_or(drv_path);
    build_ids.get(base).copied()
}

/// Hand the builds over to `hydra-build-tracker`, and put a queued check run on
/// the pull request right away.
///
/// The check run is published from here rather than from the tracker so that a
/// pull request shows its builds the moment they are created, even if the
/// tracker happens to be down.
#[tracing::instrument(skip_all, err)]
async fn publish_tracking(
    chan: &lapin::Channel,
    cfg: &ofborg::config::HydraEvaluatorConfig,
    job: &ofborg::message::hydra_eval_job::HydraEvalJob,
    drvs: &[HydraEvalDrv],
    build_ids: &HashMap<String, i32>,
) -> anyhow::Result<()> {
    let queued_at = hydra_evaluator::unix_now();

    for drv in drvs {
        let Some(build_id) = lookup_build_id(build_ids, &drv.drv_path) else {
            tracing::warn!(
                "queue-runner returned no build id for {}, it will not be reported on",
                drv.drv_path
            );
            continue;
        };

        let tracking = HydraBuildTracking {
            repo: job.repo.clone(),
            pr: job.pr.clone(),
            attr: drv.attr.clone(),
            system: job.system.clone(),
            drv_path: drv.drv_path.clone(),
            build_id,
            jobset_id: job.jobset_id,
            request_id: job.request_id.clone(),
            queued_at,
        };

        hydra_evaluator::publish_json(
            chan,
            "",
            hydra_evaluator::HYDRA_BUILD_TRACKING_QUEUE,
            &tracking,
        )
        .await?;

        let queued = HydraBuildUpdate {
            tag: HydraV1Tag::HydraV1,
            repo: job.repo.clone(),
            pr: job.pr.clone(),
            attr: drv.attr.clone(),
            system: job.system.clone(),
            build_id,
            machine: None,
            state: HydraBuildState::Queued,
            hydra_base_url: cfg.hydra_base_url.clone(),
        };

        hydra_evaluator::publish_json(chan, hydra_evaluator::BUILD_RESULTS_EXCHANGE, "", &queued)
            .await?;
    }

    Ok(())
}

#[tokio::main]
#[allow(clippy::too_many_lines)]
async fn main() -> anyhow::Result<()> {
    hydra_tracing::init()?;
    nix_utils::init_nix();

    let cli = Arc::new(config::Cli::new());
    let Some(cfg) = ofborg::config::load(&cli.config_path).hydra_evaluator else {
        tracing::error!("No ofborg/hydra evaluator configuration found!");
        panic!();
    };

    tracing::info!(
        "ofborg-evaluator starting endpoint={}",
        cli.grpc.gateway_endpoint
    );

    tracing::info!("running in AMQP consumer mode");
    let conn = ofborg::easylapin::from_config(&cfg.rabbitmq).await?;
    let chan = conn.create_channel().await?;

    // Must match what mass-rebuilder publishes to.
    hydra_evaluator::declare_durable_queue(&chan, hydra_evaluator::HYDRA_EVAL_JOBS_QUEUE).await?;
    hydra_evaluator::declare_durable_queue(&chan, hydra_evaluator::HYDRA_BUILD_TRACKING_QUEUE)
        .await?;
    hydra_evaluator::declare_build_results_exchange(&chan).await?;

    tracing::info!("connecting to queue-runner gRPC");
    let mut client = grpc::init_client(&cli.grpc).await?;

    tracing::info!("consuming from {}", hydra_evaluator::HYDRA_EVAL_JOBS_QUEUE);
    let mut consumer = chan
        .basic_consume(
            hydra_evaluator::HYDRA_EVAL_JOBS_QUEUE.into(),
            "ofborg-hydra-evaluator".into(),
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    while let Some(Ok(delivery)) = consumer.next().await {
        let body = &delivery.data;
        let job: ofborg::message::hydra_eval_job::HydraEvalJob = match serde_json::from_slice(body)
        {
            Ok(job) => job,
            Err(e) => {
                tracing::error!(
                    "Failed to deserialize HydraEvalJob: {e}, body: {:?}",
                    std::str::from_utf8(body)
                );
                let _ = chan
                    .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                    .await;
                continue;
            }
        };

        let drvs = job.all_drvs();

        tracing::info!(
            "Processing HydraEvalJob for {}/{} PR #{} ({} drv paths, system={}, jobset_id={})",
            job.repo.owner,
            job.repo.name,
            job.pr.number,
            drvs.len(),
            job.system,
            job.jobset_id,
        );

        if drvs.is_empty() {
            tracing::warn!("Received HydraEvalJob with no drv paths, acking");
            let _ = chan
                .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                .await;
            continue;
        }

        let store_dir = nix_utils::LocalStore::init().store_dir().clone();
        let drv_paths: Vec<StorePath> = drvs
            .iter()
            .map(|d| {
                StorePath::from_store_dir_str(&store_dir, &d.drv_path)
                    .unwrap_or_else(|e| panic!("Invalid store path '{}': {e}", d.drv_path))
            })
            .collect();

        match import_drvs(&mut client, &drv_paths).await {
            Ok(()) => {
                tracing::info!("Successfully imported {} drv(s)", drv_paths.len());
            }
            Err(e) => {
                tracing::error!("Failed to import drvs: {e:?}");
                // Nack and requeue so another consumer can retry
                let _ = chan
                    .basic_nack(
                        delivery.delivery_tag,
                        lapin::options::BasicNackOptions {
                            requeue: true,
                            ..Default::default()
                        },
                    )
                    .await;
                continue;
            }
        }

        let build_ids = match create_builds(&mut client, job.jobset_id, &drv_paths).await {
            Ok(build_ids) => {
                tracing::info!("Created {} build(s)", build_ids.len());
                for (drv_path, build_id) in &build_ids {
                    tracing::info!("  {build_id} <- {drv_path}");
                }
                build_ids
            }
            Err(e) => {
                tracing::error!("Failed to create builds: {e:?}");
                let _ = chan
                    .basic_nack(
                        delivery.delivery_tag,
                        lapin::options::BasicNackOptions {
                            requeue: true,
                            ..Default::default()
                        },
                    )
                    .await;
                continue;
            }
        };

        // Requeue rather than ack on failure: without tracking records nothing
        // would ever report these builds back to GitHub.
        if let Err(e) = publish_tracking(&chan, &cfg, &job, &drvs, &build_ids).await {
            tracing::error!("Failed to publish build tracking records: {e:?}");
            let _ = chan
                .basic_nack(
                    delivery.delivery_tag,
                    lapin::options::BasicNackOptions {
                        requeue: true,
                        ..Default::default()
                    },
                )
                .await;
            continue;
        }

        let _ = chan
            .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
            .await;
        tracing::info!("Finished processing job for PR #{}", job.pr.number);
    }

    drop(conn); // Close connection.
    tracing::info!("Closed the session... EOF");
    Ok(())
}
