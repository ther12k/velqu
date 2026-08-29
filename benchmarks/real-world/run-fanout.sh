#!/usr/bin/env bash
# Fan-out benchmark (M28-011-B): one request issuing 1/2/4 PARALLEL upstream
# calls. Each candidate implements GET /api/bench/fanout?n=1|2|4&ms=5.
#
#   ./run-fanout.sh [durationSec] [concurrencyCsv]
set -euo pipefail
cd "$(dirname "$0")"

DURATION="${1:-3}"
CONCURRENCY="${2:-1,10}"
OUT_ROOT="../raw/real-world/fanout"
UPSTREAM_PORT="${UPSTREAM_PORT:-8792}"
# Candidate dependencies: install from the committed lockfile if absent
# (auto-install would silently resolve UNPINNED latest versions).
if [ ! -d candidates/node_modules ]; then
  echo "== installing pinned candidate deps (candidates/bun.lock) =="
  (cd candidates && bun install --frozen-lockfile)
fi
rm -rf "$OUT_ROOT"
mkdir -p "$OUT_ROOT"

echo "== fanout: starting controlled upstream on :$UPSTREAM_PORT =="
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
  echo "== fanout: candidate $name =="
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
    echo "run-fanout: candidate $name failed to become ready" >&2
    cat "$cdir/candidate.log" >&2
    kill "$pid" 2>/dev/null || true
    exit 1
  fi
  bun load.ts \
    --base-url "http://127.0.0.1:$port" \
    --out-dir "$cdir" \
    --duration "$DURATION" \
    --concurrency "$CONCURRENCY" \
    --workloads FANOUT_1,FANOUT_2,FANOUT_4
  kill "$pid" 2>/dev/null || true
  wait "$pid" 2>/dev/null || true
}

run_candidate bun-fetch bun bun-fetch.ts
run_candidate hono bun hono.ts
run_candidate elysia2 bun elysia.ts
run_candidate fastify node fastify.js

echo "== fanout: comparison report =="
bun compare-fanout.ts --root "$OUT_ROOT"
echo "run-fanout: PASS — evidence in $OUT_ROOT"
