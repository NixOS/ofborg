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

## What happens under the hood

1. **`mprocs/start-rabbitmq.sh`** — Starts RabbitMQ via Docker (`rabbitmq:4-management`). Data is persisted in `.ofborg-data/rabbitmq/`. The admin user defaults to `admin` / `admin`.

2. **`mprocs/wait-for-rabbitmq.sh`** — Polls the RabbitMQ management API healthcheck until the server is ready.

3. **`mprocs/bootstrap-rabbitmq.sh`** — Creates the `ofborg` vhost, a restricted `ofborg` user with a random password, and writes the password to `.ofborg-data/.amqp-password`.

4. **`mprocs/bootstrap-ofborg.sh`** — Generates a random webhook secret (`.ofborg-data/.webhook-secret`) and writes a complete config at `.ofborg-data/local.json`. The config wires all services to the local RabbitMQ instance.

All services then start via `cargo r --bin <name> .ofborg-data/local.json`, so changes to source code are reflected immediately (cargo recompiles on restart).

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
