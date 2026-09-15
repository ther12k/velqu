#!/usr/bin/env python3
"""Aggregate multi-host benchmark runs into a descriptive comparison.

Scans benchmarks/raw/multihost/multihost-*/  (one directory per host run,
produced by benchmarks/multihost/run.sh) and writes COMPARISON.md next to
them: per-host median tables and velqu/baseline ratios.

Descriptive evidence only (AGENTS constraint 12): heterogeneous hosts are
never merged into one normative number, and rows from different
architectures are never averaged.
"""
import json
import sys
from collections import defaultdict
from pathlib import Path

root = Path(__file__).resolve().parents[2]
base = root / "benchmarks" / "raw" / "multihost"

ROUTES = ["C0", "C1", "C2", "C3"]
CONC = [1, 10, 50]
BASELINES = ["raw-rust", "raw-bun", "elysia2", "lugas"]


def median(values):
    s = sorted(values)
    return s[len(s) // 2]


def load_runs():
    runs = {}
    for d in sorted(base.glob("multihost-*")):
        jsonls = list(d.glob("*.jsonl"))
        if not jsonls:
            continue
        agg = defaultdict(list)
        for f in jsonls:
            for line in f.read_text().splitlines():
                r = json.loads(line)
                if r.get("errors") == 0:
                    agg[(r["candidate"], r["routeId"], r["concurrency"])].append(
                        (r["rps"], r["p95Us"])
                    )
        runs[d.name] = agg
    return runs


def cell(agg, cand, route, c, i):
    v = agg.get((cand, route, c))
    return median([x[i] for x in v]) if v else None


def main():
    runs = load_runs()
    if not runs:
        print("no multihost runs found under", base, file=sys.stderr)
        return 1
    out = ["# Multi-host benchmark comparison (descriptive)", ""]
    out += [
        "Matched protocol per host: fixed seed, 10s cells, 5 repetitions, "
        "c=1/10/50, all candidates interleaved in one harness process inside "
        "a CPU-limited container. Hosts differ in CPU/kernel/virtualization; "
        "numbers are comparable ACROSS candidates within a host, and across "
        "hosts only as separate environments. Generated from raw JSONL in "
        "benchmarks/raw/multihost.",
        "",
    ]
    for run_id, agg in sorted(runs.items()):
        label = run_id.split("-", 1)[1]
        out += [f"## {run_id}", ""]
        out.append("| candidate | route | c | rps (med) | p95 (med) |")
        out.append("|---|---|---:|---:|---:|")
        for route in ROUTES:
            for c in CONC:
                for cand in ["velqu"] + BASELINES:
                    rps = cell(agg, cand, route, c, 0)
                    p95 = cell(agg, cand, route, c, 1)
                    if rps is None:
                        continue
                    out.append(f"| {cand} | {route} | {c} | {rps:.0f} | {p95:.0f}us |")
        out += ["", f"### velqu as % of baseline ({label}, median rps)", ""]
        out.append("| route/c | " + " | ".join(BASELINES) + " |")
        out.append("|---|" + "---:|" * len(BASELINES))
        for route in ROUTES:
            for c in CONC:
                q = cell(agg, "velqu", route, c, 0)
                if q is None:
                    continue
                row = [f"| {route} c={c}"]
                for b in BASELINES:
                    bv = cell(agg, b, route, c, 0)
                    row.append(f"{q / bv * 100:.0f}%" if bv else "—")
                out.append(" | ".join(row) + " |")
        out.append("")
    dest = base / "COMPARISON.md"
    dest.write_text("\n".join(out) + "\n")
    print(f"wrote {dest} ({len(runs)} runs)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
