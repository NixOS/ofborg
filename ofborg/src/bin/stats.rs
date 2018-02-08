extern crate hyper;
extern crate prometheus;
extern crate amqp;
extern crate ofborg;

use std::env;
use ofborg::{easyamqp, tasks, worker, config, stats};

use amqp::Basic;
use ofborg::easyamqp::TypedWrappers;
use hyper::header::ContentType;
use hyper::mime::Mime;
use hyper::server::{Request, Response, Server};
use prometheus::{Counter, Encoder, Gauge, HistogramVec, TextEncoder};

use std::thread;
use std::time::Duration;

fn main() {
    let cfg = config::load(env::args().nth(1).unwrap().as_ref());
    ofborg::setup_log();

    println!("Hello, world!");


    let mut session = easyamqp::session_from_config(&cfg.rabbitmq).unwrap();
    println!("Connected to rabbitmq");

    let events = stats::RabbitMQ::new(
        &format!("{}-{}", cfg.runner.identity.clone(), cfg.nix.system.clone()),
        session.open_channel(3).unwrap()
    );

    let collector = tasks::statscollector::StatCollectorWorker::new(
        events
    );

    let mut channel = session.open_channel(1).unwrap();

    channel.basic_prefetch(1).unwrap();
    channel
        .consume(
            worker::new(collector),
            easyamqp::ConsumeConfig {
                queue: "sample-stats-events".to_owned(),
                consumer_tag: format!("{}-prometheus-stats-collector", cfg.whoami()),
                no_local: false,
                no_ack: false,
                no_wait: false,
                exclusive: false,
                arguments: None,
            },
        )
        .unwrap();


    thread::spawn(||{
        let encoder = TextEncoder::new();
        let addr = "127.0.0.1:9898";
        println!("listening addr {:?}", addr);
        Server::http(addr)
            .unwrap()
            .handle(move |_: Request, mut res: Response| {
                let metric_familys = prometheus::gather();
                let mut buffer = vec![];
                encoder.encode(&metric_familys, &mut buffer).unwrap();
                res.headers_mut()
                    .set(ContentType(encoder.format_type().parse::<Mime>().unwrap()));
                res.send(&buffer).unwrap();
            })
            .unwrap();
    });


    channel.start_consuming();

    println!("Finished consuming?");

    channel.close(200, "Bye").unwrap();
    println!("Closed the channel");
    session.close(200, "Good Bye");
    println!("Closed the session... EOF");
}
