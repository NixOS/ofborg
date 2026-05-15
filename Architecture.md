# Architecture

## Overview

ofborg is a Rust-based CI system for [Nixpkgs](https://github.com/nixos/nixpkgs). It runs as a
[GitHub App](https://docs.github.com/en/apps) that automatically processes pull requests and issue
comments. The system uses **RabbitMQ** for asynchronous message passing between components. Each
component runs as an independent process communicating exclusively through exchanges and queues.

## Components

| Binary | Purpose | Consumes From | Publishes To |
|--------|---------|---------------|--------------|
| `github-webhook-receiver` | HTTP server receiving GitHub webhooks. Validates HMAC-SHA256 signatures, parses the event type, and routes raw JSON to RabbitMQ. | GitHub HTTP POST | `github-events` (topic exchange) |
| `github-comment-filter` | Consumes issue comment events, parses `@ofborg` commands (`build` / `eval`), checks ACL (repo authorization, trusted users), fetches PR data from GitHub API, and schedules build/eval jobs. | `build-inputs` queue | `build-jobs` (fanout exchange), `mass-rebuild-check-jobs` (queue via default exchange), `build-results` (fanout exchange) |
| `evaluation-filter` | Filters pull request events for mass rebuild evaluation. Only processes Opened, Synchronize, Reopened, and Edited (base change) events on authorized repos. | `mass-rebuild-check-inputs` queue | `mass-rebuild-check-jobs` (queue via default exchange) |
| `mass-rebuilder` | Runs nixpkgs evaluation to determine which packages a PR impacts. Clones nixpkgs, merges the PR, runs nix-instantiate on release expressions, computes the diff of out-paths, and schedules builds for changed packages. | `mass-rebuild-check-jobs` queue | `build-jobs` (fanout exchange), `build-results` (fanout exchange) |
| `builder` | Performs actual Nix builds on a specific system architecture. Clones the repo, fetches the PR, merges, partitions attrs into buildable/non-buildable, runs nix-build, streams log lines, and publishes results. One instance per architecture. | `build-inputs-{system}` queue (via `build-jobs` fanout) | `logs` (topic exchange), `build-results` (fanout exchange) |
| `github-comment-poster` | Creates and updates GitHub Check Runs on pull requests. On receipt of `QueuedBuildJobs` creates an in-progress check; on receipt of `BuildResult` creates a completed check run. | `build-results` queue (via `build-results` fanout) | GitHub Check Runs API |
| `log-message-collector` | Persists build log messages to the filesystem. Receives `BuildLogStart`, `BuildLogMsg`, and `BuildResult` messages and writes metadata, log lines, and result JSON to disk. | Server-named exclusive queue (via `logs` topic exchange) | Filesystem (log storage) |
| `logapi` | HTTP server that serves build log metadata and results via JSON API at `GET /logs/{repo}`. | Filesystem (log storage) | HTTP JSON responses |
| `stats` | Dual-purpose binary: (1) Prometheus metrics HTTP server at `/metrics`, (2) consumer of `EventMessage` payloads from the stats exchange for metric collection. | `stats-events` queue (via `stats` fanout) | HTTP Prometheus `/metrics` |
| `build-faker` | Debug tool that injects a fake `BuildJob` directly into a system queue for testing. | CLI arguments | `build-inputs-{system}` queue |

## Data Flow

### 1. Webhook Reception

GitHub sends webhook events to `github-webhook-receiver` via HTTP POST. The receiver validates the
`X-Hub-Signature-256` HMAC header, parses the `X-GitHub-Event` header, and publishes the raw JSON
body to the `github-events` **topic exchange** with a routing key of `{event}.{owner}/{repo}` (e.g.,
`pull_request.nixos/nixpkgs`).

```
GitHub ──HTTP POST──▶ github-webhook-receiver ──publish──▶ github-events (topic)
```

### 2. Direct Build Flow (Issue Comments)

When someone comments `@ofborg build` on a PR:

1. **`github-comment-filter`** consumes the `issue_comment.*` event from `build-inputs` queue.
2. It parses the comment, checks the ACL (authorized repos, trusted users), fetches PR data from the
   GitHub API, and publishes:
   - A `BuildJob` message to the `build-jobs` **fanout exchange** (distributed to all per-system queues).
   - A `QueuedBuildJobs` message to the `build-results` **fanout exchange** (for creating in-progress
     check runs).

```
github-events ──issue_comment.*──▶ build-inputs ──▶ github-comment-filter
                                                      ├──▶ build-jobs (fanout)
                                                      └──▶ build-results (fanout)
```

### 3. Mass Rebuild Flow (Pull Requests)

When a PR is opened or updated:

1. **`evaluation-filter`** consumes the `pull_request.*` event from `mass-rebuild-check-inputs`.
   This queue has two bindings to `github-events`: `pull_request.*` (from the webhook-receiver) and
   `pull_request.nixos/*` (from the evaluation-filter itself).
2. It filters for Opened, Synchronize, Reopened, and Edited (base change) actions on authorized
   repos (`nixos/*`).
3. Publishes an `EvaluationJob` to `mass-rebuild-check-jobs`.

```
github-events ──pull_request.*──▶ mass-rebuild-check-inputs ──▶ evaluation-filter
                                                                  └──▶ mass-rebuild-check-jobs
```

4. **`mass-rebuilder`** consumes `EvaluationJob` from `mass-rebuild-check-jobs`. It clones nixpkgs,
   checks out the target branch, merges the PR, runs nix-instantiate on `nixos/release.nix` and
   `pkgs/top-level/release.nix`, computes the diff of out-paths between base and merge, and
   publishes `BuildJob` messages for changed packages to the `build-jobs` fanout exchange.

```
mass-rebuild-check-jobs ──▶ mass-rebuilder ──▶ build-jobs (fanout)
                                        └──▶ build-results (fanout)
```

### 4. Building

**`builder`** instances consume `BuildJob` messages from per-system queues
(`build-inputs-x86_64-linux`, `build-inputs-aarch64-linux`, `build-inputs-x86_64-darwin`,
`build-inputs-aarch64-darwin`), all bound to the `build-jobs` fanout exchange. Each builder:

1. Clones the repo, fetches the PR, and merges.
2. Partitions attrs into buildable/non-buildable via nix-instantiate.
3. Runs nix-build on buildable attrs.
4. Streams log lines to the `logs` topic exchange with routing key `{repo}.{pr}`.
5. Publishes `BuildResult` to the `build-results` fanout exchange.

```
build-jobs (fanout) ──▶ build-inputs-x86_64-linux ──▶ builder
                  ──▶ build-inputs-aarch64-linux ──▶ builder
                  ──▶ build-inputs-x86_64-darwin ──▶ builder
                  ──▶ build-inputs-aarch64-darwin ──▶ builder

builder ──▶ logs (topic)
     └──▶ build-results (fanout)
```

### 5. Build Results

**`github-comment-poster`** consumes from the `build-results` queue (bound to the `build-results`
fanout exchange). It receives two message types:
- `QueuedBuildJobs` — creates an in-progress GitHub Check Run on the PR.
- `BuildResult` — creates a completed GitHub Check Run (success/failure).

```
build-results (fanout) ──▶ build-results queue ──▶ github-comment-poster ──▶ GitHub Checks API
```

### 6. Log Collection

**`log-message-collector`** consumes from an exclusive queue bound to the `logs` topic exchange with
routing key `*.*`. It receives `BuildLogStart`, `BuildLogMsg`, and `BuildResult` messages and
persists them to disk:
- Metadata: `{logs_path}/{routing_key}/{attempt_id}.metadata.json`
- Log lines: `{logs_path}/{routing_key}/{attempt_id}`
- Build result: `{logs_path}/{routing_key}/{attempt_id}.result.json`

**`logapi`** is an HTTP server that serves this stored data via `GET /logs/{repo}` as JSON.

```
logs (topic) ──▶ exclusive queue ──▶ log-message-collector ──▶ disk
                                                                    │
                                                                    ▼
                                                               logapi (HTTP)
```

### 7. Stats / Metrics

Components publish `EventMessage` payloads to the `stats` fanout exchange. The `stats` binary
consumes them from the `stats-events` queue and exposes collected metrics as Prometheus text format
at `/metrics` over HTTP.

```
components ──▶ stats (fanout) ──▶ stats-events queue ──▶ stats (Prometheus HTTP /metrics)
```

## RabbitMQ Topology

### Exchanges

| Exchange | Type | Purpose |
|----------|------|---------|
| `github-events` | Topic | Ingress for all raw GitHub webhook events. Routing keys: `{event}.{owner}/{repo}` |
| `build-jobs` | Fanout | Distributes `BuildJob` messages to all per-system build queues |
| `build-results` | Fanout | Distributes `BuildResult` and `QueuedBuildJobs` to all interested consumers |
| `logs` | Topic | Distributes build log messages. Routing keys: `{repo}.{pr}` |
| `stats` | Fanout | Distributes `EventMessage` metrics payloads |

### Queues

| Queue | Type | Exchange Binding | Routing Key | Consumer |
|-------|------|------------------|-------------|----------|
| `build-inputs` | Durable | `github-events` | `issue_comment.*` | `github-comment-filter` |
| `github-events-unknown` | Durable | `github-events` | `unknown.*` | — (no consumer) |
| `mass-rebuild-check-inputs` | Durable | `github-events` | `pull_request.*` (webhook-receiver), `pull_request.nixos/*` (evaluation-filter) | `evaluation-filter` |
| `mass-rebuild-check-jobs` | Durable | Default exchange | `mass-rebuild-check-jobs` (routing key = queue name) | `mass-rebuilder` |
| `build-inputs-x86_64-linux` | Durable | `build-jobs` (fanout) | — | `builder` (x86_64-linux) |
| `build-inputs-aarch64-linux` | Durable | `build-jobs` (fanout) | — | `builder` (aarch64-linux) |
| `build-inputs-x86_64-darwin` | Durable | `build-jobs` (fanout) | — | `builder` (x86_64-darwin) |
| `build-inputs-aarch64-darwin` | Durable | `build-jobs` (fanout) | — | `builder` (aarch64-darwin) |
| `build-results` | Durable | `build-results` (fanout) | — | `github-comment-poster` |
| `logs` | Exclusive, auto-delete | `logs` (topic) | `*.*` | `log-message-collector` |
| `stats-events` | Durable | `stats` (fanout) | — | `stats` |

## RabbitMQ Graph

```mermaid
graph
  classDef component fill:#f96
  classDef debug fill:#999999
  classDef exc fill:#08f108

  subgraph Legend
    app-example(This is an application):::component
    debug-example(This is a debug application):::debug
    exchange-example{{This is a RabbitMQ exchange}}:::exc
    queue-example[/This is a RabbitMQ queue/]
  end

  github-webhook-receiver(github-webhook-receiver):::component
  github-events{{github-events}}:::exc
  build-inputs[/build-inputs/]
  github-events-unknown[/github-events-unknown/]
  mass-rebuild-check-inputs[/mass-rebuild-check-inputs/]
  github-comment-filter(github-comment-filter):::component
  build-jobs{{build-jobs}}:::exc
  evaluation-filter(evaluation-filter):::component
  mass-rebuild-check-jobs[/mass-rebuild-check-jobs/]
  mass-rebuilder(mass-rebuilder):::component
  build-inputs-x86_64-linux[/build-inputs-x86_64-linux/]
  build-inputs-aarch64-linux[/build-inputs-aarch64-linux/]
  build-inputs-x86_64-darwin[/build-inputs-x86_64-darwin/]
  build-inputs-aarch64-darwin[/build-inputs-aarch64-darwin/]
  builder(builder):::component
  logs{{logs}}:::exc
  logs-queue[/logs/]
  log-message-collector(log-message-collector):::component
  log-storage[(Log Storage)]
  logapi(logapi):::component
  build-results{{build-results}}:::exc
  build-results-queue[/build-results/]
  github-comment-poster(github-comment-poster):::component
  stats{{stats}}:::exc
  stats-events[/stats-events/]
  stats-rs(stats):::component
  build-faker(build-faker):::debug

  github-webhook-receiver --> github-events
  github-events -->|issue_comment.*| build-inputs
  github-events -->|unknown.*| github-events-unknown
  github-events -->|pull_request.*| mass-rebuild-check-inputs
  build-inputs --> github-comment-filter
  github-comment-filter --> build-jobs
  github-comment-filter --> mass-rebuild-check-jobs
  mass-rebuild-check-inputs --> evaluation-filter
  evaluation-filter --> mass-rebuild-check-jobs
  mass-rebuild-check-jobs --> mass-rebuilder
  mass-rebuilder --> build-jobs
  mass-rebuilder --> build-results
  build-jobs --> build-inputs-x86_64-linux
  build-jobs --> build-inputs-aarch64-linux
  build-jobs --> build-inputs-x86_64-darwin
  build-jobs --> build-inputs-aarch64-darwin
  build-inputs-x86_64-linux --> builder
  build-inputs-aarch64-linux --> builder
  build-inputs-x86_64-darwin --> builder
  build-inputs-aarch64-darwin --> builder
  builder --> logs
  builder --> build-results
  logs --> logs-queue
  logs-queue --> log-message-collector
  log-message-collector --> log-storage
  log-storage --> logapi
  build-results --> build-results-queue
  build-results-queue --> github-comment-poster
  stats --> stats-events
  stats-events --> stats-rs
  build-faker --> build-inputs-aarch64-linux
```

## Message Types

| Message | Serialization | Fields | Published To |
|---------|---------------|--------|--------------|
| `BuildJob` | JSON | `repo`, `pr`, `system`, `attrs`, `checkout` | `build-jobs` exchange |
| `EvaluationJob` | JSON | `repo`, `pr`, `system` | `mass-rebuild-check-jobs` queue |
| `QueuedBuildJobs` | JSON | `repo`, `pr`, `attempt_id`, `total_jobs` | `build-results` exchange |
| `BuildResult` | JSON | `repo`, `pr`, `attempt_id`, `status` (Success/Failure), `system`, `attrs` | `build-results` exchange, `logs` exchange |
| `BuildLogStart` | JSON | `repo`, `pr`, `attempt_id`, `system` | `logs` exchange |
| `BuildLogMsg` | JSON | `repo`, `pr`, `attempt_id`, `line`, `msg` | `logs` exchange |
| `EventMessage` | JSON | `event` (tagged enum), `repo`, `pr`, `value` | `stats` exchange |
| Raw webhook JSON | JSON | Raw GitHub webhook payload | `github-events` exchange |
