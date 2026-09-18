#!/usr/bin/env bash
# benchmarks/c3-crosshost — matched old/new A/B for the C3 slotless packets,
# run on local / Halotec / Oracle inside the pinned multihost benchmark image
# (digest-pinned ubuntu:24.04 + rustc 1.96.0 + Bun 1.4.0 — same toolchain on
# every host). Descriptive/diagnostic evidence only (AGENTS constraint 12).
#
#   benchmarks/c3-crosshost/run.sh <label> <src-old.tar.gz> <src-new.tar.gz> <reps> [cpus]
#
# <label>          host label (local | halotec | oracle)
# <src-old.tar.gz> git archive of the PRE-slotless snapshot  (5170c5a6)
# <src-new.tar.gz> git archive of the POST-slotless snapshot (b51927ed)
# <reps>           matched pairs per concurrency (>=5; 8-10 on noisy hosts)
# [cpus]           measurement-container CPU cap (default 2)
#
# Requires the velqu-bench:multihost image (built by benchmarks/multihost).
# Env:
#   DOCKER=docker|podman       C3X_NO_CPU_LIMIT=1   (rootless crun hosts)
#   C3X_MOUNT_Z=1              (SELinux-Enforcing hosts)
#   BENCH_CARGO_JOBS=N         build parallelism
#   C3X_DURATION=10 C3X_CONC=1,10,50
#
# Two-phase on purpose: the BUILD container is uncapped (cargo needs the
# headroom); only the MEASUREMENT container is CPU/memory-limited, so the
# cap can never distort compiler timing vs runtime timing.
set -euo pipefail

LABEL=${1:?usage: run.sh <label> <src-old.tar.gz> <src-new.tar.gz> <reps> [cpus]}
SRC_OLD=$(realpath "${2:?src-old.tar.gz}")
SRC_NEW=$(realpath "${3:?src-new.tar.gz}")
REPS=${4:?reps}
CPUS=${5:-2}
DOCKER=${DOCKER:-docker}
DURATION=${C3X_DURATION:-10}
CONC=${C3X_CONC:-1,10,50}
JOBS=${BENCH_CARGO_JOBS:-0}
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
OUT_BASE="$ROOT/benchmarks/raw/c3-probe"
IMAGE=velqu-bench:multihost

command -v "$DOCKER" >/dev/null || { echo "no $DOCKER" >&2; exit 1; }
for f in "$SRC_OLD" "$SRC_NEW"; do [ -f "$f" ] || { echo "missing $f" >&2; exit 1; }; done
"$DOCKER" image inspect "$IMAGE" >/dev/null 2>&1 || {
  echo "image $IMAGE not found — build it with benchmarks/multihost/run.sh first" >&2; exit 1;
}

# harness snapshot: this wrapper's own tree (carries the v3 balanced probe).
# git archive when this is a repo; plain-tree fallback for remote hosts that
# received the kit as a tarball (no .git). C3X_HARNESS_TAR overrides both.
HARNESS_TAR=$(mktemp /tmp/c3x-harness-XXXXXX.tar.gz)
if [ -n "${C3X_HARNESS_TAR:-}" ]; then
  cp "$C3X_HARNESS_TAR" "$HARNESS_TAR"
  HARNESS_COMMIT="explicit-tar"
elif git -C "$ROOT" rev-parse --git-dir >/dev/null 2>&1; then
  git -C "$ROOT" archive --format=tar.gz -o "$HARNESS_TAR" HEAD
  HARNESS_COMMIT=$(git -C "$ROOT" rev-parse HEAD)
else
  tar -czf "$HARNESS_TAR" -C "$ROOT" benchmarks/harness/c3-probe.ts
  HARNESS_COMMIT="tree-fallback:benchmarks/harness/c3-probe.ts"
fi
trap 'rm -f "$HARNESS_TAR"' EXIT

WORK=$(mktemp -d "${C3X_WORK_DIR:-/tmp}/c3x-work-XXXXXX")
mkdir -p "$WORK/bin" "$WORK/pack" "$WORK/out"
trap 'rm -rf "$WORK"; rm -f "$HARNESS_TAR"' EXIT
cp "$SRC_OLD" "$WORK/src-old.tar.gz"
cp "$SRC_NEW" "$WORK/src-new.tar.gz"
cp "$HARNESS_TAR" "$WORK/src-harness.tar.gz"

MFLAGS=""
[ "${C3X_MOUNT_Z:-0}" = 1 ] && MFLAGS=":Z"
UIDGID="$(id -u):$(id -g)"
RUN_ID="c3x-${LABEL}-$(date -u +%Y%m%dT%H%M%SZ)"

echo "== phase 1: build both snapshots (uncapped) =="
"$DOCKER" run --rm \
  -e BENCH_CARGO_JOBS="$JOBS" -e UIDGID="$UIDGID" \
  -v "$WORK:/work$MFLAGS" \
  "$IMAGE" bash -ceu '
    set -eu
    mkdir -p /work/old /work/new
    tar -xzf /work/src-old.tar.gz -C /work/old
    tar -xzf /work/src-new.tar.gz -C /work/new
    rm -f /work/src-old.tar.gz /work/src-new.tar.gz

    cd /work/old
    export CARGO_TARGET_DIR=/work/old-target
    export RUSTFLAGS="--remap-path-prefix=/work/old=/velqu-src-old"
    [ "$BENCH_CARGO_JOBS" = 0 ] || export CARGO_BUILD_JOBS="$BENCH_CARGO_JOBS"
    cargo build --release -p velqu-runtime --features bench-instrumentation
    cp /work/old-target/release/velqu-runtime /work/bin/velqu-runtime-old
    rm -rf /work/old-target /work/old

    cd /work/new
    bun install --frozen-lockfile
    export CARGO_TARGET_DIR=/work/new-target
    export RUSTFLAGS="--remap-path-prefix=/work/new=/velqu-src-new"
    cargo build --release -p velqu-runtime --features bench-instrumentation
    cp /work/new-target/release/velqu-runtime /work/bin/velqu-runtime-new
    bun packages/cli/src/index.ts build --project examples/proof
    cp examples/proof/dist/app.qpack /work/pack/app.qpack
    rm -rf /work/new-target
    sha256sum /work/bin/velqu-runtime-old /work/bin/velqu-runtime-new /work/pack/app.qpack
    [ "$(sha256sum /work/bin/velqu-runtime-old | cut -d" " -f1)" != "$(sha256sum /work/bin/velqu-runtime-new | cut -d" " -f1)" ]
    chown -R "$UIDGID" /work
  ' | tee "$WORK/build.log"
echo "build done: $(sha256sum "$WORK/bin/velqu-runtime-old" | cut -c1-16) / $(sha256sum "$WORK/bin/velqu-runtime-new" | cut -c1-16)"

echo "== phase 2: matched measurement (cpus=${C3X_NO_CPU_LIMIT:-$CPUS}) run-id=$RUN_ID =="
CAPS=(--cpus="$CPUS" --memory=1g --memory-swap=1g)
if [ "${C3X_NO_CPU_LIMIT:-0}" = 1 ]; then CAPS=(); fi
"$DOCKER" run --rm \
  "${CAPS[@]}" \
  --security-opt no-new-privileges \
  -e C3PROBE_OLD=/work/bin/velqu-runtime-old \
  -e C3PROBE_NEW=/work/bin/velqu-runtime-new \
  -e C3PROBE_PACK=/work/pack/app.qpack \
  -e C3PROBE_OUT=/work/out \
  -e C3PROBE_RUN_ID="$RUN_ID" \
  -e C3PROBE_LABEL="$LABEL" \
  -e C3PROBE_DURATION="$DURATION" \
  -e C3PROBE_REPS="$REPS" \
  -e C3PROBE_CONC="$CONC" \
  -e UIDGID="$UIDGID" \
  -v "$WORK:/work$MFLAGS" \
  "$IMAGE" bash -ceu '
    set -eu
    mkdir -p /work/harness && tar -xzf /work/src-harness.tar.gz -C /work/harness
    rm -f /work/src-harness.tar.gz
    cd /work/harness
    bun benchmarks/harness/c3-probe.ts
    chown -R "$UIDGID" /work
  ' | tee "$WORK/measure.log"

DEST="$OUT_BASE"
mkdir -p "$DEST"
cp -r "$WORK/out/." "$DEST/"
{
  echo "label=$LABEL"
  echo "run-id=$RUN_ID"
  echo "harness-commit=$HARNESS_COMMIT"
  echo "docker=$($DOCKER --version)"
  echo "image-id=$($DOCKER image inspect -f "{{.Id}}" "$IMAGE" 2>/dev/null || echo unknown)"
  echo "cpus-limit=${C3X_NO_CPU_LIMIT:-$CPUS}"
  [ "${C3X_NO_CPU_LIMIT:-0}" = 1 ] && echo "cpus-limit-note=rootless crun rejects --cpus (cpu controller not delegated); measurement container uncapped — recorded deviation"
  [ "${C3X_MOUNT_Z:-0}" = 1 ] && echo "mount-flags=Z (SELinux Enforcing host)"
  echo "host-cpu=$(LC_ALL=C lscpu 2>/dev/null | grep -m1 "Model name" | cut -d: -f2 | xargs || grep -m1 "model name" /proc/cpuinfo | cut -d: -f2 | xargs)"
  echo "host-kernel=$(uname -r)"
  v=$(systemd-detect-virt 2>/dev/null || true); [ -n "$v" ] || v=unknown
  echo "virtualization=$v"
  echo "build-features=bench-instrumentation (both candidates, identical instrumentation)"
  echo "protocol=balanced-interleaved duration=${DURATION}s reps=$REPS conc=$CONC"
  sha256sum "$WORK/bin/velqu-runtime-old" "$WORK/bin/velqu-runtime-new" "$WORK/pack/app.qpack"
} > "$DEST/$RUN_ID/host-$LABEL.txt"
cp "$WORK/build.log" "$WORK/measure.log" "$DEST/$RUN_ID/" 2>/dev/null || true

# fail-closed postcondition: every candidate/concurrency must have exactly
# $REPS cells and ZERO errors, and every pair must have produced a ratio
# (summary lives at $DEST/$RUN_ID.summary.json — probe OUT_DIR convention)
python3 - "$DEST/$RUN_ID.summary.json" "$REPS" <<'EOF'
import json, sys
s = json.load(open(sys.argv[1])); reps = int(sys.argv[2])
for row in s["aggregate"]:
    assert row["reps"] == reps, f"{row['candidate']} c={row['concurrency']}: {row['reps']} reps != {reps}"
    assert row["totalErrors"] == 0, f"{row['candidate']} c={row['concurrency']}: {row['totalErrors']} errors"
for row in s["pairedRatios"]:
    assert row["pairs"] == reps, f"c={row['concurrency']}: {row['pairs']} pairs != {reps}"
print(f"POSTCONDITION-OK: {len(s['aggregate'])} cells x {reps} reps, 0 errors, all pairs matched")
EOF

echo "DONE: $DEST/$RUN_ID"
