use std::env;
use std::error::Error;

use async_std::task;
use lapin::message::Delivery;
use lapin::BasicProperties;

use ofborg::commentparser;
use ofborg::config;
use ofborg::easylapin;
use ofborg::message::{buildjob, Pr, Repo};
use ofborg::notifyworker::NotificationReceiver;
use ofborg::worker;

fn main() -> Result<(), Box<dyn Error>> {
    ofborg::setup_log();

    let arg = env::args().nth(1).expect("usage: build-faker <config>");
    let cfg = config::load(arg.as_ref());

    let conn = easylapin::from_config(&cfg.rabbitmq)?;
    let mut chan = task::block_on(conn.create_channel())?;

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
        repo: repo_msg,
        pr: pr_msg,
        subset: Some(commentparser::Subset::Nixpkgs),
        attrs: vec!["success".to_owned()],
        logs: Some((Some("logs".to_owned()), Some(logbackrk.to_lowercase()))),
        statusreport: Some((None, Some("scratch".to_owned()))),
        request_id: "bogus-request-id".to_owned(),
    };

    {
        let deliver = Delivery {
            delivery_tag: 0,
            exchange: "no-exchange".into(),
            routing_key: "".into(),
            redelivered: false,
            properties: BasicProperties::default(),
            data: vec![],
        };
        let mut recv = easylapin::ChannelNotificationReceiver::new(&mut chan, &deliver);

        for _i in 1..2 {
            recv.tell(worker::publish_serde_action(
                None,
                Some("build-inputs-x86_64-darwin".to_owned()),
                &msg,
            ));
        }
    }

    Ok(())
}
