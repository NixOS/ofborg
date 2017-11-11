extern crate ofborg;
extern crate amqp;
extern crate env_logger;

use std::{thread, time};
use std::env;

use std::path::Path;
use amqp::Basic;
use amqp::Session;
use amqp::Table;
use std::process;

use ofborg::config;
use ofborg::checkout;
use ofborg::worker;
use ofborg::tasks;
use ofborg::nix;


fn main() {
    let cfg = config::load(env::args().nth(1).unwrap().as_ref());
    env_logger::init().unwrap();
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
    let nix = nix::new(cfg.nix.system.clone(), cfg.nix.remote.clone());

    channel.basic_consume(
        worker::new(tasks::build::BuildWorker::new(cloner, nix, cfg.nix.system.clone())),
        format!("build-inputs-{}", cfg.nix.system.clone()).as_ref(),
        format!("{}-builder", cfg.whoami()).as_ref(),
        false,
        false,
        false,
        false,
        Table::new()
    ).unwrap();

    let ten_sec = time::Duration::from_secs(10);
    thread::sleep(ten_sec);

    channel.start_consuming();

    println!("Finished consuming?");

    channel.close(200, "Bye").unwrap();
    println!("Closed the channel");
    session.close(200, "Good Bye");
    println!("Closed the session... EOF");
}
