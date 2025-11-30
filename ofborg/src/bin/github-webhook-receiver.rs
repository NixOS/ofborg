use std::env;
use std::error::Error;
use std::io::Read as _;
#[macro_use]
extern crate hyper;

use async_std::task;
use hmac::{Hmac, Mac};
use hyper::header::ContentType;
use hyper::mime;
use hyper::{
    server::{Request, Response, Server},
    status::StatusCode,
};
use lapin::options::BasicPublishOptions;
use lapin::{BasicProperties, Channel};
use ofborg::ghevent::GenericWebhook;
use ofborg::{config, easyamqp, easyamqp::ChannelExt, easylapin};
use sha2::Sha256;
use tracing::{error, info, warn};

header! { (XHubSignature256, "X-Hub-Signature-256") => [String] }
header! { (XGithubEvent, "X-Github-Event") => [String] }

/// Prepares the the exchange we will write to, the queues that are bound to it
/// and binds them.
fn setup_amqp(chan: &mut Channel) -> Result<(), Box<dyn Error>> {
    chan.declare_exchange(easyamqp::ExchangeConfig {
        exchange: "github-events".to_owned(),
        exchange_type: easyamqp::ExchangeType::Topic,
        passive: false,
        durable: true,
        auto_delete: false,
        no_wait: false,
        internal: false,
    })?;

    let queue_name = String::from("build-inputs");
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
        exchange: "github-events".to_owned(),
        routing_key: Some(String::from("issue_comment.*")),
        no_wait: false,
    })?;

    let queue_name = String::from("github-events-unknown");
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
        exchange: "github-events".to_owned(),
        routing_key: Some(String::from("unknown.*")),
        no_wait: false,
    })?;

    let queue_name = String::from("mass-rebuild-check-inputs");
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
        exchange: "github-events".to_owned(),
        routing_key: Some(String::from("pull_request.*")),
        no_wait: false,
    })?;
    Ok(())
}

fn handle_request(mut req: Request, mut res: Response, webhook_secret: &str, chan: &Channel) {
    // HTTP 405
    if req.method != hyper::Post {
        *res.status_mut() = StatusCode::MethodNotAllowed;
        return;
    }
    let hdr = req.headers.clone();

    // Read body
    let mut raw = Vec::new();
    if req.read_to_end(&mut raw).is_err() {
        warn!("Failed to read body from client");
        *res.status_mut() = StatusCode::InternalServerError;
        return;
    }
    let raw = raw.as_slice();

    // Validate signature
    {
        let Some(sig) = hdr.get::<XHubSignature256>() else {
            *res.status_mut() = StatusCode::BadRequest;
            let _ = res.send(b"Missing signature header");
            return;
        };
        let mut components = sig.splitn(2, '=');
        let Some(algo) = components.next() else {
            *res.status_mut() = StatusCode::BadRequest;
            let _ = res.send(b"Signature hash method missing");
            return;
        };
        let Some(hash) = components.next() else {
            *res.status_mut() = StatusCode::BadRequest;
            let _ = res.send(b"Signature hash missing");
            return;
        };
        let Ok(hash) = hex::decode(hash) else {
            *res.status_mut() = StatusCode::BadRequest;
            let _ = res.send(b"Invalid signature hash hex");
            return;
        };

        if algo != "sha256" {
            *res.status_mut() = StatusCode::BadRequest;
            let _ = res.send(b"Invalid signature hash method");
            return;
        }

        let Ok(mut mac) = Hmac::<Sha256>::new_from_slice(webhook_secret.as_bytes()) else {
            *res.status_mut() = StatusCode::InternalServerError;
            error!("Unable to create HMAC from secret");
            return;
        };
        mac.update(raw);
        if mac.verify_slice(hash.as_slice()).is_err() {
            *res.status_mut() = StatusCode::BadRequest;
            let _ = res.send(b"Signature verification failed");
            return;
        }
    }

    // Parse body
    let Some(ct) = hdr.get::<ContentType>() else {
        *res.status_mut() = StatusCode::BadRequest;
        let _ = res.send(b"No Content-Type header passed");
        return;
    };
    if ct
        != &ContentType(mime::Mime(
            mime::TopLevel::Application,
            mime::SubLevel::Json,
            Vec::new(),
        ))
    {
        *res.status_mut() = StatusCode::BadRequest;
        let _ = res.send(b"Content-Type is not application/json. Webhook misconfigured?");
        return;
    }
    let input = match serde_json::from_slice::<GenericWebhook>(raw) {
        Ok(i) => i,
        Err(e) => {
            *res.status_mut() = StatusCode::BadRequest;
            let _ = res.send(b"Invalid JSON");
            error!("Invalid JSON received: {e}");
            return;
        }
    };

    // Build routing key
    let Some(event_type) = hdr.get::<XGithubEvent>() else {
        *res.status_mut() = StatusCode::BadRequest;
        let _ = res.send(b"Missing event type");
        return;
    };
    let routing_key = format!("{event_type}.{}", input.repository.full_name.to_lowercase());

    // Publish message
    let _confirmation = task::block_on(async {
        chan.basic_publish(
            "github-events",
            &routing_key,
            BasicPublishOptions::default(),
            raw,
            BasicProperties::default()
                .with_content_type("application/json".into())
                .with_delivery_mode(2), // persistent
        )
        .await
    });
    *res.status_mut() = StatusCode::NoContent;
}

fn main() -> Result<(), Box<dyn Error>> {
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
    let webhook_secret = webhook_secret.trim().to_string();

    let conn = easylapin::from_config(&cfg.rabbitmq)?;
    let mut chan = task::block_on(conn.create_channel())?;
    setup_amqp(&mut chan)?;

    //let events = stats::RabbitMq::from_lapin(&cfg.whoami(), task::block_on(conn.create_channel())?);
    let threads = std::thread::available_parallelism()
        .map(|x| x.get())
        .unwrap_or(1);
    info!("Will listen on {} with {threads} threads", cfg.listen);
    Server::http(cfg.listen)?.handle_threads(
        move |req: Request, res: Response| {
            handle_request(req, res, &webhook_secret, &chan);
        },
        threads,
    )?;
    Ok(())
}
