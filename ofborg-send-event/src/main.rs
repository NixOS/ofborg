use anyhow::{Context as _, Result, bail};
use clap::Parser;
use hmac::KeyInit;
use hmac::{Hmac, Mac as _};
use octocrab::Octocrab;
use ofborg::ghevent::{
    PullRequest, PullRequestAction, PullRequestEvent, PullRequestRef, PullRequestState, Repository,
    User,
};
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue, USER_AGENT};
use sha2::Sha256;
use std::str::FromStr as _;
use uuid::Uuid;

#[derive(Debug, Parser)]
#[command(about = "Send a GitHub-style webhook to a local or staging receiver")]
struct Args {
    /// Destination webhook URL
    #[arg(long, default_value = "http://localhost:9899")]
    webhook_receiver_url: String,

    /// PR that should be fetched
    #[arg(long)]
    pr_nr: u64,

    /// Webhook event name, e.g. push, pull_request, ping
    #[arg(long, default_value = "pull_request")]
    event: String,

    /// Shared secret path used to generate X-Hub-Signature-256
    #[arg(long)]
    secret_path: Option<std::path::PathBuf>,

    /// Delivery ID header. Defaults to a random UUID.
    #[arg(long)]
    delivery_id: Option<String>,

    /// Add arbitrary header(s), format: Name=Value
    #[arg(long = "header")]
    headers: Vec<String>,

    /// Print response headers too
    #[arg(long)]
    verbose: bool,

    /// Timeout in seconds
    #[arg(long, default_value_t = 30)]
    timeout_secs: u64,

    /// Full Repo Name which should be used as repo
    #[arg(long, default_value = "NixOS/nixpkgs")]
    full_repo_name: String,
}

async fn make_pull_request_body(full_repo_name: &str, number: u64) -> Result<PullRequestEvent> {
    let (org_name, repo_name) = full_repo_name
        .split_once("/")
        .with_context(|| format!("Unexpected Full Repo Name! {full_repo_name}"))?;

    let octocrab = Octocrab::builder()
        .build()
        .context("failed to create Octocrab client")?;
    let pr = octocrab
        .pulls(org_name, repo_name)
        .get(number)
        .await
        .with_context(|| format!("failed to fetch PR {number}"))?;
    tracing::info!("Fetched PR {number}: {}", pr.title);

    Ok(PullRequestEvent {
        action: PullRequestAction::Opened,
        number,
        repository: Repository {
            owner: User {
                login: org_name.to_owned(),
            },
            name: repo_name.to_owned(),
            full_name: format!("{org_name}/{repo_name}"),
            clone_url: format!("https://github.com/{org_name}/{repo_name}.git"),
        },
        pull_request: PullRequest {
            state: PullRequestState::Open,
            base: PullRequestRef {
                git_ref: pr.base.ref_field,
                sha: pr.base.sha,
            },
            head: PullRequestRef {
                git_ref: pr.head.ref_field,
                sha: pr.head.sha,
            },
        },
        changes: None,
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    ofborg::setup_log();
    let args = Args::parse();

    let event = make_pull_request_body(&args.full_repo_name, args.pr_nr).await?;
    let body = serde_json::to_vec(&event)?;

    let delivery_id = args
        .delivery_id
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(args.timeout_secs))
        .build()
        .context("failed to build reqwest client")?;

    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("ofborg-send-event/0.1"),
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(
        HeaderName::from_static("x-github-event"),
        HeaderValue::from_str(&args.event).context("invalid X-GitHub-Event value")?,
    );
    headers.insert(
        HeaderName::from_static("x-github-delivery"),
        HeaderValue::from_str(&delivery_id).context("invalid X-GitHub-Delivery value")?,
    );

    if let Some(secret_path) = &args.secret_path {
        let secret = std::fs::read_to_string(secret_path)?;
        let signature = github_signature_256(secret.trim(), &body)?;
        headers.insert(
            HeaderName::from_static("x-hub-signature-256"),
            HeaderValue::from_str(&signature).context("invalid X-Hub-Signature-256 value")?,
        );
    }

    for raw in &args.headers {
        let (name, value) = parse_header(raw)?;
        headers.insert(name, value);
    }

    let response = client
        .post(&args.webhook_receiver_url)
        .headers(headers)
        .body(body)
        .send()
        .await
        .with_context(|| format!("failed to POST to {}", args.webhook_receiver_url))?;

    let status = response.status();
    let resp_headers = response.headers().clone();
    let resp_body = response
        .text()
        .await
        .context("failed to read response body")?;

    println!("status: {}", status);

    if args.verbose {
        println!("response headers:");
        for (name, value) in resp_headers.iter() {
            println!(
                "{}: {}",
                name.as_str(),
                value.to_str().unwrap_or("<non-utf8>")
            );
        }
    }

    if !resp_body.is_empty() {
        println!();
        println!("{}", resp_body);
    }

    Ok(())
}

fn github_verify_signature(secret: &str, body: &[u8], tag: &[u8]) -> Result<bool> {
    let mut mac =
        Hmac::<Sha256>::new_from_slice(secret.as_bytes()).context("invalid HMAC secret bytes")?;
    mac.update(body);
    Ok(mac.verify_slice(tag).is_ok())
}

fn github_signature_256(secret: &str, body: &[u8]) -> Result<String> {
    let mut mac =
        Hmac::<Sha256>::new_from_slice(secret.as_bytes()).context("invalid HMAC secret bytes")?;
    mac.update(body);
    let tag = mac.finalize().into_bytes();
    assert!(github_verify_signature(secret, body, &tag)?);
    Ok(format!("sha256={}", hex::encode(tag)))
}

fn parse_header(input: &str) -> Result<(HeaderName, HeaderValue)> {
    let Some((name, value)) = input.split_once('=') else {
        bail!(
            "invalid --header format: expected Name=Value, got {}",
            input
        );
    };

    let name = HeaderName::from_str(name.trim())
        .with_context(|| format!("invalid header name: {}", name.trim()))?;
    let value = HeaderValue::from_str(value.trim())
        .with_context(|| format!("invalid header value for {}", name))?;

    Ok((name, value))
}
