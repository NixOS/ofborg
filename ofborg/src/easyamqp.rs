
use ofborg;
use ofborg::config::RabbitMQConfig;
use amqp;
use amqp::Basic;

pub struct ConsumeConfig {
    /// Specifies the name of the queue to consume from.
    pub queue: String,

    /// Specifies the identifier for the consumer. The consumer tag is
    /// local to a channel, so two clients can use the same consumer
    /// tags. If this field is empty the server will generate a unique
    /// tag.
    ///
    /// The client MUST NOT specify a tag that refers to an existing
    /// consumer. Error code: not-allowed
    pub consumer_tag: String,

    /// If the no-local field is set the server will not send messages
    /// to the connection that published them.
    pub no_local: bool,

    /// If this field is set the server does not expect
    /// acknowledgements for messages. That is, when a message is
    /// delivered to the client the server assumes the delivery will
    /// succeed and immediately dequeues it. This functionality may
    /// increase performance but at the cost of reliability. Messages
    /// can get lost if a client dies before they are delivered to the
    /// application.
    pub no_ack: bool,

    /// Request exclusive consumer access, meaning only this consumer
    /// can access the queue.
    ///
    /// The client MAY NOT gain exclusive access to a queue that
    /// already has active consumers. Error code: access-refused
    pub exclusive: bool,

    /// If set, the server will not respond to the method. The client
    /// should not wait for a reply method. If the server could not
    /// complete the method it will raise a channel or connection
    /// exception.
    pub no_wait: bool,

    ///  A set of arguments for the consume. The syntax and semantics
    /// of these arguments depends on the server implementation.
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
            config.no_wait,
            config.arguments.unwrap_or(amqp::Table::new()),
        )
    }
}
