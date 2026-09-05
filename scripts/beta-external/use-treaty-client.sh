#!/usr/bin/env bash
# BETA-016-F — external-user Treaty client verification.
#
# Runs INSIDE the fresh beta environment as the unprivileged `beta`
# user against the scaffold from BETA-016-C (~/hello-velqu). Starts the
# documented dev server on 127.0.0.1:3000, drives the live runtime via
# the scaffold's type-safe Treaty client (`bun run client`), and runs
# the scaffold test suite asserting the Treaty contract tests actually
# execute against the live runtime (no skip fallback).
#
# Usage: use-treaty-client.sh [app-dir]
set -euo pipefail

APP="${1:-$HOME/hello-velqu}"
STEP=0
step() { STEP=$((STEP+1)); echo "== [$STEP] \$ $*"; }
fail() { echo "TREATY-FAIL at step $[$STEP]: $*" >&2; exit 1; }
# bun run nests processes: wrapper subshell -> `bun run dev` -> dev CLI
# -> spawned velqu-runtime. A plain kill of the wrapper orphans the rest,
# and pkill(1) is not installed in the beta image — so reap our own tree
# by matching /proc cmdlines (own-user processes only).
kill_own_matching() {
  local pat="$1" d pid owner
  owner="$(id -u)"
  for d in /proc/[0-9]*; do
    pid="${d#/proc/}"
    [ "$pid" = "$$" ] && continue
    if tr '\0' ' ' < "$d/cmdline" 2>/dev/null | grep -qF "$pat"; then
      [ "$(stat -c %u "$d" 2>/dev/null)" = "$owner" ] && kill "$pid" 2>/dev/null || true
    fi
  done
}
cleanup() {
  [ -n "${DEV_PID:-}" ] && kill "$DEV_PID" 2>/dev/null || true
  kill_own_matching "bun run dev"
  kill_own_matching "index.ts dev --project"
  kill_own_matching "velqu-runtime --pack /tmp/velqu-incremental"
  return 0
}
trap cleanup EXIT

[ "$(id -un)" = "beta" ] || { echo "run as the unprivileged 'beta' user" >&2; exit 1; }
[ -d "$APP" ] || { echo "scaffold not found at $APP — run scaffold-app.sh first" >&2; exit 1; }

echo "== external treaty client transcript =="
echo "user=$(id -un) app=$APP"

step "pre-flight: port 3000 must be free (a foreign service here poisons every probe)"
if curl -sf --max-time 2 http://127.0.0.1:3000/health/live >/dev/null 2>&1; then
  fail "something is already listening on 127.0.0.1:3000 — stop it first (BETA-016-F: a leftover proof runtime from an earlier packet masked every greeting probe with 404s)"
fi

step "cd $APP && bun run dev --port 3000 (background; Treaty client targets 127.0.0.1:3000)"
(cd "$APP" && bun run dev >/dev/null 2>&1) &
DEV_PID=$!
for _ in $(seq 1 50); do
  curl -sf --max-time 2 http://127.0.0.1:3000/health/live >/dev/null 2>&1 && break
  sleep 0.2
done
curl -sf http://127.0.0.1:3000/health/live >/dev/null || fail "dev server not reachable on 3000"

step "assert the 3000 listener is OUR scaffold (a greetings route must answer)"
OUT="$(curl -sf --max-time 2 http://127.0.0.1:3000/greetings/precheck)" || fail "greetings route 404 on :3000 — the listener is not the scaffold dev server"
case "$OUT" in *'"message"'*) ;; *) fail "unexpected precheck body: $OUT";; esac
echo "precheck OK: $OUT"

step "cd $APP && bun run client (Treaty: health.live, greetings.create, greetings.get)"
OUT="$(cd "$APP" && bun run client 2>&1)" || fail "client run failed: $OUT"
echo "$OUT"
case "$OUT" in *"Health OK"*) ;; *) fail "client did not report Health OK";; esac
case "$OUT" in *"Created greeting:"*) ;; *) fail "client did not create a greeting";; esac
case "$OUT" in *"Message:"*) ;; *) fail "client did not fetch the greeting message";; esac

step "cd $APP && bun test (Treaty contract tests must RUN, not skip)"
OUT="$(cd "$APP" && bun test 2>&1)" || fail "scaffold test suite failed: $OUT"
echo "$OUT" | grep -E "[0-9]+ pass|[0-9]+ fail"
case "$OUT" in *"skipping: dev server not reachable"*) fail "Treaty tests skipped — they must run against the live dev server";; esac
echo "$OUT" | grep -q "0 fail" || fail "test failures in scaffold suite"

cleanup
wait "$DEV_PID" 2>/dev/null || true; DEV_PID=""
# SIGTERM propagation + child reap is asynchronous; wait bounded for the
# port to actually close before judging teardown.
PORT_OPEN=0
for _ in $(seq 1 50); do
  if ! curl -sf --max-time 1 http://127.0.0.1:3000/health/live >/dev/null 2>&1; then
    PORT_OPEN=0; break
  fi
  PORT_OPEN=1; sleep 0.1
done
[ "$PORT_OPEN" = "1" ] && fail "dev server still answering after teardown"
echo "teardown OK: port 3000 released"
echo "TREATY-OK: typed client calls executed against the live runtime; contract tests ran without skipping"
echo "rollback: nothing persisted (dev server state is in-memory); rm -rf $APP uninstalls the app"
