extern crate amqp;
extern crate env_logger;
extern crate ofborg;

extern crate hubcaps;
extern crate hyper;
extern crate hyper_native_tls;

use std::env;

use ofborg::commentparser;
use ofborg::config;
use ofborg::easyamqp;
use ofborg::message::buildjob;
use ofborg::message::{Pr, Repo};
use ofborg::notifyworker;
use ofborg::notifyworker::NotificationReceiver;
use ofborg::worker;

fn main() {
    let cfg = config::load(env::args().nth(1).unwrap().as_ref());
    ofborg::setup_log();

    println!("Hello, world!");

    let mut session = easyamqp::session_from_config(&cfg.rabbitmq).unwrap();
    println!("Connected to rabbitmq");

    let mut channel = session.open_channel(1).unwrap();

    let repo_msg = Repo {
        clone_url: "https://github.com/nixos/ofborg.git".to_owned(),
        full_name: "NixOS/ofborg".to_owned(),
        owner: "NixOS".to_owned(),
        name: "ofborg".to_owned(),
    };

    let pr_msg = Pr {
        number: 42,
        head_sha: "6dd9f0265d52b946dd13daf996f30b64e4edb446".to_owned(),
        target_branch: Some("scratch".to_owned()),
    };

    let logbackrk = "NixOS/ofborg.42".to_owned();

    let msg = buildjob::BuildJob {
        repo: repo_msg.clone(),
        pr: pr_msg.clone(),
        subset: Some(commentparser::Subset::Nixpkgs),
        attrs: vec!["success".to_owned()],
        logs: Some((Some("logs".to_owned()), Some(logbackrk.to_lowercase()))),
        statusreport: Some((None, Some("scratch".to_owned()))),
        request_id: "bogus-request-id".to_owned(),
    };

    {
        let mut recv = notifyworker::ChannelNotificationReceiver::new(&mut channel, 0);

        for _i in 1..2 {
            recv.tell(worker::publish_serde_action(
                None,
                Some("build-inputs-x86_64-darwin".to_owned()),
                &msg,
            ));
        }
    }

    channel.close(200, "Bye").unwrap();
    println!("Closed the channel");
    session.close(200, "Good Bye");
    println!("Closed the session... EOF");
}
