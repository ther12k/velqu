#!/usr/bin/env python3
"""Validate raw benchmark/summary parity without interpreting performance claims."""
import hashlib
import json
import subprocess
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

manifest = load("benchmarks/manifest.json")
if manifest:
    try:
        head = subprocess.run(["git", "rev-parse", "HEAD"], cwd=ROOT, capture_output=True, text=True, check=True).stdout.strip()
        captured_commit = manifest.get("commit")
        if not captured_commit or subprocess.run(["git", "merge-base", "--is-ancestor", captured_commit, head], cwd=ROOT, capture_output=True).returncode != 0:
            errors.append(f"manifest: capture commit {captured_commit} is not an ancestor of HEAD {head}")
        for name, artifact in manifest.get("artifacts", {}).items():
            path = ROOT / artifact.get("path", "")
            if not path.is_file():
                errors.append(f"manifest: missing artifact {name}: {artifact.get('path')}")
            elif artifact.get("sha256") != hashlib.sha256(path.read_bytes()).hexdigest():
                errors.append(f"manifest: hash mismatch for {name}")
        startup = manifest.get("runs", {}).get("startup", {})
        if startup.get("profile") != "benchmarks/raw/profiles/startup-10000.json":
            errors.append("manifest: startup profile reference mismatch")
        if startup.get("allocation", {}).get("status") != "captured":
            errors.append("manifest: allocation profile is not captured")
    except Exception as exc:
        errors.append(f"manifest: validation failed: {exc}")

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

# M25-002-C codec instrumentation evidence: strict checks — every cell timed,
# every sample correct, allocator deltas present, hashes matched.
codec_summary = load("benchmarks/raw/codec-c/codec-summary.json")
if codec_summary:
    codec_raw_rel = "benchmarks/raw/codec-c/codec.jsonl"
    codec_raw_path = ROOT / codec_raw_rel
    required_row_fields = [
        "runId", "case", "candidate", "i", "ok", "inBytes", "outBytes",
        "us", "totalUs", "codecUs", "engineUs", "cpuUserUs", "cpuSystemUs", "cpuUs",
        "bridgeAccessUs", "bridgeHostCalls", "bridgeMaterializedFields",
        "bridgeMaterializedBytes", "allocMallocCalls", "allocCallocCalls",
        "allocReallocCalls", "allocFreeCalls", "allocAllocatedBytes",
        "allocReallocatedBytes",
    ]
    if codec_summary.get("format") != "velqu-codec-bench-v2":
        errors.append("codec-c: summary format is not velqu-codec-bench-v2")
    if not str(codec_summary.get("instrumentation", {}).get("allocator", "")).startswith("captured"):
        errors.append("codec-c: allocator instrumentation is not captured")
    rows = []
    if not codec_raw_path.is_file():
        errors.append(f"codec-c: raw path does not exist: {codec_raw_rel}")
    else:
        for line in codec_raw_path.read_text().splitlines():
            if line.strip():
                try:
                    rows.append(json.loads(line))
                except Exception as exc:
                    errors.append(f"codec-c: invalid raw JSONL row: {exc}")
    iters = codec_summary.get("iters")
    cell_keys = {(r.get("case"), r.get("candidate")) for r in rows}
    summary_keys = {(c.get("case"), c.get("candidate")) for c in codec_summary.get("cases", [])}
    if cell_keys != summary_keys:
        errors.append("codec-c: raw case/candidate cells do not match summary cells")
    for cell in sorted(cell_keys):
        cell_rows = [r for r in rows if (r.get("case"), r.get("candidate")) == cell]
        if iters is not None and len(cell_rows) != iters:
            errors.append(f"codec-c: {cell} has {len(cell_rows)} raw rows, expected {iters}")
        for r in cell_rows:
            for field in required_row_fields:
                if field not in r:
                    errors.append(f"codec-c: {cell} row {r.get('i')} missing field {field}")
                    break
            if not r.get("ok"):
                errors.append(f"codec-c: {cell} row {r.get('i')} is not correct")
                break
            if r.get("allocMallocCalls") is None or r.get("allocAllocatedBytes") is None:
                errors.append(f"codec-c: {cell} row {r.get('i')} has null allocator deltas")
                break
    for c in codec_summary.get("cases", []):
        if c.get("status") != "OK" or c.get("correct") != c.get("samples"):
            errors.append(f"codec-c: cell {c.get('case')}/{c.get('candidate')} is not fully OK")
        for metric in ("totalUs", "codecUs", "engineUs", "cpuUs", "bridgeAccessUs", "allocAllocatedBytes", "allocCalls"):
            stats = c.get("metrics", {}).get(metric, {})
            for stat in ("n", "mean", "p50", "p95", "p99"):
                if stat not in stats:
                    errors.append(f"codec-c: {c.get('case')}/{c.get('candidate')} metric {metric} missing {stat}")
    evidence_path = ROOT / "benchmarks/raw/codec-c/evidence.json"
    if not evidence_path.is_file():
        errors.append("codec-c: missing evidence.json")
    else:
        try:
            evidence = json.loads(evidence_path.read_text())
            for entry in evidence.get("files", []):
                path = ROOT / entry.get("path", "")
                if not path.is_file():
                    errors.append(f"codec-c: evidence file missing: {entry.get('path')}")
                elif entry.get("sha256") not in (None, "written-at-tracer-exit"):
                    if entry["sha256"] != hashlib.sha256(path.read_bytes()).hexdigest():
                        errors.append(f"codec-c: hash mismatch for {entry.get('path')}")
        except Exception as exc:
            errors.append(f"codec-c: invalid evidence JSON: {exc}")
    alloc_profile = ROOT / "benchmarks/raw/codec-c/codec.alloc.json"
    if not alloc_profile.is_file():
        errors.append("codec-c: missing final allocator profile codec.alloc.json")
    else:
        try:
            profile = json.loads(alloc_profile.read_text())
            for field in ("mallocCalls", "callocCalls", "reallocCalls", "freeCalls", "allocatedBytes", "reallocatedBytes"):
                if field not in profile:
                    errors.append(f"codec-c: allocator profile missing {field}")
        except Exception as exc:
            errors.append(f"codec-c: invalid allocator profile JSON: {exc}")
    if not (ROOT / "benchmarks/raw/codec-c/codec.process.time.txt").is_file():
        errors.append("codec-c: missing /usr/bin/time process capture")

print(json.dumps({"errors": errors}, indent=2))
sys.exit(1 if errors else 0)
