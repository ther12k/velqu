#!/usr/bin/env bash
# BETA-008-A — trusted reverse-proxy boundary smoke test.
#
# Usage: scripts/proxy-smoke.sh [runtime-binary] [pack-file] [port]
#
# This is a container-friendly smoke: it does not start a proxy or
# publish a port. It proves the runtime defaults to the loopback-bound
# reverse-proxy posture, serves health + a real route, and exits
# deterministically on SIGTERM. A public bind in reverse-proxy mode is
# expected to reject before ready (covered by q-runtime config tests).
set -euo pipefail

ROOT=$(cd "$(dirname "$0")/.." && pwd)
RUNTIME=${1:-target/release/velqu-runtime}
PACK=${2:-examples/proof/dist/app.qpack}
PORT=${3:-3998}
LOG=$(mktemp)
cleanup() {
  if [[ -n "${PID:-}" ]] && kill -0 "$PID" 2>/dev/null; then
    kill -TERM "$PID" 2>/dev/null || true
    wait "$PID" 2>/dev/null || true
  fi
  rm -f "$LOG"
}
trap cleanup EXIT
cd "$ROOT"

[[ -x "$RUNTIME" ]] || { echo "FAIL: runtime not executable: $RUNTIME" >&2; exit 1; }
[[ -f "$PACK" ]] || { echo "FAIL: pack not found: $PACK" >&2; exit 1; }

"./$RUNTIME" --pack "$PACK" --port "$PORT" --log off >"$LOG" 2>&1 &
PID=$!
for _ in $(seq 1 100); do
  if curl -sf "http://127.0.0.1:$PORT/health/live" >/dev/null 2>&1; then break; fi
  sleep 0.05
done
curl -sf "http://127.0.0.1:$PORT/health/live" | grep -Fq '{"status":"ok"}'
curl -sf "http://127.0.0.1:$PORT/hello/smoke" >/dev/null
# The startup JSON is non-secret and must identify the safe posture.
grep -Fq '"proxyMode":"reverse-proxy"' "$LOG"
grep -Fq '"addr":"127.0.0.1:' "$LOG"
kill -TERM "$PID"
# Bounded shutdown: the process must be gone promptly, not left orphaned.
for _ in $(seq 1 100); do
  if ! kill -0 "$PID" 2>/dev/null; then break; fi
  sleep 0.05
done
wait "$PID" 2>/dev/null || true
PID=

echo "PROXY-SMOKE-OK: loopback reverse-proxy posture, health/route, deterministic SIGTERM"
