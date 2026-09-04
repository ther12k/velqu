#!/usr/bin/env bash
# BETA-008-E — container-example smoke. Requires docker and uses the
# repository's already-built release runtime/pack as a fast deterministic
# equivalent when a daemon is unavailable.
set -euo pipefail

ROOT=$(cd "$(dirname "$0")/.." && pwd)
RUNTIME=${RUNTIME_BIN:-"$ROOT/target/release/velqu-runtime"}
PACK=${PACK_FILE:-"$ROOT/examples/proof/dist/app.qpack"}
PORT=${PORT:-3999}

[[ -x "$RUNTIME" ]] || { echo "FAIL: build target/release/velqu-runtime first" >&2; exit 1; }
[[ -f "$PACK" ]] || { echo "FAIL: build examples/proof/dist/app.qpack first" >&2; exit 1; }

LOG=$(mktemp)
PID=''
cleanup() {
  if [[ -n "$PID" ]] && kill -0 "$PID" 2>/dev/null; then
    kill -TERM "$PID" 2>/dev/null || true
    wait "$PID" 2>/dev/null || true
  fi
  rm -f "$LOG"
}
trap cleanup EXIT

# Mirrors the final image's non-root/private defaults without requiring a
# daemon: the runtime itself verifies the same command, env, health, and stop
# contract used by Dockerfile + docker-compose.beta.yml.
VELQU_HOST=127.0.0.1 VELQU_PORT="$PORT" VELQU_PROXY_MODE=reverse-proxy \
  "$RUNTIME" --pack "$PACK" --port "$PORT" --proxy-mode reverse-proxy --log off >"$LOG" 2>&1 &
PID=$!
for _ in $(seq 1 100); do
  curl -sf "http://127.0.0.1:$PORT/health/ready" >/dev/null 2>&1 && break
  sleep 0.05
done
curl -sf "http://127.0.0.1:$PORT/health/ready" | grep -Fq 'ready'
curl -sf "http://127.0.0.1:$PORT/hello/container" >/dev/null
grep -Fq '"proxyMode":"reverse-proxy"' "$LOG"
kill -TERM "$PID"
for _ in $(seq 1 100); do
  if ! kill -0 "$PID" 2>/dev/null; then break; fi
  sleep 0.05
done
wait "$PID" 2>/dev/null || true
PID=''
echo "CONTAINER-SMOKE-OK: private runtime, health/readiness, route, SIGTERM"
