extern crate ofborg;
extern crate amqp;
extern crate env_logger;

extern crate hyper;
extern crate hubcaps;
extern crate hyper_native_tls;


use std::collections::HashMap;
use std::env;

use amqp::Basic;

use ofborg::config;
use ofborg::worker;
use ofborg::tasks;
use ofborg::easyamqp;
use ofborg::easyamqp::TypedWrappers;


fn main() {
    let mut label_map = HashMap::new();
    label_map.insert("bug".to_owned(), "0.kind: bug".to_owned());
    label_map.insert("enhancement".to_owned(), "0.kind: enhancement".to_owned());
    label_map.insert("question".to_owned(), "0.kind: question".to_owned());
    label_map.insert("regression".to_owned(), "0.kind: regression".to_owned());
    // TODO: add more labels (this is an implicit whitelist) and load from config file

    let cfg = config::load(env::args().nth(1).unwrap().as_ref());
    ofborg::setup_log();

    let mut session = easyamqp::session_from_config(&cfg.rabbitmq).unwrap();
    let mut channel = session.open_channel(1).unwrap();

    // TODO: I have no idea what I'm doing, this is basically github-comment-parser
    // adapted in a "let's hope this will work" fashion.
    channel
        .declare_exchange(easyamqp::ExchangeConfig {
            exchange: "label-jobs".to_owned(),
            exchange_type: easyamqp::ExchangeType::Fanout,
            passive: false,
            durable: true,
            auto_delete: false,
            no_wait: false,
            internal: false,
            arguments: None,
        })
        .unwrap();

    channel
        .declare_queue(easyamqp::QueueConfig {
            queue: "label-jobs".to_owned(),
            passive: false,
            durable: true,
            exclusive: false,
            auto_delete: false,
            no_wait: false,
            arguments: None,
        })
        .unwrap();

    channel
        .bind_queue(easyamqp::BindQueueConfig {
            queue: "label-jobs".to_owned(),
            exchange: "label-jobs".to_owned(),
            routing_key: None,
            no_wait: false,
            arguments: None,
        })
        .unwrap();

    channel.basic_prefetch(1).unwrap();
    channel
        .consume(
            worker::new(tasks::githublabeler::GitHubLabeler::new(
                cfg.github(),
                label_map,
            )),
            easyamqp::ConsumeConfig {
                queue: "label-jobs".to_owned(),
                consumer_tag: format!("{}-github-labeler", cfg.whoami()),
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
