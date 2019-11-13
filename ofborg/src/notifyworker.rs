use crate::worker::Action;
use amqp::protocol::basic::{BasicProperties, Deliver};
use amqp::Basic;
use amqp::{Channel, Consumer};
use std::marker::Send;

pub struct NotifyWorker<T: SimpleNotifyWorker> {
    internal: T,
}

pub trait SimpleNotifyWorker {
    type J;

    fn consumer(&self, job: &Self::J, notifier: &mut dyn NotificationReceiver);

    fn msg_to_job(
        &self,
        method: &Deliver,
        headers: &BasicProperties,
        body: &[u8],
    ) -> Result<Self::J, String>;
}

pub trait NotificationReceiver {
    fn tell(&mut self, action: Action);
}

#[derive(Default)]
pub struct DummyNotificationReceiver {
    pub actions: Vec<Action>,
}

impl DummyNotificationReceiver {
    pub fn new() -> DummyNotificationReceiver {
        Default::default()
    }
}

impl NotificationReceiver for DummyNotificationReceiver {
    fn tell(&mut self, action: Action) {
        self.actions.push(action);
    }
}

pub struct ChannelNotificationReceiver<'a> {
    channel: &'a mut Channel,
    delivery_tag: u64,
}

impl<'a> ChannelNotificationReceiver<'a> {
    pub fn new(channel: &'a mut Channel, delivery_tag: u64) -> ChannelNotificationReceiver<'a> {
        ChannelNotificationReceiver {
            channel,
            delivery_tag,
        }
    }
}

impl<'a> NotificationReceiver for ChannelNotificationReceiver<'a> {
    fn tell(&mut self, action: Action) {
        match action {
            Action::Ack => {
                self.channel.basic_ack(self.delivery_tag, false).unwrap();
            }
            Action::NackRequeue => {
                self.channel
                    .basic_nack(self.delivery_tag, false, true)
                    .unwrap();
            }
            Action::NackDump => {
                self.channel
                    .basic_nack(self.delivery_tag, false, false)
                    .unwrap();
            }
            Action::Publish(mut msg) => {
                let exch = msg.exchange.take().unwrap_or_else(|| "".to_owned());
                let key = msg.routing_key.take().unwrap_or_else(|| "".to_owned());

                let props = msg.properties.take().unwrap_or(BasicProperties {
                    ..Default::default()
                });
                self.channel
                    .basic_publish(exch, key, msg.mandatory, msg.immediate, props, msg.content)
                    .unwrap();
            }
        }
    }
}

pub fn new<T: SimpleNotifyWorker>(worker: T) -> NotifyWorker<T> {
    NotifyWorker { internal: worker }
}

impl<T: SimpleNotifyWorker + Send> Consumer for NotifyWorker<T> {
    fn handle_delivery(
        &mut self,
        channel: &mut Channel,
        method: Deliver,
        headers: BasicProperties,
        body: Vec<u8>,
    ) {
        let mut receiver = ChannelNotificationReceiver::new(channel, method.delivery_tag);

        let job = self.internal.msg_to_job(&method, &headers, &body).unwrap();
        self.internal.consumer(&job, &mut receiver);
    }
}
