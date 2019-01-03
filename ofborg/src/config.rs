use hubcaps::{Credentials, Github, InstallationTokenGenerator, JWTCredentials};
use hyper::net::HttpsConnector;
use hyper::Client;
use hyper_native_tls::NativeTlsClient;
use nix::Nix;
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use ofborg::acl;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub runner: RunnerConfig,
    pub feedback: FeedbackConfig,
    pub checkout: CheckoutConfig,
    pub nix: NixConfig,
    pub rabbitmq: RabbitMQConfig,
    pub github: Option<GithubConfig>,
    pub github_app: Option<GithubAppConfig>,
    pub log_storage: Option<LogStorage>,
    pub tag_paths: Option<HashMap<String, Vec<String>>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeedbackConfig {
    pub full_logs: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RabbitMQConfig {
    pub ssl: bool,
    pub host: String,
    pub virtualhost: Option<String>,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NixConfig {
    pub system: String,
    pub remote: String,
    pub build_timeout_seconds: u16,
    pub initial_heap_size: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubConfig {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubAppConfig {
    pub app_id: i32,
    pub installation_id: i32,
    pub private_key: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogStorage {
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RunnerConfig {
    pub identity: String,
    pub repos: Option<Vec<String>>,
    pub trusted_users: Option<Vec<String>>,
    pub known_users: Option<Vec<String>>,

    /// If true, will create its own queue attached to the build job
    /// exchange. This means that builders with this enabled will
    /// trigger duplicate replies to the request for this
    /// architecture.
    ///
    /// This should only be turned on for development.
    pub build_all_jobs: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckoutConfig {
    pub root: String,
}

impl Config {
    pub fn whoami(&self) -> String {
        format!("{}-{}", self.runner.identity, self.nix.system)
    }

    pub fn acl(&self) -> acl::ACL {
        acl::ACL::new(
            self.runner
                .repos
                .clone()
                .expect("fetching config's runner.repos"),
            self.runner
                .trusted_users
                .clone()
                .expect("fetching config's runner.trusted_users"),
            self.runner
                .known_users
                .clone()
                .expect("fetching config's runner.known_users"),
        )
    }

    pub fn github(&self) -> Github {
        Github::new(
            "github.com/grahamc/ofborg",
            // tls configured hyper client
            Client::with_connector(HttpsConnector::new(NativeTlsClient::new().unwrap())),
            Credentials::Token(self.github.clone().unwrap().token),
        )
    }

    pub fn github_app(&self) -> Github {
        let conf = self.github_app.clone().unwrap();
        Github::new(
            "github.com/grahamc/ofborg (app)",
            // tls configured hyper client
            Client::with_connector(HttpsConnector::new(NativeTlsClient::new().unwrap())),
            Credentials::InstallationToken(InstallationTokenGenerator::new(
                conf.installation_id,
                JWTCredentials::new(conf.app_id, conf.private_key),
            )),
        )
    }

    pub fn nix(&self) -> Nix {
        if self.nix.build_timeout_seconds < 1200 {
            error!(
                "Note: {} is way too low for build_timeout_seconds!",
                self.nix.build_timeout_seconds
            );
            error!("Please set build_timeout_seconds to at least 1200");
            panic!();
        }

        Nix::new(
            self.nix.system.clone(),
            self.nix.remote.clone(),
            self.nix.build_timeout_seconds,
            self.nix.initial_heap_size.clone(),
        )
    }
}

impl RabbitMQConfig {
    pub fn as_uri(&self) -> String {
        format!(
            "{}://{}:{}@{}/{}",
            if self.ssl { "amqps" } else { "amqp" },
            self.username,
            self.password,
            self.host,
            self.virtualhost.clone().unwrap_or_else(|| "/".to_owned()),
        )
    }
}

pub fn load(filename: &Path) -> Config {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let deserialized: Config = serde_json::from_str(&contents).unwrap();

    deserialized
}
