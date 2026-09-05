#!/usr/bin/env bash
# BWASM-K-004 — measure the real wasm32 size contribution of
# q-schema-runtime via the exported-validate probe. gzip-9 is an
# interim proxy (brotli/wasm-opt absent from the measurement host;
# ADR-0039 §measurability notes).
set -euo pipefail
cd "$(dirname "$0")"
cargo build --release --target wasm32-unknown-unknown
W=target/wasm32-unknown-unknown/release/bwasm_size_probe.wasm
echo "raw=$(stat -c%s "$W") bytes"
echo "gzip9=$(gzip -9 -kc "$W" | wc -c) bytes"
sha256sum "$W"
