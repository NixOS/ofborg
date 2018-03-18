use serde_json;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use hubcaps::{Credentials, Github};
use nix::Nix;
use std::collections::HashMap;


use ofborg::acl;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub runner: RunnerConfig,
    pub feedback: FeedbackConfig,
    pub checkout: CheckoutConfig,
    pub nix: NixConfig,
    pub rabbitmq: RabbitMQConfig,
    pub github: Option<GithubConfig>,
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
    pub initial_heap_size: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubConfig {
    pub token: String,
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
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckoutConfig {
    pub root: String,
}

impl Config {
    pub fn whoami(&self) -> String {
        return format!("{}-{}", self.runner.identity, self.nix.system);
    }

    pub fn acl(&self) -> acl::ACL {
        return acl::ACL::new(
            self.runner.repos.clone().expect(
                "fetching config's runner.repos",
            ),
            self.runner.trusted_users.clone().expect(
                "fetching config's runner.trusted_users",
            ),
            self.runner.known_users.clone().expect(
                "fetching config's runner.known_users",
            ),
        );
    }

    pub fn github(&self) -> Github {
        Github::new(
            "github.com/grahamc/ofborg",
            // tls configured hyper client
            Client::with_connector(HttpsConnector::new(NativeTlsClient::new().unwrap())),
            Credentials::Token(self.github.clone().unwrap().token),
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

        return Nix::new(
            self.nix.system.clone(),
            self.nix.remote.clone(),
            self.nix.build_timeout_seconds,
            self.nix.initial_heap_size.clone(),
        );
    }
}

impl RabbitMQConfig {
    pub fn as_uri(&self) -> String {
        return format!(
            "{}://{}:{}@{}/{}",
            if self.ssl { "amqps" } else { "amqp" },
            self.username,
            self.password,
            self.host,
            self.virtualhost.clone().unwrap_or("/".to_owned()),
        );
    }
}

pub fn load(filename: &Path) -> Config {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let deserialized: Config = serde_json::from_str(&contents).unwrap();

    return deserialized;
}
