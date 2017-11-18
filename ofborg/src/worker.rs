use amqp::Basic;
use amqp::{Consumer, Channel};
use amqp::protocol::basic::{Deliver,BasicProperties};
use std::marker::Send;


pub struct Worker<T: SimpleWorker> {
    internal: T
}

pub struct Response {
}

pub type Actions = Vec<Action>;

#[derive(Debug)]
pub enum Action {
    Ack,
    NackRequeue,
    NackDump,
    Publish(QueueMsg),
}

#[derive(Debug)]
pub struct QueueMsg {
    pub exchange: Option<String>,
    pub routing_key: Option<String>,
    pub mandatory: bool,
    pub immediate: bool,
    pub properties: Option<BasicProperties>,
    pub content: Vec<u8>,
}


pub trait SimpleWorker {
    type J;

    fn consumer(&self, job: &Self::J) -> Actions;

    fn msg_to_job(&self, method: &Deliver, headers: &BasicProperties,
                  body: &Vec<u8>) -> Result<Self::J, String>;
}

pub fn new<T: SimpleWorker>(worker: T) -> Worker<T> {
    return Worker{
        internal: worker,
    };
}



impl <T: SimpleWorker + Send> Consumer for Worker<T> {
    fn handle_delivery(&mut self,
                       channel: &mut Channel,
                       method: Deliver,
                       headers: BasicProperties,
                       body: Vec<u8>) {



        let job = self.internal.msg_to_job(&method, &headers, &body).unwrap();
        for action in self.internal.consumer(&job) {
            match action {
                Action::Ack => {
                    channel.basic_ack(method.delivery_tag, false).unwrap();
                }
                Action::NackRequeue => {
                    channel.basic_nack(method.delivery_tag, false, true).unwrap();
                }
                Action::NackDump => {
                    channel.basic_nack(method.delivery_tag, false, false).unwrap();
                }
                Action::Publish(msg) => {
                    let exch = msg.exchange.clone().unwrap_or("".to_owned());
                    let key = msg.routing_key.clone().unwrap_or("".to_owned());

                    let props = msg.properties.unwrap_or(BasicProperties{ ..Default::default()});
                    channel.basic_publish(
                        exch,
                        key,
                        msg.mandatory,
                        msg.immediate,
                        props,
                        msg.content
                    ).unwrap();
                }
            }
        }
    }
}
