extern crate ofborg;
extern crate amqp;
extern crate env_logger;

use std::env;
use std::path::Path;
use ofborg::tasks;
use ofborg::config;
use ofborg::checkout;

use ofborg::stats;
use ofborg::worker;
use amqp::Session;
use amqp::Table;
use amqp::Basic;

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

    let cloner = checkout::cached_cloner(Path::new(&cfg.checkout.root));
    let nix = cfg.nix();

    let events = stats::RabbitMQ::new(session.open_channel(3).unwrap());

    let mrw = tasks::massrebuilder::MassRebuildWorker::new(
        cloner,
        nix,
        cfg.github(),
        cfg.runner.identity.clone(),
        events
    );

    channel.basic_prefetch(1).unwrap();
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
