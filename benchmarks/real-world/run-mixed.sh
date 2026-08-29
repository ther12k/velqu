#!/usr/bin/env bash
# Mixed-outcome benchmark (M28-011-C): each request drives a deterministic
# upstream outcome — success (200 relay), client-deadline timeout (typed
# 504), or malformed body (typed 502) — proving error handling under load.
#
#   ./run-mixed.sh [durationSec] [concurrencyCsv]
set -euo pipefail
cd "$(dirname "$0")"

DURATION="${1:-3}"
CONCURRENCY="${2:-1,10}"
OUT_ROOT="../raw/real-world/mixed"
UPSTREAM_PORT="${UPSTREAM_PORT:-8793}"
rm -rf "$OUT_ROOT"
mkdir -p "$OUT_ROOT"

if [ ! -d candidates/node_modules ]; then
  echo "== installing pinned candidate deps (candidates/bun.lock) =="
  (cd candidates && bun install --frozen-lockfile)
fi

echo "== mixed: starting controlled upstream on :$UPSTREAM_PORT =="
PORT="$UPSTREAM_PORT" bun upstream.ts >"$OUT_ROOT/upstream.log" 2>&1 &
UPSTREAM_PID=$!
trap 'kill "$UPSTREAM_PID" 2>/dev/null || true' EXIT

for _ in $(seq 1 100); do
  curl -fsS "http://127.0.0.1:$UPSTREAM_PORT/health" >/dev/null 2>&1 && break
  sleep 0.1
done
curl -fsS "http://127.0.0.1:$UPSTREAM_PORT/bad" >/dev/null

run_candidate() {
  local name="$1" runner="$2" script="$3"
  echo "== mixed: candidate $name =="
  local cdir="$OUT_ROOT/$name"
  mkdir -p "$cdir"
  PORT=0 UPSTREAM_URL="http://127.0.0.1:$UPSTREAM_PORT" "$runner" "candidates/$script" >"$cdir/candidate.log" 2>&1 &
  local pid=$!
  local port=""
  for _ in $(seq 1 100); do
    port=$(grep -o '"port":[0-9]*' "$cdir/candidate.log" | tail -1 | cut -d: -f2 || true)
    if [ -n "$port" ] && [ "$port" != "0" ]; then break; fi
    sleep 0.1
  done
  if [ -z "$port" ] || [ "$port" = "0" ]; then
    echo "run-mixed: candidate $name failed to become ready" >&2
    cat "$cdir/candidate.log" >&2
    kill "$pid" 2>/dev/null || true
    exit 1
  fi
  bun load.ts \
    --base-url "http://127.0.0.1:$port" \
    --out-dir "$cdir" \
    --duration "$DURATION" \
    --concurrency "$CONCURRENCY" \
    --workloads MIX_SUCCESS,MIX_TIMEOUT,MIX_MALFORMED
  kill "$pid" 2>/dev/null || true
  wait "$pid" 2>/dev/null || true
}

run_candidate bun-fetch bun bun-fetch.ts
run_candidate hono bun hono.ts
run_candidate elysia2 bun elysia.ts
run_candidate fastify node fastify.js

echo "== mixed: comparison report =="
bun compare-mixed.ts --root "$OUT_ROOT"
echo "run-mixed: PASS — evidence in $OUT_ROOT"
