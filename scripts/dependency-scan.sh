#!/usr/bin/env bash
# BETA-009-B — reproducible dependency/license inventory when cargo-audit,
# cargo-deny, or a network-backed scanner is not installed. The generated
# report is evidence only; absence of an advisory database is disclosed.
set -euo pipefail
ROOT=$(cd "$(dirname "$0")/.." && pwd)
cd "$ROOT"
OUT=${1:-docs/reports/beta-009-b-dependency-scan.json}
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT
cargo metadata --format-version 1 > "$TMP/metadata.json"
cargo tree --workspace --edges normal --prefix none > "$TMP/tree.txt"
python3 - "$TMP/metadata.json" "$TMP/tree.txt" "$OUT" <<'PY'
import json, sys, pathlib, collections
metadata_path, tree_path, out_path = sys.argv[1:]
m = json.load(open(metadata_path))
packages = m['packages']
workspace = [p for p in packages if p.get('source') is None]
external = [p for p in packages if p.get('source') is not None]
licenses = collections.Counter(p.get('license') or ('license-file' if p.get('license_file') else 'MISSING') for p in packages)
missing_external = [p['name'] for p in external if not p.get('license') and not p.get('license_file')]
result = {
  'format': 'velqu-beta-dependency-scan-v1',
  'generatedFrom': ['Cargo.lock', 'Cargo.toml', 'cargo metadata', 'cargo tree'],
  'workspacePackages': len(workspace),
  'resolvedPackages': len(packages),
  'externalPackages': len(external),
  'dependencyTreeLines': sum(1 for _ in open(tree_path)),
  'licenseDistribution': dict(sorted(licenses.items())),
  'externalPackagesMissingLicense': missing_external,
  'workspaceLicensePosture': sorted(set(p.get('license') or 'MISSING' for p in workspace)),
  'scannerAvailability': {
    'cargoAudit': bool(__import__('shutil').which('cargo-audit')),
    'cargoDeny': bool(__import__('shutil').which('cargo-deny')),
    'osvScanner': bool(__import__('shutil').which('osv-scanner')),
    'syft': bool(__import__('shutil').which('syft')),
  },
  'advisoryDatabaseScan': 'not-run: no cargo-audit/cargo-deny/osv-scanner installed in environment',
  'verdict': 'PASS_WITH_DISCLOSURE' if not missing_external else 'BLOCKED_MISSING_EXTERNAL_LICENSES',
}
pathlib.Path(out_path).parent.mkdir(parents=True, exist_ok=True)
pathlib.Path(out_path).write_text(json.dumps(result, indent=2) + '\n')
print(json.dumps(result, indent=2))
PY
