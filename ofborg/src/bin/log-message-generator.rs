use std::env;
use std::error::Error;
use std::thread;
use std::time::Duration;

use async_std::task;
use lapin::message::Delivery;
use lapin::BasicProperties;
use tracing::info;

use ofborg::config;
use ofborg::easylapin;
use ofborg::message::{buildjob, Pr, Repo};
use ofborg::tasks::build;

fn main() -> Result<(), Box<dyn Error>> {
    ofborg::setup_log();

    let arg = env::args()
        .nth(1)
        .expect("usage: log-message-generator <config>");
    let cfg = config::load(arg.as_ref());

    let conn = easylapin::from_config(&cfg.rabbitmq)?;
    let mut chan = task::block_on(conn.create_channel())?;

    let deliver = Delivery {
        delivery_tag: 0,
        exchange: "no-exchange".into(),
        routing_key: "".into(),
        redelivered: false,
        properties: BasicProperties::default(),
        data: vec![],
    };
    let mut receiver = easylapin::ChannelNotificationReceiver::new(&mut chan, &deliver);
    let job = buildjob::BuildJob {
        attrs: vec![],
        pr: Pr {
            head_sha: String::from("bogus"),
            number: 1,
            target_branch: Some("master".to_owned()),
        },
        repo: Repo {
            clone_url: String::from("bogus"),
            full_name: "test-git".to_owned(),
            name: "nixos".to_owned(),
            owner: "ofborg-test".to_owned(),
        },
        subset: None,
        logs: Some((Some(String::from("logs")), Some(String::from("build.log")))),
        statusreport: Some((Some(String::from("build-results")), None)),
        request_id: "bogus-request-id".to_owned(),
    };

    loop {
        info!("Starting a new build simulation");
        let mut actions =
            build::JobActions::new(&cfg.nix.system, &cfg.runner.identity, &job, &mut receiver);
        actions.log_started(vec![], vec![]);

        for i in 1..51 {
            actions.log_line(&format!("Bogus message  #{:?}/50", i));
            thread::sleep(Duration::from_secs(3))
        }

        thread::sleep(Duration::from_secs(10))
    }
}
