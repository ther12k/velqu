#!/usr/bin/env bash
# Crossover matrices (BETA-003-A): 0/1/5/10/25ms controlled I/O, response
# payload scaling, and CPU operation levels for every W4 candidate.
#
#   ./run-crossover.sh [durationSec] [concurrencyCsv]
#
# Starts the controlled upstream, runs each candidate (bun-fetch, hono,
# elysia2, fastify) against the full cell matrix, samples candidate RSS via
# load.ts --server-pid, retains raw JSONL + per-candidate summaries, and
# generates the comparison reports. Fails fast; no hidden fallbacks.
set -euo pipefail
cd "$(dirname "$0")"

DURATION="${1:-3}"
CONCURRENCY="${2:-1,10}"
OUT_ROOT="../raw/real-world/crossover"
UPSTREAM_PORT="${UPSTREAM_PORT:-8791}"
CELLS="W4_0ms,W4_1ms,W4_5ms,W4_10ms,W4_25ms,PAYLOAD_1,PAYLOAD_10,PAYLOAD_20,PAYLOAD_50,CPU_0,CPU_100,CPU_1000,CPU_10000"

# Candidate dependencies: install from the committed lockfile if absent
# (auto-install would silently resolve UNPINNED latest versions).
if [ ! -d candidates/node_modules ]; then
  echo "== crossover: installing pinned candidate deps (candidates/bun.lock) =="
  (cd candidates && bun install --frozen-lockfile)
fi
rm -rf "$OUT_ROOT"
mkdir -p "$OUT_ROOT"

echo "== crossover: starting controlled upstream on :$UPSTREAM_PORT =="
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
  echo "== crossover: candidate $name =="
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
    echo "run-crossover: candidate $name failed to become ready" >&2
    cat "$cdir/candidate.log" >&2
    kill "$pid" 2>/dev/null || true
    exit 1
  fi
  bun load.ts \
    --base-url "http://127.0.0.1:$port" \
    --out-dir "$cdir" \
    --duration "$DURATION" \
    --concurrency "$CONCURRENCY" \
    --server-pid "$pid" \
    --workloads "$CELLS"
  kill "$pid" 2>/dev/null || true
  wait "$pid" 2>/dev/null || true
  echo "== crossover: retaining raw samples for $name =="
  bun retain.ts --run "$cdir"
}

run_candidate bun-fetch bun bun-fetch.ts
run_candidate hono bun hono.ts
run_candidate elysia2 bun elysia.ts
run_candidate fastify node fastify.js

echo "== crossover: W4 latency comparison (incl. 0ms) =="
bun compare-w4.ts --root "$OUT_ROOT" \
  --workloads W4_0ms,W4_1ms,W4_5ms,W4_10ms,W4_25ms \
  --title "Crossover I/O Latency Matrix (0/1/5/10/25ms) — Candidate Comparison (BETA-003-A)" \
  --out w4-latency.md

echo "== crossover: payload matrix =="
bun compare-w4.ts --root "$OUT_ROOT" \
  --workloads PAYLOAD_1,PAYLOAD_10,PAYLOAD_20,PAYLOAD_50 \
  --title "Payload Matrix (W3 route, limit 1/10/20/50) — Candidate Comparison (BETA-003-A)" \
  --out payload-matrix.md

echo "== crossover: CPU operation levels =="
bun compare-w4.ts --root "$OUT_ROOT" \
  --workloads CPU_0,CPU_100,CPU_1000,CPU_10000 \
  --title "CPU Operation Levels (deterministic in-handler loop) — Candidate Comparison (BETA-003-A)" \
  --out cpu-matrix.md

echo "run-crossover: PASS — evidence in $OUT_ROOT"
