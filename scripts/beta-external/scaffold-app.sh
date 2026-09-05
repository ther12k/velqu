#!/usr/bin/env bash
# BETA-016-C — external-user app scaffold verification.
#
# Runs INSIDE the fresh beta environment as the unprivileged `beta`
# user, using only the tree installed by BETA-016-B (~/velqu). Repeats
# the documented quickstart journey (docs/beta/QUICKSTART.md) verbatim:
# scaffold a starter app, link the workspace packages as documented,
# and structurally verify the result with `velqu check`.
#
# Usage: scaffold-app.sh [install-dir] [app-dir]
set -euo pipefail

INSTALL="${1:-$HOME/velqu}"
APP="${2:-$HOME/hello-velqu}"
CLI="$INSTALL/packages/cli/src/index.ts"
STEP=0
step() { STEP=$((STEP+1)); echo "== [$STEP] \$ $*"; }
fail() { echo "SCAFFOLD-FAIL at step $[$STEP]: $*" >&2; exit 1; }

[ "$(id -un)" = "beta" ] || { echo "run as the unprivileged 'beta' user" >&2; exit 1; }
[ -f "$CLI" ] || { echo "installed CLI not found at $CLI — run install-cli-runtime.sh first" >&2; exit 1; }

echo "== external scaffold transcript =="
echo "user=$(id -un) install=$INSTALL app=$APP"

step "rm -rf $APP (clean slate / uninstall path)"
rm -rf "$APP"

step "bun $CLI create $(basename "$APP") --name $(basename "$APP")"
(cd "$INSTALL" && bun "$CLI" create "$APP" --name "$(basename "$APP")") \
  || fail "scaffold creation failed"

step "link @velqu workspace packages (documented quickstart step)"
mkdir -p "$APP/node_modules/@velqu"
for p in core schema treaty; do
  ln -sfn "$INSTALL/packages/$p" "$APP/node_modules/@velqu/$p"
done
ls "$APP/node_modules/@velqu"

step "verify scaffold structure (package.json, src/app.ts, health route)"
[ -f "$APP/package.json" ] || fail "package.json missing"
[ -f "$APP/src/app.ts" ] || fail "src/app.ts missing"
grep -q "health" "$APP/src/app.ts" || fail "health route missing from src/app.ts"

step "bun $CLI check --project $APP"
OUT="$(cd "$INSTALL" && bun "$CLI" check --project "$APP" 2>&1)" || fail "check failed: $OUT"
echo "$OUT"
echo "$OUT" | grep -q "3 routes" || fail "check did not report 3 routes: $OUT"

echo "SCAFFOLD-OK"
echo "uninstall: rm -rf $APP; nothing outside \$HOME and the install tree was touched"
