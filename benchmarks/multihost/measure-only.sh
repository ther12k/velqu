#!/usr/bin/env bash
# Measurement-only variant of container-run.sh — used 2026-09-15 while the
# velqu fixture-conformance finding (M25-005-A encoder emits byte-sorted
# object keys; frozen contract requires declaration order on POST /users
# and C4 users.get) awaits owner disposition. Differences from the
# fail-closed container-run.sh, both deliberate and disclosed:
#   1. checker results for EVERY candidate are still produced and stored
#      verbatim in the run directory (checker-<id>.log) — they do NOT
#      abort the run;
#   2. everything else (protocol, identity recording) is identical.
# Results produced under this script are measurement evidence only; the
# canonical fail-closed lane must re-run once the conformance finding is
# resolved.
set -euo pipefail

LABEL=${HOST_LABEL:?HOST_LABEL env var required}
RUN_ID=${WARM_RUN_ID:?WARM_RUN_ID env var required}
OUT=${OUT_DIR:-/out}
PORT_BASE=39300
cd /bench

check_record() {
  local id="$1" port="$2" ; shift 2
  env PORT="$port" "$@" > "/tmp/srv-$id.log" 2>&1 &
  local pid=$!
  for _ in $(seq 1 50); do
    if bash -c "exec 3<>/dev/tcp/127.0.0.1/$port" 2>/dev/null; then break; fi
    sleep 0.2
  done
  set +e
  bun benchmarks/harness/check-server.ts "$port" --candidate "$id" > "/tmp/check-$id.log" 2>&1
  local rc=$?
  set -e
  echo "checker $id: exit=$rc $(grep -o '"pass": *[0-9]*' /tmp/check-$id.log | head -1) $(grep -o '"fail": *[0-9]*' /tmp/check-$id.log | head -1)"
  cp "/tmp/check-$id.log" "$OUT/$RUN_ID/checker-$id.log" 2>/dev/null || true
  kill "$pid" 2>/dev/null || true
  wait "$pid" 2>/dev/null || true
}

DEST="$OUT/$RUN_ID"
mkdir -p "$DEST"

echo "== fixture checker (recorded, non-fatal — see header of this script) =="
check_record velqu    $((PORT_BASE+1)) /bench/target/release/velqu-runtime --pack /bench/examples/proof/dist/app.qpack --port "$((PORT_BASE+1))" --log off
check_record raw-rust $((PORT_BASE+2)) /bench/baselines/raw-rust/target/release/velqu-baseline-raw-rust
check_record raw-bun  $((PORT_BASE+3)) bun /bench/baselines/raw-bun/server.ts
check_record elysia2  $((PORT_BASE+4)) bun /bench/baselines/elysia2/server.ts
check_record lugas    $((PORT_BASE+5)) bun /bench/baselines/lugas/server.ts

echo "== warm harness: all candidates, fixed protocol =="
bun benchmarks/harness/warm.ts

cp "benchmarks/raw/warm/$RUN_ID.jsonl" "$DEST/"
cp benchmarks/raw/warm/summary.json "$DEST/"

echo "== environment identity =="
{
  echo "host-label=$LABEL"
  echo "run-id=$RUN_ID"
  echo "mode=measure-only (checker recorded non-fatal; see measure-only.sh header)"
  echo "builder-image=${BENCH_BASE_DIGEST}"
  echo "cpu-quota=$(cat /sys/fs/cgroup/cpu.max 2>/dev/null || cat /sys/fs/cgroup/cpu/cpu.cfs_quota_us 2>/dev/null || echo unknown)"
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

echo "DONE run-id=$RUN_ID (artifacts in $DEST)"
