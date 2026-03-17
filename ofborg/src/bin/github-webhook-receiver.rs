use std::env;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;

use hmac::{Hmac, Mac};
use http::{Method, StatusCode};
use http_body_util::{BodyExt, Full};
use hyper::body::{Bytes, Incoming};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use lapin::options::BasicPublishOptions;
use lapin::{BasicProperties, Channel};
use ofborg::ghevent::GenericWebhook;
use ofborg::{config, easyamqp, easyamqp::ChannelExt, easylapin};
use sha2::Sha256;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tracing::{error, info, warn};

/// Prepares the the exchange we will write to, the queues that are bound to it
/// and binds them.
async fn setup_amqp(chan: &mut Channel) -> Result<(), Box<dyn Error + Send + Sync>> {
    chan.declare_exchange(easyamqp::ExchangeConfig {
        exchange: "github-events".to_owned(),
        exchange_type: easyamqp::ExchangeType::Topic,
        passive: false,
        durable: true,
        auto_delete: false,
        no_wait: false,
        internal: false,
    })
    .await?;

    let queue_name = String::from("build-inputs");
    chan.declare_queue(easyamqp::QueueConfig {
        queue: queue_name.clone(),
        passive: false,
        durable: true,
        exclusive: false,
        auto_delete: false,
        no_wait: false,
    })
    .await?;
    chan.bind_queue(easyamqp::BindQueueConfig {
        queue: queue_name.clone(),
        exchange: "github-events".to_owned(),
        routing_key: Some(String::from("issue_comment.*")),
        no_wait: false,
    })
    .await?;

    let queue_name = String::from("github-events-unknown");
    chan.declare_queue(easyamqp::QueueConfig {
        queue: queue_name.clone(),
        passive: false,
        durable: true,
        exclusive: false,
        auto_delete: false,
        no_wait: false,
    })
    .await?;
    chan.bind_queue(easyamqp::BindQueueConfig {
        queue: queue_name.clone(),
        exchange: "github-events".to_owned(),
        routing_key: Some(String::from("unknown.*")),
        no_wait: false,
    })
    .await?;

    let queue_name = String::from("mass-rebuild-check-inputs");
    chan.declare_queue(easyamqp::QueueConfig {
        queue: queue_name.clone(),
        passive: false,
        durable: true,
        exclusive: false,
        auto_delete: false,
        no_wait: false,
    })
    .await?;
    chan.bind_queue(easyamqp::BindQueueConfig {
        queue: queue_name.clone(),
        exchange: "github-events".to_owned(),
        routing_key: Some(String::from("pull_request.*")),
        no_wait: false,
    })
    .await?;
    Ok(())
}

fn response(status: StatusCode, body: &'static str) -> Response<Full<Bytes>> {
    Response::builder()
        .status(status)
        .body(Full::new(Bytes::from(body)))
        .unwrap()
}

fn empty_response(status: StatusCode) -> Response<Full<Bytes>> {
    Response::builder()
        .status(status)
        .body(Full::new(Bytes::new()))
        .unwrap()
}

async fn handle_request(
    req: Request<Incoming>,
    webhook_secret: Arc<String>,
    chan: Arc<Mutex<Channel>>,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    // HTTP 405
    if req.method() != Method::POST {
        return Ok(empty_response(StatusCode::METHOD_NOT_ALLOWED));
    }

    // Get headers before consuming body
    let sig_header = req
        .headers()
        .get("X-Hub-Signature-256")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());
    let event_type = req
        .headers()
        .get("X-Github-Event")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());
    let content_type = req
        .headers()
        .get("Content-Type")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    // Read body
    let raw = match req.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(e) => {
            warn!("Failed to read body from client: {e}");
            return Ok(response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read body",
            ));
        }
    };

    // Validate signature
    let Some(sig) = sig_header else {
        return Ok(response(
            StatusCode::BAD_REQUEST,
            "Missing signature header",
        ));
    };
    let mut components = sig.splitn(2, '=');
    let Some(algo) = components.next() else {
        return Ok(response(
            StatusCode::BAD_REQUEST,
            "Signature hash method missing",
        ));
    };
    let Some(hash) = components.next() else {
        return Ok(response(StatusCode::BAD_REQUEST, "Signature hash missing"));
    };
    let Ok(hash) = hex::decode(hash) else {
        return Ok(response(
            StatusCode::BAD_REQUEST,
            "Invalid signature hash hex",
        ));
    };

    if algo != "sha256" {
        return Ok(response(
            StatusCode::BAD_REQUEST,
            "Invalid signature hash method",
        ));
    }

    let Ok(mut mac) = Hmac::<Sha256>::new_from_slice(webhook_secret.as_bytes()) else {
        error!("Unable to create HMAC from secret");
        return Ok(response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal error",
        ));
    };
    mac.update(&raw);
    if mac.verify_slice(hash.as_slice()).is_err() {
        return Ok(response(
            StatusCode::BAD_REQUEST,
            "Signature verification failed",
        ));
    }

    // Parse body
    let Some(ct) = content_type else {
        return Ok(response(
            StatusCode::BAD_REQUEST,
            "No Content-Type header passed",
        ));
    };
    if !ct.contains("application/json") {
        return Ok(response(
            StatusCode::BAD_REQUEST,
            "Content-Type is not application/json. Webhook misconfigured?",
        ));
    }

    let input = match serde_json::from_slice::<GenericWebhook>(&raw) {
        Ok(i) => i,
        Err(e) => {
            error!("Invalid JSON received: {e}");
            return Ok(response(StatusCode::BAD_REQUEST, "Invalid JSON"));
        }
    };

    // Build routing key
    let Some(event_type) = event_type else {
        return Ok(response(StatusCode::BAD_REQUEST, "Missing event type"));
    };
    let routing_key = format!("{event_type}.{}", input.repository.full_name.to_lowercase());

    // Publish message
    let chan = chan.lock().await;
    let _confirmation = chan
        .basic_publish(
            "github-events".into(),
            routing_key.as_str().into(),
            BasicPublishOptions::default(),
            &raw,
            BasicProperties::default()
                .with_content_type("application/json".into())
                .with_delivery_mode(2), // persistent
        )
        .await;

    Ok(empty_response(StatusCode::NO_CONTENT))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    ofborg::setup_log();

    let arg = env::args()
        .nth(1)
        .unwrap_or_else(|| panic!("usage: {} <config>", std::env::args().next().unwrap()));
    let Some(cfg) = config::load(arg.as_ref()).github_webhook_receiver else {
        error!("No GitHub Webhook configuration found!");
        panic!();
    };

    let webhook_secret = std::fs::read_to_string(cfg.webhook_secret_file)
        .expect("Unable to read webhook secret file");
    let webhook_secret = Arc::new(webhook_secret.trim().to_string());

    let conn = easylapin::from_config(&cfg.rabbitmq).await?;
    let mut chan = conn.create_channel().await?;
    setup_amqp(&mut chan).await?;
    let chan = Arc::new(Mutex::new(chan));

    let addr: SocketAddr = cfg.listen.parse()?;
    let listener = TcpListener::bind(addr).await?;
    info!("Listening on {}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        let webhook_secret = webhook_secret.clone();
        let chan = chan.clone();

        tokio::task::spawn(async move {
            let service =
                service_fn(move |req| handle_request(req, webhook_secret.clone(), chan.clone()));

            if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                warn!("Error serving connection: {:?}", err);
            }
        });
    }
}
