use std::pin::Pin;

use crate::config::RabbitMqConfig;
use crate::easyamqp::{
    BindQueueConfig, ChannelExt, ConsumeConfig, ConsumerExt, ExchangeConfig, ExchangeType,
    QueueConfig,
};
use crate::notifyworker::{NotificationReceiver, SimpleNotifyWorker};
use crate::ofborg;
use crate::worker::{Action, SimpleWorker};

use async_std::future::Future;
use async_std::stream::StreamExt;
use async_std::task;
use lapin::message::Delivery;
use lapin::options::{
    BasicAckOptions, BasicConsumeOptions, BasicNackOptions, BasicPublishOptions, BasicQosOptions,
    ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions,
};
use lapin::types::{AMQPValue, FieldTable};
use lapin::{BasicProperties, Channel, Connection, ConnectionProperties, ExchangeKind};
use tracing::{debug, trace};

pub fn from_config(cfg: &RabbitMqConfig) -> Result<Connection, lapin::Error> {
    let mut props = FieldTable::default();
    props.insert(
        "ofborg_version".into(),
        AMQPValue::LongString(ofborg::VERSION.into()),
    );
    let opts = ConnectionProperties {
        client_properties: props,
        ..Default::default()
    };
    task::block_on(Connection::connect(&cfg.as_uri()?, opts))
}

impl ChannelExt for Channel {
    type Error = lapin::Error;

    fn declare_exchange(&mut self, config: ExchangeConfig) -> Result<(), Self::Error> {
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
        task::block_on(self.exchange_declare(&config.exchange, kind, opts, FieldTable::default()))?;
        Ok(())
    }

    fn declare_queue(&mut self, config: QueueConfig) -> Result<(), Self::Error> {
        let opts = QueueDeclareOptions {
            passive: config.passive,
            durable: config.durable,
            exclusive: config.exclusive,
            auto_delete: config.auto_delete,
            nowait: config.no_wait,
        };

        task::block_on(self.queue_declare(&config.queue, opts, FieldTable::default()))?;
        Ok(())
    }

    fn bind_queue(&mut self, config: BindQueueConfig) -> Result<(), Self::Error> {
        let opts = QueueBindOptions {
            nowait: config.no_wait,
        };

        task::block_on(self.queue_bind(
            &config.queue,
            &config.exchange,
            &config.routing_key.unwrap_or_else(|| "".into()),
            opts,
            FieldTable::default(),
        ))?;
        Ok(())
    }
}

impl<'a, W: SimpleWorker + 'a> ConsumerExt<'a, W> for Channel {
    type Error = lapin::Error;
    type Handle = Pin<Box<dyn Future<Output = ()> + 'a>>;

    fn consume(self, mut worker: W, config: ConsumeConfig) -> Result<Self::Handle, Self::Error> {
        let mut consumer = task::block_on(self.basic_consume(
            &config.queue,
            &config.consumer_tag,
            BasicConsumeOptions::default(),
            FieldTable::default(),
        ))?;
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
                    .expect("worker unexpected message consumed");

                for action in worker.consumer(&job) {
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

    fn consume(self, worker: W, config: ConsumeConfig) -> Result<Self::Handle, Self::Error> {
        task::block_on(self.0.basic_qos(1, BasicQosOptions::default()))?;
        self.0.consume(worker, config)
    }
}

pub struct ChannelNotificationReceiver<'a> {
    channel: &'a mut lapin::Channel,
    deliver: &'a Delivery,
}

impl<'a> ChannelNotificationReceiver<'a> {
    pub fn new(channel: &'a mut lapin::Channel, deliver: &'a Delivery) -> Self {
        ChannelNotificationReceiver { channel, deliver }
    }
}

impl<'a> NotificationReceiver for ChannelNotificationReceiver<'a> {
    fn tell(&mut self, action: Action) {
        task::block_on(action_deliver(self.channel, self.deliver, action))
            .expect("action deliver failure");
    }
}

// FIXME the consumer trait for SimpleWorker and SimpleNotifyWorker conflict,
// but one could probably be implemented in terms of the other instead.
pub struct NotifyChannel(pub Channel);

impl<'a, W: SimpleNotifyWorker + 'a + Send> ConsumerExt<'a, W> for NotifyChannel {
    type Error = lapin::Error;
    type Handle = Pin<Box<dyn Future<Output = ()> + 'a + Send>>;

    fn consume(self, worker: W, config: ConsumeConfig) -> Result<Self::Handle, Self::Error> {
        task::block_on(self.0.basic_qos(1, BasicQosOptions::default()))?;

        let mut consumer = task::block_on(self.0.basic_consume(
            &config.queue,
            &config.consumer_tag,
            BasicConsumeOptions::default(),
            FieldTable::default(),
        ))?;
        let mut chan = self.0;
        Ok(Box::pin(async move {
            while let Some(Ok(deliver)) = consumer.next().await {
                debug!(?deliver.delivery_tag, "consumed delivery");
                let mut receiver = ChannelNotificationReceiver {
                    channel: &mut chan,
                    deliver: &deliver,
                };

                let content_type = deliver.properties.content_type();
                let job = worker
                    .msg_to_job(
                        deliver.routing_key.as_str(),
                        &content_type.as_ref().map(|s| s.to_string()),
                        &deliver.data,
                    )
                    .expect("worker unexpected message consumed");

                worker.consumer(&job, &mut receiver);
                debug!(?deliver.delivery_tag, "done");
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
        Action::Publish(mut msg) => {
            let exch = msg.exchange.take().unwrap_or_else(|| "".to_owned());
            let key = msg.routing_key.take().unwrap_or_else(|| "".to_owned());
            trace!(?exch, ?key, "action publish");

            let mut props = BasicProperties::default().with_delivery_mode(2); // persistent.

            if let Some(s) = msg.content_type {
                props = props.with_content_type(s.into());
            }

            let _confirmaton = chan
                .basic_publish(
                    &exch,
                    &key,
                    BasicPublishOptions::default(),
                    &msg.content,
                    props,
                )
                .await?
                .await?;
            Ok(())
        }
    }
}
