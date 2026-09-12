#!/usr/bin/env bash
# #1318 / M6-002 — sustained fuzz + sanitizer campaign driver.
#
# Owner corrections (2026-09-12 review):
#   1. FAIL-CLOSED: the script's exit status aggregates ALL campaign
#      outcomes — any target rc != 0, any new finding artifact, any
#      sanitizer-stage failure makes the final exit non-zero. The ledger
#      is ALWAYS written completely before exiting (no naive set -e);
#      automation must never report success while evidence contains a
#      finding.
#   2. SANITIZER STRATEGY: `-Zsanitizer=address,undefined` was invalid —
#      rustc's -Zsanitizer takes ONE sanitizer and `undefined` is not in
#      the supported set. Replaced with two honest stages:
#        S1  ASan workspace pass  (-Zsanitizer=address, the documented
#            form) over `cargo +nightly test --workspace`.
#        S2  UBSan over the QuickJS C FFI — the boundary Miri cannot
#            see: the C sources are compiled by rquickjs-sys's cc build
#            with clang and -fsanitize=undefined -fno-sanitize-recover=all,
#            and the test binary is linked by clang so the UBSan runtime
#            is attached (RUSTFLAGS=-Clinker=clang -Clink-arg=...).
#            Owner-preferred path (2026-09-12): instrument the C/FFI
#            boundary itself. If the configuration cannot build, the
#            stage records configuration-blocked with the log — never a
#            green claim.
#
# Chained via --wait-for-pid: blocks until the #1319 soak PID exits so
# campaign load never pollutes the soak's flat-RSS host.
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

TARGETS=(pack_verify router_match http_decode schema_validate bridge_handles codec_encoders capabilities_policy)
# plus the TS Treaty encoder campaign (below) — see fuzz/COVERAGE.md
# Sustained: 900 s (15 min) per target. Smoke: 60 s (CI-free local check).
if [ "$SMOKE" = 1 ]; then DURATION=60; else DURATION=900; fi
SEED="$(date +%s)"
OUT="benchmarks/raw/ga-m6-fuzz"
mkdir -p "$OUT"

# Aggregate failure flags — the final exit is non-zero if any is set.
FUZZ_FAILED=0
SAN_FAILED=0

find_artifacts() {
  find fuzz \( -name 'crash-*' -o -name 'leak-*' -o -name 'timeout-*' -o -name 'oom-*' \) 2>/dev/null
}

{
  echo "{"
  echo ' "campaign": "ga-m6-sustained-fuzz",'
  echo " \"startedAt\": \"$CAMPAIGN_DATE\","
  echo " \"durationPerTargetSecs\": $DURATION,"
  echo " \"seed\": $SEED,"
  echo " \"fuzzer\": \"libFuzzer (cargo-fuzz 0.13.2, nightly $(rustc +nightly --version | cut -d' ' -f2))\","
  echo ' "failClosed": "exit is non-zero if any target errors, any new finding appears, or any sanitizer stage fails",'
  echo ' "targets": ['
} > "$OUT/campaign-ledger.json"

first=1
crash_total=0
for t in "${TARGETS[@]}"; do
  before=$(find_artifacts | wc -l)
  corpus_before=$(ls "fuzz/corpus/$t" 2>/dev/null | wc -l)
  nice -n 10 cargo +nightly fuzz run "$t" -- \
    -max_total_time="$DURATION" -rss_limit_mb=4096 -seed="$SEED" -print_final_stats=1 \
    > "$OUT/$t.log" 2>&1
  rc=$?
  after=$(find_artifacts | wc -l)
  corpus_after=$(ls "fuzz/corpus/$t" 2>/dev/null | wc -l)
  findings=$((after - before))
  crash_total=$((crash_total + findings))
  # libFuzzer/cargo-fuzz: non-zero rc means the run aborted (crash, OOM,
  # timeout, internal error). Zero findings AND non-zero rc is still a
  # failure the ledger must surface.
  if [ "$rc" != "0" ] || [ "$findings" != "0" ]; then
    FUZZ_FAILED=1
  fi

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
    if [ "$findings" = 0 ] && [ "$rc" = "0" ]; then
      echo '   "result": "zero-findings",'
      echo '   "note": "no crash/leak/timeout/OOM artifact and clean exit during the configured campaign; explicit zero-findings record, not a bare green claim"'
    elif [ "$findings" = 0 ]; then
      echo '   "result": "fuzzer-error",'
      echo '   "note": "no finding artifacts but the fuzzer run itself failed — inspect the log; counted as a campaign failure"'
    else
      echo '   "result": "findings",'
      echo "   \"artifacts\": $(find_artifacts | jq -R . | jq -s .),"
      echo '   "disposition": "PENDING — each finding requires a regression test or an owner-accepted risk record before this campaign can close"'
    fi
    echo -n "  }"
  } >> "$OUT/campaign-ledger.json"
done

echo "" >> "$OUT/campaign-ledger.json"
echo "], \"totalNewFindings\": $crash_total }" >> "$OUT/campaign-ledger.json"
echo "campaign complete: $OUT/campaign-ledger.json (total new findings: $crash_total, FUZZ_FAILED=$FUZZ_FAILED)"

# ---- TS-side encoder campaign (Treaty; see fuzz/COVERAGE.md) ----
echo "running TS Treaty encoder campaign..."
TS_RC=0
if [ "$SMOKE" = 1 ]; then TS_DUR=30; else TS_DUR=600; fi
nice -n 10 bun scripts/ts-fuzz-campaign.ts --duration-secs "$TS_DUR" --seed "$SEED" \
  --out "$OUT/ts-treaty-ledger.json" > "$OUT/ts-treaty.log" 2>&1
TS_RC=$?
TS_FINDINGS=$(python3 -c "import json;print(json.load(open('$OUT/ts-treaty-ledger.json'))['totalFindings'])" 2>/dev/null || echo 1)
if [ "$TS_RC" != "0" ] || [ "$TS_FINDINGS" != "0" ]; then
  FUZZ_FAILED=1
fi

# ---- Sanitizer stage S1: ASan workspace pass (documented form) ----
echo "running sanitizer stage S1: ASan workspace pass..."
S1_RC=0
RUSTFLAGS="-Zsanitizer=address" cargo +nightly test --workspace --quiet -- --test-threads=2 \
  > "$OUT/asan-workspace.log" 2>&1
S1_RC=$?
if [ "$S1_RC" != "0" ]; then
  SAN_FAILED=1
fi

# ---- Sanitizer stage S2: UBSan over the QuickJS C FFI ----
# The C/FFI boundary (quickjs-ng compiled by rquickjs-sys) is exactly
# what Miri cannot execute. Compile the C sources with clang's UBSan
# instrumentation and link the test binary with clang so the runtime is
# attached. If the toolchain cannot realize this configuration, record
# configuration-blocked honestly.
echo "running sanitizer stage S2: UBSan on the QuickJS C FFI (clang)..."
S2_RC=0
CC=clang \
CFLAGS="-fsanitize=undefined -fno-sanitize-recover=all -fno-omit-frame-pointer" \
RUSTFLAGS="-Clinker=clang -Clink-arg=-fsanitize=undefined" \
cargo +nightly test -p q-engine-quickjs --quiet -- --test-threads=2 \
  > "$OUT/ubsan-quickjs-ffi.log" 2>&1
S2_RC=$?
if [ "$S2_RC" != "0" ] && ! grep -qi "SanitizerUnique\|runtime error" "$OUT/ubsan-quickjs-ffi.log"; then
  # Distinguish a real UBSan finding from a configuration failure: if
  # the log shows no sanitizer report AND the build failed, it is a
  # configuration problem — still a failure flag, but labeled honestly.
  if grep -qi "error:.*linker\|error: unknown argument\|cannot find -l" "$OUT/ubsan-quickjs-ffi.log"; then
    S2_RESULT="configuration-blocked"
  else
    S2_RESULT="findings-or-failure"
  fi
elif [ "$S2_RC" != "0" ]; then
  S2_RESULT="findings-or-failure"
else
  S2_RESULT="zero-findings"
fi
if [ "$S2_RC" != "0" ]; then
  SAN_FAILED=1
fi

# ---- Finalize ledger with sanitizer stages, then exit fail-closed ----
FINALIZER_FAILED=0
python3 - "$OUT/campaign-ledger.json" "$S1_RC" "$S2_RC" "$S2_RESULT" <<'PYEOF' || FINALIZER_FAILED=1
import json, sys
path, s1, s2, s2_result = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), sys.argv[4]
d = json.load(open(path))
d["sanitizerCampaign"] = {
    "S1": {
        "stage": "ASan workspace pass",
        "command": "RUSTFLAGS='-Zsanitizer=address' cargo +nightly test --workspace",
        "exitCode": s1,
        "log": "benchmarks/raw/ga-m6-fuzz/asan-workspace.log",
        "result": "zero-findings" if s1 == 0 else "findings-or-failure — disposition required",
    },
    "S2": {
        "stage": "UBSan over the QuickJS C FFI (clang -fsanitize=undefined, -fno-sanitize-recover=all; test binary linked by clang)",
        "command": "CC=clang CFLAGS='-fsanitize=undefined -fno-sanitize-recover=all -fno-omit-frame-pointer' RUSTFLAGS='-Clinker=clang -Clink-arg=-fsanitize=undefined' cargo +nightly test -p q-engine-quickjs",
        "exitCode": s2,
        "log": "benchmarks/raw/ga-m6-fuzz/ubsan-quickjs-ffi.log",
        "result": s2_result if s2 != 0 else "zero-findings",
    },
    "note": "Two separate stages: rustc -Zsanitizer accepts ONE sanitizer and 'undefined' is not in its set (owner correction 2026-09-12); C-side UBSan is realized through clang instrumentation of the quickjs-ng sources at the rquickjs-sys cc build plus a clang-linked test binary.",
}
d["verdict"] = {
    "totalNewFindings": d.get("totalNewFindings", 0),
    "fuzzFailed": False,  # recomputed below
}
json.dump(d, open(path, "w"), indent=1)
print("ledger finalized")
PYEOF

# Append the fail-closed verdict flags to the ledger.
python3 - "$OUT/campaign-ledger.json" "$FUZZ_FAILED" "$SAN_FAILED" <<'PYEOF' || FINALIZER_FAILED=1
import json, sys
path = sys.argv[1]
fuzz_failed = sys.argv[2] == "1"
san_failed = sys.argv[3] == "1"
d = json.load(open(path))
import os
ts_findings = 0
ts_path = os.path.join(os.path.dirname(path), "ts-treaty-ledger.json")
if os.path.exists(ts_path):
    ts_findings = json.load(open(ts_path)).get("totalFindings", 0)
d["applicabilityMatrix"] = json.load(open("fuzz/sanitizer-applicability.json"))
d["tsEncoderCampaign"] = {
    "surface": "packages/treaty encoders (TypeScript; cargo-fuzz cannot reach)",
    "ledger": "benchmarks/raw/ga-m6-fuzz/ts-treaty-ledger.json",
    "totalFindings": ts_findings,
}
d["verdict"] = {
    "totalNewFindings": d.get("totalNewFindings", 0) + ts_findings,
    "fuzzFailed": fuzz_failed,
    "sanitizerFailed": san_failed,
    "campaignPassed": not fuzz_failed and not san_failed and ts_findings == 0,
    "rule": "closing #1318 requires campaignPassed=true AND every finding dispositioned (regression test or owner-accepted risk)",
}
json.dump(d, open(path, "w"), indent=1)
print("verdict recorded: campaignPassed =", d["verdict"]["campaignPassed"])
PYEOF

# Validate that the resulting ledger is well-formed JSON
python3 -m json.tool "$OUT/campaign-ledger.json" > /dev/null 2>&1 || {
  echo "ERROR: final ledger is not valid JSON" >&2
  FINALIZER_FAILED=1
}

if [ "$FUZZ_FAILED" != "0" ] || [ "$SAN_FAILED" != "0" ] || [ "$FINALIZER_FAILED" != "0" ]; then
  echo "CAMPAIGN FAILED (FUZZ_FAILED=$FUZZ_FAILED SAN_FAILED=$SAN_FAILED FINALIZER_FAILED=$FINALIZER_FAILED) — ledger contains the evidence; exit non-zero."
  exit 1
fi
echo "CAMPAIGN PASSED — zero findings across all targets and sanitizer stages."
exit 0
