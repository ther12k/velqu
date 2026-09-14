#!/usr/bin/env python3
"""Regression tests for scripts/analyze-soak.py (M6-009 evidence tooling).

Generates synthetic JSONL/summary fixtures in a temp directory and asserts
the analyzer's verdicts and rejection paths:

  - rising terminal tail      -> verdict FAIL on T2 (small full-run drift)
  - plateau / declining tails -> T2 PASS
  - summary missing fields    -> INVALID EVIDENCE (exit 2), never a zero
  - empty-object summary      -> INVALID EVIDENCE (exit 2)
  - retained partial run      -> progress-only, no verdict (exit 0)
  - short observed duration   -> INVALID EVIDENCE (exit 2)
  - sub-minimum configured    -> INVALID EVIDENCE (historical replay shape)
  - ownership over workers    -> verdict FAIL on T5
  - ownership within workers  -> T5 PASS; capacity bound not assumed
  - queue over capacity       -> verdict FAIL on T5
  - arithmetic replay         -> drift recomputes to the published 0.298

Run: python3 scripts/analyze-soak.test.py  (exit 0 = all pass)
"""
import json
import subprocess
import sys
import tempfile
from pathlib import Path

ANALYZER = Path(__file__).resolve().parent / "analyze-soak.py"
HISTORICAL = Path(__file__).resolve().parents[1] / "benchmarks/raw/worker-scaling"
FAILURES = []


def window(seq, elapsed, rss_kib, pending=2, queue=2048, requests=600_000, tput=2_000.0):
    return {
        "seq": seq,
        "elapsedSecs": elapsed,
        "windowSecs": 240.0,
        "requests": requests,
        "throughputOpsPerSec": tput,
        "processRssKib": rss_kib,
        "processCpuSecsCumulative": seq * 240.0,
        "queueLens": [1024, 1024],
        "queueTotal": queue,
        "queueRejectedTotal": seq * 1_000_000,
        "ownershipPendingSlots": pending,
    }


def tail_series(base_kib, step_kib, n=12):
    """360-window (24h at 240s) series; the final `n` windows move by step_kib."""
    wins = []
    for i in range(360):
        rss = base_kib + (step_kib * (i - (360 - n)) if i >= 360 - n else 0)
        wins.append(window(i, (i + 1) * 240.0, rss))
    return wins


def write_case(root, name, windows, summary):
    d = root / name
    d.mkdir(parents=True)
    (d / "soak.jsonl").write_text("\n".join(json.dumps(w) for w in windows) + "\n")
    if summary is not None:
        (d / "soak-summary.json").write_text(json.dumps(summary, indent=1))
    return d


def base_summary(**over):
    s = {
        "format": "velqu-soak-v2",
        "engine": "quickjs-ng/0.15.1 via rquickjs 0.12.2",
        "workers": 2,
        "chaos": {"enabled": True},
        "configuredDurationSecs": 86400,
        "windowSecs": 240,
        "actualDurationSecs": 86400,
        "totalCompletedVerified": 216_000_000,
        "retainedMemory": {"processRssGrowthKib": 0},
    }
    s.update(over)
    return s


def tail_case(root, name, step_kib, n=12):
    """Tail fixture whose summary carries the series' true growth, so the
    agreement check passes and the verdict reflects the tolerances alone."""
    wins = tail_series(4000, step_kib, n)
    growth = step_kib * (n - 1)
    return write_case(root, name, wins,
                      base_summary(retainedMemory={"processRssGrowthKib": growth}))


def run(case, *extra):
    cmd = [sys.executable, str(ANALYZER), str(case / "soak.jsonl")]
    if not any(a == "--min-hours" for a in extra):
        cmd += ["--min-hours", "24"]
    cmd += list(extra)
    return subprocess.run(cmd, capture_output=True, text=True)


def expect(name, cond, detail=""):
    if cond:
        print(f"  PASS  {name}")
    else:
        print(f"  FAIL  {name} {detail}")
        FAILURES.append(name)


def main():
    with tempfile.TemporaryDirectory() as tmp:
        root = Path(tmp)

        # 1. rising terminal tail: tiny full-run drift, T1 passes, T2 catches it
        d = tail_case(root, "rising-tail", 600)
        r = run(d, "--capacity", "2048")
        expect("rising tail exits 1", r.returncode == 1, f"got {r.returncode}")
        expect("rising tail fails T2", "FAIL  T2" in r.stdout, r.stdout[-300:])
        expect("rising tail drift is small (T1 passes)",
               "PASS  T1" in r.stdout, r.stdout[-300:])

        # 2. plateau tail -> T2 PASS
        r = run(tail_case(root, "plateau", 0), "--capacity", "2048")
        expect("plateau T2 PASS", "PASS  T2" in r.stdout, r.stdout[-300:])

        # 3. declining tail -> T2 PASS
        r = run(tail_case(root, "declining", -400), "--capacity", "2048")
        expect("declining T2 PASS", "PASS  T2" in r.stdout, r.stdout[-300:])

        # 4. summary missing required fields -> INVALID EVIDENCE, not zero-drift
        s = base_summary()
        del s["totalCompletedVerified"]
        del s["retainedMemory"]
        d = write_case(root, "missing-fields", tail_series(4000, 0), s)
        r = run(d, "--capacity", "2048")
        expect("missing fields exits 2", r.returncode == 2, f"got {r.returncode}")
        expect("missing fields reports INVALID EVIDENCE", "INVALID EVIDENCE" in r.stdout)

        # 5. empty-object summary -> INVALID EVIDENCE
        d = write_case(root, "empty-summary", tail_series(4000, 0), {})
        r = run(d, "--capacity", "2048")
        expect("empty summary exits 2", r.returncode == 2, f"got {r.returncode}")
        expect("no verdict printed", "verdict:" not in r.stdout)

        # 6. retained partial run (historical-style) -> progress-only, exit 0
        d = write_case(root, "retained", [window(i, (i + 1) * 240.0, 4000) for i in range(360)], {
            "format": "velqu-soak-v1",
            "status": "retained-evidence",
            "workers": 2,
            "configuredDurationSecs": 86400,
            "actualElapsedSecs": 56200,
            "totalWindows": 360,
        })
        r = run(d)
        expect("retained run exits 0 progress-only",
               r.returncode == 0 and "PARTIAL RUN" in r.stdout, f"got {r.returncode}")
        expect("retained run prints no verdict", "verdict:" not in r.stdout)

        # 7. completed schema but short observed duration -> INVALID EVIDENCE
        d = write_case(root, "short-run", tail_series(4000, 0),
                       base_summary(actualDurationSecs=56_000))
        r = run(d)
        expect("short observed duration exits 2", r.returncode == 2, f"got {r.returncode}")
        expect("short duration names the requirement", "required minimum" in r.stdout)

        # 8. sub-minimum configured duration (historical replay shape) -> exit 2
        r = run(HISTORICAL, "--capacity", "2048", "--expected-workers", "2")
        expect("historical run rejected on duration floor", r.returncode == 2,
               f"got {r.returncode}")
        expect("historical rejection names configured duration",
               "configured duration" in r.stdout and "required minimum" in r.stdout)

        # 9. ownership pending exceeds configured workers -> T5 FAIL
        wins = [window(i, (i + 1) * 240.0, 4000, pending=3) for i in range(360)]
        d = write_case(root, "ownership-excess", wins, base_summary())
        r = run(d, "--capacity", "2048")
        expect("ownership over workers exits 1", r.returncode == 1, f"got {r.returncode}")
        expect("ownership failure cites T5", "FAIL  T5" in r.stdout, r.stdout[-300:])

        # 10. ownership within workers -> T5 PASS; capacity bound not assumed
        r = run(tail_case(root, "within-bounds", 0))
        expect("within-bounds T5 PASS without --capacity",
               "PASS  T5" in r.stdout and "not evaluated" in r.stdout, r.stdout[-300:])

        # 11. queue over configured capacity -> T5 FAIL
        wins = [window(i, (i + 1) * 240.0, 4000, queue=4096) for i in range(360)]
        d = write_case(root, "queue-over", wins, base_summary())
        r = run(d, "--capacity", "2048")
        expect("queue over capacity exits 1", r.returncode == 1, f"got {r.returncode}")

        # 12. arithmetic replay (m3-010 totals): growth 700 KiB over 2,407,340
        #     requests = 0.298 B/req. Windows are synthesized to match the
        #     summary within the agreement bound; the run is far below the
        #     M6-009 duration floor, so this replays arithmetic only via
        #     --min-hours 0 (it asserts nothing about qualification).
        wins = [window(i, (i + 1) * 240.0, 5760 + (700 * i) // 28, requests=82_000)
                for i in range(29)]
        d = write_case(root, "arithmetic", wins,
                       base_summary(configuredDurationSecs=6960,
                                    actualDurationSecs=6960,
                                    totalCompletedVerified=2_407_340,
                                    retainedMemory={"processRssGrowthKib": 700}))
        r = run(d, "--capacity", "2048", "--min-hours", "0")
        expect("arithmetic replay prints 0.298 drift",
               "0.298 B/completed-request" in r.stdout, r.stdout[-400:])
        expect("arithmetic replay T1 PASS", "PASS  T1" in r.stdout)

        # 13. front-trimmed log: remove the first 15 of 720 windows (~2.08%
        #     of requests — inside the 5% agreement bound) from a valid 48h
        #     fixture. The last timestamp still says 48h; only raw start
        #     coverage can catch it.
        full = [window(i, (i + 1) * 240.0, 4000) for i in range(720)]
        trimmed_pct = 100.0 * (720 - 705) / 720
        d = write_case(root, "front-trimmed", full[15:],
                       base_summary(configuredDurationSecs=172_800,
                                    actualDurationSecs=172_800,
                                    totalCompletedVerified=432_000_000))
        r = run(d, "--capacity", "2048")
        expect(f"front-trimmed ({trimmed_pct:.2f}% requests, inside agreement bound) exits 2",
               r.returncode == 2, f"got {r.returncode}")
        expect("front-trimmed rejected on raw start coverage",
               "does not start at run start" in r.stdout, r.stdout[-300:])

        # 14. middle hole: drop one window; the gap is ~2x windowSecs.
        holed = [w for i, w in enumerate(full) if i != 300]
        d = write_case(root, "middle-hole", holed,
                       base_summary(configuredDurationSecs=172_800,
                                    actualDurationSecs=172_800,
                                    totalCompletedVerified=432_000_000))
        r = run(d, "--capacity", "2048")
        expect("middle hole exits 2", r.returncode == 2, f"got {r.returncode}")
        expect("middle hole rejected on coverage gap", "coverage hole" in r.stdout,
               r.stdout[-300:])

        # 15. NaN configuredDurationSecs in an otherwise valid summary: NaN
        #     comparisons are all false, so type checks alone let it reach
        #     PASS; isfinite must reject it first.
        s = base_summary(configuredDurationSecs=float("nan"))
        d = write_case(root, "nan-configured", full[:360], s)
        r = run(d)
        expect("NaN configured duration exits 2", r.returncode == 2, f"got {r.returncode}")
        expect("NaN rejection names the field",
               "non-finite summary field 'configuredDurationSecs'" in r.stdout,
               r.stdout[-300:])

        # 16. Infinity actualDurationSecs -> invalid, not an instant pass.
        d = write_case(root, "inf-actual", full[:360],
                       base_summary(actualDurationSecs=float("inf")))
        r = run(d)
        expect("infinite actual duration exits 2", r.returncode == 2, f"got {r.returncode}")

        # 17. top-level array summary: the object check must run before any
        #     attribute access — controlled invalid, not a traceback.
        d = write_case(root, "array-summary", full[:360], [])
        r = run(d)
        expect("array summary exits 2 (no traceback)",
               r.returncode == 2 and "Traceback" not in r.stderr
               and "summary is not a JSON object" in r.stdout,
               f"rc={r.returncode} stderr={r.stderr[-200:]}")

        # 18. non-finite raw measurement -> invalid evidence.
        bad = window(0, 240.0, 4000)
        bad["throughputOpsPerSec"] = float("nan")
        d = write_case(root, "nan-raw", [bad] + full[1:360],
                       base_summary())
        r = run(d)
        expect("non-finite raw field exits 2", r.returncode == 2, f"got {r.returncode}")
        expect("raw rejection names the field", "non-finite raw field" in r.stdout)

    print(f"\n{'ALL PASS' if not FAILURES else 'FAILURES: ' + ', '.join(FAILURES)}")
    return 1 if FAILURES else 0


if __name__ == "__main__":
    sys.exit(main())
