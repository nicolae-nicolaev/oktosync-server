#!/usr/bin/env bash
set -euo pipefail

# Simple DX wrapper around docker compose for Oktosync.
# Usage:
#   ./run.sh up         # build + start in background (creates secret if missing)
#   ./run.sh down       # stop stack (keeps volumes)
#   ./run.sh destroy    # stop and remove volumes and images
#   ./run.sh logs       # tail logs
#   ./run.sh ps         # show containers
#
# Env:
#   PORT (default: 3000)    -> host/container port for server
#   COMPOSE_FILE (optional) -> defaults to docker-compose.yml

PROJECT="oktosync"
PORT="${PORT:-3000}"
COMPOSE_FILE="${COMPOSE_FILE:-docker-compose.yml}"

need() { command -v "$1" >/dev/null 2>&1 || {
  echo "Missing dependency: $1" >&2
  exit 1
}; }

ensure_secret() {
  mkdir -p secrets
  if [[ ! -f secrets/pg_password.txt ]]; then
    echo "Generating a random Postgres password at secrets/pg_password.txt"
    openssl rand -base64 32 >secrets/pg_password.txt
    chmod 600 secrets/pg_password.txt
  fi
}

wait_for_server() {
  local retries=60
  local url="http://127.0.0.1:${PORT}/health"
  echo -n "Waiting for Oktosync server on ${url} "
  until curl -fsS "$url" >/dev/null 2>&1; do
    ((retries--)) || {
      echo
      echo "Server did not become healthy in time."
      exit 1
    }
    echo -n "."
    sleep 1
  done
  echo " OK"
}

compose() {
  COMPOSE_PROJECT_NAME="$PROJECT" docker compose -f "$COMPOSE_FILE" "$@"
}

cmd="${1:-up}"
case "$cmd" in
up)
  need docker
  need openssl
  ensure_secret
  # Make sure iptables is present on Linux hosts (common Arch gotcha)
  if [[ "$(uname -s)" == "Linux" ]] && ! iptables -V >/dev/null 2>&1; then
    echo "Warning: 'iptables' not found on host. Docker port publishing may fail." >&2
  fi
  export COMPOSE_PROJECT_NAME="$PROJECT"
  # Pass through port override to compose if you wire it there; otherwise the default compose uses 3000:3000
  compose up -d --build
  wait_for_server
  echo "🐙 Oktosync is up: http://127.0.0.1:${PORT}/"
  ;;
down)
  compose down
  ;;
destroy)
  compose down -v --rmi local
  ;;
logs)
  compose logs -f
  ;;
ps)
  compose ps
  ;;
*)
  echo "Unknown command: $cmd"
  echo "Usage: $0 {up|down|destroy|logs|ps}"
  exit 1
  ;;
esac
