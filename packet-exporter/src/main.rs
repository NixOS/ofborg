use std::error::Error;

use async_std::task;
use lazy_static::lazy_static;
use prometheus::{register_gauge_vec, Encoder, GaugeVec, TextEncoder};
use serde_derive::{Deserialize, Serialize};
use tide::http::StatusCode;
use tide::{Body, Request, Response};

#[derive(Debug, Serialize, Deserialize)]
struct StatusPage {
    status: Status,
}

#[derive(Debug, Serialize, Deserialize)]
struct Status {
    indicator: String,
    description: String,
}

static STATUS_URL: &'static str = "https://status.packet.com/api/v2/status.json";

lazy_static! {
    static ref PACKET_STATUS: GaugeVec = register_gauge_vec!(
        "packet_status_page_indicator",
        "The current indicator of the packet status page.",
        &["indicator"]
    )
    .unwrap();
}

async fn metrics(mut _req: Request<()>) -> Result<Response, tide::Error> {
    for indicator in vec!["none", "minor", "critical"] {
        PACKET_STATUS.with_label_values(&[indicator]).set(0 as f64);
    }

    let page: StatusPage = surf::get(STATUS_URL).recv_json().await?;
    println!(
        "Packet status {}, {}",
        page.status.indicator, page.status.description
    );

    PACKET_STATUS
        .with_label_values(&[&page.status.indicator])
        .set(1 as f64);

    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer)?;

    let resp = Response::new(StatusCode::Ok).body(Body::from(buffer));
    Ok(resp)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut server = tide::new();

    server.at("/metrics").get(metrics);

    println!("Listening on http://127.0.0.1:9122");
    task::block_on(server.listen("0.0.0.0:9122"))?;
    Ok(())
}
