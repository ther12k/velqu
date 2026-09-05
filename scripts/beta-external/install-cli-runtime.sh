#!/usr/bin/env bash
# BETA-016-B — external-user install of the Velqu CLI and runtime.
#
# Runs INSIDE the fresh beta environment (scripts/beta-external/Dockerfile)
# as the unprivileged `beta` user, from a source archive only — no git
# checkout, no workspace links, no prebuilt artifacts. Repeats the
# documented path from docs/beta/INSTALL.md verbatim:
#   bun install --frozen-lockfile
#   cargo build --release -p velqu-runtime
# then proves the runtime and CLI binaries respond, and prints how to
# uninstall (guardrail: artifacts can be rolled back/uninstalled).
#
# Usage: install-cli-runtime.sh <source-archive> [workdir]
set -euo pipefail

ARCHIVE="${1:?usage: install-cli-runtime.sh <source-archive> [workdir]}"
WORK="${2:-$HOME/velqu}"
STEP=0
step() { STEP=$((STEP+1)); echo "== [$STEP] \$ $*"; }
fail() { echo "INSTALL-FAIL at step $[$STEP]: $*" >&2; exit 1; }

[ "$(id -un)" = "beta" ] || { echo "run as the unprivileged 'beta' user" >&2; exit 1; }
[ -f "$ARCHIVE" ] || { echo "archive not found: $ARCHIVE" >&2; exit 1; }

echo "== external install transcript =="
echo "user=$(id -un) home=$HOME"
echo "archive=$(basename "$ARCHIVE") sha256=$(sha256sum "$ARCHIVE" | cut -d' ' -f1)"

step "rm -rf $WORK (clean slate / uninstall path)"
rm -rf "$WORK"

step "mkdir -p $WORK && tar -xzf $(basename "$ARCHIVE") -C $WORK --strip-components=1"
mkdir -p "$WORK"
tar -xzf "$ARCHIVE" -C "$WORK" --strip-components=1
[ -f "$WORK/package.json" ] || fail "package.json missing after extraction — the archive must carry a single root directory (git archive --prefix=velqu/), and --strip-components=1 removes exactly that"

step "cd $WORK && bun install --frozen-lockfile"
cd "$WORK"
bun install --frozen-lockfile || fail "bun install failed"

step "cargo build --release -p velqu-runtime"
cargo build --release -p velqu-runtime || fail "cargo build failed"

step "./target/release/velqu-runtime --help"
"$WORK/target/release/velqu-runtime" --help >/dev/null || fail "runtime --help failed"

step "bun packages/cli/src/index.ts --help"
bun packages/cli/src/index.ts --help >/dev/null || fail "cli --help failed"

echo "INSTALL-OK"
echo "uninstall: rm -rf $WORK (and the archive); nothing else was written outside \$HOME"
