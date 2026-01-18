use crate::acl;
use crate::nix::Nix;

use std::collections::{HashMap, hash_map::Entry};
use std::fmt;
use std::fs::File;
use std::io::{BufReader, Read};
use std::marker::PhantomData;
use std::path::{Path, PathBuf};

use hubcaps::{Credentials, Github, InstallationTokenGenerator, JWTCredentials};
use serde::de::{self, Deserializer};
use tracing::{debug, error, info, warn};

/// Main ofBorg configuration
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Config {
    /// Configuration for the webhook receiver
    pub github_webhook_receiver: Option<GithubWebhookConfig>,
    /// Configuration for the logapi receiver
    pub log_api_config: Option<LogApiConfig>,
    /// Configuration for the evaluation filter
    pub evaluation_filter: Option<EvaluationFilter>,
    /// Configuration for the GitHub comment filter
    pub github_comment_filter: Option<GithubCommentFilter>,
    /// Configuration for the GitHub comment poster
    pub github_comment_poster: Option<GithubCommentPoster>,
    /// Configuration for the mass rebuilder
    pub mass_rebuilder: Option<MassRebuilder>,
    /// Configuration for the builder
    pub builder: Option<Builder>,
    /// Configuration for the log message collector
    pub log_message_collector: Option<LogMessageCollector>,
    /// Configuration for the stats server
    pub stats: Option<Stats>,
    pub runner: RunnerConfig,
    pub checkout: CheckoutConfig,
    pub nix: NixConfig,
    pub github_app: Option<GithubAppConfig>,
}

/// Configuration for the webhook receiver
#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GithubWebhookConfig {
    /// Listen host/port
    pub listen: String,
    /// Path to the GitHub webhook secret
    pub webhook_secret_file: String,
    /// RabbitMQ broker to connect to
    pub rabbitmq: RabbitMqConfig,
}

fn default_logs_path() -> String {
    "/var/log/ofborg".into()
}

fn default_serve_root() -> String {
    "https://logs.ofborg.org/logfile".into()
}

/// Configuration for logapi
#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct LogApiConfig {
    /// Listen host/port
    pub listen: String,
    #[serde(default = "default_logs_path")]
    pub logs_path: String,
    #[serde(default = "default_serve_root")]
    pub serve_root: String,
}

/// Configuration for the evaluation filter
#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct EvaluationFilter {
    /// RabbitMQ broker to connect to
    pub rabbitmq: RabbitMqConfig,
}

/// Configuration for the GitHub comment filter
#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GithubCommentFilter {
    /// RabbitMQ broker to connect to
    pub rabbitmq: RabbitMqConfig,
}

/// Configuration for the GitHub comment poster
#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GithubCommentPoster {
    /// RabbitMQ broker to connect to
    pub rabbitmq: RabbitMqConfig,
}

/// Configuration for the mass rebuilder
#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct MassRebuilder {
    /// RabbitMQ broker to connect to
    pub rabbitmq: RabbitMqConfig,
}

/// Configuration for the builder
#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Builder {
    /// RabbitMQ broker to connect to
    pub rabbitmq: RabbitMqConfig,
}

/// Configuration for the log message collector
#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct LogMessageCollector {
    /// RabbitMQ broker to connect to
    pub rabbitmq: RabbitMqConfig,
    /// Path where the logs reside
    pub logs_path: String,
}

/// Configuration for the stats exporter
#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Stats {
    /// RabbitMQ broker to connect to
    pub rabbitmq: RabbitMqConfig,
}

/// Configures the connection to a RabbitMQ instance
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct RabbitMqConfig {
    /// Whether or not to use SSL
    pub ssl: bool,
    /// Hostname to conenct to
    pub host: String,
    /// Virtual host to use (defaults to /)
    pub virtualhost: Option<String>,
    /// Username to connect with
    pub username: String,
    /// File to read the user password from. Contents are automatically stripped
    pub password_file: PathBuf,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct NixConfig {
    #[serde(deserialize_with = "deserialize_one_or_many")]
    pub system: Vec<String>,
    pub remote: String,
    pub build_timeout_seconds: u16,
    pub initial_heap_size: Option<String>,
    /// CPU cores for package listing
    pub list_cores: Option<u64>,
    /// Chunk size for package listing
    pub list_chunk_size: Option<u64>,
    /// System to evaluate when calculating package diff
    pub list_system: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct GithubAppConfig {
    pub app_id: u64,
    pub private_key: PathBuf,
    pub oauth_client_id: String,
    pub oauth_client_secret_file: PathBuf,
}

const fn default_instance() -> u8 {
    1
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct RunnerConfig {
    #[serde(default = "default_instance")]
    pub instance: u8,
    pub identity: String,
    /// List of GitHub repos we feel responsible for
    pub repos: Option<Vec<String>>,
    /// Whether to use the `trusted_users` field or just allow everyone
    #[serde(default = "Default::default")]
    pub disable_trusted_users: bool,
    /// List of users who are allowed to build on less sandboxed platforms
    pub trusted_users: Option<Vec<String>>,

    /// If true, will create its own queue attached to the build job
    /// exchange. This means that builders with this enabled will
    /// trigger duplicate replies to the request for this
    /// architecture.
    ///
    /// This should only be turned on for development.
    pub build_all_jobs: Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct CheckoutConfig {
    pub root: String,
}

impl Config {
    pub fn whoami(&self) -> String {
        format!("{}-{}", self.runner.identity, self.nix.system.join(","))
    }

    pub fn acl(&self) -> acl::Acl {
        let repos = self
            .runner
            .repos
            .clone()
            .expect("fetching config's runner.repos");

        let trusted_users = if self.runner.disable_trusted_users {
            None
        } else {
            Some(
                self.runner
                    .trusted_users
                    .clone()
                    .expect("fetching config's runner.trusted_users"),
            )
        };

        acl::Acl::new(repos, trusted_users)
    }

    pub fn github(&self) -> Github {
        let token = std::fs::read_to_string(
            self.github_app
                .clone()
                .expect("No GitHub app configured")
                .oauth_client_secret_file,
        )
        .expect("Couldn't read from GitHub app token");
        let token = token.trim();
        Github::new(
            "github.com/NixOS/ofborg",
            Credentials::Client(
                self.github_app
                    .clone()
                    .expect("No GitHub app configured")
                    .oauth_client_id,
                token.to_owned(),
            ),
        )
        .expect("Unable to create a github client instance")
    }

    pub fn github_app_vendingmachine(&self) -> GithubAppVendingMachine {
        GithubAppVendingMachine {
            conf: self.github_app.clone().unwrap(),
            id_cache: HashMap::new(),
            client_cache: HashMap::new(),
        }
    }

    pub fn nix(&self) -> Nix {
        if self.nix.build_timeout_seconds < 1200 {
            error!(?self.nix.build_timeout_seconds, "Please set build_timeout_seconds to at least 1200");
            panic!();
        }

        Nix::new(
            self.nix
                .system
                .first()
                .expect("expected at least one system")
                .clone(),
            self.nix.remote.clone(),
            self.nix.build_timeout_seconds,
            self.nix.initial_heap_size.clone(),
        )
    }
}

impl RabbitMqConfig {
    pub fn as_uri(&self) -> Result<String, std::io::Error> {
        let password = std::fs::read_to_string(&self.password_file).inspect_err(|_| {
            error!(
                "Unable to read RabbitMQ password file at {:?}",
                self.password_file
            );
        })?;
        let uri = format!(
            "{}://{}:{}@{}/{}",
            if self.ssl { "amqps" } else { "amqp" },
            self.username,
            password,
            self.host,
            self.virtualhost.clone().unwrap_or_else(|| "/".to_owned()),
        );
        Ok(uri)
    }
}

pub fn load(filename: &Path) -> Config {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let deserialized: Config = serde_json::from_str(&contents).unwrap();

    deserialized
}

pub struct GithubAppVendingMachine {
    conf: GithubAppConfig,
    id_cache: HashMap<(String, String), Option<u64>>,
    client_cache: HashMap<u64, Github>,
}

impl GithubAppVendingMachine {
    fn useragent(&self) -> &'static str {
        "github.com/NixOS/ofborg (app)"
    }

    fn jwt(&self) -> JWTCredentials {
        let private_key_file =
            File::open(self.conf.private_key.clone()).expect("Unable to read private_key");
        let mut private_key_reader = BufReader::new(private_key_file);
        let private_keys = rustls_pemfile::rsa_private_keys(&mut private_key_reader)
            .collect::<Result<Vec<_>, _>>()
            .expect("Unable to convert private_key to DER format");
        // We can be reasonably certain that there will only be one private key in this file
        let private_key = &private_keys[0];
        JWTCredentials::new(self.conf.app_id, private_key.secret_pkcs1_der().to_vec())
            .expect("Unable to create JWTCredentials")
    }

    async fn install_id_for_repo(&mut self, owner: &str, repo: &str) -> Option<u64> {
        let useragent = self.useragent();
        let jwt = self.jwt();

        let key = (owner.to_owned(), repo.to_owned());

        match self.id_cache.entry(key) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => {
                info!("Looking up install ID for {}/{}", owner, repo);

                let lookup_gh = Github::new(useragent, Credentials::JWT(jwt)).unwrap();

                let v = match lookup_gh.app().find_repo_installation(owner, repo).await {
                    Ok(install_id) => {
                        debug!("Received install ID {:?}", install_id);
                        Some(install_id.id)
                    }
                    Err(e) => {
                        warn!("Error during install ID lookup: {:?}", e);
                        None
                    }
                };
                *entry.insert(v)
            }
        }
    }

    pub async fn for_repo<'a>(&'a mut self, owner: &str, repo: &str) -> Option<&'a Github> {
        let useragent = self.useragent();
        let jwt = self.jwt();
        let install_id = self.install_id_for_repo(owner, repo).await?;

        Some(self.client_cache.entry(install_id).or_insert_with(|| {
            Github::new(
                useragent,
                Credentials::InstallationToken(InstallationTokenGenerator::new(install_id, jwt)),
            )
            .expect("Unable to create a github client instance")
        }))
    }
}

// Copied from https://stackoverflow.com/a/43627388
fn deserialize_one_or_many<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringOrVec(PhantomData<Vec<String>>);

    impl<'de> de::Visitor<'de> for StringOrVec {
        type Value = Vec<String>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or list of strings")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(vec![value.to_owned()])
        }

        fn visit_seq<S>(self, visitor: S) -> Result<Self::Value, S::Error>
        where
            S: de::SeqAccess<'de>,
        {
            serde::de::Deserialize::deserialize(de::value::SeqAccessDeserializer::new(visitor))
        }
    }

    deserializer.deserialize_any(StringOrVec(PhantomData))
}
