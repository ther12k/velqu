#!/usr/bin/env python3
"""Refresh benchmarks/manifest.json from current evidence and build artifacts.

Canonical capture flow (#1429): the manifest must describe the exact binary
that produced the committed evidence. This script therefore refuses to
produce a manifest that lies:

  1. refuses ambient RUSTFLAGS/CFLAGS beyond the canonical remaps — composed
     ambient flags silently break byte identity with `bun run verify` and
     release packets;
  2. refuses uncommitted changes under the binary/pack/harness inputs — the
     manifest's commit field must describe the tree that was built;
  3. builds the canonical release pair itself (cargo build --release
     -p velqu-runtime -p q-bytecode-tool, the REBUILD.md §3 / ga-native-repro
     mode) so the hashed binary is never a stale or wrong-mode artifact left
     in target/ by an unrelated earlier build — a `--workspace` build of the
     same tree produces a byte-different velqu-runtime;
  4. refuses to bind evidence recorded by a different binary: every suite
     summary records binaryHashes and must match the freshly built hash.
     This makes a repeat of the 2026-09-16 refresh (which bound 09-10 runs
     to a 09-16 binary — see
     docs/reports/benchmark-manifest-capture-integrity.md) mechanically
     impossible.

Run at an evidence checkpoint after `scripts/benchmark` has recaptured the
full suite.
"""
import hashlib
import json
import os
import platform
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CANONICAL_BUILD = ["cargo", "build", "--release", "-p", "velqu-runtime", "-p", "q-bytecode-tool"]
# Everything that feeds the binary, the packs, or the harness. Evidence
# OUTPUTS (benchmarks/raw/**, summaries, reports) may be dirty — they are
# what is being captured.
INPUT_PATHS = [
    "crates", "packages", "examples", "baselines", "scripts",
    "benchmarks/harness", "benchmarks/type-scale/measure.ts",
    "Cargo.toml", "Cargo.lock", ".cargo",
]
SUMMARIES = [
    "benchmarks/raw/route-count/summary.json",
    "benchmarks/raw/cold-start/summary.json",
    "benchmarks/raw/warm/summary.json",
]

def die(msg):
    print(f"refresh-benchmark-manifest: {msg}", file=sys.stderr)
    sys.exit(1)

def sha256(path):
    h = hashlib.sha256()
    with path.open('rb') as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b''):
            h.update(chunk)
    return h.hexdigest()

def read_json(rel):
    return json.loads((ROOT / rel).read_text())

def version(command):
    try:
        return subprocess.run(command, cwd=ROOT, capture_output=True, text=True, check=False).stdout.splitlines()[0]
    except Exception:
        return 'unavailable'

def canonical_flags():
    src = str(ROOT)
    return (
        f"--remap-path-prefix={src}=/velqu-src",
        f"-ffile-prefix-map={src}=/velqu-src -fdebug-prefix-map={src}=/velqu-src",
    )

# 1. Ambient flag guard: exactly the canonical remaps or nothing at all.
want_rustflags, want_cflags = canonical_flags()
for var, want in (("RUSTFLAGS", want_rustflags), ("CFLAGS", want_cflags)):
    got = os.environ.get(var, "").strip()
    if got and got != want:
        die(f"ambient {var}={got!r} differs from the canonical remaps; a capture made under "
            f"composed ambient flags cannot be reproduced by scripts/verify — unset {var} or export exactly {want!r}")

# 2. Input-cleanliness guard: the commit field must describe the built tree.
dirty = subprocess.run(["git", "status", "--porcelain", "--", *INPUT_PATHS],
                       cwd=ROOT, capture_output=True, text=True).stdout.strip()
if dirty:
    die("uncommitted changes under build inputs — the manifest commit field would not describe "
        "the tree that was built; commit or restore these first:\n" + dirty)

# 3. Canonical build: the hashed binary is built here, in the canonical mode,
#    never inherited from whatever target/ last contained.
env = dict(os.environ)
env["RUSTFLAGS"] = want_rustflags
env["CFLAGS"] = want_cflags
if subprocess.run(CANONICAL_BUILD, cwd=ROOT, env=env).returncode != 0:
    die("canonical release build failed: " + " ".join(CANONICAL_BUILD))
runtime_hash = sha256(ROOT / "target/release/velqu-runtime")

# 4. Evidence-binding guard: every suite summary must have been recorded by
#    this binary.
for rel in SUMMARIES:
    path = ROOT / rel
    if not path.is_file():
        die(f"missing {rel} — run scripts/benchmark to capture the full suite first")
    summary = json.loads(path.read_text())
    recorded = (summary.get("binaryHashes") or {}).get("target/release/velqu-runtime")
    if not recorded:
        die(f"{rel} does not record binaryHashes — rerun scripts/benchmark with the updated "
            "harness (summaries must self-identify the binary that produced them)")
    if recorded != runtime_hash:
        die(f"{rel} records harness binary {recorded[:12]}... but the canonical build hashes "
            f"{runtime_hash[:12]}... — the committed evidence was produced by a different binary; "
            "rerun scripts/benchmark to recapture with this binary")

reviewed_commit = '4e6904951729ea14b48ca39a9564a950cc83e98e'
commit = version(['git', 'rev-parse', 'HEAD'])
warm = read_json('benchmarks/raw/warm/summary.json')
cold = read_json('benchmarks/raw/cold-start/summary.json')
route = read_json('benchmarks/raw/route-count/summary.json')
profile = read_json('benchmarks/raw/profiles/startup-10000.json')

def artifact(rel):
    path = ROOT / rel
    return {'path': rel, 'sha256': sha256(path) if path.is_file() else 'missing'}

manifest = {
    'format': 'velqu-benchmark-manifest-v3',
    'generatedAt': datetime.now(timezone.utc).isoformat().replace('+00:00', 'Z'),
    'commit': commit,
    'reviewedImplementationCommit': reviewed_commit,
    'environment': {
        'platform': platform.platform(),
        'kernel': platform.release(),
        'cpu': platform.processor(),
        'bunVersion': version(['bun', '--version']),
        'typescriptVersion': '5.9.3',
        'rustcVersion': version(['rustc', '--version']),
        'pinnedEngine': 'quickjs-ng 0.15.1 via rquickjs 0.12.2',
        # #1429: the release binary's build mode is part of its identity —
        # workspace-wide feature unification produces a different binary.
        'buildCommand': ' '.join(CANONICAL_BUILD),
        'loadGenerator': 'benchmarks/harness/warm.ts, cold-start.ts, route-count.ts',
        'startupProfiler': 'scripts/capture-startup-profile.py',
        'allocatorProfiler': 'scripts/alloc-tracer.c via LD_PRELOAD',
        'perfEventParanoid': Path('/proc/sys/kernel/perf_event_paranoid').read_text().strip() if Path('/proc/sys/kernel/perf_event_paranoid').is_file() else 'unavailable',
    },
    'artifacts': dict(
        [
            ('qRuntimeRelease', artifact('target/release/velqu-runtime')),
            ('proofPack', artifact('examples/proof/dist/app.qpack')),
            # M26-010-A: the full five-size route-count ladder
        ]
        + [
            (f'routeCountPack{n}', artifact(f'benchmarks/raw/packs/app-{n}.qpack'))
            for n in (25, 100, 1000, 5000, 10000)
        ]
        + [
            (f'routeCountBytecodePack{n}', artifact(f'benchmarks/raw/packs/app-{n}-bc.qpack'))
            for n in (25, 100, 1000, 5000, 10000)
        ]
    ),
    'runs': {
        'warm': {'runId': warm['runId'], 'repetitions': warm['repetitions'], 'rows': len(warm['results']), 'raw': warm['raw']},
        'cold': {'runId': cold['runId'], 'samplesPer': cold['samplesPer'], 'rows': len(cold['results']) * cold['samplesPer'], 'raw': cold['results'][0]['raw']},
        'routeCount': {'runId': route['runId'], 'samples': route['samples'], 'rows': len(route['results']) * route['samples'], 'raw': route['raw']},
        'startup': {'profile': 'benchmarks/raw/profiles/startup-10000.json', 'allocation': profile['allocation'], 'termination': profile.get('termination')},
    },
    'evidence': {
        'coldStartSummary': 'benchmarks/raw/cold-start/summary.json',
        'routeCountSummary': 'benchmarks/raw/route-count/summary.json',
        'warmSummary': 'benchmarks/raw/warm/summary.json',
        'startupProfile': 'benchmarks/raw/profiles/startup-10000.json',
        'allocationProfile': 'benchmarks/raw/profiles/startup-10000.alloc.json',
        'reports': ['docs/reports/cold-start-report.md', 'docs/reports/warm-performance-report.md'],
        'validator': 'scripts/validate-benchmark-evidence.py',
        'reportGenerator': 'scripts/generate-benchmark-reports.py',
    },
}
(ROOT / 'benchmarks/manifest.json').write_text(json.dumps(manifest, indent=2) + '\n')
print(json.dumps({'commit': commit, 'format': manifest['format'], 'runtime': runtime_hash}, indent=2))
