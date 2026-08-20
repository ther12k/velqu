#!/usr/bin/env python3
"""Validate raw benchmark/summary parity without interpreting performance claims."""
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
errors = []

def load(rel):
    path = ROOT / rel
    if not path.is_file():
        errors.append(f"missing summary: {rel}")
        return None
    try:
        return json.loads(path.read_text())
    except Exception as exc:
        errors.append(f"invalid JSON {rel}: {exc}")
        return None

def read_raw_path(raw, summary_rel):
    raw_path = Path(raw)
    if not raw_path.is_absolute():
        raw_path = ROOT / raw_path
    if not raw_path.is_file():
        errors.append(f"{summary_rel}: raw path does not exist: {raw}")
        return []
    rows = []
    for line in raw_path.read_text().splitlines():
        if line.strip():
            try:
                rows.append(json.loads(line))
            except Exception as exc:
                errors.append(f"{summary_rel}: invalid raw JSONL row: {exc}")
    return rows

def raw_rows(summary, summary_rel):
    raw = summary.get("raw")
    if raw:
        return read_raw_path(raw, summary_rel)
    # Legacy cold summaries stored the same JSONL path on each result cell.
    paths = {r.get("raw") for r in summary.get("results", []) if r.get("raw")}
    if not paths:
        errors.append(f"{summary_rel}: missing raw path")
        return []
    rows = []
    for path in sorted(paths):
        rows.extend(read_raw_path(path, summary_rel))
    return rows

cold = load("benchmarks/raw/cold-start/summary.json")
if cold:
    rows = raw_rows(cold, "cold-start")
    expected = cold.get("samplesPer")
    cells = {(r.get("candidate"), r.get("class")) for r in rows}
    for cell in cells:
        count = sum(1 for r in rows if (r.get("candidate"), r.get("class")) == cell)
        if expected is not None and count != expected:
            errors.append(f"cold-start: {cell} has {count} raw rows, expected {expected}")
    for result in cold.get("results", []):
        cell = (result.get("candidate"), result.get("class"))
        count = sum(1 for r in rows if (r.get("candidate"), r.get("class")) == cell)
        if result.get("total", {}).get("n") != count:
            errors.append(f"cold-start: summary n mismatch for {cell}")

warm = load("benchmarks/raw/warm/summary.json")
if warm:
    rows = raw_rows(warm, "warm")
    repetitions = warm.get("repetitions", 1)
    required = len(warm.get("concurrencyLevels", [])) * 4 * 4 * repetitions
    if warm.get("format", "").startswith("velqu-warm-load-v3") and len(rows) != required:
        errors.append(f"warm: raw row count {len(rows)} != expected {required}")
    if warm.get("format", "").startswith("velqu-warm-load-v3") and repetitions < 5:
        errors.append(f"warm: repetitions {repetitions} < required 5")
    if warm.get("format", "").startswith("velqu-warm-load-v3") and not warm.get("randomizedCandidateOrder"):
        errors.append("warm: randomizedCandidateOrder is not true")

profile = ROOT / "benchmarks/raw/profiles/startup-10000.json"
if not profile.is_file():
    errors.append("missing startup allocation/profile artifact")
else:
    try:
        profile_data = json.loads(profile.read_text())
        if profile_data.get("status") not in {"captured", "missing-artifact", "not-run"}:
            errors.append("startup profile has unknown status")
    except Exception as exc:
        errors.append(f"invalid startup profile JSON: {exc}")

route = load("benchmarks/raw/route-count/summary.json")
if route:
    rows = raw_rows(route, "route-count")
    expected = route.get("samples")
    for result in route.get("results", []):
        cell = (result.get("candidate"), result.get("routes"))
        count = sum(1 for r in rows if (r.get("candidate"), r.get("n")) == cell)
        if expected is not None and count != expected:
            errors.append(f"route-count: {cell} has {count} raw rows, expected {expected}")
        if result.get("samples") != count - int(result.get("failures", 0)):
            errors.append(f"route-count: summary sample count mismatch for {cell}")

print(json.dumps({"errors": errors}, indent=2))
sys.exit(1 if errors else 0)
