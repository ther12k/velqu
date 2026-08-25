#!/usr/bin/env python3
"""Refresh benchmarks/manifest.json from current evidence and build artifacts."""
import hashlib
import json
import platform
import subprocess
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

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
    'format': 'velqu-benchmark-manifest-v2',
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
print(json.dumps({'commit': commit, 'format': manifest['format']}, indent=2))
