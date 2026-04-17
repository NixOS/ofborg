use std::net::SocketAddr;
use std::{collections::HashMap, error::Error, path::PathBuf, sync::Arc};

use http::{Method, StatusCode};
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use ofborg::config;
use tokio::net::TcpListener;
use tracing::{error, info, warn};

#[derive(serde::Serialize, Default)]
struct Attempt {
    metadata: Option<serde_json::Value>,
    result: Option<serde_json::Value>,
    log_url: Option<String>,
}

#[derive(serde::Serialize)]
struct LogResponse {
    attempts: HashMap<String, Attempt>,
}

#[derive(Clone)]
struct LogApiConfig {
    logs_path: PathBuf,
    serve_root: String,
}

fn response(status: StatusCode, body: &'static str) -> Response<Full<Bytes>> {
    Response::builder()
        .status(status)
        .body(Full::new(Bytes::from(body)))
        .unwrap()
}

fn json_response(status: StatusCode, body: String) -> Response<Full<Bytes>> {
    Response::builder()
        .status(status)
        .header("Content-Type", "application/json")
        .body(Full::new(Bytes::from(body)))
        .unwrap()
}

async fn handle_request(
    req: Request<hyper::body::Incoming>,
    cfg: Arc<LogApiConfig>,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    if req.method() != Method::GET {
        return Ok(response(StatusCode::METHOD_NOT_ALLOWED, ""));
    }

    let uri = req.uri().path().to_string();
    let Some(reqd) = uri.strip_prefix("/logs/").map(ToOwned::to_owned) else {
        return Ok(response(StatusCode::NOT_FOUND, "invalid uri"));
    };
    let path: PathBuf = cfg.logs_path.join(&reqd);
    let Ok(path) = std::fs::canonicalize(&path) else {
        return Ok(response(StatusCode::NOT_FOUND, "absent"));
    };
    if !path.starts_with(&cfg.logs_path) {
        return Ok(response(StatusCode::NOT_FOUND, "invalid path"));
    }
    let Ok(iter) = std::fs::read_dir(path) else {
        return Ok(response(StatusCode::NOT_FOUND, "non dir"));
    };

    let mut attempts = HashMap::<String, Attempt>::new();
    for e in iter {
        let Ok(e) = e else { continue };
        let e_metadata = e.metadata();
        if e_metadata.as_ref().map(|v| v.is_dir()).unwrap_or(true) {
            return Ok(response(StatusCode::INTERNAL_SERVER_ERROR, "dir found"));
        }

        if e_metadata.as_ref().map(|v| v.is_file()).unwrap_or_default() {
            let Ok(file_name) = e.file_name().into_string() else {
                warn!("entry filename is not a utf-8 string: {:?}", e.file_name());
                continue;
            };

            if file_name.ends_with(".metadata.json") || file_name.ends_with(".result.json") {
                let Ok(file) = std::fs::File::open(e.path()) else {
                    warn!("could not open file: {file_name}");
                    continue;
                };
                let Ok(json) = serde_json::from_reader::<_, serde_json::Value>(file) else {
                    warn!("file is not a valid json file: {file_name}");
                    continue;
                };
                let Some(attempt_id) = json
                    .get("attempt_id")
                    .and_then(|v| v.as_str())
                    .map(ToOwned::to_owned)
                else {
                    warn!("attempt_id not found in file: {file_name}");
                    continue;
                };
                let attempt_obj = attempts.entry(attempt_id).or_default();
                if file_name.ends_with(".metadata.json") {
                    attempt_obj.metadata = Some(json);
                } else {
                    attempt_obj.result = Some(json);
                }
            } else {
                let attempt_obj = attempts.entry(file_name.clone()).or_default();
                attempt_obj.log_url = Some(format!("{}/{reqd}/{file_name}", &cfg.serve_root));
            }
        }
    }

    let body = serde_json::to_string(&LogResponse { attempts }).unwrap_or_default();
    Ok(json_response(StatusCode::OK, body))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    ofborg::setup_log();

    let arg = std::env::args()
        .nth(1)
        .unwrap_or_else(|| panic!("usage: {} <config>", std::env::args().next().unwrap()));
    let Some(cfg) = config::load(arg.as_ref()).log_api_config else {
        error!("No LogApi configuration found!");
        panic!();
    };

    let logs_path = std::fs::canonicalize(&cfg.logs_path)
        .expect("logs_path does not exist or is not accessible");

    let api_cfg = Arc::new(LogApiConfig {
        logs_path,
        serve_root: cfg.serve_root,
    });

    let addr: SocketAddr = cfg.listen.parse()?;
    let listener = TcpListener::bind(addr).await?;
    info!("Listening on {}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        let api_cfg = api_cfg.clone();

        tokio::task::spawn(async move {
            let service = service_fn(move |req| handle_request(req, api_cfg.clone()));

            if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                warn!("Error serving connection: {:?}", err);
            }
        });
    }
}
