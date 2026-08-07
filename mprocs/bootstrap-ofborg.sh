#!/usr/bin/env bash

set -euo pipefail

DATA_DIR="${DATA_DIR:-$(pwd)/.ofborg-data}"
mkdir -pv "${DATA_DIR}"

gen_secret() {
  openssl rand -base64 18 | tr -d '/+=' | head -c 24
}
if [[ ! -f "${DATA_DIR}/.webhook-secret" ]]; then
  gen_secret > "${DATA_DIR}/.webhook-secret"
fi

cat <<EOF > .ofborg-data/local.json
{
    "github_webhook_receiver": {
        "listen": "[::1]:9899",
        "webhook_secret_file": "${DATA_DIR}/.webhook-secret",
        "rabbitmq": {
            "host": "localhost:5672",
            "ssl": false,
            "username": "ofborg",
            "password_file": "${DATA_DIR}/.amqp-password",
            "virtualhost": "ofborg"
        }
    },
    "mass_rebuilder": {
        "rabbitmq": {
            "host": "localhost:5672",
            "ssl": false,
            "username": "ofborg",
            "password_file": "${DATA_DIR}/.amqp-password",
            "virtualhost": "ofborg"
        }
    },
    "evaluation_filter": {
        "rabbitmq": {
            "host": "localhost:5672",
            "ssl": false,
            "username": "ofborg",
            "password_file": "${DATA_DIR}/.amqp-password",
            "virtualhost": "ofborg"
        }
    },
    "github_comment_filter": {
        "rabbitmq": {
            "host": "localhost:5672",
            "ssl": false,
            "username": "ofborg",
            "password_file": "${DATA_DIR}/.amqp-password",
            "virtualhost": "ofborg"
        }
    },
    "hydra_evaluator": {
        "rabbitmq": {
            "host": "localhost:5672",
            "ssl": false,
            "username": "ofborg",
            "password_file": "${DATA_DIR}/.amqp-password",
            "virtualhost": "ofborg"
        },
        "gateway_endpoint": "[::1]:50051",
        "jobset_id": 7,
        "hydra_base_url": "http://localhost:3000",
        "queue_runner_status_url": "http://localhost:8080/status",
        "stale_after_seconds": 86400
    },
    "github_comment_poster": {
        "rabbitmq": {
            "host": "localhost:5672",
            "ssl": false,
            "username": "ofborg",
            "password_file": "${DATA_DIR}/.amqp-password",
            "virtualhost": "ofborg"
        }
    },
    "stats": {
        "listen": "[::1]:9898",
        "rabbitmq": {
            "host": "localhost:5672",
            "ssl": false,
            "username": "ofborg",
            "password_file": "${DATA_DIR}/.amqp-password",
            "virtualhost": "ofborg"
        }
    },
    "runner": {
        "identity": "...",
        "repos": [
          "ofborg/testpkgs"
        ],
        "disable_trusted_users": true
    },
    "checkout": {
        "root": "${DATA_DIR}/checkout"
    },
    "nix": {
        "system": "x86_64-linux",
        "remote": "daemon",
        "build_timeout_seconds": 3600,
        "initial_heap_size": "4g"
    }
}
EOF
