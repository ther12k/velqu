#!/usr/bin/env bash
# W4 controlled-upstream latency matrix (M28-011-A).
#
# Starts the controlled upstream, then runs each W4 candidate (bun-fetch,
# hono, elysia2, fastify) against the 1/5/10/25ms latency cells at the
# pinned concurrency set, retaining raw JSONL + per-candidate summaries and
# generating the comparison report. Fails fast; no hidden fallbacks.
#
#   ./run-w4.sh [durationSec] [concurrencyCsv]
set -euo pipefail
cd "$(dirname "$0")"

DURATION="${1:-3}"
CONCURRENCY="${2:-1,10}"
OUT_ROOT="../raw/real-world/w4-latency"
UPSTREAM_PORT="${UPSTREAM_PORT:-8791}"
rm -rf "$OUT_ROOT"
mkdir -p "$OUT_ROOT"

echo "== w4: starting controlled upstream on :$UPSTREAM_PORT =="
PORT="$UPSTREAM_PORT" bun upstream.ts >"$OUT_ROOT/upstream.log" 2>&1 &
UPSTREAM_PID=$!
trap 'kill "$UPSTREAM_PID" 2>/dev/null || true' EXIT

for _ in $(seq 1 100); do
  curl -fsS "http://127.0.0.1:$UPSTREAM_PORT/health" >/dev/null 2>&1 && break
  sleep 0.1
done
curl -fsS "http://127.0.0.1:$UPSTREAM_PORT/health" >/dev/null

run_candidate() {
  local name="$1" runner="$2" script="$3"
  echo "== w4: candidate $name =="
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
    echo "run-w4: candidate $name failed to become ready" >&2
    cat "$cdir/candidate.log" >&2
    kill "$pid" 2>/dev/null || true
    exit 1
  fi
  bun load.ts \
    --base-url "http://127.0.0.1:$port" \
    --out-dir "$cdir" \
    --duration "$DURATION" \
    --concurrency "$CONCURRENCY" \
    --workloads W4_1ms,W4_5ms,W4_10ms,W4_25ms
  kill "$pid" 2>/dev/null || true
  wait "$pid" 2>/dev/null || true
}

run_candidate bun-fetch bun bun-fetch.ts
run_candidate hono bun hono.ts
run_candidate elysia2 bun elysia.ts
run_candidate fastify node fastify.js

echo "== w4: comparison report =="
bun compare-w4.ts --root "$OUT_ROOT"
echo "run-w4: PASS — evidence in $OUT_ROOT"
