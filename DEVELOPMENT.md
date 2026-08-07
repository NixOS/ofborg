# Local Development

## Prerequisites

- [Nix](https://nixos.org/download.html) (with flakes enabled)
- [Docker](https://docs.docker.com/engine/install/)
- [mprocs](https://github.com/pvolok/mprocs) — available in the dev shell

## Quick start

```shell
# Enter the development environment
nix-shell

# Start all services
mprocs
```

This brings up:

| Process                   | Autostart | Description                                                     |
|---------------------------|-----------|-----------------------------------------------------------------|
| `rabbitmq`                | yes       | Docker container (`rabbitmq:4-management`) on ports 5672, 15672 |
| `rabbitmq-init`           | yes       | Waits for RabbitMQ, then creates the `ofborg` vhost/user        |
| `evaluation-filter`       | yes       | Processes evaluation events from the queue                      |
| `github-webhook-receiver` | yes       | HTTP server on `[::1]:9899` — receives GitHub webhooks          |
| `mass-rebuilder`          | yes       | Orchestrates mass rebuild jobs                                  |
| `stats`                   | yes       | Stats server on `[::1]:9898`                                    |
| `github-comment-filter`   | yes       | Disabled by default; start manually in mprocs                   |
| `github-comment-poster`   | no        | Consumes `build-results` and writes GitHub Check Runs; needs a `github_app` config section, so start it manually |
| `hydra-evaluator`         | yes       | Consumes `HydraEvalJob` messages, imports derivations into queue-runner via gRPC |
| `hydra-build-tracker`     | yes       | Turns queue-runner build events into check run updates; runs with `--replay` by default |

## What happens under the hood

1. **`mprocs/start-rabbitmq.sh`** — Starts RabbitMQ via Docker (`rabbitmq:4-management`). Data is persisted in `.ofborg-data/rabbitmq/`. The admin user defaults to `admin` / `admin`.

2. **`mprocs/wait-for-rabbitmq.sh`** — Polls the RabbitMQ management API healthcheck until the server is ready.

3. **`mprocs/bootstrap-rabbitmq.sh`** — Creates the `ofborg` vhost, a restricted `ofborg` user with a random password, and writes the password to `.ofborg-data/.amqp-password`.

4. **`mprocs/bootstrap-ofborg.sh`** — Generates a random webhook secret (`.ofborg-data/.webhook-secret`) and writes a complete config at `.ofborg-data/local.json`. The config wires all services to the local RabbitMQ instance.

All services then start via `cargo r --bin <name> .ofborg-data/local.json`, so changes to source code are reflected immediately (cargo recompiles on restart). The `hydra-evaluator` crate now ships two binaries, so it needs `--bin`: `cargo r -p hydra-evaluator --bin hydra-evaluator -- .ofborg-data/local.json`.

## Hydra evaluator flow

When the `hydra_evaluator` config section is present, the mass-rebuilder publishes `HydraEvalJob` messages to the `hydra-eval-jobs` queue after a successful evaluation. Each job carries one `(attr, drv_path)` pair per derivation plus the system they were instantiated for. The `hydra-evaluator` binary then:

1. Consumes `HydraEvalJob` messages from `hydra-eval-jobs`.
2. Resolves each drv path against the local Nix store.
3. Streams the corresponding NARs (zstd-compressed) to the queue-runner via the `BuildResult` gRPC stream.
4. Calls `CreateBuild` on the queue-runner to register the builds under the configured `jobset_id`.
5. For every build the queue-runner created, publishes a `HydraBuildTracking` record to `hydra-build-tracking` and a queued `HydraBuildUpdate` to `build-results`.

If the `hydra_evaluator` config is absent, the mass-rebuilder skips the hydra integration step entirely.

## Reporting builds back to GitHub

`hydra-build-tracker` is what closes the loop between a Hydra build and the pull request:

- It consumes `hydra-build-tracking` **without acking**. Each unacked delivery is a build still in flight, so the queue *is* the tracker's state — restart it and RabbitMQ redelivers exactly the pending builds. There is no database.
- It subscribes to the queue-runner's build event stream and publishes a `HydraBuildUpdate` for every event that matches a tracked build. `github-comment-poster` turns those into a single check run per build that moves queued → in progress (naming the builder machine) → completed, linking to the Hydra build page.
- A terminal event acks the tracking record. Builds Hydra never reports on are swept after `stale_after_seconds` and completed as timed out.

### Enabling the queue-runner event stream

The subscription uses `RunnerService.SubscribeBuildEvents`, added to the queue-runner in
`helsinki-systems/hydra`. To turn it on here:

1. Push the queue-runner change and note its revision.
2. Bump all four `helsinki-systems/hydra` `rev = "..."` pins in
   [`hydra-evaluator/Cargo.toml`](./hydra-evaluator/Cargo.toml) to it, and `cargo update -p hydra-proto`.
3. Build with `--features queue-runner-events` and drop `--replay` from the `hydra-build-tracker`
   line in [`mprocs.yaml`](./mprocs.yaml).

Note that `PROTO_API_VERSION` is a hash of the proto files and `CheckVersion` rejects mismatches, so
the builders have to be redeployed together with the queue-runner.

### Driving it without a queue-runner

Without the feature the tracker reads events from a file instead, which exercises the whole
RabbitMQ → check run path locally:

```shell
cargo run -p hydra-evaluator --bin hydra-build-tracker -- \
    .ofborg-data/local.json --replay mprocs/example-build-events.jsonl
```

The file holds one `BuildEvent` per line (`#` starts a comment); see
[`mprocs/example-build-events.jsonl`](./mprocs/example-build-events.jsonl). Set `build_id` to a real
id — watch the `hydra-evaluator` log, or the `hydra-build-tracking` queue in the RabbitMQ management
UI on :15672. `--replay-delay-secs` controls the pacing.

### New config keys

`hydra_evaluator` gained three keys (the section uses `deny_unknown_fields`, so they have to be
present in any config that has the section at all):

| Key | Required | Meaning |
|-----|----------|---------|
| `hydra_base_url` | yes | Base URL of the Hydra web UI; check runs link to `{hydra_base_url}/build/{build_id}` |
| `queue_runner_status_url` | no | Queue-runner HTTP status endpoint, used to reconcile after the event stream reports it dropped events |
| `stale_after_seconds` | no (default 86400) | How long to keep tracking a build before completing its check run as timed out |

## Sending test webhook events

Use the `ofborg-send-event` crate to simulate GitHub pull_request webhooks:

```shell
# Send PR #123456 from NixOS/nixpkgs to localhost:9899
cargo run -p ofborg-send-event -- 123456

# Use a different repo
cargo run -p ofborg-send-event -- --full-repo-name "ofborg/testpkgs" 42

# Custom event type
cargo run -p ofborg-send-event -- --event push 123456

# Point at a different webhook receiver
cargo run -p ofborg-send-event -- --webhook-receiver-url http://localhost:9999 123456

# Supply the webhook secret (required if the receiver validates signatures)
cargo run -p ofborg-send-event -- --secret-path .ofborg-data/.webhook-secret 123456

# Custom delivery ID and extra headers
cargo run -p ofborg-send-event -- --delivery-id my-id --header X-Custom=val 123456

# Verbose mode (shows response headers)
cargo run -p ofborg-send-event -- --verbose 123456
```

The tool fetches the real PR data from GitHub, constructs a `PullRequestEvent` payload, signs it with HMAC-SHA256 (if `--secret-path` is given), and POSTs it to the receiver.

### CLI reference

```
Usage: ofborg-send-event [OPTIONS] <PR_NR>

Arguments:
  <PR_NR>  PR that should be fetched

Options:
      --webhook-receiver-url <URL>      [default: http://localhost:9899]
      --event <EVENT>                   [default: pull_request]
      --secret-path <PATH>              Shared secret for X-Hub-Signature-256
      --delivery-id <ID>                X-GitHub-Delivery header (default: random UUID)
      --header <NAME=VALUE>             Add arbitrary header (repeatable)
      --verbose                         Print response headers
      --timeout-secs <SECS>             [default: 30]
      --full-repo-name <ORG/REPO>       [default: NixOS/nixpkgs]
```

## Building and testing

```shell
# All in the dev shell:

cargo build                   # build all binaries
cargo test                    # run tests
cargo clippy                  # lint
cargo fmt                     # format code
cargo r --bin <name> <config> # run a specific binary

# Full CI check (from repo root):
nix-shell --pure --run checkPhase
```
