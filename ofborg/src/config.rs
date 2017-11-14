use serde_json;
use std::fs::File;
use std::path::Path;
use std::io::Read;

use ofborg::acl;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub runner: RunnerConfig,
    pub checkout: CheckoutConfig,
    pub nix: NixConfig,
    pub rabbitmq: RabbitMQConfig,
    pub github: Option<GithubConfig>,
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
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubConfig {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RunnerConfig {
    pub identity: String,
    pub authorized_users: Option<Vec<String>>
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
            self.runner.authorized_users
                .clone()
                .expect("fetching config's runner.authorized_users")
        );
    }
}


impl RabbitMQConfig {
    pub fn as_uri(&self) -> String{
        return format!("{}://{}:{}@{}//",
                       if self.ssl { "amqps" } else { "amqp" },
                       self.username, self.password, self.host);
    }
}

pub fn load(filename: &Path) -> Config {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let deserialized: Config = serde_json::from_str(&contents).unwrap();

    return deserialized;
}
