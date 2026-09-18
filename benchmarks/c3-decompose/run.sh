#!/usr/bin/env bash
# benchmarks/c3-decompose — engine-only C3 context-vs-engine decomposition
# (#1392 follow-up diagnostic). Builds q-c3-decompose from a source tar in
# the pinned multihost image (uncapped) and runs it (CPU-capped where the
# host allows). Diagnostic evidence only.
#
#   benchmarks/c3-decompose/run.sh <label> <src.tar.gz> [cpus]
#
# Env: DOCKER=podman C3D_NO_CPU_LIMIT=1 C3D_MOUNT_Z=1 C3D_WORK_DIR=...
#      BENCH_CARGO_JOBS=N C3D_BATCHES=120
set -euo pipefail

LABEL=${1:?usage: run.sh <label> <src.tar.gz> [cpus]}
SRC=$(realpath "${2:?src.tar.gz}")
CPUS=${3:-2}
DOCKER=${DOCKER:-docker}
BATCHES=${C3D_BATCHES:-120}
JOBS=${BENCH_CARGO_JOBS:-0}
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
OUT_BASE="$ROOT/benchmarks/raw/c3-decompose-engine"
IMAGE=velqu-bench:multihost

command -v "$DOCKER" >/dev/null || { echo "no $DOCKER" >&2; exit 1; }
[ -f "$SRC" ] || { echo "missing $SRC" >&2; exit 1; }
"$DOCKER" image inspect "$IMAGE" >/dev/null 2>&1 || {
  echo "image $IMAGE not found — build it with benchmarks/multihost/run.sh first" >&2; exit 1;
}

MFLAGS=""
[ "${C3D_MOUNT_Z:-0}" = 1 ] && MFLAGS=":Z"
UIDGID="$(id -u):$(id -g)"
RUN_ID="c3d-${LABEL}-$(date -u +%Y%m%dT%H%M%SZ)"

WORK=$(mktemp -d "${C3D_WORK_DIR:-/tmp}/c3d-work-XXXXXX")
mkdir -p "$WORK/src" "$WORK/out" "$WORK/bin"
cp "$SRC" "$WORK/src.tar.gz"
SRC_SHA=$(sha256sum "$SRC" | cut -d' ' -f1)
SRC_COMMIT=${C3D_SRC_COMMIT:-$(tar -xzOf "$SRC" .git-commit 2>/dev/null || echo unknown)}

echo "== phase 1: build q-c3-decompose (uncapped) =="
"$DOCKER" run --rm \
  -e BENCH_CARGO_JOBS="$JOBS" -e UIDGID="$UIDGID" \
  -v "$WORK:/work$MFLAGS" \
  "$IMAGE" bash -ceu '
    set -eu
    tar -xzf /work/src.tar.gz -C /work/src
    rm -f /work/src.tar.gz
    cd /work/src
    export CARGO_TARGET_DIR=/work/target
    export RUSTFLAGS="--remap-path-prefix=/work/src=/velqu-src"
    [ "$BENCH_CARGO_JOBS" = 0 ] || export CARGO_BUILD_JOBS="$BENCH_CARGO_JOBS"
    cargo build --release -p q-bench-support --bin q-c3-decompose
    cp /work/target/release/q-c3-decompose /work/bin/q-c3-decompose
    sha256sum /work/bin/q-c3-decompose
    rm -rf /work/target /work/src
    chmod -R a+rwX /work/bin /work/out || true
  ' | tee "$WORK/build.log"
BIN_SHA=$(sha256sum "$WORK/bin/q-c3-decompose" | cut -d' ' -f1)


CAPS=(--cpus="$CPUS" --memory=1g --memory-swap=1g)
if [ "${C3D_NO_CPU_LIMIT:-0}" = 1 ]; then CAPS=(); fi
echo "== phase 2: run decomposition run-id=$RUN_ID =="
"$DOCKER" run --rm \
  "${CAPS[@]}" \
  --security-opt no-new-privileges \
  -e C3D_LABEL="$LABEL" -e C3D_COMMIT="$SRC_COMMIT" \
  -v "$WORK:/work$MFLAGS" \
  "$IMAGE" bash -ceu '
    set -eu
    /work/bin/q-c3-decompose --out-dir /work/out --batches '"$BATCHES"'
    chmod -R a+rwX /work/out || true
  ' | tee "$WORK/run.log"

DEST="$OUT_BASE/$RUN_ID"
mkdir -p "$DEST"
cp "$WORK/out/decompose.jsonl" "$WORK/out/decompose-summary.json" "$DEST/"
{
  echo "label=$LABEL"
  echo "run-id=$RUN_ID"
  echo "docker=$($DOCKER --version)"
  echo "image-id=$($DOCKER image inspect -f "{{.Id}}" "$IMAGE" 2>/dev/null || echo unknown)"
  echo "cpus-limit=$( [ "${C3D_NO_CPU_LIMIT:-0}" = 1 ] && echo none || echo "$CPUS" )"
  [ "${C3D_NO_CPU_LIMIT:-0}" = 1 ] && echo "cpus-limit-note=rootless crun rejects --cpus; measurement container uncapped (recorded deviation)"
  [ "${C3D_MOUNT_Z:-0}" = 1 ] && echo "mount-flags=Z (SELinux Enforcing host)"
  echo "host-cpu=$(LC_ALL=C lscpu 2>/dev/null | grep -m1 "Model name" | cut -d: -f2 | xargs || grep -m1 "model name" /proc/cpuinfo | cut -d: -f2 | xargs)"
  echo "host-kernel=$(uname -r)"
  v=$(systemd-detect-virt 2>/dev/null || true); [ -n "$v" ] || v=unknown
  echo "virtualization=$v"
  echo "source-tar-sha256=$SRC_SHA"
  echo "source-commit=$SRC_COMMIT"
  echo "bin-sha256=$BIN_SHA"
  echo "batches=$BATCHES"
} > "$DEST/host-$LABEL.txt"
cp "$WORK/build.log" "$WORK/run.log" "$DEST/" 2>/dev/null || true
chmod -R u+rwX "$WORK" 2>/dev/null || true
rm -rf "$WORK"

# fail-closed postcondition: all three tiers present, all batches landed
python3 - "$DEST/decompose-summary.json" "$BATCHES" <<'EOF'
import json, sys
s = json.load(open(sys.argv[1])); batches = int(sys.argv[2])
names = {t["tier"] for t in s["tiers"]}
assert names == {"A_raw_call", "A2_raw_call_extract", "B_slotless_invoke"}, names
for t in s["tiers"]:
    assert t["samples"] == batches, f"{t['tier']}: {t['samples']} != {batches}"
    assert t["correctness_checks"] >= 1, t["tier"]
print("POSTCONDITION-OK:", ", ".join(
    f"{t['tier']} p50={t['p50_us']:.3f}us" for t in s["tiers"]))
print("derived:", s["derived"])
EOF

echo "DONE: $DEST"
