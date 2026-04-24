use std::net::SocketAddr;
use std::sync::Arc;

use async_trait::async_trait;
use hmac::{Hmac, KeyInit as _, Mac as _};
use http::{Method, StatusCode};
use http_body_util::{BodyExt as _, Full};
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use sha2::Sha256;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tracing::{error, info, warn};

use ofborg::ghevent::GenericWebhook;
use ofborg::{MessagePublisher, config, easyamqp, easyamqp::ChannelExt, easylapin};

pub struct LapinPublisher {
    chan: Arc<Mutex<lapin::Channel>>,
}

impl LapinPublisher {
    pub fn new(chan: Arc<Mutex<lapin::Channel>>) -> Self {
        Self { chan }
    }
}

#[async_trait]
impl MessagePublisher for LapinPublisher {
    async fn publish(&self, exchange: &str, routing_key: &str, body: &[u8]) -> anyhow::Result<()> {
        let chan = self.chan.lock().await;
        let _confirmation = chan
            .basic_publish(
                exchange.into(),
                routing_key.into(),
                lapin::options::BasicPublishOptions::default(),
                body,
                lapin::BasicProperties::default()
                    .with_content_type("application/json".into())
                    .with_delivery_mode(2),
            )
            .await?;
        Ok(())
    }
}

/// Prepares the the exchange we will write to, the queues that are bound to it
/// and binds them.
async fn setup_amqp(chan: &mut lapin::Channel) -> anyhow::Result<()> {
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

async fn handle_request<B, F>(
    req: Request<B>,
    webhook_secret: Arc<String>,
    publisher: Arc<dyn MessagePublisher>,
) -> Result<Response<Full<Bytes>>, hyper::Error>
where
    B: http_body::Body<Data = F> + Send + Sync,
    F: bytes::Buf,
    B::Error: std::fmt::Debug + Send + Sync,
{
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
            warn!("Failed to read body from client: {e:?}");
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
    if let Err(e) = publisher.publish("github-events", &routing_key, &raw).await {
        error!("Failed to publish message: {e}");
        return Ok(response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to publish message",
        ));
    }

    Ok(empty_response(StatusCode::NO_CONTENT))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    ofborg::setup_log();

    let arg = std::env::args()
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
    let publisher: Arc<dyn MessagePublisher> = Arc::new(LapinPublisher::new(chan.clone()));

    let addr: SocketAddr = cfg.listen.parse()?;
    let listener = TcpListener::bind(addr).await?;
    info!("Listening on {}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        let webhook_secret = webhook_secret.clone();
        let publisher = publisher.clone();

        tokio::task::spawn(async move {
            let service = service_fn(move |req| {
                handle_request(req, webhook_secret.clone(), publisher.clone())
            });

            if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                warn!("Error serving connection: {:?}", err);
            }
        });
    }
}

mod github_webhook_receiver {
    #[cfg(test)]
    mod test {
        use super::super::*;
        use http::header;

        use hyper::body::Bytes;
        use ofborg::test_utils::MockPublisher;

        fn create_request(
            method: http::Method,
            headers: Vec<(http::header::HeaderName, http::header::HeaderValue)>,
            body: Bytes,
        ) -> http::Request<http_body_util::Full<Bytes>> {
            let mut builder = http::Request::builder().method(method);
            for (name, value) in headers {
                builder = builder.header(name, value);
            }
            let body = http_body_util::Full::new(body);
            builder.body(body).unwrap()
        }

        fn hv(s: &str) -> http::header::HeaderValue {
            http::header::HeaderValue::from_str(s).unwrap()
        }

        fn hn(s: &'static str) -> http::header::HeaderName {
            http::header::HeaderName::from_bytes(s.as_bytes()).unwrap()
        }

        fn compute_signature(secret: &str, body: &[u8]) -> String {
            let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap();
            mac.update(body);
            let result = mac.finalize();
            format!("sha256={}", hex::encode(result.into_bytes()))
        }

        fn valid_headers(
            secret: &str,
            body: &[u8],
        ) -> Vec<(header::HeaderName, header::HeaderValue)> {
            vec![
                (header::CONTENT_TYPE, hv("application/json")),
                (
                    hn("X-Hub-Signature-256"),
                    hv(&compute_signature(secret, body)),
                ),
                (hn("X-Github-Event"), hv("pull_request")),
            ]
        }

        fn minimal_valid_webhook() -> &'static str {
            r#"{"repository":{"owner":{"login":"test"},"name":"test-repo","full_name":"test/test-repo","clone_url":"https://github.com/test/test-repo.git"}}"#
        }

        fn pr_event_webhook() -> &'static str {
            include_str!("../../test-srcs/events/pr-changed-base.json")
        }

        #[tokio::test]
        async fn test_method_not_allowed() {
            let publisher = Arc::new(MockPublisher::new());
            let secret = Arc::new("test-secret".to_string());
            let req = create_request(http::Method::GET, vec![], Bytes::new());

            let resp = handle_request(req, secret, publisher).await.unwrap();
            assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED);
        }

        #[tokio::test]
        async fn test_missing_signature_header() {
            let publisher = Arc::new(MockPublisher::new());
            let secret = Arc::new("test-secret".to_string());
            let body = Bytes::from(minimal_valid_webhook());
            let req = create_request(
                http::Method::POST,
                vec![(header::CONTENT_TYPE, hv("application/json"))],
                body,
            );

            let resp = handle_request(req, secret, publisher).await.unwrap();
            assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        }

        #[tokio::test]
        async fn test_invalid_signature_hash_method() {
            let publisher = Arc::new(MockPublisher::new());
            let secret = Arc::new("test-secret".to_string());
            let body = Bytes::from(minimal_valid_webhook());
            let req = create_request(
                http::Method::POST,
                vec![
                    (header::CONTENT_TYPE, hv("application/json")),
                    (hn("X-Hub-Signature-256"), hv("sha1=abc123")),
                ],
                body,
            );

            let resp = handle_request(req, secret, publisher).await.unwrap();
            assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        }

        #[tokio::test]
        async fn test_signature_verification_failed() {
            let publisher = Arc::new(MockPublisher::new());
            let secret = Arc::new("test-secret".to_string());
            let body = Bytes::from(minimal_valid_webhook());
            let req = create_request(
                http::Method::POST,
                vec![
                    (header::CONTENT_TYPE, hv("application/json")),
                    (
                        hn("X-Hub-Signature-256"),
                        hv(
                            "sha256=0000000000000000000000000000000000000000000000000000000000000000",
                        ),
                    ),
                ],
                body,
            );

            let resp = handle_request(req, secret, publisher).await.unwrap();
            assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        }

        #[tokio::test]
        async fn test_missing_content_type() {
            let publisher = Arc::new(MockPublisher::new());
            let secret = Arc::new("test-secret".to_string());
            let body = Bytes::from(minimal_valid_webhook());
            let req = create_request(
                http::Method::POST,
                vec![(
                    hn("X-Hub-Signature-256"),
                    hv(&compute_signature(
                        "test-secret",
                        minimal_valid_webhook().as_bytes(),
                    )),
                )],
                body,
            );

            let resp = handle_request(req, secret, publisher).await.unwrap();
            assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        }

        #[tokio::test]
        async fn test_invalid_content_type() {
            let publisher = Arc::new(MockPublisher::new());
            let secret = Arc::new("test-secret".to_string());
            let body = Bytes::from(minimal_valid_webhook());
            let req = create_request(
                http::Method::POST,
                vec![
                    (header::CONTENT_TYPE, hv("text/plain")),
                    (
                        hn("X-Hub-Signature-256"),
                        hv(&compute_signature(
                            "test-secret",
                            minimal_valid_webhook().as_bytes(),
                        )),
                    ),
                ],
                body,
            );

            let resp = handle_request(req, secret, publisher).await.unwrap();
            assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        }

        #[tokio::test]
        async fn test_invalid_json() {
            let publisher = Arc::new(MockPublisher::new());
            let secret = Arc::new("test-secret".to_string());
            let body = Bytes::from("not valid json {{{");
            let req = create_request(
                http::Method::POST,
                vec![
                    (header::CONTENT_TYPE, hv("application/json")),
                    (
                        hn("X-Hub-Signature-256"),
                        hv(&compute_signature("test-secret", b"not valid json {{{")),
                    ),
                    (hn("X-Github-Event"), hv("pull_request")),
                ],
                body,
            );

            let resp = handle_request(req, secret, publisher).await.unwrap();
            assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        }

        #[tokio::test]
        async fn test_missing_event_type() {
            let publisher = Arc::new(MockPublisher::new());
            let secret = Arc::new("test-secret".to_string());
            let body = Bytes::from(minimal_valid_webhook());
            let req = create_request(
                http::Method::POST,
                vec![
                    (header::CONTENT_TYPE, hv("application/json")),
                    (
                        hn("X-Hub-Signature-256"),
                        hv(&compute_signature(
                            "test-secret",
                            minimal_valid_webhook().as_bytes(),
                        )),
                    ),
                ],
                body,
            );

            let resp = handle_request(req, secret, publisher).await.unwrap();
            assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        }

        #[tokio::test]
        async fn test_successful_webhook_with_routing_key() {
            let publisher = Arc::new(MockPublisher::new());
            let secret = Arc::new("test-secret".to_string());
            let body_bytes = pr_event_webhook().as_bytes();
            let req = create_request(
                http::Method::POST,
                valid_headers("test-secret", body_bytes),
                Bytes::from(pr_event_webhook()),
            );

            let resp = handle_request(req, secret, publisher.clone())
                .await
                .unwrap();
            assert_eq!(resp.status(), StatusCode::NO_CONTENT);

            let published = publisher.get_published().await;
            assert_eq!(published.len(), 1);
            assert_eq!(published[0].exchange, "github-events");
            assert_eq!(published[0].routing_key, "pull_request.nixos/nixpkgs");
            assert_eq!(&published[0].body, pr_event_webhook().as_bytes());
        }

        #[tokio::test]
        async fn test_issue_comment_routing_key() {
            let publisher = Arc::new(MockPublisher::new());
            let secret = Arc::new("test-secret".to_string());
            let body = r#"{"repository":{"owner":{"login":"test"},"name":"my-repo","full_name":"test/my-repo","clone_url":"https://github.com/test/my-repo.git"}}"#;
            let body_bytes = body.as_bytes();

            let headers = vec![
                (header::CONTENT_TYPE, hv("application/json")),
                (
                    hn("X-Hub-Signature-256"),
                    hv(&compute_signature("test-secret", body_bytes)),
                ),
                (hn("X-Github-Event"), hv("issue_comment")),
            ];

            let req = create_request(http::Method::POST, headers, Bytes::from(body));
            let resp = handle_request(req, secret, publisher.clone())
                .await
                .unwrap();

            assert_eq!(resp.status(), StatusCode::NO_CONTENT);

            let published = publisher.get_published().await;
            assert_eq!(published[0].routing_key, "issue_comment.test/my-repo");
        }

        #[tokio::test]
        async fn test_routing_key_lowercases_repo_name() {
            let publisher = Arc::new(MockPublisher::new());
            let secret = Arc::new("test-secret".to_string());
            let body = r#"{"repository":{"owner":{"login":"Test"},"name":"MyRepo","full_name":"Test/MyRepo","clone_url":"https://github.com/Test/MyRepo.git"}}"#;
            let body_bytes = body.as_bytes();

            let headers = vec![
                (header::CONTENT_TYPE, hv("application/json")),
                (
                    hn("X-Hub-Signature-256"),
                    hv(&compute_signature("test-secret", body_bytes)),
                ),
                (hn("X-Github-Event"), hv("pull_request")),
            ];

            let req = create_request(http::Method::POST, headers, Bytes::from(body));
            let resp = handle_request(req, secret, publisher.clone())
                .await
                .unwrap();

            assert_eq!(resp.status(), StatusCode::NO_CONTENT);

            let published = publisher.get_published().await;
            assert_eq!(published[0].routing_key, "pull_request.test/myrepo");
        }
    }
}
