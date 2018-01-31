extern crate ofborg;
extern crate amqp;
extern crate env_logger;

use std::env;
use std::time::Duration;
use std::thread;

use ofborg::message::{Pr, Repo};

use ofborg::config;
use ofborg::notifyworker;
use ofborg::tasks::build;
use ofborg::message::buildjob;
use ofborg::easyamqp;

fn main() {
    let cfg = config::load(env::args().nth(1).unwrap().as_ref());
    ofborg::setup_log();

    let mut session = easyamqp::session_from_config(&cfg.rabbitmq).unwrap();
    println!("Connected to rabbitmq");

    println!("About to open channel #1");
    let mut chan = session.open_channel(1).unwrap();

    let mut receiver = notifyworker::ChannelNotificationReceiver::new(&mut chan, 0);
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
        logs: Some((
            Some(String::from("logs")),
            Some(String::from("build.log")),
        )),
        statusreport: Some((Some(String::from("build-results")), None)),
    };

    loop {
        println!("Starting a new build simulation");
        let mut actions =
            build::JobActions::new(&cfg.nix.system, &cfg.runner.identity, &job, &mut receiver);
        actions.log_started();

        for i in 1..51 {
            actions.log_line(&format!("Bogus message  #{:?}/50", i));
            thread::sleep(Duration::from_secs(3))
        }

        thread::sleep(Duration::from_secs(10))
    }
}
