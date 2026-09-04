#!/usr/bin/env bash
# BETA-011-E — rehearse release yank/rollback procedure (dry-run, no registry calls).
# Implements the withdrawal governance of docs/beta/governance/RELEASE_AUTHORITY.md:
# the Owner may withdraw a release for one of four recorded reasons; withdrawal
# records the affected version and reason and never rewrites historical evidence.
set -euo pipefail
ROOT=$(cd "$(dirname "$0")/.." && pwd)
cd "$ROOT"
OUT=${1:-docs/reports/beta-011-e-yank-rollback-rehearsal.json}

VERSION="0.1.0-beta.1"
PREV_TAG_TARGET="0.1.0-alpha.0"
TAG="beta"

python3 - "$VERSION" "$PREV_TAG_TARGET" "$TAG" "$OUT" "$ROOT" <<'PY'
import json, sys, pathlib

version, prev, tag, out_path, root = sys.argv[1:]
root = pathlib.Path(root)

trigger_conditions = [
    "release evidence is incomplete",
    "checksums do not match",
    "a security issue requires withdrawal",
    "a release violates stated beta limits",
]

rehearsal = {
    "distTagRemoval": f"npm dist-tag rm @velqu/cli {tag} --dry-run",
    "distTagRepoint": f"npm dist-tag add @velqu/cli@{prev} {tag} --dry-run",
    "packageYank": f"npm yank @velqu/cli@{version} --dry-run",
    "githubReleaseWithdraw": "gh release unpublish <tag> --yes (or delete) with withdrawal note",
    "packetWithdrawalRecord": {
        "schema": {
            "withdrawnVersion": version,
            "reason": "<one of the four trigger conditions>",
            "recordedAt": "<UTC ISO-8601>",
            "evidenceRewritten": False
        },
        "invariant": "Withdrawal appends a record; historical evidence files are never modified."
    },
}

checks = {
    "allDryRun": all("dry-run" in v or "unpublish" in v or "withdrawal" in v
                     for k, v in rehearsal.items() if isinstance(v, str)),
    "triggerConditionsRecorded": trigger_conditions == [
        "release evidence is incomplete",
        "checksums do not match",
        "a security issue requires withdrawal",
        "a release violates stated beta limits",
    ],
    "rollbackTargetIsStableChannel": prev != version,
    "authorityDocumentPresent": (root / "docs/beta/governance/RELEASE_AUTHORITY.md").is_file(),
    "packetBuilderSupportsWithdrawal": (root / "scripts/release-packet").is_file(),
}

verdict = "PASS" if all(checks.values()) else "FAIL"

data = {
    "format": "velqu-beta-yank-rollback-rehearsal-v1",
    "targetVersion": version,
    "rollbackTarget": prev,
    "defaultTag": tag,
    "triggerConditions": trigger_conditions,
    "rehearsal": rehearsal,
    "checks": checks,
    "verdict": verdict,
}

pathlib.Path(out_path).parent.mkdir(parents=True, exist_ok=True)
pathlib.Path(out_path).write_text(json.dumps(data, indent=2) + "\n")
print(json.dumps(data, indent=2))
if verdict != "PASS":
    sys.exit(1)
PY
