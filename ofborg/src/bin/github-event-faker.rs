extern crate ofborg;
extern crate amqp;
extern crate env_logger;

extern crate hyper;
extern crate hubcaps;
extern crate hyper_native_tls;

use std::env;

use amqp::protocol::basic::{BasicProperties};

use ofborg::config;
use ofborg::easyamqp;
use ofborg::worker;
use ofborg::notifyworker;
use ofborg::notifyworker::NotificationReceiver;

fn main () {
    let cfg = config::load(env::args().nth(1).unwrap().as_ref());
    ofborg::setup_log();

    let mut session = easyamqp::session_from_config(&cfg.rabbitmq).unwrap();
    println!("Connected to rabbitmq");

    let mut channel = session.open_channel(1).unwrap();
    let props = BasicProperties {
        content_type: Some("application/json".to_string()),
        ..Default::default()
    };
    let content = "{}".to_string();
    let action = worker::Action::Publish(worker::QueueMsg {
        exchange: Some("github-events".to_string()),
        routing_key: Some("pull_request.nixos/nixpkgs".to_string()),
        mandatory: false,
        immediate: false,
        properties: Some(props),
        content: content.into_bytes(),
    });
    {
        let mut recv = notifyworker::ChannelNotificationReceiver::new(&mut channel, 0);
        recv.tell(action);
    }
    channel.close(200, "Bye").unwrap();
    println!("Closed the channel");
    session.close(200, "Good Bye");
    println!("Closed the session... EOF");
}
