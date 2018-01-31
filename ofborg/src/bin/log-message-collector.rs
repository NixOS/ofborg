extern crate ofborg;
extern crate amqp;
extern crate env_logger;

use std::env;
use std::path::PathBuf;

use amqp::Table;

use ofborg::config;
use ofborg::worker;
use ofborg::tasks;
use ofborg::easyamqp;
use ofborg::easyamqp::TypedWrappers;


fn main() {
    let cfg = config::load(env::args().nth(1).unwrap().as_ref());
    ofborg::setup_log();

    let mut session = easyamqp::session_from_config(&cfg.rabbitmq).unwrap();
    println!("Connected to rabbitmq");

    let mut channel = session.open_channel(1).unwrap();

    let queue_name = channel
        .queue_declare(
            "",
            false, // passive
            false, // durable
            true, // exclusive
            true, // auto_delete
            false, //nowait
            Table::new(),
        )
        .expect("Failed to declare an anon queue for log collection!")
        .queue;

    channel
        .queue_bind(
            queue_name.as_ref(),
            "logs",
            "*.*".as_ref(),
            false,
            Table::new(),
        )
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
