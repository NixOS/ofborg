use serde_json;
use std::path::Path;
use std::fs::File;
use std::io::Read;

use ircbot::factoids::Factoids;
use irc::client::prelude::Config as IrcConfig;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    nickname: String,
    alternate_nicknames: Vec<String>,
    password: String,
    channels: Vec<String>,
    pub rabbitmq: RabbitMQConfig,
    factoid_toml: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RabbitMQConfig {
    pub ssl: bool,
    pub host: String,
    pub vhost: String,
    pub username: String,
    pub password: String,
}


impl RabbitMQConfig {
    pub fn as_uri(&self) -> String{
        return format!("{}://{}:{}@{}/{}",
                       if self.ssl { "amqps" } else { "amqp" },
                       self.username, self.password, self.host,
                       self.vhost
        );
    }
}

impl Config {
    pub fn factoids(&self) -> Factoids {
        Factoids::load(&Path::new(&self.factoid_toml))
    }

    pub fn irc_config(&self) -> IrcConfig {
        IrcConfig {
            nickname: Some(self.nickname.clone()),
            nick_password: Some(self.password.clone()),
            server: Some("chat.freenode.net".to_owned()),
            port: Some(6697),
            use_ssl: Some(true),
            should_ghost: Some(true),

            user_info: Some("a bot by Graham".to_owned()),
            source: Some("https://github.com/grahamc/ofborg".to_owned()),
            version: Some("lolidunno".to_owned()),

            username: Some("graham".to_owned()),
            realname: Some("Graham Christensen".to_owned()),
            channels: Some(self.channels.clone()),

            alt_nicks: Some(self.alternate_nicknames.clone()),
            cert_path: None,
            channel_keys: None,
            encoding: None,
            ghost_sequence: Some(vec!["RECOVER".to_owned(), "RELEASE".to_owned()]),

            max_messages_in_burst: Some(5),
            burst_window_length: Some(5),

            ping_time: Some(180),
            ping_timeout: Some(10),

            use_mock_connection: Some(false),
            mock_initial_value: None,

            options: None,
            owners: None,
            password: Some("".to_owned()),
            umodes: None,


        }
    }
}

pub fn load(src: &Path) -> Config {
    let mut file = File::open(src).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    return serde_json::from_str(&contents).unwrap();
}
