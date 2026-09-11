#!/usr/bin/env bash
# #1318 / M6-002 — sustained fuzz + sanitizer campaign driver.
#
# Runs the sustained, duration-configured campaigns and records a
# machine-readable findings ledger per campaign. Deliberately CHAINED
# after the 24 h soak (#1319) via --wait-for-pid: the soak's leak
# analysis (flat-RSS claim) requires an unpolluted host, so heavy
# fuzzing/sanitizer load must not overlap it. With --wait-for-pid N the
# driver blocks until PID N exits, then runs.
#
# Findings policy (owner directive, 2026-09-11): every campaign records
# duration/seed/corpus size, and for each finding either a regression
# test reference or — if none were found — an explicit zero-findings
# record. No bare "green" claims.
#
# Usage:
#   bash scripts/fuzz-campaign.sh [--wait-for-pid N] [--smoke]
set -uo pipefail
cd "$(dirname "$0")/.."

CAMPAIGN_DATE="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
SMOKE=0
WAIT_PID=""

while [ $# -gt 0 ]; do
  case "$1" in
    --wait-for-pid) WAIT_PID="$2"; shift ;;
    --smoke) SMOKE=1 ;;
  esac
  shift
done

if [ -n "$WAIT_PID" ]; then
  echo "waiting for pid $WAIT_PID (soak) to finish before fuzzing..."
  while kill -0 "$WAIT_PID" 2>/dev/null; do sleep 60; done
  echo "soak finished; starting campaigns at $(date -u +%Y-%m-%dT%H:%M:%SZ)"
fi

TARGETS=(pack_verify router_match http_decode schema_validate bridge_handles)
# Sustained: 900 s (15 min) per target = 75 min total. Smoke: 60 s.
if [ "$SMOKE" = 1 ]; then DURATION=60; else DURATION=900; fi
SEED="$(date +%s)"
OUT="benchmarks/raw/ga-m6-fuzz"
mkdir -p "$OUT"

findings_for() { # target -> "0" when no crash/leak/timeout artifacts exist
  local n
  n=$(find fuzz -name "crash-*" -o -name "leak-*" -o -name "timeout-*" -o -name "oom-*" 2>/dev/null | wc -l)
  echo "$n"
}

{
  echo "{"
  echo ' "campaign": "ga-m6-sustained-fuzz",'
  echo " \"startedAt\": \"$CAMPAIGN_DATE\","
  echo " \"durationPerTargetSecs\": $DURATION,"
  echo " \"seed\": $SEED,"
  echo " \"fuzzer\": \"libFuzzer (cargo-fuzz 0.13.2, nightly $(rustc +nightly --version | cut -d' ' -f2))\","
  echo ' "targets": ['
} > "$OUT/campaign-ledger.json"

first=1
crash_total=0
for t in "${TARGETS[@]}"; do
  before=$(find fuzz -name "crash-*" -o -name "leak-*" -o -name "timeout-*" -o -name "oom-*" 2>/dev/null | wc -l)
  corpus_before=$(ls "fuzz/corpus/$t" 2>/dev/null | wc -l)
  nice -n 10 cargo +nightly fuzz run "$t" -- \
    -max_total_time="$DURATION" -rss_limit_mb=4096 -seed="$SEED" -print_final_stats=1 \
    > "$OUT/$t.log" 2>&1
  rc=$?
  after=$(find fuzz -name "crash-*" -o -name "leak-*" -o -name "timeout-*" -o -name "oom-*" 2>/dev/null | wc -l)
  corpus_after=$(ls "fuzz/corpus/$t" 2>/dev/null | wc -l)
  findings=$((after - before))
  crash_total=$((crash_total + findings))

  [ $first = 0 ] && echo "," >> "$OUT/campaign-ledger.json"
  first=0
  {
    echo "  {"
    echo "   \"target\": \"$t\","
    echo "   \"durationSecs\": $DURATION,"
    echo "   \"exitCode\": $rc,"
    echo "   \"corpusBefore\": $corpus_before,"
    echo "   \"corpusAfter\": $corpus_after,"
    echo "   \"newFindings\": $findings,"
    if [ "$findings" = 0 ]; then
      echo '   "result": "zero-findings",'
      echo '   "note": "no crash/leak/timeout/OOM artifact produced during the configured campaign; explicit zero-findings record, not a bare green claim"'
    else
      echo '   "result": "findings",'
      echo "   \"artifacts\": $(find fuzz \( -name 'crash-*' -o -name 'leak-*' -o -name 'timeout-*' -o -name 'oom-*' \) | jq -R . | jq -s .),"
      echo '   "disposition": "PENDING — each finding requires a regression test or an owner-accepted risk record before this campaign can close"'
    fi
    echo -n "  }"
  } >> "$OUT/campaign-ledger.json"
done

echo "" >> "$OUT/campaign-ledger.json"
echo "], \"totalNewFindings\": $crash_total }" >> "$OUT/campaign-ledger.json"
echo "campaign complete: $OUT/campaign-ledger.json (total new findings: $crash_total)"

# ---- sanitizer campaigns (ASan+UBSan workspace test pass) ----
echo "running ASan+UBSan workspace pass..."
export RUSTFLAGS="-Zsanitizer=address,undefined"
cargo +nightly test --workspace --quiet -- --test-threads=2 \
  > "$OUT/asan-ubsan-workspace.log" 2>&1
SAN_RC=$?
unset RUSTFLAGS
python3 - "$OUT/campaign-ledger.json" "$SAN_RC" <<'PYEOF'
import json, sys
path, rc = sys.argv[1], int(sys.argv[2])
d = json.load(open(path))
d["sanitizerCampaign"] = {
    "kind": "address+undefined via -Zsanitizer over `cargo +nightly test --workspace`",
    "exitCode": rc,
    "log": "benchmarks/raw/ga-m6-fuzz/asan-ubsan-workspace.log",
    "result": "zero-findings" if rc == 0 else "findings — see log; disposition required",
}
json.dump(d, open(path, "w"), indent=1)
print("sanitizer campaign recorded, exit", rc)
PYEOF
exit $SAN_RC
