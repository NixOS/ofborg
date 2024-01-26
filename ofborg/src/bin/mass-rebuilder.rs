use std::env;
use std::error::Error;
use std::path::Path;
use std::process;

use async_std::task;
use tracing::{error, info};

use ofborg::checkout;
use ofborg::config;
use ofborg::easyamqp::{self, ChannelExt, ConsumerExt};
use ofborg::easylapin;
use ofborg::stats;
use ofborg::tasks;

// FIXME: remove with rust/cargo update
#[allow(clippy::cognitive_complexity)]
fn main() -> Result<(), Box<dyn Error>> {
    ofborg::setup_log();

    let arg = env::args().nth(1).expect("usage: mass-rebuilder <config>");
    let cfg = config::load(arg.as_ref());

    let memory_info = sys_info::mem_info().expect("Unable to get memory information from OS");

    if memory_info.avail < 8 * 1024 * 1024 {
        // seems this stuff is in kilobytes?
        error!(
            "Less than 8Gb of memory available (got {:.2}Gb). Aborting.",
            (memory_info.avail as f32) / 1024.0 / 1024.0
        );
        process::exit(1);
    };

    let conn = easylapin::from_config(&cfg.rabbitmq)?;
    let mut chan = task::block_on(conn.create_channel())?;

    let root = Path::new(&cfg.checkout.root);
    let cloner = checkout::cached_cloner(&root.join(cfg.runner.instance.to_string()));
    let nix = cfg.nix();

    let events = stats::RabbitMq::from_lapin(&cfg.whoami(), task::block_on(conn.create_channel())?);

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
            &nix,
            cfg.github(),
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
    task::block_on(handle);

    drop(conn); // Close connection.
    info!("Closed the session... EOF");
    Ok(())
}
