use crate::config::RabbitMQConfig;
use crate::ofborg;
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

pub struct BindQueueConfig {
    /// Specifies the name of the queue to bind.
    ///
    /// The client MUST either specify a queue name or have previously
    /// declared a queue on the same channel Error code: not-found
    ///
    /// The client MUST NOT attempt to bind a queue that does not
    /// exist. Error code: not-found
    pub queue: String,

    /// Name of the exchange to bind to.
    ///
    /// A client MUST NOT be allowed to bind a queue to a non-existent
    /// exchange. Error code: not-found
    ///
    /// The server MUST accept a blank exchange name to mean the
    /// default exchange.
    pub exchange: String,

    /// Specifies the routing key for the binding. The routing key is
    /// used for routing messages depending on the exchange
    /// configuration. Not all exchanges use a routing key - refer to
    /// the specific exchange documentation. If the queue name is
    /// empty, the server uses the last queue declared on the channel.
    /// If the routing key is also empty, the server uses this queue
    /// name for the routing key as well. If the queue name is
    /// provided but the routing key is empty, the server does the
    /// binding with that empty routing key. The meaning of empty
    /// routing keys depends on the exchange implementation.
    ///
    /// If a message queue binds to a direct exchange using routing
    /// key K and a publisher sends the exchange a message with
    /// routing key R, then the message MUST be passed to the message
    /// queue if K = R.
    pub routing_key: Option<String>,

    /// If set, the server will not respond to the method. The client
    /// should not wait for a reply method. If the server could not
    /// complete the method it will raise a channel or connection
    /// exception.
    pub no_wait: bool,

    ///  A set of arguments for the binding. The syntax and semantics
    ///  of these arguments depends on the exchange class.
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
    pub exchange: String,

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
    pub exchange_type: ExchangeType,

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
    pub passive: bool,

    /// If set when creating a new exchange, the exchange will be
    /// marked as durable. Durable exchanges remain active when a
    /// server restarts. Non-durable exchanges (transient exchanges)
    /// are purged if/when a server restarts.
    ///
    /// The server MUST support both durable and transient exchanges.
    pub durable: bool,

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
    pub auto_delete: bool,

    /// If set, the exchange may not be used directly by publishers,
    /// but only when bound to other exchanges. Internal exchanges are
    /// used to construct wiring that is not visible to applications.
    pub internal: bool,

    /// If set, the server will not respond to the method. The client
    /// should not wait for a reply method. If the server could not
    /// complete the method it will raise a channel or connection
    /// exception.
    pub no_wait: bool,

    /// A set of arguments for the declaration. The syntax and
    /// semantics of these arguments depends on the server
    /// implementation.
    pub arguments: Option<amqp::Table>,
}

pub struct QueueConfig {
    /// The queue name MAY be empty, in which case the server MUST
    /// create a new queue with a unique generated name and return
    /// this to the client in the Declare-Ok method.
    ///
    /// Queue names starting with "amq." are reserved for pre-declared
    /// and standardised queues. The client MAY declare a queue
    /// starting with "amq." if the passive option is set, or the
    /// queue already exists. Error code: access-refused
    ///
    /// The queue name can be empty, or a sequence of these
    /// characters: letters, digits, hyphen, underscore, period, or
    /// colon. Error code: precondition-failed
    pub queue: String,

    ///  If set, the server will reply with Declare-Ok if the queue
    ///  already exists with the same name, and raise an error if not.
    ///  The client can use this to check whether a queue exists
    ///  without modifying the server state. When set, all other
    ///  method fields except name and no-wait are ignored. A declare
    ///  with both passive and no-wait has no effect. Arguments are
    ///  compared for semantic equivalence.
    ///
    /// The client MAY ask the server to assert that a queue exists
    /// without creating the queue if not. If the queue does not
    /// exist, the server treats this as a failure. Error code:
    /// not-found
    ///
    /// If not set and the queue exists, the server MUST check that
    /// the existing queue has the same values for durable, exclusive,
    /// auto-delete, and arguments fields. The server MUST respond
    /// with Declare-Ok if the requested queue matches these fields,
    /// and MUST raise a channel exception if not.
    pub passive: bool,

    /// If set when creating a new queue, the queue will be marked as
    /// durable. Durable queues remain active when a server restarts.
    /// Non-durable queues (transient queues) are purged if/when a
    /// server restarts. Note that durable queues do not necessarily
    /// hold persistent messages, although it does not make sense to
    /// send persistent messages to a transient queue.
    ///
    /// The server MUST recreate the durable queue after a restart.
    ///
    /// The server MUST support both durable and transient queues.
    pub durable: bool,

    /// Exclusive queues may only be accessed by the current
    /// connection, and are deleted when that connection closes.
    /// Passive declaration of an exclusive queue by other connections
    /// are not allowed.
    ///
    /// The server MUST support both exclusive (private) and
    /// non-exclusive (shared) queues.
    /// The client MAY NOT attempt to use a queue that was declared as
    /// exclusive by another still-open connection. Error code:
    /// resource-locked
    pub exclusive: bool,

    /// If set, the queue is deleted when all consumers have finished
    /// using it. The last consumer can be cancelled either explicitly
    /// or because its channel is closed. If there was no consumer
    /// ever on the queue, it won't be deleted. Applications can
    /// explicitly delete auto-delete queues using the Delete method
    /// as normal.
    ///
    /// The server MUST ignore the auto-delete field if the queue
    /// already exists.
    pub auto_delete: bool,

    /// If set, the server will not respond to the method. The client
    /// should not wait for a reply method. If the server could not
    /// complete the method it will raise a channel or connection
    /// exception.
    pub no_wait: bool,

    /// A set of arguments for the declaration. The syntax and
    /// semantics of these arguments depends on the server
    /// implementation.
    pub arguments: Option<amqp::Table>,
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
        vhost: config.virtualhost.clone().unwrap_or_else(|| "/".to_owned()),
        login: config.username.clone(),
        password: config.password.clone(),
        scheme,
        properties,
        ..amqp::Options::default()
    };

    let session = r#try!(amqp::Session::new(options));

    info!("Connected to {}", &config.host);
    Ok(session)
}

pub trait TypedWrappers {
    fn consume<T>(&mut self, callback: T, config: ConsumeConfig) -> Result<String, amqp::AMQPError>
    where
        T: amqp::Consumer + 'static;

    fn declare_exchange(
        &mut self,
        config: ExchangeConfig,
    ) -> Result<amqp::protocol::exchange::DeclareOk, amqp::AMQPError>;

    fn declare_queue(
        &mut self,
        config: QueueConfig,
    ) -> Result<amqp::protocol::queue::DeclareOk, amqp::AMQPError>;

    fn bind_queue(
        &mut self,
        config: BindQueueConfig,
    ) -> Result<amqp::protocol::queue::BindOk, amqp::AMQPError>;
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
            config.arguments.unwrap_or_else(amqp::Table::new),
        )
    }

    fn declare_exchange(
        &mut self,
        config: ExchangeConfig,
    ) -> Result<amqp::protocol::exchange::DeclareOk, amqp::AMQPError> {
        self.exchange_declare(
            config.exchange,
            config.exchange_type.into(),
            config.passive,
            config.durable,
            config.auto_delete,
            config.internal,
            config.no_wait,
            config.arguments.unwrap_or_else(amqp::Table::new),
        )
    }

    fn declare_queue(
        &mut self,
        config: QueueConfig,
    ) -> Result<amqp::protocol::queue::DeclareOk, amqp::AMQPError> {
        self.queue_declare(
            config.queue,
            config.passive,
            config.durable,
            config.exclusive,
            config.auto_delete,
            config.no_wait,
            config.arguments.unwrap_or_else(amqp::Table::new),
        )
    }

    fn bind_queue(
        &mut self,
        config: BindQueueConfig,
    ) -> Result<amqp::protocol::queue::BindOk, amqp::AMQPError> {
        self.queue_bind(
            config.queue,
            config.exchange,
            config.routing_key.unwrap_or_else(|| "".to_owned()),
            config.no_wait,
            config.arguments.unwrap_or_else(amqp::Table::new),
        )
    }
}
