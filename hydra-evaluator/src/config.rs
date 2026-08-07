use clap::{Args, Parser};

/// How to reach the queue-runner. Shared by every binary in this crate.
#[derive(Args, Debug)]
pub struct GrpcOpts {
    /// Queue-runner gRPC endpoint
    #[clap(short, long, default_value = "http://[::1]:50051")]
    pub gateway_endpoint: String,

    /// File containing the bearer token for authentication
    #[clap(long)]
    pub authorization_file: Option<std::path::PathBuf>,

    /// Whether to use mTLS
    #[clap(long)]
    pub mtls: bool,

    /// Path to Server root CA cert
    #[clap(long)]
    pub server_root_ca_cert_path: Option<std::path::PathBuf>,

    /// Path to Client cert
    #[clap(long)]
    pub client_cert_path: Option<std::path::PathBuf>,

    /// Path to Client key
    #[clap(long)]
    pub client_key_path: Option<std::path::PathBuf>,

    /// Domain name for mTLS
    #[clap(long)]
    pub domain_name: Option<String>,
}

#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about,
    long_about = "ofborg-evaluator: injects derivations into a queue-runner jobset"
)]
pub struct Cli {
    #[clap(flatten)]
    pub grpc: GrpcOpts,

    /// Config file path
    #[clap()]
    pub config_path: std::path::PathBuf,
}

/// `hydra-build-tracker`'s options: the same connection details, plus a way to
/// drive it without a queue-runner.
#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about,
    long_about = "ofborg-build-tracker: reports queue-runner build progress back to GitHub"
)]
pub struct TrackerCli {
    #[clap(flatten)]
    pub grpc: GrpcOpts,

    /// Config file path
    #[clap()]
    pub config_path: std::path::PathBuf,

    /// Read build events from a JSON-lines file instead of the queue-runner.
    /// One `BuildEvent` per line; `#` starts a comment.
    #[clap(long)]
    pub replay: Option<std::path::PathBuf>,

    /// Seconds to wait between replayed events
    #[clap(long, default_value = "1")]
    pub replay_delay_secs: u64,
}

impl Cli {
    #[must_use]
    pub fn new() -> Self {
        Self::parse()
    }
}

impl Default for Cli {
    fn default() -> Self {
        Self::new()
    }
}

impl TrackerCli {
    #[must_use]
    pub fn new() -> Self {
        Self::parse()
    }
}

impl Default for TrackerCli {
    fn default() -> Self {
        Self::new()
    }
}

impl GrpcOpts {
    pub async fn get_authorization_token(&self) -> anyhow::Result<Option<String>> {
        let Some(path) = &self.authorization_file else {
            return Ok(None);
        };

        let content = fs_err::tokio::read_to_string(path).await?;

        // Try parsing as TOML and extracting the "token" field.
        // This allows reusing the same token file format as the queue-runner.
        if let Ok(value) = content.parse::<toml::Value>()
            && let Some(token) = value.get("token").and_then(toml::Value::as_str)
        {
            return Ok(Some(token.to_string()));
        }

        // Fall back: treat entire file content as a plain-text token
        Ok(Some(content.trim().to_string()))
    }

    #[must_use]
    pub const fn mtls_enabled(&self) -> bool {
        self.mtls
    }

    #[must_use]
    pub fn mtls_configured_correctly(&self) -> bool {
        if self.mtls {
            self.server_root_ca_cert_path.is_some()
                && self.client_cert_path.is_some()
                && self.client_key_path.is_some()
                && self.domain_name.is_some()
        } else {
            self.server_root_ca_cert_path.is_none()
                && self.client_cert_path.is_none()
                && self.client_key_path.is_none()
                && self.domain_name.is_none()
        }
    }

    pub async fn get_mtls(
        &self,
    ) -> anyhow::Result<(
        tonic::transport::Certificate,
        tonic::transport::Identity,
        String,
    )> {
        let server_root_ca_cert_path = self
            .server_root_ca_cert_path
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("server_root_ca_cert_path not provided"))?;
        let client_cert_path = self
            .client_cert_path
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("client_cert_path not provided"))?;
        let client_key_path = self
            .client_key_path
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("client_key_path not provided"))?;
        let domain_name = self
            .domain_name
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("domain_name not provided"))?;

        let server_root_ca_cert = fs_err::tokio::read_to_string(server_root_ca_cert_path).await?;
        let server_root_ca_cert = tonic::transport::Certificate::from_pem(server_root_ca_cert);

        let client_cert = fs_err::tokio::read_to_string(client_cert_path).await?;
        let client_key = fs_err::tokio::read_to_string(client_key_path).await?;
        let client_identity = tonic::transport::Identity::from_pem(client_cert, client_key);

        Ok((server_root_ca_cert, client_identity, domain_name.to_owned()))
    }
}
