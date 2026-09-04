#!/usr/bin/env bash
# BETA-010-C — inspect npm package publication metadata without publishing.
set -euo pipefail
ROOT=$(cd "$(dirname "$0")/.." && pwd)
cd "$ROOT"
OUT=${1:-docs/reports/beta-010-c-npm-package-inventory.json}
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT
bun pm pack --help >/dev/null 2>&1 || true
python3 - "$OUT" <<'PY'
import json, pathlib, glob
out=pathlib.Path(__import__('sys').argv[1])
items=[]
for p in sorted(glob.glob('packages/*/package.json')):
    d=json.load(open(p))
    items.append({
      'path':p, 'name':d.get('name'), 'version':d.get('version'),
      'private':d.get('private',False), 'type':d.get('type'),
      'main':d.get('main'), 'types':d.get('types'), 'bin':d.get('bin'),
      'dependencies':d.get('dependencies',{}),
      'publishConfig':d.get('publishConfig'),
      'license':d.get('license'), 'repository':d.get('repository'),
      'publishable': not d.get('private',False),
    })
result={
  'format':'velqu-beta-npm-package-inventory-v1',
  'tagPolicy':'beta (owner-authorized publication only)',
  'packageCount':len(items),
  'packages':items,
  'privatePackageCount':sum(i['private'] for i in items),
  'publishablePackageCount':sum(i['publishable'] for i in items),
  'verdict':'PREPARED_NOT_PUBLISHED',
  'disclosure':'All current workspace packages are private; npm publication and beta tag require owner release authorization and repository/license decision.',
}
out.parent.mkdir(parents=True,exist_ok=True); out.write_text(json.dumps(result,indent=2)+'\n'); print(json.dumps(result,indent=2))
PY
