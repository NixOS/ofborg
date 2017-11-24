extern crate ofborg;
extern crate amqp;
extern crate env_logger;

#[macro_use]
extern crate log;

use std::env;
use std::path::Path;
use ofborg::tasks;
use ofborg::config;
use ofborg::checkout;

use ofborg::worker;
use amqp::Session;
use amqp::Table;
use amqp::Basic;

fn main() {
    let cfg = config::load(env::args().nth(1).unwrap().as_ref());


    if let Err(_) = env::var("RUST_LOG") {
        env::set_var("RUST_LOG", "info");
        env_logger::init().unwrap();
        info!("Defaulting RUST_LOG environment variable to info");
    } else {
        env_logger::init().unwrap();
    }

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

    let cloner = checkout::cached_cloner(Path::new(&cfg.checkout.root));
    let nix = cfg.nix();


    let mrw = tasks::massrebuilder::MassRebuildWorker::new(
        cloner,
        nix,
        cfg.github()
    );
    /*
    println!("{:?}", mrw.consumer(&message::massrebuildjob::MassRebuildJob{
        pr: ofborg::message::Pr {
            head_sha: String::from("85589b80e81d5839cc91eb6be2cc3f7c041b760a"),
            number: 30777,
            target_branch: Some(String::from("master")),
        },
        repo: ofborg::message::Repo {
            clone_url: String::from("https://github.com/NixOS/nixpkgs.git"),
            full_name: String::from("NixOS/nixpkgs"),
            owner: String::from("NixOS"),
            name: String::from("nixpkgs"),
        }
    }));
     */

    channel.basic_prefetch(1);
    channel.basic_consume(
        worker::new(mrw),
        "mass-rebuild-check-jobs",
        format!("{}-mass-rebuild-checker", cfg.whoami()).as_ref(),
        false,
        false,
        false,
        false,
        Table::new()
    ).unwrap();

    channel.start_consuming();

    println!("Finished consuming?");

    channel.close(200, "Bye").unwrap();
    println!("Closed the channel");
    session.close(200, "Good Bye");
    println!("Closed the session... EOF");
}
