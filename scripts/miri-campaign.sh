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
#
# Robust ledger generation (owner review 2026-09-12): JSON is managed
# through Python's json library — no shell string concatenation accidents —
# and validated before exit.
set -uo pipefail
cd "$(dirname "$0")/.."

while pgrep -f "scripts/fuzz-campaign.sh" | grep -v "^$$\$" >/dev/null 2>&1; do
  sleep 120
done

OUT="benchmarks/raw/ga-m6-fuzz"
mkdir -p "$OUT"

# The crate list is read from the single machine-readable source of
# truth shared with scripts/unsafe-audit.py (owner review 2026-09-12):
# the runner and the audit can no longer disagree about scope.
MIRI_CRATES=($(python3 -c "import json;print(' '.join(json.load(open('fuzz/miri-scope.json'))['included']))"))
echo "miri scope from fuzz/miri-scope.json: ${MIRI_CRATES[*]}"

# Initialize the ledger cleanly via Python
python3 - "$OUT/miri-ledger.json" <<'PYEOF'
import json, sys, subprocess, datetime
path = sys.argv[1]
try:
    toolchain = subprocess.check_output(["rustc", "+nightly", "--version"], text=True).strip()
except Exception:
    toolchain = "unknown"
scope = json.load(open("fuzz/miri-scope.json"))
doc = {
    "campaign": "ga-m6-miri",
    "startedAt": datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
    "toolchain": toolchain,
    "scopeSource": "fuzz/miri-scope.json",
    "excludedWithReason": [{k: e[k] for k in ("crate", "reason")} for e in scope["excluded"]],
    "crates": [],
}
json.dump(doc, open(path, "w"), indent=2)
PYEOF

fail=0
for c in "${MIRI_CRATES[@]}"; do
  nice -n 10 cargo +nightly miri test -p "$c" --quiet > "$OUT/miri-$c.log" 2>&1
  rc=$?
  if [ $rc -ne 0 ]; then
    fail=1
  fi
  python3 - "$OUT/miri-ledger.json" "$c" "$rc" "$OUT/miri-$c.log" <<'PYEOF'
import json, sys
path, c, rc_str, log_path = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4]
rc = int(rc_str)
doc = json.load(open(path))
doc["crates"].append({
    "crate": c,
    "exitCode": rc,
    "log": log_path,
    "result": "zero-findings" if rc == 0 else "findings — UB reported; regression test or disposition required"
})
json.dump(doc, open(path, "w"), indent=2)
PYEOF
done

# Finalize ledger with overall verdict
python3 - "$OUT/miri-ledger.json" "$fail" <<'PYEOF'
import json, sys
path, fail_str = sys.argv[1], sys.argv[2]
doc = json.load(open(path))
doc["anyFindings"] = (fail_str != "0")
doc["verdict"] = "passed" if fail_str == "0" else "failed"
json.dump(doc, open(path, "w"), indent=2)
PYEOF

# Validate that the resulting ledger is well-formed JSON
python3 -m json.tool "$OUT/miri-ledger.json" > /dev/null 2>&1 || {
  echo "ERROR: miri ledger is not valid JSON" >&2
  fail=1
}

echo "miri campaign complete: $OUT/miri-ledger.json (fail=$fail)"
exit $fail
