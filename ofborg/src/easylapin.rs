use std::pin::Pin;

use crate::config::RabbitMQConfig;
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
use lapin::{
    BasicProperties, Channel, CloseOnDrop, Connection, ConnectionProperties, ExchangeKind,
};
use tracing::debug;

pub fn from_config(cfg: &RabbitMQConfig) -> Result<CloseOnDrop<Connection>, lapin::Error> {
    let mut props = FieldTable::default();
    props.insert(
        "ofborg_version".into(),
        AMQPValue::LongString(ofborg::VERSION.into()),
    );
    let mut opts = ConnectionProperties::default();
    opts.client_properties = props;
    task::block_on(Connection::connect(&cfg.as_uri(), opts))
}

impl ChannelExt for CloseOnDrop<Channel> {
    type Error = lapin::Error;

    fn declare_exchange(&mut self, config: ExchangeConfig) -> Result<(), Self::Error> {
        let mut opts = ExchangeDeclareOptions::default();
        opts.passive = config.passive;
        opts.durable = config.durable;
        opts.auto_delete = config.auto_delete;
        opts.internal = config.internal;
        opts.nowait = config.no_wait;

        let kind = match config.exchange_type {
            ExchangeType::Topic => ExchangeKind::Topic,
            ExchangeType::Fanout => ExchangeKind::Fanout,
            _ => panic!("exchange kind"),
        };
        task::block_on(self.exchange_declare(&config.exchange, kind, opts, FieldTable::default()))?;
        Ok(())
    }

    fn declare_queue(&mut self, config: QueueConfig) -> Result<(), Self::Error> {
        let mut opts = QueueDeclareOptions::default();
        opts.passive = config.passive;
        opts.durable = config.durable;
        opts.exclusive = config.exclusive;
        opts.auto_delete = config.auto_delete;
        opts.nowait = config.no_wait;

        task::block_on(self.queue_declare(&config.queue, opts, FieldTable::default()))?;
        Ok(())
    }

    fn bind_queue(&mut self, config: BindQueueConfig) -> Result<(), Self::Error> {
        let mut opts = QueueBindOptions::default();
        opts.nowait = config.no_wait;

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

impl<W: SimpleWorker + 'static> ConsumerExt<W> for CloseOnDrop<Channel> {
    type Error = lapin::Error;
    type Handle = Pin<Box<dyn Future<Output = ()> + 'static>>;

    fn consume(self, mut worker: W, config: ConsumeConfig) -> Result<Self::Handle, Self::Error> {
        task::block_on(self.basic_qos(1, BasicQosOptions::default()))?;

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

struct ChannelNotificationReceiver<'a> {
    channel: &'a mut CloseOnDrop<lapin::Channel>,
    deliver: &'a Delivery,
}

impl<'a> NotificationReceiver for ChannelNotificationReceiver<'a> {
    fn tell(&mut self, action: Action) {
        task::block_on(action_deliver(self.channel, self.deliver, action))
            .expect("action deliver failure");
    }
}

// FIXME the consumer trait for SimpleWorker and SimpleNotifyWorker conflict,
// but one could probably be implemented in terms of the other instead.
pub struct NotifyChannel(pub CloseOnDrop<Channel>);

impl<W: SimpleNotifyWorker + 'static> ConsumerExt<W> for NotifyChannel {
    type Error = lapin::Error;
    type Handle = Pin<Box<dyn Future<Output = ()> + 'static>>;

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
    chan: &CloseOnDrop<Channel>,
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
            let mut opts = BasicNackOptions::default();
            opts.requeue = true;
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
            debug!(?exch, ?key, "action publish");

            let mut props = BasicProperties::default().with_delivery_mode(2); // persistent.

            if let Some(s) = msg.content_type {
                props = props.with_content_type(s.into());
            }

            let _confirmaton = chan
                .basic_publish(
                    &exch,
                    &key,
                    BasicPublishOptions::default(),
                    msg.content,
                    props,
                )
                .await?
                .await?;
            Ok(())
        }
    }
}
