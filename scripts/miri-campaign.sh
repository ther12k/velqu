#!/usr/bin/env bash
# #1318 / M6-003 — Miri campaign over the FFI-free crates.
#
# Miri cannot execute the rquickjs C FFI boundary (quickjs-ng is native
# C, not Rust) — those crates are excluded with an explicit, recorded
# exclusion, per the owner's findings-ledger requirement. Covered: the
# pure-Rust logic crates where UB classes (alignment, overflow, OOB,
# aliasing) live.
#
# Chained: waits for the fuzz-campaign driver to finish (no CPU
# competition with the soak or the libFuzzer campaigns).
set -uo pipefail
cd "$(dirname "$0")/.."

while pgrep -f "scripts/fuzz-campaign.sh" | grep -v "^$$\$" >/dev/null 2>&1; do
  sleep 120
done

OUT="benchmarks/raw/ga-m6-fuzz"
mkdir -p "$OUT"
MIRI_CRATES=(q-runtime-model q-router q-schema-runtime q-bridge q-pack)
EXCLUDED="q-engine-quickjs, q-engine, q-runtime, q-http, q-capabilities, q-capability-postgres, q-bytecode-tool, q-browser-kernel, velqu-runtime (FFI/tokio/hyper/memmap2/rquickjs boundaries — Miri does not execute foreign C code; covered by the ASan+UBSan pass and the libFuzzer campaigns instead)"

{
  echo "{"
  echo " \"campaign\": \"ga-m6-miri\","
  echo " \"startedAt\": \"$(date -u +%Y-%m-%dT%H:%M:%SZ)\","
  echo " \"toolchain\": \"$(rustc +nightly --version)\","
  echo ' "excludedWithReason": "'"$(echo "$EXCLUDED" | sed 's/"/\\"/g')"',"
  echo ' "crates": ['
} > "$OUT/miri-ledger.json"

first=1
fail=0
for c in "${MIRI_CRATES[@]}"; do
  nice -n 10 cargo +nightly miri test -p "$c" --quiet > "$OUT/miri-$c.log" 2>&1
  rc=$?
  [ $first = 0 ] && echo "," >> "$OUT/miri-ledger.json"
  first=0
  {
    echo "  { \"crate\": \"$c\", \"exitCode\": $rc, \"log\": \"benchmarks/raw/ga-m6-fuzz/miri-$c.log\","
    if [ $rc = 0 ]; then
      echo '    "result": "zero-findings" }'
    else
      echo '    "result": "findings — UB reported; regression test or disposition required" }'
      fail=1
    fi
  } >> "$OUT/miri-ledger.json"
done

echo "" >> "$OUT/miri-ledger.json"
echo "], \"anyFindings\": $fail }" >> "$OUT/miri-ledger.json"
echo "miri campaign complete: $OUT/miri-ledger.json"
exit $fail
