use std::pin::Pin;
use std::sync::Arc;

use crate::config::RabbitMqConfig;
use crate::easyamqp::{
    BindQueueConfig, ChannelExt, ConsumeConfig, ConsumerExt, ExchangeConfig, ExchangeType,
    QueueConfig,
};
use crate::notifyworker::{NotificationReceiver, SimpleNotifyWorker};
use crate::ofborg;
use crate::worker::{Action, SimpleWorker};

use lapin::message::Delivery;
use lapin::options::{
    BasicAckOptions, BasicConsumeOptions, BasicNackOptions, BasicPublishOptions, BasicQosOptions,
    ConfirmSelectOptions, ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions,
};
use lapin::types::FieldTable;
use lapin::{
    BasicProperties, Channel, Confirmation, Connection, ConnectionProperties, ExchangeKind,
};
use tokio_stream::StreamExt;
use tracing::{debug, trace};

pub async fn from_config(cfg: &RabbitMqConfig) -> Result<Connection, lapin::Error> {
    let opts = ConnectionProperties::default()
        .with_client_property("ofborg_version".into(), ofborg::VERSION.into());
    Connection::connect(&cfg.as_uri()?, opts).await
}

impl ChannelExt for Channel {
    type Error = lapin::Error;

    async fn declare_exchange(&mut self, config: ExchangeConfig) -> Result<(), Self::Error> {
        let opts = ExchangeDeclareOptions {
            passive: config.passive,
            durable: config.durable,
            auto_delete: config.auto_delete,
            internal: config.internal,
            nowait: config.no_wait,
        };

        let kind = match config.exchange_type {
            ExchangeType::Topic => ExchangeKind::Topic,
            ExchangeType::Fanout => ExchangeKind::Fanout,
            _ => panic!("exchange kind"),
        };
        self.exchange_declare(config.exchange.into(), kind, opts, FieldTable::default())
            .await?;
        Ok(())
    }

    async fn declare_queue(&mut self, config: QueueConfig) -> Result<(), Self::Error> {
        let opts = QueueDeclareOptions {
            passive: config.passive,
            durable: config.durable,
            exclusive: config.exclusive,
            auto_delete: config.auto_delete,
            nowait: config.no_wait,
        };

        self.queue_declare(config.queue.into(), opts, FieldTable::default())
            .await?;
        Ok(())
    }

    async fn bind_queue(&mut self, config: BindQueueConfig) -> Result<(), Self::Error> {
        let opts = QueueBindOptions {
            nowait: config.no_wait,
        };

        self.queue_bind(
            config.queue.into(),
            config.exchange.into(),
            config.routing_key.unwrap_or_else(|| "".into()).into(),
            opts,
            FieldTable::default(),
        )
        .await?;
        Ok(())
    }
}

impl<'a, W: SimpleWorker + 'a> ConsumerExt<'a, W> for Channel {
    type Error = lapin::Error;
    type Handle = Pin<Box<dyn Future<Output = ()> + 'a>>;

    async fn consume(
        self,
        mut worker: W,
        config: ConsumeConfig,
    ) -> Result<Self::Handle, Self::Error> {
        self.confirm_select(ConfirmSelectOptions::default()).await?;
        let mut consumer = self
            .basic_consume(
                config.queue.into(),
                config.consumer_tag.into(),
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;
        Ok(Box::pin(async move {
            while let Some(Ok(deliver)) = consumer.next().await {
                debug!(?deliver.delivery_tag, "consumed delivery");
                let content_type = deliver.properties.content_type();
                let job = worker
                    .msg_to_job(
                        deliver.routing_key.as_str(),
                        &content_type.as_ref().map(|s| s.to_string()),
                        &deliver.data,
                    )
                    .await
                    .expect("worker unexpected message consumed");

                for action in worker.consumer(&job).await {
                    action_deliver(&self, &deliver, action)
                        .await
                        .expect("action deliver failure");
                }
                debug!(?deliver.delivery_tag, "done");
            }
        }))
    }
}

/// Same as a regular channel, but without prefetching,
/// used for services with multiple instances.
pub struct WorkerChannel(pub Channel);

impl<'a, W: SimpleWorker + 'a> ConsumerExt<'a, W> for WorkerChannel {
    type Error = lapin::Error;
    type Handle = Pin<Box<dyn Future<Output = ()> + 'a>>;

    async fn consume(self, worker: W, config: ConsumeConfig) -> Result<Self::Handle, Self::Error> {
        self.0.basic_qos(1, BasicQosOptions::default()).await?;
        self.0.consume(worker, config).await
    }
}

pub struct ChannelNotificationReceiver {
    channel: lapin::Channel,
    deliver: Delivery,
}

impl ChannelNotificationReceiver {
    pub fn new(channel: lapin::Channel, deliver: Delivery) -> Self {
        ChannelNotificationReceiver { channel, deliver }
    }
}

#[async_trait::async_trait]
impl NotificationReceiver for ChannelNotificationReceiver {
    async fn tell(&self, action: Action) {
        action_deliver(&self.channel, &self.deliver, action)
            .await
            .expect("action deliver failure");
    }
}

// FIXME the consumer trait for SimpleWorker and SimpleNotifyWorker conflict,
// but one could probably be implemented in terms of the other instead.
pub struct NotifyChannel(pub Channel);

impl<'a, W: SimpleNotifyWorker + 'a + Send> ConsumerExt<'a, W> for NotifyChannel {
    type Error = lapin::Error;
    type Handle = Pin<Box<dyn Future<Output = ()> + 'a + Send>>;

    async fn consume(self, worker: W, config: ConsumeConfig) -> Result<Self::Handle, Self::Error> {
        self.0
            .confirm_select(ConfirmSelectOptions::default())
            .await?;
        self.0.basic_qos(1, BasicQosOptions::default()).await?;

        let mut consumer = self
            .0
            .basic_consume(
                config.queue.into(),
                config.consumer_tag.into(),
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;
        let chan = self.0;
        Ok(Box::pin(async move {
            while let Some(Ok(deliver)) = consumer.next().await {
                let delivery_tag = deliver.delivery_tag;
                debug!(?delivery_tag, "consumed delivery");
                let receiver = ChannelNotificationReceiver {
                    channel: chan.clone(),
                    deliver,
                };

                let content_type = receiver.deliver.properties.content_type();
                let job = worker
                    .msg_to_job(
                        receiver.deliver.routing_key.as_str(),
                        &content_type.as_ref().map(|s| s.to_string()),
                        &receiver.deliver.data,
                    )
                    .expect("worker unexpected message consumed");

                worker.consumer(job, Arc::new(receiver)).await;
                debug!(?delivery_tag, "done");
            }
        }))
    }
}

async fn action_deliver(
    chan: &Channel,
    deliver: &Delivery,
    action: Action,
) -> Result<(), lapin::Error> {
    match action {
        Action::Ack => {
            debug!(?deliver.delivery_tag, "action ack");
            chan.basic_ack(deliver.delivery_tag, BasicAckOptions::default())
                .await
        }
        Action::NackRequeue => {
            debug!(?deliver.delivery_tag, "action nack requeue");
            let opts = BasicNackOptions {
                requeue: true,
                ..Default::default()
            };
            chan.basic_nack(deliver.delivery_tag, opts).await
        }
        Action::NackDump => {
            debug!(?deliver.delivery_tag, "action nack dump");
            chan.basic_nack(deliver.delivery_tag, BasicNackOptions::default())
                .await
        }
        Action::Publish(msg) => {
            let exch = msg.exchange.as_deref().unwrap_or("");
            let key = msg.routing_key.as_deref().unwrap_or("");
            trace!(?exch, ?key, "action publish");

            let mut props = BasicProperties::default().with_delivery_mode(2); // persistent.

            if let Some(s) = msg.content_type.as_deref() {
                props = props.with_content_type(s.into());
            }

            let confirmation = chan
                .basic_publish(
                    exch.into(),
                    key.into(),
                    BasicPublishOptions {
                        mandatory: msg.mandatory,
                        immediate: msg.immediate,
                    },
                    &msg.content,
                    props,
                )
                .await?
                .await?;

            ensure_publisher_confirmation(confirmation)
        }
    }
}

fn ensure_publisher_confirmation(confirmation: Confirmation) -> Result<(), lapin::Error> {
    match confirmation {
        Confirmation::Ack(None) => Ok(()),
        Confirmation::Ack(Some(returned)) => Err(std::io::Error::other(format!(
            "message was returned as unroutable: {returned:?}"
        ))
        .into()),
        Confirmation::Nack(returned) => Err(std::io::Error::other(format!(
            "message was rejected by the broker: {returned:?}"
        ))
        .into()),
        Confirmation::NotRequested => {
            Err(std::io::Error::other("publisher confirmation was not requested").into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn publisher_confirmations_must_ack_without_returning_the_message() {
        assert!(ensure_publisher_confirmation(Confirmation::Ack(None)).is_ok());
        assert!(ensure_publisher_confirmation(Confirmation::Nack(None)).is_err());
        assert!(ensure_publisher_confirmation(Confirmation::NotRequested).is_err());
    }
}
