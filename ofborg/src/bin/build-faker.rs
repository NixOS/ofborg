extern crate ofborg;
extern crate amqp;
extern crate env_logger;

extern crate hyper;
extern crate hubcaps;
extern crate hyper_native_tls;


use std::env;

use amqp::Session;

use ofborg::config;
use ofborg::worker;
use ofborg::notifyworker;
use ofborg::notifyworker::NotificationReceiver;
use ofborg::commentparser;
use ofborg::message::buildjob;


use ofborg::message::{Pr,Repo};
use ofborg::tasks;


fn main() {
    let cfg = config::load(env::args().nth(1).unwrap().as_ref());
    ofborg::setup_log();

    println!("Hello, world!");


    let mut session = Session::open_url(&cfg.rabbitmq.as_uri()).unwrap();
    println!("Connected to rabbitmq");
    {
        println!("About to open channel #1");
        let hbchan = session.open_channel(1).unwrap();

        println!("Opened channel #1");

        tasks::heartbeat::start_on_channel(hbchan, cfg.whoami());
    }


    let mut channel = session.open_channel(2).unwrap();


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
        attrs: vec![ "success".to_owned() ],
        logs: Some((Some("logs".to_owned()), Some(logbackrk.to_lowercase()))),
        statusreport: Some((None, Some("scratch".to_owned()))),
    };

    {
        let mut recv = notifyworker::ChannelNotificationReceiver::new(&mut channel, 0);

        for i in 1..2 {
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
