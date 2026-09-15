#!/usr/bin/env bash
# Runs INSIDE the benchmark container (see Dockerfile).
# Fail-closed: every candidate must pass the fixture checker before any timing.
set -euo pipefail

LABEL=${HOST_LABEL:?HOST_LABEL env var required (e.g. local|halotec|oracle)}
RUN_ID=${WARM_RUN_ID:-multihost-${LABEL}-$(date -u +%Y%m%dT%H%M%SZ)}
OUT=${OUT_DIR:-/out}
PORT_BASE=39100
cd /bench

echo "== fixture checker (fail-closed) =="
check_candidate() {
  local id="$1" port="$2" ; shift 2
  env PORT="$port" "$@" &
  local pid=$!
  for _ in $(seq 1 50); do
    if bash -c "exec 3<>/dev/tcp/127.0.0.1/$port" 2>/dev/null; then break; fi
    sleep 0.2
  done
  set +e
  bun benchmarks/harness/check-server.ts "$port" --candidate "$id" > "/tmp/check-$id.log" 2>&1
  local rc=$?
  set -e
  tail -1 "/tmp/check-$id.log"
  kill "$pid" 2>/dev/null || true
  wait "$pid" 2>/dev/null || true
  [ "$rc" = 0 ] || { echo "CHECKER FAILED for $id"; exit 1; }
}

check_candidate velqu    $((PORT_BASE+1)) /bench/target/release/velqu-runtime --pack /bench/examples/proof/dist/app.qpack --port "$((PORT_BASE+1))" --log off
check_candidate raw-rust $((PORT_BASE+2)) /bench/baselines/raw-rust/target/release/velqu-baseline-raw-rust
check_candidate raw-bun  $((PORT_BASE+3)) bun /bench/baselines/raw-bun/server.ts
check_candidate elysia2  $((PORT_BASE+4)) bun /bench/baselines/elysia2/server.ts
check_candidate lugas    $((PORT_BASE+5)) bun /bench/baselines/lugas/server.ts

echo "== warm harness: all candidates, fixed protocol =="
export WARM_RUN_ID="$RUN_ID"
bun benchmarks/harness/warm.ts

DEST="$OUT/$RUN_ID"
mkdir -p "$DEST"
cp "benchmarks/raw/warm/$RUN_ID.jsonl" "$DEST/"
cp benchmarks/raw/warm/summary.json "$DEST/"

echo "== environment identity =="
{
  echo "host-label=$LABEL"
  echo "run-id=$RUN_ID"
  echo "builder-image=${BENCH_BASE_DIGEST}"
  echo "cpu-quota=$(cat /sys/fs/cgroup/cpu.max 2>/dev/null || echo unknown)"
  echo "nproc=$(nproc)"
  echo "cpu=$(grep -m1 'model name' /proc/cpuinfo | cut -d: -f2 | xargs)"
  echo "uname=$(uname -srm)"
  echo "kernel=$(uname -r)"
  echo "bun=$(bun --version)"
  echo "rustc=$(rustc --version)"
  echo "gcc=$(gcc --version | head -1)"
  echo "protocol=seed=${WARM_SEED} duration=${WARM_DURATION}s reps=${WARM_REPETITIONS} conc=${WARM_CONCURRENCY}"
  sha256sum target/release/velqu-runtime baselines/raw-rust/target/release/velqu-baseline-raw-rust
} | tee "$DEST/hashes-$LABEL.txt"

echo "DONE run-id=$RUN_ID (artifacts in benchmarks/raw/multihost/$RUN_ID/)"
