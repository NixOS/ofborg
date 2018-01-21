use serde_json;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use hubcaps::{Credentials, Github};
use nix::Nix;


use ofborg::acl;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub runner: RunnerConfig,
    pub feedback: Option<FeedbackConfig>,
    pub checkout: CheckoutConfig,
    pub nix: NixConfig,
    pub rabbitmq: RabbitMQConfig,
    pub github: Option<GithubConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeedbackConfig {
    pub full_logs: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RabbitMQConfig {
    pub ssl: bool,
    pub host: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NixConfig {
    pub system: String,
    pub remote: String,
    pub build_timeout_seconds: u16,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubConfig {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RunnerConfig {
    pub identity: String,
    pub authorized_users: Option<Vec<String>>,
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
        return acl::ACL::new(self.runner.authorized_users.clone().expect(
            "fetching config's runner.authorized_users",
        ));
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
        );
    }
}


impl RabbitMQConfig {
    pub fn as_uri(&self) -> String {
        return format!(
            "{}://{}:{}@{}//",
            if self.ssl { "amqps" } else { "amqp" },
            self.username,
            self.password,
            self.host
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
