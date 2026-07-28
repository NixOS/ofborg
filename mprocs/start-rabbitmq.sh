#!/usr/bin/env bash

set -euo pipefail

ADMIN_USER="${RABBITMQ_ADMIN_USER:-admin}"
ADMIN_PASS="${RABBITMQ_ADMIN_PASS:-admin}"
DATA_DIR="${DATA_DIR:-$(pwd)/.ofborg-data/rabbitmq}"
CONTAINER_NAME="${CONTAINER_NAME:-ofborg-rabbitmq}"

mkdir -p "${DATA_DIR}"

# Remove a stopped container with the same name if it exists
if docker inspect "${CONTAINER_NAME}" &>/dev/null; then
  echo "Container '${CONTAINER_NAME}' already exists."
  echo "Run 'docker rm -f ${CONTAINER_NAME}' to remove it first"
  exit 1
fi

echo "Starting RabbitMQ..."
docker run \
  --name "${CONTAINER_NAME}" \
  --hostname ofborg-rabbitmq \
  --rm \
  -p 5672:5672 \
  -p 15672:15672 \
  -e RABBITMQ_DEFAULT_USER="${ADMIN_USER}" \
  -e RABBITMQ_DEFAULT_PASS="${ADMIN_PASS}" \
  -e RABBITMQ_DEFAULT_VHOST="/" \
  -v "${DATA_DIR}:/var/lib/rabbitmq" \
  rabbitmq:4-management
