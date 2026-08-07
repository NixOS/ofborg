use anyhow::Context as _;
use tonic::Request;
use tonic::transport::Channel;

use hydra_proto::runner_service_client::RunnerServiceClient;

#[derive(Debug, Clone)]
pub enum AuthInterceptor {
    Token {
        token: tonic::metadata::MetadataValue<tonic::metadata::Ascii>,
    },
    Noop,
}

impl tonic::service::Interceptor for AuthInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, tonic::Status> {
        if let Self::Token { token } = self {
            request
                .metadata_mut()
                .insert("authorization", token.clone());
        }

        Ok(request)
    }
}

pub type OfborgClient =
    RunnerServiceClient<tonic::service::interceptor::InterceptedService<Channel, AuthInterceptor>>;

#[tracing::instrument(err)]
pub async fn init_client(cli: &crate::config::GrpcOpts) -> anyhow::Result<OfborgClient> {
    if !cli.mtls_configured_correctly() {
        tracing::error!(
            "mtls configured improperly, please pass all options: \
            server_root_ca_cert_path, client_cert_path, client_key_path and domain_name!"
        );
        return Err(anyhow::anyhow!("Configuration issue"));
    }

    tracing::info!("connecting to {}", cli.gateway_endpoint);
    let channel = if cli.mtls_enabled() {
        tracing::info!("mtls is enabled");
        let (server_root_ca_cert, client_identity, domain_name) = cli
            .get_mtls()
            .await
            .context("Failed to get_mtls Certificate and Identity")?;
        let tls = tonic::transport::ClientTlsConfig::new()
            .domain_name(domain_name)
            .ca_certificate(server_root_ca_cert)
            .identity(client_identity);

        Channel::builder(cli.gateway_endpoint.parse()?)
            .tls_config(tls)
            .context("Failed to attach tls config")?
            .connect()
            .await
            .context("Failed to establish connection with Channel")?
    } else if let Some(path) = cli.gateway_endpoint.strip_prefix("unix://") {
        let path = path.to_owned();
        tonic::transport::Endpoint::try_from("http://[::]:50051")?
            .connect_with_connector(tower::service_fn(move |_: tonic::transport::Uri| {
                let path = path.clone();
                async move {
                    Ok::<_, std::io::Error>(hyper_util::rt::TokioIo::new(
                        tokio::net::UnixStream::connect(&path).await?,
                    ))
                }
            }))
            .await
            .context("Failed to establish unix socket connection with Channel")?
    } else if cli.gateway_endpoint.starts_with("https://") {
        let uri: url::Url = cli
            .gateway_endpoint
            .parse()
            .context("Failed to parse gateway_endpoint")?;

        let tls = tonic::transport::ClientTlsConfig::new()
            .domain_name(
                uri.domain()
                    .ok_or_else(|| anyhow::anyhow!("No domain_name found for gateway_endpoint"))?,
            )
            .with_enabled_roots();
        Channel::builder(cli.gateway_endpoint.parse()?)
            .tls_config(tls)
            .context("Failed to attach tls config")?
            .connect()
            .await
            .context("Failed to establish connection with Channel")?
    } else {
        Channel::builder(cli.gateway_endpoint.parse()?)
            .connect()
            .await
            .context("Failed to establish connection with Channel")?
    };

    let interceptor = if let Some(t) = cli.get_authorization_token().await? {
        AuthInterceptor::Token {
            token: format!("Bearer {t}").parse()?,
        }
    } else {
        AuthInterceptor::Noop
    };

    Ok(RunnerServiceClient::with_interceptor(channel, interceptor)
        .max_decoding_message_size(50 * 1024 * 1024)
        .max_encoding_message_size(50 * 1024 * 1024))
}

/// Subscribe to the queue-runner's build lifecycle events.
///
/// Gated behind `queue-runner-events` because it needs the
/// `SubscribeBuildEvents` RPC that is being added to helsinki-systems/hydra
/// alongside this component; until the `hydra-proto` pin in `Cargo.toml` moves
/// to a revision that has it, `hydra-build-tracker --replay` covers the same
/// path locally.
#[cfg(feature = "queue-runner-events")]
pub async fn subscribe_build_events(
    cli: &crate::config::GrpcOpts,
) -> anyhow::Result<crate::events::BuildEventStream> {
    use tokio_stream::StreamExt as _;

    let mut client = init_client(cli).await?;

    let stream = client
        .subscribe_build_events(hydra_proto::SubscribeBuildEventsRequest {
            // Empty means "every ofborg jobset the credentials cover"; the
            // tracker ignores events for builds it has no record of anyway.
            jobset_ids: vec![],
        })
        .await
        .context("Failed to call SubscribeBuildEvents")?
        .into_inner();

    Ok(Box::pin(stream.map(|event| {
        event
            .map_err(anyhow::Error::from)
            .and_then(convert_build_event)
    })))
}

#[cfg(feature = "queue-runner-events")]
fn convert_build_event(
    event: hydra_proto::BuildEvent,
) -> anyhow::Result<crate::events::BuildEvent> {
    use crate::events::{BuildEvent, BuildEventKind};
    use hydra_proto::build_event::Event;

    let inner = event
        .event
        .ok_or_else(|| anyhow::anyhow!("BuildEvent without an event variant"))?;

    let (machine, kind) = match inner {
        Event::Queued(_) => (None, BuildEventKind::Queued),
        Event::Running(running) => (
            Some(running.machine),
            BuildEventKind::Running {
                step: step_status_label(running.step_status),
            },
        ),
        Event::Finished(finished) => (
            Some(finished.machine),
            BuildEventKind::Finished {
                status: hydra_build_status_to_status(finished.status),
            },
        ),
        Event::Lagged(lagged) => (
            None,
            BuildEventKind::Lagged {
                dropped: lagged.dropped,
            },
        ),
    };

    Ok(BuildEvent {
        build_id: event.build_id,
        // The runner sends an empty string when no builder was involved.
        machine: machine.filter(|m| !m.is_empty()),
        kind,
    })
}

/// Human-readable label for the step the builder is currently on, shown in the
/// check run while a build is in progress.
#[cfg(feature = "queue-runner-events")]
fn step_status_label(step_status: i32) -> Option<String> {
    let label = match hydra_proto::StepStatus::try_from(step_status).ok()? {
        hydra_proto::StepStatus::Preparing => "Preparing",
        hydra_proto::StepStatus::Connecting => "Connecting",
        // The queue-runner's proto has this typo; do not repeat it in the UI.
        hydra_proto::StepStatus::SeningInputs => "Sending inputs",
        hydra_proto::StepStatus::Building => "Building",
        hydra_proto::StepStatus::WaitingForLocalSlot => "Waiting for a local slot",
        hydra_proto::StepStatus::ReceivingOutputs => "Receiving outputs",
        hydra_proto::StepStatus::PostProcessing => "Post-processing",
    };

    Some(label.to_owned())
}

/// Map hydra's build status onto the status vocabulary ofborg already renders.
#[cfg(feature = "queue-runner-events")]
fn hydra_build_status_to_status(status: i32) -> ofborg::message::buildresult::BuildStatus {
    use hydra_proto::build_finished::Status;
    use ofborg::message::buildresult::BuildStatus;

    let status = match Status::try_from(status) {
        Ok(s) => s,
        Err(e) => {
            return BuildStatus::UnexpectedError {
                err: format!("unknown hydra build status {status}: {e}"),
            };
        }
    };

    match status {
        Status::Success => BuildStatus::Success,
        // The build itself failed — the contributor's problem, and the thing
        // they actually want to see.
        Status::Failed | Status::FailedWithOutput | Status::DepFailed | Status::CachedFailure => {
            BuildStatus::Failure
        }
        Status::TimedOut => BuildStatus::TimedOut,
        Status::NotDeterministic => BuildStatus::HashMismatch,
        // Everything else went wrong around the build rather than in it.
        other => BuildStatus::UnexpectedError {
            err: format!("{other:?}"),
        },
    }
}
