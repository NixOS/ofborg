use std::env;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;

use http::StatusCode;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use ofborg::block_on;
use tokio::net::TcpListener;
use tracing::{error, info, warn};

use ofborg::easyamqp::{ChannelExt, ConsumerExt};
use ofborg::{config, easyamqp, easylapin, stats, tasks};

fn response(body: String) -> Response<Full<Bytes>> {
    Response::builder()
        .status(StatusCode::OK)
        .body(Full::new(Bytes::from(body)))
        .unwrap()
}

async fn run_http_server(
    addr: SocketAddr,
    metrics: Arc<stats::MetricCollector>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let listener = TcpListener::bind(addr).await?;
    info!("HTTP server listening on {}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        let metrics = metrics.clone();

        tokio::task::spawn(async move {
            let service = service_fn(move |_req: Request<hyper::body::Incoming>| {
                let metrics = metrics.clone();
                async move { Ok::<_, hyper::Error>(response(metrics.prometheus_output())) }
            });

            if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                warn!("Error serving connection: {:?}", err);
            }
        });
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    ofborg::setup_log();

    let arg = env::args()
        .nth(1)
        .unwrap_or_else(|| panic!("usage: {} <config>", std::env::args().next().unwrap()));
    let cfg = config::load(arg.as_ref());

    let Some(stats_cfg) = config::load(arg.as_ref()).stats else {
        error!("No stats configuration found!");
        panic!();
    };

    let conn = easylapin::from_config(&stats_cfg.rabbitmq)?;

    let mut chan = block_on(conn.create_channel())?;

    let events = stats::RabbitMq::from_lapin(&cfg.whoami(), block_on(conn.create_channel())?);

    let metrics = Arc::new(stats::MetricCollector::new());
    let collector = tasks::statscollector::StatCollectorWorker::new(events, (*metrics).clone());

    chan.declare_exchange(easyamqp::ExchangeConfig {
        exchange: "stats".to_owned(),
        exchange_type: easyamqp::ExchangeType::Fanout,
        passive: false,
        durable: true,
        auto_delete: false,
        no_wait: false,
        internal: false,
    })?;

    let queue_name = String::from("stats-events");
    chan.declare_queue(easyamqp::QueueConfig {
        queue: queue_name.clone(),
        passive: false,
        durable: true,
        exclusive: false,
        auto_delete: false,
        no_wait: false,
    })?;

    chan.bind_queue(easyamqp::BindQueueConfig {
        queue: queue_name.clone(),
        exchange: "stats".to_owned(),
        routing_key: None,
        no_wait: false,
    })?;

    let handle = chan.consume(
        collector,
        easyamqp::ConsumeConfig {
            queue: "stats-events".to_owned(),
            consumer_tag: format!("{}-prometheus-stats-collector", cfg.whoami()),
            no_local: false,
            no_ack: false,
            no_wait: false,
            exclusive: false,
        },
    )?;

    // Spawn HTTP server in a separate thread with its own tokio runtime
    let metrics_clone = metrics.clone();
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
        let addr: SocketAddr = "0.0.0.0:9898".parse().unwrap();
        if let Err(e) = rt.block_on(run_http_server(addr, metrics_clone)) {
            error!("HTTP server error: {:?}", e);
        }
    });

    info!("Fetching jobs from {}", &queue_name);
    block_on(handle);

    drop(conn); // Close connection.
    info!("Closed the session... EOF");
    Ok(())
}
