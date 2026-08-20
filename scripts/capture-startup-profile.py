#!/usr/bin/env python3
"""Capture bounded startup profile evidence for a generated runtime pack.

Uses perf stat when available and always records /usr/bin/time -v output when
available. Missing profilers are recorded explicitly rather than synthesized.
"""
import json
import os
import re
import selectors
import shutil
import subprocess
import sys
import time
from pathlib import Path

root = Path(__file__).resolve().parents[1]
pack = Path(sys.argv[1]) if len(sys.argv) > 1 else root / "benchmarks/raw/packs/app-10000.qpack"
out = Path(sys.argv[2]) if len(sys.argv) > 2 else root / "benchmarks/raw/profiles/startup-10000.json"
runtime = root / "target/release/velqu-runtime"
out.parent.mkdir(parents=True, exist_ok=True)

record = {
    "format": "velqu-startup-profile-v1",
    "generatedAt": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
    "pack": str(pack.relative_to(root) if pack.is_relative_to(root) else pack),
    "runtime": str(runtime.relative_to(root) if runtime.is_relative_to(root) else runtime),
    "tools": {
        "perf": shutil.which("perf"),
        "time": "/usr/bin/time" if Path("/usr/bin/time").exists() else shutil.which("time"),
        "valgrind": shutil.which("valgrind"),
    },
    "status": "not-run",
}

if not runtime.is_file() or not pack.is_file():
    record["status"] = "missing-artifact"
    record["error"] = f"runtime or pack missing: {runtime}, {pack}"
    out.write_text(json.dumps(record, indent=2) + "\n")
    print(json.dumps(record, indent=2))
    sys.exit(0)

cmd = [str(runtime), "--pack", str(pack), "--port", "0", "--log", "off"]
perf = record["tools"]["perf"]
commands = []
if perf:
    commands.append(("perf", [perf, "stat", "-x,", "-e", "task-clock,context-switches,page-faults", "--", *cmd]))
commands.append(("wall-clock", cmd))

def run_attempt(tool, command):
    started = time.monotonic()
    proc = subprocess.Popen(command, cwd=root, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    streams = {proc.stdout: "stdout", proc.stderr: "stderr"}
    selector = selectors.DefaultSelector()
    for stream, name in streams.items():
        selector.register(stream, selectors.EVENT_READ, name)
    captured = {"stdout": [], "stderr": []}
    ready = None
    deadline = started + (2 if tool == "perf" else 8)
    try:
        while time.monotonic() < deadline and ready is None:
            events = selector.select(max(0, deadline - time.monotonic()))
            if not events:
                break
            for key, _ in events:
                line = key.fileobj.readline()
                if not line:
                    selector.unregister(key.fileobj)
                    continue
                captured[key.data].append(line)
                if key.data == "stdout":
                    try:
                        value = json.loads(line)
                    except json.JSONDecodeError:
                        value = None
                    if value and value.get("event") == "ready":
                        ready = value
                        break
    finally:
        selector.close()
        if proc.poll() is None:
            proc.terminate()
        try:
            remainder_out, remainder_err = proc.communicate(timeout=3)
        except subprocess.TimeoutExpired:
            proc.kill()
            remainder_out, remainder_err = proc.communicate(timeout=3)
        captured["stdout"].append(remainder_out)
        captured["stderr"].append(remainder_err)
    return {
        "tool": tool,
        "exitCode": proc.returncode,
        "elapsedMs": round((time.monotonic() - started) * 1000, 3),
        "stdout": "".join(captured["stdout"])[-12000:],
        "stderr": "".join(captured["stderr"])[-12000:],
        "ready": ready,
    }

try:
    attempts = []
    for tool, command in commands:
        attempt = run_attempt(tool, command)
        attempts.append(attempt)
        stderr = attempt["stderr"]
        if tool == "perf" and ("perf_event_paranoid" in stderr or "No supported events" in stderr):
            record["perfUnavailable"] = "host denied performance counters; retried with wall-clock"
            continue
        record.update({k: v for k, v in attempt.items() if k not in {"tool", "ready"}})
        record["tool"] = tool
        if attempt.get("ready"):
            record["ready"] = attempt["ready"]
        stages = attempt.get("ready", {}).get("stages", []) if attempt.get("ready") else []
        record["startupStages"] = stages
        if tool == "perf":
            metrics = {}
            for line in stderr.splitlines():
                fields = line.split(",")
                if len(fields) >= 3 and fields[2].strip():
                    metrics[fields[2].strip()] = fields[0].strip()
            record["perfStat"] = metrics
        if stages or tool == "wall-clock":
            break
    record["attempts"] = attempts
    record["status"] = "captured"
    if not record.get("startupStages"):
        record["warning"] = "runtime did not emit a ready line"
except Exception as exc:
    record["status"] = "error"
    record["error"] = str(exc)

out.write_text(json.dumps(record, indent=2) + "\n")
print(json.dumps(record, indent=2))
