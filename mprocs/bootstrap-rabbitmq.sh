#!/usr/bin/env bash

set -euo pipefail

RABBITMQ_HOST="${RABBITMQ_HOST:-localhost}"
RABBITMQ_MGMT_PORT="${RABBITMQ_MGMT_PORT:-15672}"
ADMIN_USER="${RABBITMQ_ADMIN_USER:-admin}"
ADMIN_PASS="${RABBITMQ_ADMIN_PASS:-admin}"
CONFIG_OUT="${CONFIG_OUT:-$(pwd)/.ofborg-data/rabbitmq-config.json}"

BASE_URL="http://${RABBITMQ_HOST}:${RABBITMQ_MGMT_PORT}/api"

rmq() {
  set -x
  local method="$1" path="$2"; shift 2
  curl -sf -u "${ADMIN_USER}:${ADMIN_PASS}" \
       -X "${method}" \
       -H "Content-Type: application/json" \
       "${BASE_URL}${path}" "$@"
}

urlencode() {
  python3 -c "import urllib.parse,sys; print(urllib.parse.quote(sys.argv[1],safe=''))" "$1"
}

gen_secret() {
  openssl rand -base64 18 | tr -d '/+=' | head -c 24
}

mkdir -p "$(dirname "${CONFIG_OUT}")" "$(pwd)/.ofborg-data/rabbitmq"

rmq PUT "/vhosts/$(urlencode "ofborg")" -d '{}' >/dev/null

pass="$(gen_secret)"
rmq PUT "/users/ofborg" -d "{\"password\":\"${pass}\", \"tags\": \"\"}" >/dev/null

rmq PUT "/permissions/$(urlencode "ofborg")/ofborg" \
    -d "{\"configure\":\".*\",\"write\":\".*\",\"read\":\".*\"}" >/dev/null

echo "${pass}" > .ofborg-data/.amqp-password
