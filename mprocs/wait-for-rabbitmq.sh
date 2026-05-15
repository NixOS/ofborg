#!/usr/bin/env bash

set -euo pipefail
set -x

RABBIT_HOST=${RABBIT_HOST:-localhost}
RABBIT_USER=${RABBIT_USER:-admin}
RABBIT_PASS=${RABBIT_PASS:-admin}

until curl -sf "http://${RABBIT_HOST}:15672/api/healthchecks/node" \
  -u "${RABBIT_USER}:${RABBIT_PASS}" &>/dev/null; do
  echo "Waiting for RabbitMQ..."
  sleep 1
done

echo "RabbitMQ is ready"
