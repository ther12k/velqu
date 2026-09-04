#!/usr/bin/env bash
# BETA-015-D — npm package tarballs for the release evidence directory.
# Packs every @velqu/* workspace package with `bun pm pack` into
# release/npm-tarballs/ and verifies SHA256SUMS.txt from inside it.
# The packages are private: this produces shippable tarballs + checksums
# only; publication remains Owner-gated (BETA-010-C/BETA-011 posture).
set -euo pipefail
cd "$(dirname "$0")/.."

OUT=release/npm-tarballs
rm -rf "$OUT" && mkdir -p "$OUT"

shopt -s nullglob
PACKAGES=(packages/*/package.json)
shopt -u nullglob
[[ ${#PACKAGES[@]} -gt 0 ]] || { echo "no workspace packages found" >&2; exit 1; }

for pkgjson in "${PACKAGES[@]}"; do
  dir=$(dirname "$pkgjson")
  name=$(python3 -c "import json;print(json.load(open('$pkgjson'))['name'])")
  echo "pack: $name ($dir)"
  (cd "$dir" && bun pm pack --destination "$(cd ../.. && pwd)/$OUT" >/dev/null)
done

(
  cd "$OUT"
  : > SHA256SUMS.txt
  for f in *.tgz; do
    sha256sum "$f" >> SHA256SUMS.txt
  done
)

count=$(ls "$OUT"/*.tgz | wc -l)
echo "npm tarballs: $count packages -> $OUT"
(cd "$OUT" && sha256sum -c SHA256SUMS.txt)
echo "NPM-TARBALLS-OK"
