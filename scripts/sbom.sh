#!/usr/bin/env bash
# BETA-015-F — CycloneDX SBOM for the beta release packet.
# Generates release/sbom.cdx.json (CycloneDX 1.5) from cargo metadata
# (Rust dependency graph incl. licenses/versions) plus the shipped
# @velqu/* npm workspace packages, bound to the current source commit.
# Deterministic: components sorted by (type, name, version).
set -euo pipefail
ROOT=$(cd "$(dirname "$0")/.." && pwd)
cd "$ROOT"
OUT=${1:-release/sbom.cdx.json}

COMMIT=$(git rev-parse HEAD)
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT
cargo metadata --format-version 1 > "$TMP/metadata.json"

python3 - "$TMP/metadata.json" "$OUT" "$COMMIT" <<'PY'
import json, sys, pathlib, datetime, collections

meta_path, out_path, commit = sys.argv[1:]
m = json.load(open(meta_path))
packages = m['packages']

def component_for_crate(p):
    licenses = p.get('license') or ('license-file' if p.get('license_file') else 'NOASSERTION')
    version = p.get('version') or ''
    c = {
        'type': 'library',
        'bom-ref': f'pkg:cargo/{p["name"]}@{version}',
        'name': p['name'],
        'version': version,
        'purl': f'pkg:cargo/{p["name"]}@{version}',
        'licenses': [{'license': {'id': licenses}}] if '/' not in licenses and ' OR ' not in licenses and ' AND ' not in licenses
                     else [{'expression': licenses}],
    }
    return c

components = []
for p in packages:
    components.append(component_for_crate(p))

# Shipped npm workspace packages (BETA-015-D tarballs).
import glob
npm_names = []
for pkgjson in sorted(glob.glob('packages/*/package.json')):
    d = json.load(open(pkgjson))
    name, version = d['name'], d['version']
    npm_names.append(name)
    components.append({
        'type': 'library',
        'bom-ref': f'pkg:npm/%40velqu/{name.split("/")[-1]}@{version}',
        'name': name,
        'version': version,
        'purl': f'pkg:npm/%40velqu/{name.split("/")[-1]}@{version}',
        'licenses': [{'license': {'id': 'NOASSERTION'}}],
        'properties': [
            {'name': 'velqu:shipped-tarball', 'value': 'true'},
            {'name': 'velqu:license-posture', 'value': 'owner-decision-pending'},
        ],
    })

components.sort(key=lambda c: (c['type'], c['name'], c['version']))

workspace = [p for p in packages if p.get('source') is None]
external = [p for p in packages if p.get('source') is not None]
missing_ext = [p['name'] for p in external if not p.get('license') and not p.get('license_file')]

sbom = {
    '$schema': 'https://cyclonedx.org/schema/bom-1.5.schema.json',
    'bomFormat': 'CycloneDX',
    'specVersion': '1.5',
    'serialNumber': f'urn:uuid:{commit[:8]}-{commit[8:12]}-{commit[12:16]}-{commit[16:20]}-{commit[20:32] if len(commit)>=32 else commit[20:]}',
    'version': 1,
    'metadata': {
        'timestamp': datetime.datetime.now(datetime.timezone.utc).isoformat().replace('+00:00', 'Z'),
        'component': {
            'type': 'application',
            'bom-ref': f'pkg:generic/velqu@0.1.0',
            'name': 'velqu',
            'version': '0.1.0',
            'purl': 'pkg:generic/velqu@0.1.0',
        },
        'properties': [
            {'name': 'velqu:source-commit', 'value': commit},
            {'name': 'velqu:workspace-crates', 'value': str(len(workspace))},
            {'name': 'velqu:external-crates', 'value': str(len(external))},
            {'name': 'velqu:npm-workspace-packages', 'value': str(len(npm_names))},
        ],
    },
    'components': components,
}

pathlib.Path(out_path).parent.mkdir(parents=True, exist_ok=True)
pathlib.Path(out_path).write_text(json.dumps(sbom, indent=2, sort_keys=False) + '\n')

summary = {
    'format': 'CycloneDX 1.5',
    'commit': commit,
    'components': len(components),
    'workspaceCrates': len(workspace),
    'externalCrates': len(external),
    'npmPackages': len(npm_names),
    'externalPackagesMissingLicense': missing_ext,
    'verdict': 'PASS' if not missing_ext else 'BLOCKED_MISSING_EXTERNAL_LICENSES',
}
print(json.dumps(summary, indent=2))
if missing_ext:
    sys.exit(1)
PY
