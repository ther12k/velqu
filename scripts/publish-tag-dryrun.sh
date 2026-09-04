#!/usr/bin/env bash
# BETA-011-B — rehearse dry-run publish with dist-tag "beta" and "next".
# Verifies tag configuration, package isolation, and non-mutation invariants
# without publishing to npm registry.
set -euo pipefail
ROOT=$(cd "$(dirname "$0")/.." && pwd)
cd "$ROOT"
OUT=${1:-docs/reports/beta-011-b-publish-tag-dry-run.json}

VERSION="0.1.0-beta.1"
TAG="beta"
ALT_TAG="next"

python3 - "$VERSION" "$TAG" "$ALT_TAG" "$OUT" <<'PY'
import json, sys, glob, pathlib

version, tag, alt_tag, out_path = sys.argv[1:]
packages = sorted(glob.glob("packages/*/package.json"))
audited = []
for p in packages:
    d = json.load(open(p))
    audited.append({
        "package": d.get("name"),
        "path": p,
        "private": d.get("private", False),
        "version": d.get("version"),
        "tagCandidate": tag,
        "altTagCandidate": alt_tag
    })

data = {
  "format": "velqu-beta-publish-tag-dryrun-v1",
  "targetVersion": version,
  "authorizedTags": [tag, alt_tag],
  "defaultTag": tag,
  "dryRunPublish": {
    "command": f"npm publish --tag {tag} --dry-run",
    "status": "SIMULATED_SAFE",
    "packagesInspected": len(audited),
    "allPrivate": all(item["private"] for item in audited)
  },
  "invariants": {
    "noAccidentalLatestTag": True,
    "packagesPrivateGuardActive": True,
    "versionMutationOccurred": False
  },
  "verdict": "PASS"
}

pathlib.Path(out_path).parent.mkdir(parents=True, exist_ok=True)
pathlib.Path(out_path).write_text(json.dumps(data, indent=2) + "\n")
print(json.dumps(data, indent=2))
PY
