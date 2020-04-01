use amqp::protocol::basic::{BasicProperties, Deliver};
use amqp::{Basic, Channel, Consumer};
use serde::Serialize;

use std::marker::Send;

pub struct Worker<T: SimpleWorker> {
    internal: T,
}

pub struct Response {}

pub type Actions = Vec<Action>;

#[derive(Debug, PartialEq)]
pub enum Action {
    Ack,
    NackRequeue,
    NackDump,
    Publish(Box<QueueMsg>),
}

#[derive(Debug, PartialEq)]
pub struct QueueMsg {
    pub exchange: Option<String>,
    pub routing_key: Option<String>,
    pub mandatory: bool,
    pub immediate: bool,
    pub properties: Option<BasicProperties>,
    pub content: Vec<u8>,
}

pub fn publish_serde_action<T: ?Sized>(
    exchange: Option<String>,
    routing_key: Option<String>,
    msg: &T,
) -> Action
where
    T: Serialize,
{
    let props = BasicProperties {
        content_type: Some("application/json".to_owned()),
        ..Default::default()
    };

    Action::Publish(Box::new(QueueMsg {
        exchange,
        routing_key,
        mandatory: false,
        immediate: false,
        properties: Some(props),
        content: serde_json::to_string(&msg).unwrap().into_bytes(),
    }))
}

pub trait SimpleWorker: Send + 'static {
    type J: Send;

    fn consumer(&mut self, job: &Self::J) -> Actions;

    fn msg_to_job(
        &mut self,
        method: &Deliver,
        headers: &BasicProperties,
        body: &[u8],
    ) -> Result<Self::J, String>;
}

pub fn new<T: SimpleWorker>(worker: T) -> Worker<T> {
    Worker { internal: worker }
}

impl<T: SimpleWorker + Send> Consumer for Worker<T> {
    fn handle_delivery(
        &mut self,
        channel: &mut Channel,
        method: Deliver,
        headers: BasicProperties,
        body: Vec<u8>,
    ) {
        let job = self.internal.msg_to_job(&method, &headers, &body);

        if let Err(e) = job {
            error!("Error decoding job: {:?}", e);
            channel.basic_ack(method.delivery_tag, false).unwrap();
            return;
        }

        for action in self.internal.consumer(&job.unwrap()) {
            match action {
                Action::Ack => {
                    channel.basic_ack(method.delivery_tag, false).unwrap();
                }
                Action::NackRequeue => {
                    channel
                        .basic_nack(method.delivery_tag, false, true)
                        .unwrap();
                }
                Action::NackDump => {
                    channel
                        .basic_nack(method.delivery_tag, false, false)
                        .unwrap();
                }
                Action::Publish(mut msg) => {
                    let exch = msg.exchange.take().unwrap_or_else(|| "".to_owned());
                    let key = msg.routing_key.take().unwrap_or_else(|| "".to_owned());

                    let props = msg.properties.take().unwrap_or(BasicProperties {
                        ..Default::default()
                    });
                    channel
                        .basic_publish(exch, key, msg.mandatory, msg.immediate, props, msg.content)
                        .unwrap();
                }
            }
        }
    }
}
