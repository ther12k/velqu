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
#   bash scripts/publish-beta.sh                 # every package not already live
#   bash scripts/publish-beta.sh --only browser-pglite
#
# The publish order follows the dependency graph so every workspace:*
# range resolves to an already-published version. In real mode a package
# whose exact version already exists in the registry is SKIPPED (the
# registry rejects republishing), which makes re-runs idempotent: the run
# proceeds to packages still missing — e.g. a package added after the
# original seven went live. --only restricts the run to one package;
# it publishes nothing else, so any @velqu/* dependency the target
# imports must already be on the registry (true for every package in
# dependency order). --dry-run performs no registry queries and lists
# every selected package.
set -euo pipefail
cd "$(dirname "$0")/.."

ALL_PKGS=(contract schema core treaty browser-runtime browser-pglite compiler cli)
VERSION="$(bun -e 'console.log(require("./packages/contract/package.json").version)')"
DRY_RUN=0
ONLY=""

while [ $# -gt 0 ]; do
  case "$1" in
    --dry-run) DRY_RUN=1 ;;
    --only)
      [ $# -ge 2 ] || { echo "--only needs a package name" >&2; exit 2; }
      ONLY="$2"; shift ;;
    --only=*) ONLY="${1#--only=}" ;;
    *) echo "usage: $0 [--dry-run] [--only <pkg>]" >&2; exit 2 ;;
  esac
  shift
done

if [ -n "$ONLY" ]; then
  known=0
  for pkg in "${ALL_PKGS[@]}"; do [ "$pkg" = "$ONLY" ] && known=1; done
  [ "$known" = 1 ] || { echo "unknown package: $ONLY (known: ${ALL_PKGS[*]})" >&2; exit 2; }
fi

bun scripts/build-packages.ts

ARGS=(--tag beta)
if [ "$DRY_RUN" = 1 ]; then
  ARGS+=(--dry-run)
  echo "== DRY RUN — nothing will be published; registry skip-check bypassed =="
fi

for pkg in "${ALL_PKGS[@]}"; do
  if [ -n "$ONLY" ] && [ "$pkg" != "$ONLY" ]; then continue; fi
  if [ "$DRY_RUN" = 0 ]; then
    # Anonymous registry read: exits non-zero when this exact version
    # does not exist yet. Existing version => skip (republish is a
    # registry rejection under set -e; idempotent re-runs must reach
    # packages that are still missing).
    if existing="$(npm view "@velqu/$pkg@$VERSION" version 2>/dev/null)" \
      && [ "$existing" = "$VERSION" ]; then
      echo "== @velqu/$pkg == SKIP (already published as $VERSION)"
      continue
    fi
  fi
  echo "== @velqu/$pkg =="
  (cd "packages/$pkg" && bun publish "${ARGS[@]}")
done

echo "done."
