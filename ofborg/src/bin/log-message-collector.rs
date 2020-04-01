use ofborg::config;
use ofborg::easyamqp::{self, TypedWrappers};
use ofborg::tasks;
use ofborg::worker;

use std::env;
use std::path::PathBuf;

fn main() {
    let cfg = config::load(env::args().nth(1).unwrap().as_ref());
    ofborg::setup_log();

    let mut session = easyamqp::session_from_config(&cfg.rabbitmq).unwrap();
    println!("Connected to rabbitmq");

    let mut channel = session.open_channel(1).unwrap();

    channel
        .declare_exchange(easyamqp::ExchangeConfig {
            exchange: "logs".to_owned(),
            exchange_type: easyamqp::ExchangeType::Topic,
            passive: false,
            durable: true,
            auto_delete: false,
            no_wait: false,
            internal: false,
            arguments: None,
        })
        .unwrap();

    let queue_name = channel
        .declare_queue(easyamqp::QueueConfig {
            queue: "".to_owned(),
            passive: false,
            durable: false,
            exclusive: true,
            auto_delete: true,
            no_wait: false,
            arguments: None,
        })
        .unwrap()
        .queue;

    channel
        .bind_queue(easyamqp::BindQueueConfig {
            queue: queue_name.clone(),
            exchange: "logs".to_owned(),
            routing_key: Some("*.*".to_owned()),
            no_wait: false,
            arguments: None,
        })
        .unwrap();

    channel
        .consume(
            worker::new(tasks::log_message_collector::LogMessageCollector::new(
                PathBuf::from(cfg.log_storage.clone().unwrap().path),
                100,
            )),
            easyamqp::ConsumeConfig {
                queue: queue_name,
                consumer_tag: format!("{}-log-collector", cfg.whoami()),
                no_local: false,
                no_ack: false,
                no_wait: false,
                exclusive: false,
                arguments: None,
            },
        )
        .unwrap();

    channel.start_consuming();

    println!("Finished consuming?");

    channel.close(200, "Bye").unwrap();
    println!("Closed the channel");
    session.close(200, "Good Bye");
    println!("Closed the session... EOF");
}
