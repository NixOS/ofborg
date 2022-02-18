use crate::acl;
use crate::nix::Nix;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use hubcaps::{Credentials, Github, InstallationTokenGenerator, JWTCredentials};
use hyper::net::HttpsConnector;
use hyper::Client;
use hyper_native_tls::NativeTlsClient;
use tracing::{debug, error, info, warn};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub runner: RunnerConfig,
    pub feedback: FeedbackConfig,
    pub checkout: CheckoutConfig,
    pub nix: NixConfig,
    pub rabbitmq: RabbitMqConfig,
    pub github: Option<GithubConfig>,
    pub github_app: Option<GithubAppConfig>,
    pub log_storage: Option<LogStorage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeedbackConfig {
    pub full_logs: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RabbitMqConfig {
    pub ssl: bool,
    pub host: String,
    pub virtualhost: Option<String>,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NixConfig {
    pub system: String,
    pub additional_build_systems: Option<Vec<String>>,
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
    #[serde(default = "Default::default")]
    pub disable_trusted_users: bool,
    pub trusted_users: Option<Vec<String>>,

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
        Github::new(
            "github.com/grahamc/ofborg",
            // tls configured hyper client
            Client::with_connector(HttpsConnector::new(NativeTlsClient::new().unwrap())),
            Credentials::Token(self.github.clone().unwrap().token),
        )
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
            self.nix.system.clone(),
            self.nix.remote.clone(),
            self.nix.build_timeout_seconds,
            self.nix.initial_heap_size.clone(),
        )
    }
}

impl RabbitMqConfig {
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

pub struct GithubAppVendingMachine {
    conf: GithubAppConfig,
    id_cache: HashMap<(String, String), Option<i32>>,
    client_cache: HashMap<i32, Github>,
}

impl GithubAppVendingMachine {
    fn useragent(&self) -> &'static str {
        "github.com/grahamc/ofborg (app)"
    }

    fn jwt(&self) -> JWTCredentials {
        JWTCredentials::new(self.conf.app_id, self.conf.private_key.clone())
    }

    fn install_id_for_repo(&mut self, owner: &str, repo: &str) -> Option<i32> {
        let useragent = self.useragent();
        let jwt = self.jwt();

        let key = (owner.to_owned(), repo.to_owned());

        *self.id_cache.entry(key).or_insert_with(|| {
            info!("Looking up install ID for {}/{}", owner, repo);

            let lookup_gh = Github::new(
                useragent,
                Client::with_connector(HttpsConnector::new(NativeTlsClient::new().unwrap())),
                Credentials::JWT(jwt),
            );

            match lookup_gh.app().find_repo_installation(owner, repo) {
                Ok(install_id) => {
                    debug!("Received install ID {:?}", install_id);
                    Some(install_id.id)
                }
                Err(e) => {
                    warn!("Error during install ID lookup: {:?}", e);
                    None
                }
            }
        })
    }

    pub fn for_repo<'a>(&'a mut self, owner: &str, repo: &str) -> Option<&'a Github> {
        let useragent = self.useragent();
        let jwt = self.jwt();
        let install_id = self.install_id_for_repo(owner, repo)?;

        Some(self.client_cache.entry(install_id).or_insert_with(|| {
            Github::new(
                useragent,
                Client::with_connector(HttpsConnector::new(NativeTlsClient::new().unwrap())),
                Credentials::InstallationToken(InstallationTokenGenerator::new(install_id, jwt)),
            )
        }))
    }
}
