
use ofborg;
use ofborg::config::RabbitMQConfig;
use amqp;
use amqp::Basic;

pub struct ConsumeConfig {
    pub queue: String,
    pub consumer_tag: String,
    pub no_local: bool,
    pub no_ack: bool,
    pub exclusive: bool,
    pub nowait: bool,
    pub arguments: Option<amqp::Table>,
}

pub fn session_from_config(config: &RabbitMQConfig)
                           -> Result<amqp::Session, amqp::AMQPError> {
    let scheme = if config.ssl {
        amqp::AMQPScheme::AMQPS
    } else {
        amqp::AMQPScheme::AMQP
    };

    let mut properties = amqp::Table::new();
    //  properties.insert("identity".to_owned(), amqp::TableEntry::LongString(identity.to_owned()));
    properties.insert(
        "ofborg_version".to_owned(),
        amqp::TableEntry::LongString(ofborg::VERSION.to_owned())
    );

    amqp::Session::new(
        amqp::Options{
            host: config.host.clone(),
            login: config.username.clone(),
            password: config.password.clone(),
            scheme: scheme,
            properties: properties,
            .. amqp::Options::default()
        }
    )
}

pub trait TypedWrappers {
    fn consume<T>(&mut self, callback: T, config: ConsumeConfig)
                  -> Result<String, amqp::AMQPError>
        where T: amqp::Consumer + 'static;
}

impl TypedWrappers for amqp::Channel {
    fn consume<T>(&mut self, callback: T, config: ConsumeConfig)
                  -> Result<String, amqp::AMQPError>
        where T: amqp::Consumer + 'static
    {
        self.basic_consume(
            callback,
            config.queue,
            config.consumer_tag,
            config.no_local,
            config.no_ack,
            config.exclusive,
            config.nowait,
            config.arguments.unwrap_or(amqp::Table::new()),
        )
    }
}
