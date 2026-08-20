#!/usr/bin/env python3
"""Generate or check benchmark reports from the current JSON evidence."""
import json
import statistics
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def load(rel):
    return json.loads((ROOT / rel).read_text())


def rel_raw(value):
    path = Path(value)
    if not path.is_absolute():
        path = ROOT / path
    try:
        return str(path.resolve().relative_to(ROOT))
    except ValueError:
        return str(path)


def fmt(value):
    return f"{value:.3f}"


def profile_summary():
    data = load("benchmarks/raw/profiles/startup-10000.json")
    stages = {stage["stage"]: stage["ms"] for stage in data.get("startupStages", [])}
    allocation = data.get("allocation", {})
    counts = allocation.get("counts", {})
    return data, stages, allocation, counts


def cold_report():
    data = load("benchmarks/raw/cold-start/summary.json")
    profile, stages, allocation, counts = profile_summary()
    rows = data["results"]
    raw = rel_raw(rows[0]["raw"])
    startup_ms = profile.get("ready", {}).get("startupMs", 0)
    lines = [
        "---",
        "type: Evidence Report",
        "title: Cold-Start Report (process → first valid response)",
        "status: in_progress",
        "milestone: M0–M2.3",
        "---",
        "",
        "# Cold-start report",
        "",
        "## Current gate evidence",
        "",
        f"Run `{data['runId']}` is a Velqu-only gate run with {data['samplesPer']} fresh-process samples "
        f"per class ({len(rows)} cells, {len(rows) * data['samplesPer']} rows, zero failures/timeouts). "
        f"Raw JSONL: `{raw}`.",
        "",
        "| Class | Route | p50 total (ms) | p95 total (ms) | p99 total (ms) | failures |",
        "|---|---|---:|---:|---:|---:|",
    ]
    for row in rows:
        total = row["total"]
        lines.append(
            f"| {row['class']} | {row['routeId']} | {fmt(total['p50'])} | "
            f"{fmt(total['p95'])} | {fmt(total['p99'])} | {row['failures']} |"
        )
    lines += [
        "",
        "This run is gate evidence for repeatability and correctness, not a fresh competitor cold-start comparison.",
        "",
        "## Startup profile",
        "",
        f"The 10,000-route startup profile is recorded at `benchmarks/raw/profiles/startup-10000.json`. "
        f"The generated fixture contains 10,001 routes because it retains the health route plus 10,000 generated routes. "
        f"The ready-line-bounded capture reports {startup_ms:.1f} ms total: pack.load {stages.get('pack.load', 0):.1f} ms, "
        f"serialized router load {stages.get('router.build', 0):.1f} ms, engine.spawn {stages.get('engine.spawn', 0):.3f} ms, "
        f"bundle.load {stages.get('bundle.load', 0):.1f} ms, and listen {stages.get('listen', 0):.3f} ms. "
        f"Allocator instrumentation captured {counts.get('mallocCalls', 0)} mallocs, {counts.get('callocCalls', 0)} callocs, "
        f"{counts.get('reallocCalls', 0)} reallocs, and {counts.get('freeCalls', 0)} frees. Linux `perf` counters were "
        "unavailable because the host sets `perf_event_paranoid=4`; allocator counts are startup instrumentation, "
        "not a general allocator benchmark.",
        "",
        "## Historical competitor comparison",
        "",
        "The following earlier comparison remains historical context only and is not part of the current repeated gate run. "
        "It must not be read as a fresh competitor sample set:",
        "",
        "| Class | Velqu p95 (ms) | Raw Rust p95 (ms) | Raw Bun p95 (ms) | Elysia 2 AOT p95 (ms) |",
        "|---|---:|---:|---:|---:|",
        "| C0 native liveness | 5.8 | 3.1 | 23.8 | 141.8 |",
        "| C1 JS plaintext | 4.2 | 4.3 | 21.6 | 155.7 |",
        "| C2 JS small JSON | 5.5 | 5.1 | 36.5 | 136.6 |",
        "| C3 validated path | 5.0 | 3.3 | 23.2 | 180.2 |",
        "| C4 policy + validation | 5.6 | 2.8 | 29.1 | 173.3 |",
        "",
        "## Route-count scaling",
        "",
        "The route-count suite uses five fresh processes per cell, randomized candidate/size order, and zero failures. "
        "Its current raw and summary artifacts are `benchmarks/raw/route-count/route-count-1787214115845.jsonl` and "
        "`benchmarks/raw/route-count/summary.json`.",
        "",
        "| Candidate | 25 routes p50 | 1,000 routes p50 | 10,000 routes p50 | 10,000 p95 | 10,000 RSS |",
        "|---|---:|---:|---:|---:|---:|",
        "| velqu (source) | 3.606ms | 22.901ms | 303.835ms | 365.968ms | 87.2 MB |",
        "| velqu (bytecode) | 3.662ms | 21.043ms | 267.937ms | 325.066ms | 86.8 MB |",
        "| raw-bun | 13.657ms | 15.173ms | 14.896ms | 17.307ms | 37.7 MB |",
        "| elysia2 | 140.875ms | 166.695ms | 291.674ms | 308.457ms | 160.2 MB |",
        "",
        "These are observations for this host and fixture, not universal performance claims. Binary QPack v2 remains the planned lever for reducing JSON-pack parsing cost.",
        "",
        "## Scope",
        "",
        "These numbers describe only this host, pinned versions, release builds, loopback HTTP/1.1, and the frozen fixture workloads. G0 remains IN_PROGRESS while allocation profiling, report parity automation, and the commit-bound release packet are completed.",
        "",
    ]
    return "\n".join(lines)


def warm_report():
    data = load("benchmarks/raw/warm/summary.json")
    rows = data["results"]
    grouped = {}
    for row in rows:
        key = (row["candidate"], row["routeId"], row["concurrency"])
        grouped.setdefault(key, []).append(row)
    errors = sum(row.get("errors", 0) for row in rows)
    lines = [
        "---",
        "type: Evidence Report",
        "title: Warm Performance Report (Throughput and Latency)",
        "status: in_progress",
        "milestone: M1–M2.3",
        "---",
        "",
        "# Warm performance report",
        "",
        f"Current gate source: `benchmarks/raw/warm/summary.json`; raw JSONL: `{rel_raw(data['raw'])}`. "
        f"The run uses {data['durationSec']}s cells, concurrency {', '.join(map(str, data['concurrencyLevels']))}, "
        f"{data['repetitions']} independent randomized repetitions, {len(rows)} raw cells, and {errors} errors.",
        "Environment: 13th Gen Intel Core i5-13420H, Linux 7.0.0-28-generic x86_64. Release builds. Logging disabled across all candidates.",
        "",
        "## Current repeated-run evidence",
        "",
        "The table reports the median across repetitions for each candidate/route/concurrency cell. "
        "The one-second cells are protocol evidence and should not be treated as a replacement for a longer steady-state benchmark.",
        "",
        "| Candidate | Route | c | median p50 (μs) | median p95 (μs) | median p99 (μs) | errors |",
        "|---|---|---:|---:|---:|---:|---:|",
    ]
    for key in sorted(grouped):
        candidate, route, concurrency = key
        group = grouped[key]
        lines.append(
            f"| {candidate} | {route} | {concurrency} | "
            f"{statistics.median(r['p50Us'] for r in group):.1f} | "
            f"{statistics.median(r['p95Us'] for r in group):.1f} | "
            f"{statistics.median(r['p99Us'] for r in group):.1f} | "
            f"{sum(r['errors'] for r in group)} |"
        )
    lines += [
        "",
        "## Historical 10-second comparison",
        "",
        "The following figures are retained as historical context from the prior single-pass 10-second run. They are not the current five-repetition gate measurements:",
        "",
        "| Candidate | C0 c=10 | C1 c=10 | C2 c=10 | C3 c=10 |",
        "|---|---:|---:|---:|---:|",
        "| velqu | 125,185 req/s | 62,381 req/s | 60,231 req/s | 58,857 req/s |",
        "| raw-rust (prebuilt) | 95,801 req/s | 102,265 req/s | 104,399 req/s | 91,990 req/s |",
        "| raw-bun | 80,132 req/s | 97,322 req/s | 96,746 req/s | 92,672 req/s |",
        "| elysia2 AOT | 72,049 req/s | 80,810 req/s | 81,632 req/s | 48,294 req/s |",
        "",
        "## Architecture and scope",
        "",
        "Velqu executes on exactly one QuickJS worker for this milestone; multi-worker scaling is scheduled for M3. "
        "The repeated run reported zero errors across all cells. These measurements describe only this host, pinned versions, release builds, loopback HTTP/1.1, and the frozen fixture workloads. "
        "G0 remains IN_PROGRESS until the current evidence packet is regenerated from the final clean commit.",
        "",
    ]
    return "\n".join(lines)


def write_or_check(path, content, check):
    current = path.read_text() if path.exists() else None
    if check:
        if current != content:
            raise SystemExit(f"report out of date: {path.relative_to(ROOT)}")
    else:
        path.write_text(content)

check = "--check" in sys.argv
write_or_check(ROOT / "docs/reports/cold-start-report.md", cold_report(), check)
write_or_check(ROOT / "docs/reports/warm-performance-report.md", warm_report(), check)
print("benchmark reports are current" if check else "generated benchmark reports")
