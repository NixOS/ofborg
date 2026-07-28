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
pub async fn init_client(cli: &crate::config::Cli) -> anyhow::Result<OfborgClient> {
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
