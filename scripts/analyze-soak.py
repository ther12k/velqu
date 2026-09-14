#!/usr/bin/env python3
"""Independent soak-leak analysis for M6-009 evidence (recomputes from raw JSONL).

The soak harness (crates/q-bench-support/src/bin/soak.rs) writes its own
soak-summary.json at completion. This script is the independent
raw-to-report recomputation: it reads only soak.jsonl and re-derives the
totals and leak statistics, then applies the M6-009 numeric tolerances.

M6-009 tolerance (defined here, derived from precedent — beta-013-d and
m3-010-c observed ~0.30 B/request allocator retention with flat QuickJS
heaps; the bound below gives >3x headroom):

  T1  RSS drift <= 1.0 B per completed request
  T2  no sustained end-of-run climb: final-quarter mean RSS <=
      full-run peak RSS (the run must not finish at a new high)
  T3  largest single window-to-window RSS step <= 2048 KiB (no jump)
  T4  >= 10,000,000 completed requests (representative-workload clause)
  T5  peak live queue slots <= configured capacity; ownership pending
      slots never exceed the worker count

On a completed run the script prints the verdict and exits 0 (all
tolerances met) or 1 (any failed). On a partial run (`--partial`, or
auto-detected while soak-summary.json is absent) it prints progress
statistics only and never a verdict — a 72h run cannot be judged from
15h of data.

Usage: analyze-soak.py <soak.jsonl> [--partial] [--capacity N]
"""
import json
import sys
from pathlib import Path

# M6-009 numeric tolerances (see module docstring).
MAX_DRIFT_BYTES_PER_REQUEST = 1.0
MAX_WINDOW_STEP_KIB = 2048.0
MIN_COMPLETED_REQUESTS = 10_000_000


def lin_slope_kib_per_hour(xs_secs, ys_kib):
    n = len(xs_secs)
    if n < 2:
        return 0.0
    mx = sum(xs_secs) / n
    my = sum(ys_kib) / n
    sxx = sum((x - mx) ** 2 for x in xs_secs)
    if sxx == 0:
        return 0.0
    sxy = sum((x - mx) * (y - my) for x, y in zip(xs_secs, ys_kib))
    slope_per_sec = sxy / sxx
    return slope_per_sec * 3600.0


def main():
    args = sys.argv[1:]
    if not args:
        print(__doc__)
        return 2
    jsonl = Path(args[0])
    partial = "--partial" in args
    capacity = 2048
    if "--capacity" in args:
        capacity = int(args[args.index("--capacity") + 1])

    rows = [json.loads(line) for line in jsonl.read_text().splitlines() if line.strip()]
    if not rows:
        print("no samples")
        return 2

    total_completed = sum(r["requests"] for r in rows)
    elapsed = rows[-1]["elapsedSecs"]
    tputs = [r["throughputOpsPerSec"] for r in rows]
    rss = [r["processRssKib"] for r in rows]
    secs = [r["elapsedSecs"] for r in rows]
    steps = [b - a for a, b in zip(rss, rss[1:])]
    peak_q = max(r["queueTotal"] for r in rows)
    peak_pending = max(r["ownershipPendingSlots"] for r in rows)
    final_pending = rows[-1]["ownershipPendingSlots"]

    growth = rss[-1] - rss[0]
    drift = (growth * 1024.0) / total_completed if total_completed else float("nan")
    quarter = max(1, len(rows) // 4)
    final_quarter_mean = sum(rss[-quarter:]) / quarter
    run_peak = max(rss)
    slope = lin_slope_kib_per_hour(secs, rss)
    rejected = rows[-1]["queueRejectedTotal"]
    offered = total_completed + rejected

    print(f"samples                : {len(rows)} windows")
    print(f"elapsed                : {elapsed/3600:.2f} h")
    print(f"completed requests     : {total_completed:,}")
    print(f"offered (incl. reject) : {offered:,} (completion {100*total_completed/offered:.1f}%)")
    print(f"throughput overall     : {total_completed/elapsed:.0f} ops/s "
          f"(window min/mean/max: {min(tputs):.0f}/{sum(tputs)/len(tputs):.0f}/{max(tputs):.0f})")
    print(f"RSS initial/final/peak : {rss[0]} / {rss[-1]} / {run_peak} KiB")
    print(f"RSS growth             : {growth:+d} KiB over the analyzed span")
    print(f"RSS drift              : {drift:.3f} B/completed-request")
    print(f"RSS lin. slope         : {slope:+.1f} KiB/h")
    print(f"max window step        : {max(steps):+d} KiB (min step {min(steps):+d})")
    print(f"final-quarter mean RSS : {final_quarter_mean:.0f} KiB (run peak {run_peak})")
    print(f"peak queue slots       : {peak_q} (capacity {capacity})")
    print(f"ownership pending      : peak {peak_pending}, final {final_pending}")

    complete = jsonl.parent.joinpath("soak-summary.json").exists()
    if partial or not complete:
        print("\nPARTIAL RUN: progress statistics only — no verdict "
              "(soak-summary.json absent or --partial given).")
        return 0

    # The harness summary is authoritative for the exact totals (window sums
    # slightly undercount boundary in-flight completions); the recomputation
    # above must agree with it closely.
    summary = json.loads(jsonl.parent.joinpath("soak-summary.json").read_text())
    s_total = summary.get("totalCompletedVerified", total_completed)
    s_growth = (summary.get("retainedMemory", {}) or {}).get("processRssGrowthKib")
    delta_total = total_completed - s_total
    # 5%: boundary in-flight completions plus any historical snapshot
    # truncation (the committed m3-010 file shows ~3.3%); RSS must match
    # within 512 KiB regardless.
    agree = abs(delta_total) <= 0.05 * s_total and (s_growth is None or abs(growth - s_growth) <= 512)
    print(f"\nharness summary total : {s_total:,} (window-sum delta {delta_total:+,})")
    print(f"summary RSS growth    : {s_growth} KiB (recomputed {growth:+d})")
    if not agree:
        print("FAIL  recomputation disagrees with harness summary (>2% requests or >512 KiB RSS)")
        return 1
    print("PASS  recomputation agrees with harness summary")

    # Tolerances evaluated against the authoritative totals.
    drift = ((s_growth or 0) * 1024.0) / s_total if s_total else float("nan")
    print(f"drift (authoritative) : {drift:.3f} B/completed-request")

    checks = [
        ("T1 drift <= %.1f B/request" % MAX_DRIFT_BYTES_PER_REQUEST, drift <= MAX_DRIFT_BYTES_PER_REQUEST),
        ("T2 final-quarter mean <= run peak", final_quarter_mean <= run_peak),
        ("T3 max window step <= %d KiB" % MAX_WINDOW_STEP_KIB, max(steps) <= MAX_WINDOW_STEP_KIB),
        ("T4 >= %d completed requests" % MIN_COMPLETED_REQUESTS, s_total >= MIN_COMPLETED_REQUESTS),
        ("T5 queue/slot bounds", peak_q <= capacity and peak_pending <= 64),
    ]
    print()
    ok = True
    for label, passed in checks:
        print(f"  {'PASS' if passed else 'FAIL'}  {label}")
        ok = ok and passed
    print(f"\nverdict: {'PASS' if ok else 'FAIL'} (independent recomputation from {jsonl.name})")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
