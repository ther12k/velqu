#!/usr/bin/env python3
"""Capture the measurement host's physical core topology (M3-009-D).

Parses /proc/cpuinfo (plus optional sysfs cache details) and writes a
deterministic JSON record: logical CPUs, sockets, physical cores,
siblings-per-core (SMT factor), CPU model, cache sizes, and a SHA-256
of the raw cpuinfo bytes so the evidence binds to the exact host
description. Nothing is fabricated: fields that cannot be read are
recorded as null.

Usage: capture-host-topology.py [out.json]
       (default out: benchmarks/raw/worker-scaling/host-topology.json)
"""

import hashlib
import json
import os
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent


def parse_cpuinfo(raw: str) -> dict:
    cpus = []
    current: dict = {}
    for line in raw.splitlines():
        if not line.strip():
            if current:
                cpus.append(current)
                current = {}
            continue
        if ":" not in line:
            continue
        key, _, value = line.partition(":")
        current[key.strip()] = value.strip()
    if current:
        cpus.append(current)

    logical = len(cpus)
    sockets = sorted({c.get("physical id", "0") for c in cpus})
    cores = sorted({(c.get("physical id", "0"), c.get("core id", "?")) for c in cpus})
    models = sorted({c.get("model name", "") for c in cpus})
    mhz = []
    for c in cpus:
        try:
            mhz.append(float(c.get("cpu MHz", "0")))
        except ValueError:
            pass
    caches = sorted({c.get("cache size", "") for c in cpus})

    return {
        "logicalCpus": logical,
        "sockets": len(sockets),
        "physicalCores": len(cores),
        "siblingsPerCore": (logical / len(cores)) if cores else None,
        "smt": (logical > len(cores)) if cores else None,
        "modelNames": models,
        "cpuMhzMin": min(mhz) if mhz else None,
        "cpuMhzMax": max(mhz) if mhz else None,
        "cacheSizeEntries": caches,
    }


def sysfs_cache() -> dict | None:
    """L1/L2/L3 sizes from sysfs for cpu0 (homogeneous hosts only —
    heterogeneous topologies must be flagged, not guessed)."""
    base = Path("/sys/devices/system/cpu/cpu0/cache")
    if not base.is_dir():
        return None
    out = {}
    try:
        for idx in sorted(base.iterdir()):
            level = (idx / "level").read_text().strip()
            ctype = (idx / "type").read_text().strip()
            size = (idx / "size").read_text().strip()
            shared = (idx / "shared_cpu_list").read_text().strip()
            out[f"L{level}-{ctype}"] = {
                "size": size,
                "sharedCpus": shared,
            }
    except OSError:
        return None
    return out or None


def numa_nodes() -> int | None:
    nodes = Path("/sys/devices/system/node")
    if not nodes.is_dir():
        return None
    try:
        return len([p for p in nodes.iterdir() if p.name.startswith("node")])
    except OSError:
        return None


def main() -> int:
    out = Path(sys.argv[2] if len(sys.argv) > 2 else sys.argv[1]) if len(sys.argv) > 1 else (
        ROOT / "benchmarks/raw/worker-scaling/host-topology.json"
    )
    cpuinfo_path = Path("/proc/cpuinfo")
    raw_bytes = cpuinfo_path.read_bytes()
    topology = parse_cpuinfo(raw_bytes.decode(errors="replace"))
    topology["sysfsCache"] = sysfs_cache()
    topology["numaNodes"] = numa_nodes()
    topology["availableParallelism"] = len(os.sched_getaffinity(0)) if hasattr(os, "sched_getaffinity") else os.cpu_count()
    topology["cpuinfoSha256"] = hashlib.sha256(raw_bytes).hexdigest()

    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(topology, indent=2, sort_keys=True) + "\n")
    print(f"host topology captured: {out}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
