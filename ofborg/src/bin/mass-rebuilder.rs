use std::env;
use std::error::Error;
use std::path::Path;

use ofborg::block_on;
use tracing::{error, info};

use ofborg::checkout;
use ofborg::config;
use ofborg::easyamqp::{self, ChannelExt, ConsumerExt};
use ofborg::easylapin;
use ofborg::stats;
use ofborg::tasks;

fn main() -> Result<(), Box<dyn Error>> {
    ofborg::setup_log();

    let arg = env::args()
        .nth(1)
        .unwrap_or_else(|| panic!("usage: {} <config>", std::env::args().next().unwrap()));
    let cfg = config::load(arg.as_ref());

    let Some(rebuilder_cfg) = config::load(arg.as_ref()).mass_rebuilder else {
        error!("No mass rebuilder configuration found!");
        panic!();
    };

    let conn = easylapin::from_config(&rebuilder_cfg.rabbitmq)?;
    let mut chan = block_on(conn.create_channel())?;

    let root = Path::new(&cfg.checkout.root);
    let cloner = checkout::cached_cloner(&root.join(cfg.runner.instance.to_string()));

    let events = stats::RabbitMq::from_lapin(&cfg.whoami(), block_on(conn.create_channel())?);

    let queue_name = String::from("mass-rebuild-check-jobs");
    chan.declare_queue(easyamqp::QueueConfig {
        queue: queue_name.clone(),
        passive: false,
        durable: true,
        exclusive: false,
        auto_delete: false,
        no_wait: false,
    })?;

    let handle = easylapin::WorkerChannel(chan).consume(
        tasks::evaluate::EvaluationWorker::new(
            cloner,
            cfg.github_app_vendingmachine(),
            cfg.acl(),
            cfg.runner.identity.clone(),
            events,
        ),
        easyamqp::ConsumeConfig {
            queue: queue_name.clone(),
            consumer_tag: format!("{}-mass-rebuild-checker", cfg.whoami()),
            no_local: false,
            no_ack: false,
            no_wait: false,
            exclusive: false,
        },
    )?;

    info!("Fetching jobs from {}", queue_name);
    block_on(handle);

    drop(conn); // Close connection.
    info!("Closed the session... EOF");
    Ok(())
}
