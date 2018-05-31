extern crate ofborg;
extern crate amqp;
extern crate env_logger;

extern crate hyper;
extern crate hubcaps;
extern crate hyper_native_tls;

use std::env;
use std::fs;
use std::io::Read;

use amqp::protocol::basic::{BasicProperties};

use ofborg::config;
use ofborg::easyamqp;
use ofborg::worker;
use ofborg::notifyworker;
use ofborg::notifyworker::NotificationReceiver;

fn main () {
    let usage = "Missing parameters\nUSAGE: github-event-faker path/to/config.json path/to/fixture.json\n";
    if env::args().nth(1).is_none() || env::args().nth(2).is_none() {
        panic!(usage);
    }
    let cfg = config::load(env::args().nth(1).unwrap().as_ref());
    let fixture_path = env::args().nth(2).unwrap();
    ofborg::setup_log();

    let mut pr_changed = Vec::new();
    let _ = match fs::File::open(fixture_path) {
        Ok(mut file) => file.read_to_end(&mut pr_changed),
        Err(_) => panic!("Missing pr changed fixture."),
    };

    let mut session = easyamqp::session_from_config(&cfg.rabbitmq).unwrap();
    println!("Connected to rabbitmq");

    let mut channel = session.open_channel(1).unwrap();
    let props = BasicProperties {
        content_type: Some("application/json".to_string()),
        ..Default::default()
    };
    let action = worker::Action::Publish(worker::QueueMsg {
        exchange: Some("github-events".to_string()),
        routing_key: Some("pull_request.nixos/nixpkgs".to_string()),
        mandatory: false,
        immediate: false,
        properties: Some(props),
        content: pr_changed,
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
