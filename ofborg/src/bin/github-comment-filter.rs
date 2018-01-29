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
    ofborg::setup_log();

    println!("Hello, world!");


    let mut session = Session::open_url(&cfg.rabbitmq.as_uri()).unwrap();
    println!("Connected to rabbitmq");

    let mut channel = session.open_channel(1).unwrap();

    channel.basic_prefetch(1).unwrap();
    channel
        .basic_consume(
            worker::new(tasks::githubcommentfilter::GitHubCommentWorker::new(
                cfg.acl(),
                cfg.github(),
            )),
            "build-inputs",
            format!("{}-github-comment-filter", cfg.whoami()).as_ref(),
            false,
            false,
            false,
            false,
            Table::new(),
        )
        .unwrap();

    channel.start_consuming();

    println!("Finished consuming?");

    channel.close(200, "Bye").unwrap();
    println!("Closed the channel");
    session.close(200, "Good Bye");
    println!("Closed the session... EOF");
}
