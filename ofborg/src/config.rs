use crate::acl;
use crate::nix::Nix;

use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{BufReader, Read};
use std::marker::PhantomData;
use std::path::{Path, PathBuf};

use hubcaps::{Credentials, Github, InstallationTokenGenerator, JWTCredentials};
use serde::de::{self, Deserialize, Deserializer};
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
    pub password_file: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NixConfig {
    #[serde(deserialize_with = "deserialize_one_or_many")]
    pub system: Vec<String>,
    pub remote: String,
    pub build_timeout_seconds: u16,
    pub initial_heap_size: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubConfig {
    pub token_file: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubAppConfig {
    pub app_id: u64,
    pub installation_id: u64,
    pub private_key: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogStorage {
    pub path: String,
}

const fn default_instance() -> u8 {
    1
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RunnerConfig {
    #[serde(default = "default_instance")]
    pub instance: u8,
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
        let token = std::fs::read_to_string(self.github.clone().unwrap().token_file)
            .expect("Couldn't read from GitHub token file");
        Github::new(
            "github.com/grahamc/ofborg",
            // tls configured hyper client
            Credentials::Token(token),
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
        let password = std::fs::read_to_string(&self.password_file)?;
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
        "github.com/grahamc/ofborg (app)"
    }

    fn jwt(&self) -> JWTCredentials {
        let private_key_file =
            File::open(self.conf.private_key.clone()).expect("Unable to read private_key");
        let mut private_key_reader = BufReader::new(private_key_file);
        let private_keys = rustls_pemfile::rsa_private_keys(&mut private_key_reader)
            .expect("Unable to convert private_key to DER format");
        // We can be reasonably certain that there will only be one private key in this file
        let private_key = &private_keys[0];
        JWTCredentials::new(self.conf.app_id, private_key.to_vec())
            .expect("Unable to create JWTCredentials")
    }

    fn install_id_for_repo(&mut self, owner: &str, repo: &str) -> Option<u64> {
        let useragent = self.useragent();
        let jwt = self.jwt();

        let key = (owner.to_owned(), repo.to_owned());

        *self.id_cache.entry(key).or_insert_with(|| {
            info!("Looking up install ID for {}/{}", owner, repo);

            let lookup_gh = Github::new(useragent, Credentials::JWT(jwt)).unwrap();

            match async_std::task::block_on(lookup_gh.app().find_repo_installation(owner, repo)) {
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
            Deserialize::deserialize(de::value::SeqAccessDeserializer::new(visitor))
        }
    }

    deserializer.deserialize_any(StringOrVec(PhantomData))
}
