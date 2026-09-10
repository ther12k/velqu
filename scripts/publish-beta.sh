#!/usr/bin/env bash
# Ordered beta publish for every @velqu/* package.
#
# OWNER ACTION: real publishing requires npm auth (2FA-enabled account,
# @velqu org) and the recorded publication decision (OD-050). Until that
# record exists, run only the dry-run form:
#
#   bash scripts/publish-beta.sh --dry-run
#
# Real publish (after the owner decision):
#
#   bash scripts/publish-beta.sh
#
# The publish order follows the dependency graph so every workspace:*
# range resolves to an already-published version.
set -euo pipefail
cd "$(dirname "$0")/.."

MODE="${1:-}"
if [ "$MODE" != "--dry-run" ] && [ "$MODE" != "" ]; then
  echo "usage: $0 [--dry-run]" >&2
  exit 2
fi

bun scripts/build-packages.ts

ARGS=(--tag beta)
if [ "$MODE" = "--dry-run" ]; then
  ARGS+=(--dry-run)
  echo "== DRY RUN — nothing will be published =="
fi

for pkg in contract schema core treaty browser-runtime browser-pglite compiler cli; do
  echo "== @velqu/$pkg =="
  (cd "packages/$pkg" && bun publish "${ARGS[@]}")
done

echo "done."
