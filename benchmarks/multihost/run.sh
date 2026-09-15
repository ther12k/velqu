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
IMAGE=velqu-bench:multihost
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"

echo "== build (not CPU-limited; only the measured run is) =="
docker build -f "$ROOT/benchmarks/multihost/Dockerfile" -t "$IMAGE" "$ROOT"

RUN_ID="multihost-${LABEL}-$(date -u +%Y%m%dT%H%M%SZ)"
OUT="$ROOT/benchmarks/raw/multihost"
mkdir -p "$OUT"

echo "== run: label=$LABEL cpus=$CPUS run-id=$RUN_ID =="
docker run --rm \
  --cpus="$CPUS" --memory=1g --memory-swap=1g \
  --security-opt no-new-privileges \
  -e HOST_LABEL="$LABEL" \
  -e WARM_RUN_ID="$RUN_ID" \
  -e OUT_DIR=/out \
  -v "$OUT:/out" \
  "$IMAGE"

# host-side identity the container cannot see
{
  echo "docker=$(docker --version)"
  echo "image-id=$(docker image inspect -f '{{.Id}}' "$IMAGE")"
  echo "cpus-limit=$CPUS"
  echo "host-cpu=$(LC_ALL=C lscpu 2>/dev/null | grep -m1 'Model name' | cut -d: -f2 | xargs || grep -m1 'model name' /proc/cpuinfo | cut -d: -f2 | xargs)"
  echo "host-kernel=$(uname -r)"
  echo "virtualization=$(systemd-detect-virt 2>/dev/null || echo unknown)"
} > "$OUT/$RUN_ID/host-$LABEL.txt"

echo "DONE: $OUT/$RUN_ID — copy this directory back for aggregation"
