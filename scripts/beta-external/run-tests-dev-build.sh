#!/usr/bin/env bash
# BETA-016-D — external-user tests/dev/build verification.
#
# Runs INSIDE the fresh beta environment as the unprivileged `beta`
# user against the scaffold produced by BETA-016-C (~/hello-velqu).
# Exercises the documented journey (docs/beta/QUICKSTART.md):
#   bun run test    — scaffold test suite
#   bun run build   — deterministic app.qpack (built twice, hashes compared)
#   velqu dev       — development reload loop on :8084, probed over HTTP
# plus the production runtime serving the built pack (INSTALL.md Step 2).
#
# Usage: run-tests-dev-build.sh [install-dir] [app-dir]
set -euo pipefail

INSTALL="${1:-$HOME/velqu}"
APP="${2:-$HOME/hello-velqu}"
CLI="$INSTALL/packages/cli/src/index.ts"
RUNTIME="$INSTALL/target/release/velqu-runtime"
STEP=0
step() { STEP=$((STEP+1)); echo "== [$STEP] \$ $*"; }
fail() { echo "DEVBUILD-FAIL at step $[$STEP]: $*" >&2; exit 1; }
cleanup() { [ -n "${DEV_PID:-}" ] && kill "$DEV_PID" 2>/dev/null; [ -n "${RT_PID:-}" ] && kill "$RT_PID" 2>/dev/null; return 0; }
trap cleanup EXIT

[ "$(id -un)" = "beta" ] || { echo "run as the unprivileged 'beta' user" >&2; exit 1; }
[ -d "$APP" ] || { echo "scaffold not found at $APP — run scaffold-app.sh first" >&2; exit 1; }
[ -x "$RUNTIME" ] || { echo "runtime not built at $RUNTIME — run install-cli-runtime.sh first" >&2; exit 1; }

probe() { # probe <url> <expected-substring>
  local out
  out="$(curl -sf --max-time 5 "$1")" || fail "GET $1 failed"
  echo "$out"
  case "$out" in *"$2"*) ;; *) fail "GET $1: expected '$2' in '$out'";; esac
}

echo "== external tests/dev/build transcript =="
echo "user=$(id -un) install=$INSTALL app=$APP"

step "link @velqu workspace packages incl. cli (documented quickstart step, repeat per project)"
mkdir -p "$APP/node_modules/@velqu"
for p in core schema treaty cli; do
  ln -sfn "$INSTALL/packages/$p" "$APP/node_modules/@velqu/$p"
done
ls "$APP/node_modules/@velqu"

step "cd $APP && bun run test"
(cd "$APP" && bun run test) || fail "scaffold test suite failed"

step "cd $APP && bun run build (first build)"
(cd "$APP" && bun run build) || fail "build failed"
PACK="$APP/dist/app.qpack"
[ -f "$PACK" ] || fail "dist/app.qpack missing after build"
H1="$(sha256sum "$PACK" | cut -d' ' -f1)"

step "bun run build again — determinism check"
(cd "$APP" && bun run build) || fail "second build failed"
H2="$(sha256sum "$PACK" | cut -d' ' -f1)"
[ "$H1" = "$H2" ] || fail "app.qpack not deterministic: $H1 vs $H2"
echo "app.qpack sha256=$H1 (identical across builds)"

step "bun run check"
(cd "$APP" && bun run check) || fail "check failed"

step "velqu dev --project $APP --port 8084 (background) + HTTP probes"
(cd "$APP" && bun "$CLI" dev --project "$APP" --port 8084 >/dev/null 2>&1) &
DEV_PID=$!
for i in $(seq 1 30); do curl -sf --max-time 2 http://127.0.0.1:8084/health/live >/dev/null 2>&1 && break; sleep 1; done
probe http://127.0.0.1:8084/health/live '"status":"ok"'
probe http://127.0.0.1:8084/greetings/dev '"message":"Hello, dev!"'
kill "$DEV_PID" 2>/dev/null || true; wait "$DEV_PID" 2>/dev/null || true; DEV_PID=""

step "velqu-runtime --pack dist/app.qpack --port 8081 (background) + HTTP probes"
"$RUNTIME" --pack "$PACK" --port 8081 >/dev/null 2>&1 &
RT_PID=$!
for i in $(seq 1 30); do curl -sf --max-time 2 http://127.0.0.1:8081/health/live >/dev/null 2>&1 && break; sleep 1; done
probe http://127.0.0.1:8081/health/live '"status":"ok"'
probe http://127.0.0.1:8081/greetings/world '"message":"Hello, world!"'
kill "$RT_PID" 2>/dev/null || true; wait "$RT_PID" 2>/dev/null || true; RT_PID=""

echo "DEVBUILD-OK"
echo "artifacts: $APP/dist (rm -rf to reset); app dir uninstall: rm -rf $APP"
