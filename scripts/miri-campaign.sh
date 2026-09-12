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
#
# Fail-closed ledger discipline (owner review 3, 2026-09-12): a scope
# read failure, a ledger writer failure, or an incomplete crate/record
# correspondence fails the run. A ledger left over from a previous run
# is removed up front and can never become a false success fallback.
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
# Fail-closed (owner review 3): a failed or empty scope read aborts the
# run — it must never degrade to "zero crates to run" + a stale ledger.
rm -f "$OUT/miri-ledger.json"
MIRI_CRATES=()
if ! MIRI_CRATES=($(python3 -c "import json;print(' '.join(json.load(open('fuzz/miri-scope.json'))['included']))")); then
  echo "ERROR: failed to read the included list from fuzz/miri-scope.json" >&2
  exit 1
fi
if [ "${#MIRI_CRATES[@]}" -eq 0 ]; then
  echo "ERROR: fuzz/miri-scope.json included list is empty" >&2
  exit 1
fi
for c in "${MIRI_CRATES[@]}"; do
  if [ ! -d "crates/$c" ]; then
    echo "ERROR: scope crate crates/$c does not exist in the workspace" >&2
    exit 1
  fi
done
echo "miri scope from fuzz/miri-scope.json: ${MIRI_CRATES[*]}"

# Initialize the ledger cleanly via Python. The previous ledger was
# removed above, so a failed initialization leaves nothing behind for a
# later stage to misread.
if ! python3 - "$OUT/miri-ledger.json" <<'PYEOF'
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
then
  echo "ERROR: miri ledger initialization failed; aborting (no stale ledger fallback)" >&2
  exit 1
fi

fail=0
export MIRIFLAGS="${MIRIFLAGS:-} -Zmiri-disable-isolation"
for c in "${MIRI_CRATES[@]}"; do
  nice -n 10 cargo +nightly miri test -p "$c" --quiet > "$OUT/miri-$c.log" 2>&1
  rc=$?
  if [ $rc -ne 0 ]; then
    fail=1
  fi
  if ! python3 - "$OUT/miri-ledger.json" "$c" "$rc" "$OUT/miri-$c.log" <<'PYEOF'
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
  then
    echo "ERROR: miri ledger append failed for crate $c" >&2
    fail=1
  fi
done

# Finalize the ledger with the overall verdict. The verdict is set only
# after the recorded results are matched against the required crate
# list (owner review 3): an incomplete, duplicated, or mismatched
# record set can never yield verdict "passed".
if ! python3 - "$OUT/miri-ledger.json" "$fail" "${MIRI_CRATES[@]}" <<'PYEOF'
import json, sys
# python3 - ARG... yields argv[0]='-'; the real arguments start at [1].
path, fail_str, *required = sys.argv[1:]
doc = json.load(open(path))
recorded = [c["crate"] for c in doc["crates"]]
missing = [c for c in required if c not in recorded]
extra = [c for c in recorded if c not in required]
duplicates = sorted({c for c in recorded if recorded.count(c) > 1})
complete = not missing and not extra and not duplicates and len(recorded) == len(required)
if missing:
    print(f"ERROR: miri ledger is missing required crate results: {missing}", file=sys.stderr)
if extra:
    print(f"ERROR: miri ledger has unexpected crate results: {extra}", file=sys.stderr)
if duplicates:
    print(f"ERROR: miri ledger has duplicate crate results: {duplicates}", file=sys.stderr)
per_crate_failures = [c["crate"] for c in doc["crates"] if c.get("exitCode") != 0]
any_findings = bool(per_crate_failures) or fail_str != "0"
doc["cratesRequired"] = list(required)
doc["cratesRecorded"] = recorded
doc["coverageComplete"] = complete
doc["anyFindings"] = any_findings
if any_findings:
    doc["verdict"] = "failed"
elif not complete:
    doc["verdict"] = "incomplete"
else:
    doc["verdict"] = "passed"
json.dump(doc, open(path, "w"), indent=2)
if not complete:
    sys.exit(1)
PYEOF
then
  echo "ERROR: miri ledger finalization reported an incomplete record set" >&2
  fail=1
fi

# Validate that the resulting ledger is well-formed JSON
python3 -m json.tool "$OUT/miri-ledger.json" > /dev/null 2>&1 || {
  echo "ERROR: miri ledger is not valid JSON" >&2
  fail=1
}

echo "miri campaign complete: $OUT/miri-ledger.json (fail=$fail)"
exit $fail
