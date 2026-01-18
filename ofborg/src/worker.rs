use std::{marker::Send, sync::Arc};

use serde::Serialize;

pub struct Response {}

pub type Actions = Vec<Action>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Action {
    Ack,
    NackRequeue,
    NackDump,
    Publish(Arc<QueueMsg>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct QueueMsg {
    pub exchange: Option<String>,
    pub routing_key: Option<String>,
    pub mandatory: bool,
    pub immediate: bool,
    pub content_type: Option<String>,
    pub content: Vec<u8>,
}

pub fn publish_serde_action<T: Serialize + ?Sized>(
    exchange: Option<String>,
    routing_key: Option<String>,
    msg: &T,
) -> Action {
    Action::Publish(Arc::new(QueueMsg {
        exchange,
        routing_key,
        mandatory: false,
        immediate: false,
        content_type: Some("application/json".to_owned()),
        content: serde_json::to_string(&msg).unwrap().into_bytes(),
    }))
}

pub trait SimpleWorker: Send {
    type J: Send;

    fn consumer(&mut self, job: &Self::J) -> impl std::future::Future<Output = Actions>;

    fn msg_to_job(
        &mut self,
        method: &str,
        headers: &Option<String>,
        body: &[u8],
    ) -> impl std::future::Future<Output = Result<Self::J, String>>;
}
