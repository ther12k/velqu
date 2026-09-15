#!/usr/bin/env bash
# Host-side wrapper: build the pinned benchmark image and run the full
# candidate matrix inside a CPU-limited container.
#
#   benchmarks/multihost/run.sh <host-label> [cpus]   (cpus default 2)
#
# Artifacts land in benchmarks/raw/multihost/<run-id>/ — copy that directory
# back from remote hosts for benchmarks/multihost/aggregate.py.
set -euo pipefail

LABEL=${1:?usage: run.sh <host-label> [cpus]}
CPUS=${2:-2}
DOCKER=${DOCKER:-docker}
# extra --build-args for busy/small hosts, e.g. BENCH_BUILD_ARGS="--build-arg CARGO_BUILD_JOBS=2"
EXTRA_BUILD_ARGS=${BENCH_BUILD_ARGS:-}
IMAGE=velqu-bench:multihost
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"

# Build from a CLEAN context: git archive when this is a repo (excludes
# untracked junk), the tree itself when it was transferred as an archive
# (remote hosts). Keeps the context identical on every host and keeps
# committed raw evidence out of the image.
CTX="$ROOT"
CTX_TMP=""
if git -C "$ROOT" rev-parse --git-dir >/dev/null 2>&1; then
  CTX_TMP=$(mktemp -d)
  git -C "$ROOT" archive --format=tar HEAD | tar -x -C "$CTX_TMP"
  printf 'benchmarks/raw\ndocs\n.github\nhistory\ntasks\n' > "$CTX_TMP/.dockerignore"
  CTX="$CTX_TMP"
fi

echo "== build (not CPU-limited; only the measured run is) =="
$DOCKER build ${EXTRA_BUILD_ARGS:+$EXTRA_BUILD_ARGS} \
  -f "$ROOT/benchmarks/multihost/Dockerfile" -t "$IMAGE" "$CTX"
[ -z "$CTX_TMP" ] || rm -rf "$CTX_TMP"

RUN_ID="multihost-${LABEL}-$(date -u +%Y%m%dT%H%M%SZ)"
OUT="$ROOT/benchmarks/raw/multihost"
mkdir -p "$OUT"

echo "== run: label=$LABEL cpus=$CPUS run-id=$RUN_ID =="
$DOCKER run --rm \
  --cpus="$CPUS" --memory=1g --memory-swap=1g \
  --security-opt no-new-privileges \
  -e HOST_LABEL="$LABEL" \
  -e WARM_RUN_ID="$RUN_ID" \
  -e OUT_DIR=/out \
  -v "$OUT:/out" \
  "$IMAGE"

# host-side identity the container cannot see
{
  echo "docker=$($DOCKER --version)"
  echo "image-id=$($DOCKER image inspect -f '{{.Id}}' "$IMAGE" 2>/dev/null || echo unknown)"
  echo "cpus-limit=$CPUS"
  echo "host-cpu=$(LC_ALL=C lscpu 2>/dev/null | grep -m1 'Model name' | cut -d: -f2 | xargs || grep -m1 'model name' /proc/cpuinfo | cut -d: -f2 | xargs)"
  echo "host-kernel=$(uname -r)"
  echo "virtualization=$(systemd-detect-virt 2>/dev/null || echo unknown)"
} > "$OUT/$RUN_ID/host-$LABEL.txt"

echo "DONE: $OUT/$RUN_ID — copy this directory back for aggregation"
