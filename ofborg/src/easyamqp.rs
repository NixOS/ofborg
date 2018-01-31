
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

pub enum ExchangeType {
    Topic,
    Headers,
    Fanout,
    Direct,
    Custom(String),
}

impl Into<String> for ExchangeType {
    fn into(self) -> String {
        match self {
            ExchangeType::Topic => "topic".to_owned(),
            ExchangeType::Headers => "headers".to_owned(),
            ExchangeType::Fanout => "fanout".to_owned(),
            ExchangeType::Direct => "direct".to_owned(),
            ExchangeType::Custom(x) => x,
        }
    }
}

pub struct ExchangeConfig {
    /// Exchange names starting with "amq." are reserved for
    /// pre-declared and standardised exchanges. The client MAY
    /// declare an exchange starting with "amq." if the passive option
    /// is set, or the exchange already exists. Error code:
    /// access-refused
    ///
    /// The exchange name consists of a non-empty sequence of these
    /// characters: letters, digits, hyphen, underscore, period, or
    /// colon. Error code: precondition-failed
    exchange: String,

    /// Each exchange belongs to one of a set of exchange types
    /// implemented by the server. The exchange types define the
    /// functionality of the exchange - i.e. how messages are routed
    /// through it. It is not valid or meaningful to attempt to change
    /// the type of an existing exchange.
    ///
    /// Exchanges cannot be redeclared with different types. The
    /// client MUST not attempt to redeclare an existing exchange with
    /// a different type than used in the original Exchange.Declare
    /// method. Error code: not-allowed
    ///
    /// The client MUST NOT attempt to declare an exchange with a type
    /// that the server does not support. Error code: command-invalid
    exchange_type: ExchangeType,

    /// If set, the server will reply with Declare-Ok if the exchange
    /// already exists with the same name, and raise an error if not.
    /// The client can use this to check whether an exchange exists
    /// without modifying the server state. When set, all other method
    /// fields except name and no-wait are ignored. A declare with
    /// both passive and no-wait has no effect. Arguments are compared
    /// for semantic equivalence.
    ///
    /// If set, and the exchange does not already exist, the server
    /// MUST raise a channel exception with reply code 404 (not
    /// found).
    ///
    /// If not set and the exchange exists, the server MUST check that
    /// the existing exchange has the same values for type, durable,
    /// and arguments fields. The server MUST respond with Declare-Ok
    /// if the requested exchange matches these fields, and MUST raise
    /// a channel exception if not.
    passive: bool,

    /// If set when creating a new exchange, the exchange will be
    /// marked as durable. Durable exchanges remain active when a
    /// server restarts. Non-durable exchanges (transient exchanges)
    /// are purged if/when a server restarts.
    ///
    /// The server MUST support both durable and transient exchanges.
    durable: bool,

    /// If set, the exchange is deleted when all queues have finished
    /// using it.
    ///
    /// The server SHOULD allow for a reasonable delay between the
    /// point when it determines that an exchange is not being used
    /// (or no longer used), and the point when it deletes the
    /// exchange. At the least it must allow a client to create an
    /// exchange and then bind a queue to it, with a small but
    /// non-zero delay between these two actions.
    ///
    /// The server MUST ignore the auto-delete field if the exchange
    /// already exists.
    auto_delete: bool,

    /// If set, the exchange may not be used directly by publishers,
    /// but only when bound to other exchanges. Internal exchanges are
    /// used to construct wiring that is not visible to applications.
    internal: bool,

    /// If set, the server will not respond to the method. The client
    /// should not wait for a reply method. If the server could not
    /// complete the method it will raise a channel or connection
    /// exception.
    nowait: bool,

    /// A set of arguments for the declaration. The syntax and
    /// semantics of these arguments depends on the server
    /// implementation.
    arguments: Option<amqp::Table>,
}

pub fn session_from_config(config: &RabbitMQConfig) -> Result<amqp::Session, amqp::AMQPError> {
    let scheme = if config.ssl {
        amqp::AMQPScheme::AMQPS
    } else {
        amqp::AMQPScheme::AMQP
    };

    let mut properties = amqp::Table::new();
    properties.insert(
        "ofborg_version".to_owned(),
        amqp::TableEntry::LongString(ofborg::VERSION.to_owned()),
    );

    let options = amqp::Options {
        host: config.host.clone(),
        port: match scheme {
            amqp::AMQPScheme::AMQPS => 5671,
            amqp::AMQPScheme::AMQP => 5672,
        },
        vhost: "/".to_owned(),
        login: config.username.clone(),
        password: config.password.clone(),
        scheme: scheme,
        properties: properties,
        ..amqp::Options::default()
    };

    let session = try!(amqp::Session::new(options));

    info!("Connected to {}", &config.host);
    return Ok(session);
}

pub trait TypedWrappers {
    fn consume<T>(&mut self, callback: T, config: ConsumeConfig) -> Result<String, amqp::AMQPError>
    where
        T: amqp::Consumer + 'static;

    fn declare_exchange<T>(
        &mut self,
        config: ExchangeConfig,
    ) -> Result<amqp::protocol::exchange::DeclareOk, amqp::AMQPError>
    where
        T: amqp::Consumer + 'static;
}

impl TypedWrappers for amqp::Channel {
    fn consume<T>(&mut self, callback: T, config: ConsumeConfig) -> Result<String, amqp::AMQPError>
    where
        T: amqp::Consumer + 'static,
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

    fn declare_exchange<T>(
        &mut self,
        config: ExchangeConfig,
    ) -> Result<amqp::protocol::exchange::DeclareOk, amqp::AMQPError>
    where
        T: amqp::Consumer + 'static,
    {
        self.exchange_declare(
            config.exchange,
            config.exchange_type.into(),
            config.passive,
            config.durable,
            config.auto_delete,
            config.internal,
            config.nowait,
            config.arguments.unwrap_or(amqp::Table::new()),
        )
    }
}
