extern crate ofborg;
extern crate amqp;
extern crate env_logger;

extern crate hyper;
extern crate hubcaps;
extern crate hyper_native_tls;


use std::env;

use amqp::Basic;
use amqp::Session;
use amqp::Table;

use ofborg::config;
use ofborg::worker;
use ofborg::tasks;


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

    channel.basic_consume(
        worker::new(tasks::buildfilter::BuildFilterWorker::new(
            cfg.acl(),
            cfg.github()
        )),
        "build-inputs",
        format!("{}-build-filter", cfg.whoami()).as_ref(),
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
