#!/usr/bin/env python3
"""Independent soak-leak analysis for M6-009 evidence (recomputes from raw JSONL).

The soak harness (crates/q-bench-support/src/bin/soak.rs) writes its own
soak-summary.json at completion. This script is the independent
raw-to-report recomputation: it reads soak.jsonl, re-derives the totals
and leak statistics, cross-checks them against the harness summary, and
applies the M6-009 numeric tolerances.

M6-009 tolerances (defined here, derived from precedent — beta-013-d and
m3-010-c observed ~0.30 B/request allocator retention with flat QuickJS
heaps; the bounds give >3x headroom):

  T1  RSS drift <= 1.0 B per completed request
  T2  terminal rise: linear RSS slope over the final quarter of windows
      <= 250 KiB/h (a sustained late climb is a leak signal even when the
      full-run drift is small; plateaus and declining series pass)
  T3  largest single window-to-window RSS step <= 2048 KiB (no jump)
  T4  >= 10,000,000 completed requests AND the run meets the minimum
      duration (--min-hours, default 24 per M6-009; pass 72 for the
      selected 72-hour qualification)
  T5  ownership pending slots never exceed the configured worker count;
      queue peak vs capacity is evaluated only when --capacity is given
      (never inferred from the observed peak, never assumed by default)

Completion is NOT inferred from soak-summary.json existing: the summary
must carry the velqu-soak-v2 measurement schema (workers,
totalCompletedVerified, retainedMemory.processRssGrowthKib,
configuredDurationSecs, actualDurationSecs, windowSecs), must not carry a
retained/interrupted status marker, and both configured and observed
duration must cover --min-hours with raw-window backing. Missing or
malformed summary measurements are INVALID EVIDENCE — they are never
replaced with raw-log substitutes or zeros.

Exit codes: 0 = qualified (verdict PASS) or progress-only output;
1 = verdict FAIL (a tolerance, agreement, or bound was violated);
2 = invalid evidence (summary missing required measurements, wrong
schema, or an explicitly retained/interrupted record submitted for
qualification).

Usage: analyze-soak.py <soak.jsonl> [--partial] [--min-hours H]
                       [--capacity N] [--expected-workers N]
"""
import json
import math
import sys
from pathlib import Path

MAX_DRIFT_BYTES_PER_REQUEST = 1.0
MAX_TERMINAL_SLOPE_KIB_PER_HOUR = 250.0
MAX_WINDOW_STEP_KIB = 2048.0
MIN_COMPLETED_REQUESTS = 10_000_000
RETAINED_STATUSES = {"retained-evidence", "interrupted", "partial", "in-progress"}


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
    return (sxy / sxx) * 3600.0


def fail_invalid(msg):
    print(f"INVALID EVIDENCE: {msg}")
    return 2


def validate_summary_schema(summary):
    """Return (error, normalized); required measurements must be present and typed."""
    if not isinstance(summary, dict):
        return "summary is not a JSON object", None
    fmt = summary.get("format")
    if fmt != "velqu-soak-v2":
        return f"unsupported summary format {fmt!r} (need velqu-soak-v2)", None
    required = [
        ("workers", int),
        ("totalCompletedVerified", int),
        ("configuredDurationSecs", (int, float)),
        ("actualDurationSecs", (int, float)),
        ("windowSecs", (int, float)),
    ]
    norm = {}
    for key, types in required:
        value = summary.get(key)
        if isinstance(value, bool) or not isinstance(value, types):
            return f"missing or malformed summary field {key!r}", None
        if isinstance(value, float) and not math.isfinite(value):
            return f"non-finite summary field {key!r}", None
        norm[key] = value
    retained = summary.get("retainedMemory")
    if not isinstance(retained, dict):
        return "missing retainedMemory block", None
    growth = retained.get("processRssGrowthKib")
    if isinstance(growth, bool) or not isinstance(growth, int):
        return "missing or malformed retainedMemory.processRssGrowthKib", None
    norm["rssGrowthKib"] = growth
    workers = norm["workers"]
    if workers < 1:
        return "workers must be >= 1", None
    if norm["totalCompletedVerified"] < 0 or norm["actualDurationSecs"] <= 0:
        return "summary totals/duration out of range", None
    if not 0 < norm["windowSecs"] <= 3600:
        return "summary windowSecs out of range", None
    return None, norm


def main():
    args = sys.argv[1:]
    if not args:
        print(__doc__)
        return 2
    jsonl = Path(args[0])
    partial = "--partial" in args
    min_hours = 24.0
    if "--min-hours" in args:
        min_hours = float(args[args.index("--min-hours") + 1])
    capacity = None
    if "--capacity" in args:
        capacity = int(args[args.index("--capacity") + 1])
    expected_workers = None
    if "--expected-workers" in args:
        expected_workers = int(args[args.index("--expected-workers") + 1])

    if not jsonl.exists():
        return fail_invalid(f"raw log not found: {jsonl}")
    rows = []
    for line in jsonl.read_text().splitlines():
        if not line.strip():
            continue
        r = json.loads(line)
        # Python's JSON decoder accepts NaN/Infinity tokens; the consumed
        # measurements must be finite or the evidence is invalid.
        for k in ("elapsedSecs", "windowSecs", "requests", "throughputOpsPerSec",
                  "processRssKib", "queueTotal", "ownershipPendingSlots"):
            v = r.get(k)
            if v is None:
                return fail_invalid(f"raw sample missing field {k!r}")
            if isinstance(v, bool) or not isinstance(v, (int, float)):
                return fail_invalid(f"raw field {k!r} is not numeric")
            if isinstance(v, float) and not math.isfinite(v):
                return fail_invalid(f"non-finite raw field {k!r}")
        rows.append(r)
    if not rows:
        return fail_invalid("raw log has no samples")

    total_completed_raw = sum(r["requests"] for r in rows)
    elapsed = rows[-1]["elapsedSecs"]
    tputs = [r["throughputOpsPerSec"] for r in rows]
    rss = [r["processRssKib"] for r in rows]
    secs = [r["elapsedSecs"] for r in rows]
    steps = [b - a for a, b in zip(rss, rss[1:])]
    peak_q = max(r["queueTotal"] for r in rows)
    peak_pending = max(r["ownershipPendingSlots"] for r in rows)
    growth = rss[-1] - rss[0]
    rejected = rows[-1]["queueRejectedTotal"]
    offered = total_completed_raw + rejected
    quarter = max(1, len(rows) // 4)
    terminal_slope = lin_slope_kib_per_hour(secs[-quarter:], rss[-quarter:])

    print(f"samples                : {len(rows)} windows")
    print(f"elapsed                : {elapsed/3600:.2f} h")
    print(f"completed (window sum) : {total_completed_raw:,}")
    print(f"offered (incl. reject) : {offered:,} (completion {100*total_completed_raw/offered:.1f}%)")
    print(f"throughput overall     : {total_completed_raw/elapsed:.0f} ops/s "
          f"(window min/mean/max: {min(tputs):.0f}/{sum(tputs)/len(tputs):.0f}/{max(tputs):.0f})")
    print(f"RSS initial/final/peak : {rss[0]} / {rss[-1]} / {max(rss)} KiB")
    print(f"RSS growth (raw span)  : {growth:+d} KiB")
    print(f"max window step        : {max(steps):+d} KiB")
    print(f"terminal-quarter slope : {terminal_slope:+.1f} KiB/h (last {quarter} windows)")
    print(f"peak queue slots       : {peak_q}"
          + (f" (configured capacity {capacity})" if capacity is not None else " (capacity not configured; bound not evaluated)"))
    print(f"ownership pending      : peak {peak_pending}")

    # ---- completion and summary validation -------------------------------
    summary_path = jsonl.parent / "soak-summary.json"
    if partial or not summary_path.exists():
        print("\nPARTIAL RUN: progress statistics only — no verdict "
              "(soak-summary.json absent or --partial given).")
        return 0

    try:
        summary = json.loads(summary_path.read_text())
    except (json.JSONDecodeError, UnicodeDecodeError) as exc:
        return fail_invalid(f"summary is not valid JSON: {exc}")
    if not isinstance(summary, dict):
        return fail_invalid("summary is not a JSON object")

    status = summary.get("status")
    if isinstance(status, str) and status in RETAINED_STATUSES:
        print(f"\nPARTIAL RUN: summary carries status {status!r} — explicitly retained "
              "or interrupted evidence stays progress-only regardless of filename.")
        return 0

    err, s = validate_summary_schema(summary)
    if err:
        return fail_invalid(err)

    if expected_workers is not None and s["workers"] != expected_workers:
        return fail_invalid(f"summary workers {s['workers']} != expected {expected_workers}")

    # Duration completion: configured AND observed must both cover the
    # minimum; raw windows must back the observed span (allow the documented
    # sampling interval of slack, never a wholesale gap).
    configured_h = s["configuredDurationSecs"] / 3600.0
    actual_h = s["actualDurationSecs"] / 3600.0
    window_h = s["windowSecs"] / 3600.0
    if configured_h < min_hours:
        return fail_invalid(
            f"configured duration {configured_h:.2f} h < required minimum {min_hours:g} h")
    if actual_h < min_hours:
        return fail_invalid(
            f"observed duration {actual_h:.2f} h < required minimum {min_hours:g} h "
            "(a retained partial run does not substitute for duration)")

    # Raw coverage must span the run continuously — start, middle, and end.
    # The last timestamp alone proves nothing: a log missing its first 30
    # minutes differs by ~2% of requests (inside the agreement bound) yet
    # does not cover the claimed duration. Each row carries its actual
    # windowSecs, so gaps are distinguishable from sampling jitter.
    ws = s["windowSecs"]
    if rows[0]["elapsedSecs"] > 2 * ws + 5:
        return fail_invalid(
            f"raw log does not start at run start (first window ends at "
            f"{rows[0]['elapsedSecs']:.0f}s; expected ~{ws:.0f}s)")
    prev = None
    for r in rows:
        if prev is not None:
            gap = r["elapsedSecs"] - prev["elapsedSecs"]
            step = prev["windowSecs"]
            if abs(gap - step) > max(5.0, 0.5 * step):
                return fail_invalid(
                    f"raw coverage hole: gap {gap:.0f}s vs window {step:.0f}s "
                    f"at seq {r.get('seq', '?')}")
        prev = r
    if elapsed < s["configuredDurationSecs"] - 2 * ws:
        return fail_invalid(
            "raw windows do not cover the configured duration "
            f"(last window at {elapsed/3600:.2f} h vs configured {configured_h:.2f} h)")

    # ---- agreement with the authoritative summary ------------------------
    s_total = s["totalCompletedVerified"]
    s_growth = s["rssGrowthKib"]
    delta_total = total_completed_raw - s_total
    agree = abs(delta_total) <= 0.05 * s_total and abs(growth - s_growth) <= 512
    print(f"\nharness summary total : {s_total:,} (window-sum delta {delta_total:+,})")
    print(f"summary RSS growth    : {s_growth} KiB (recomputed {growth:+d})")
    if not agree:
        print("FAIL  recomputation disagrees with harness summary (>5% requests or >512 KiB RSS)")
        return 1
    print("PASS  recomputation agrees with harness summary")

    drift = (s_growth * 1024.0) / s_total if s_total else float("nan")
    print(f"drift (authoritative) : {drift:.3f} B/completed-request")

    t5_notes = [f"ownership pending peak {peak_pending} <= workers {s['workers']}"]
    t5_ok = peak_pending <= s["workers"]
    if capacity is not None:
        t5_notes.append(f"queue peak {peak_q} <= capacity {capacity}")
        t5_ok = t5_ok and peak_q <= capacity
    else:
        t5_notes.append("queue-capacity bound not evaluated (no --capacity given)")

    checks = [
        ("T1 drift <= %.1f B/request" % MAX_DRIFT_BYTES_PER_REQUEST,
         drift <= MAX_DRIFT_BYTES_PER_REQUEST),
        ("T2 terminal slope <= %.0f KiB/h over final quarter" % MAX_TERMINAL_SLOPE_KIB_PER_HOUR,
         terminal_slope <= MAX_TERMINAL_SLOPE_KIB_PER_HOUR),
        ("T3 max window step <= %d KiB" % MAX_WINDOW_STEP_KIB,
         max(steps) <= MAX_WINDOW_STEP_KIB),
        ("T4 >= %d completed requests and >= %g h duration"
         % (MIN_COMPLETED_REQUESTS, min_hours),
         s_total >= MIN_COMPLETED_REQUESTS and actual_h >= min_hours),
        ("T5 config-derived slot bounds: " + "; ".join(t5_notes), t5_ok),
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
