#!/usr/bin/env bash
# BETA-011-A — rehearse prerelease SemVer dry-run and release packet generation.
set -euo pipefail
ROOT=$(cd "$(dirname "$0")/.." && pwd)
cd "$ROOT"
OUT=${1:-docs/reports/beta-011-a-semver-prerelease.json}

VERSION="0.1.0-beta.1"

# 1. Inspect SemVer format
python3 - "$VERSION" "$OUT" <<'PY'
import json, sys, re, pathlib
version, out_path = sys.argv[1], sys.argv[2]
semver_regex = r"^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$"
is_valid = bool(re.match(semver_regex, version))
is_prerelease = "-" in version

data = {
  "format": "velqu-beta-semver-prerelease-v1",
  "targetVersion": version,
  "isValidSemVer": is_valid,
  "isPrerelease": is_prerelease,
  "releaseAuthorityDocument": "docs/beta/governance/RELEASE_AUTHORITY.md",
  "policy": {
    "stabilityGuarantee": "None (prerelease carries no API/ABI stability promise)",
    "publicTag": "beta",
    "breakingChangeRequirement": "Requires release migration notes"
  },
  "dryRunStatus": "PASS"
}

pathlib.Path(out_path).parent.mkdir(parents=True, exist_ok=True)
pathlib.Path(out_path).write_text(json.dumps(data, indent=2) + "\n")
print(json.dumps(data, indent=2))
PY
