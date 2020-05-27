use std::env;

use async_std::task;
use lazy_static::lazy_static;
use prometheus::{
    register_counter_vec, register_gauge_vec, register_int_counter, CounterVec, Encoder, GaugeVec,
    IntCounter, TextEncoder,
};
use tide::http::StatusCode;
use tide::{Body, Request, Response};
use tracing::info;

use crate::ofborg;

lazy_static! {
    pub static ref JOBS_RECEIVED: IntCounter = register_int_counter!(
        "ofborg_jobs_received_total",
        "Total number of jobs received."
    )
    .unwrap();
    pub static ref BUILDS_RECEIVED: CounterVec = register_counter_vec!(
        "ofborg_builds_received_total",
        "Total number of builds received.",
        &["system"]
    )
    .unwrap();
    pub static ref BUILDS_FINISHED: CounterVec = register_counter_vec!(
        "ofborg_builds_finished_total",
        "Total number of builds finished.",
        &["system", "status"]
    )
    .unwrap();
    pub static ref BUILDS_ATTRIBUTES_ATTEMPTED: CounterVec = register_counter_vec!(
        "ofborg_builds_attributes_attempted_total",
        "Total number of attributes attempted to build.",
        &["system"]
    )
    .unwrap();
    pub static ref BUILDS_ATTRIBUTES_NOT_ATTEMPTED: CounterVec = register_counter_vec!(
        "ofborg_builds_attributes_not_attempted_total",
        "Total number of attributes not attempted to build.",
        &["system"]
    )
    .unwrap();
}

lazy_static! {
    static ref VERSION: GaugeVec = register_gauge_vec!(
        "ofborg_version",
        "Labeled OfBorg version information.",
        &["version"]
    )
    .unwrap();
}

pub fn spawn_server() {
    let port = env::var("METRICS_PORT").unwrap_or_else(|_err| String::from("9128"));

    let mut server = tide::new();
    server.at("/metrics").get(metrics);

    // Initialize version metric.
    VERSION.with_label_values(&[ofborg::VERSION]).set(1_f64);

    info!("Listening on http://127.0.0.1:{}/metrics", port);
    task::spawn(server.listen(format!("0.0.0.0:{}", port)));
}

async fn metrics(mut _req: Request<()>) -> Result<Response, tide::Error> {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer)?;

    let resp = Response::new(StatusCode::Ok).body(Body::from(buffer));
    Ok(resp)
}
