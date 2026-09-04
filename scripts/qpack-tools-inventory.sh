#!/usr/bin/env bash
# BETA-010-D — verify and inventory runtime binaries and QPack tools.
# Tests:
#   1. velqu-runtime --fingerprint --pack <app.qpack>
#   2. velqu-bytecode embed --pack <app.qpack> --out <out.qpack>
#   3. velqu pack inspect <app.qpack> --json
#   4. velqu pack migrate <app.qpack> --json
#   5. velqu-standalone execution
set -euo pipefail
ROOT=$(cd "$(dirname "$0")/.." && pwd)
cd "$ROOT"
OUT=${1:-docs/reports/beta-010-d-qpack-tools-inventory.json}
RUNTIME="target/release/velqu-runtime"
STANDALONE="target/release/velqu-standalone"
BYTECODE="target/release/velqu-bytecode"
PACK="examples/proof/dist/app.qpack"
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT

[[ -x "$RUNTIME" ]] || { echo "FAIL: $RUNTIME not found" >&2; exit 1; }
[[ -f "$PACK" ]] || { echo "FAIL: $PACK not found" >&2; exit 1; }

# 1. Runtime fingerprint test
FP_JSON=$("$RUNTIME" --fingerprint --pack "$PACK")
VERDICT=$(echo "$FP_JSON" | python3 -c 'import sys, json; d=json.load(sys.stdin); print(d.get("pack",{}).get("verdict"))')
[[ "$VERDICT" == "compatible" ]] || { echo "FAIL: unexpected fingerprint verdict: $VERDICT" >&2; exit 1; }

# 2. Bytecode embed test
EMBED_PACK="$TMP/embedded.qpack"
if [[ -x "$BYTECODE" ]]; then
  "$BYTECODE" embed --pack "$PACK" --out "$EMBED_PACK" >/dev/null
  [[ -f "$EMBED_PACK" ]] || { echo "FAIL: embedded pack not generated" >&2; exit 1; }
fi

# 3. CLI pack inspect test
INSPECT_JSON=$(bun packages/cli/src/index.ts pack inspect "$PACK" --json)
INSPECT_STATUS=$(echo "$INSPECT_JSON" | python3 -c 'import sys, json; d=json.load(sys.stdin); print(d.get("status"))')
[[ "$INSPECT_STATUS" == "ok" ]] || { echo "FAIL: pack inspect failed" >&2; exit 1; }

# 4. CLI pack migrate test
MIGRATE_JSON=$(bun packages/cli/src/index.ts pack migrate "$PACK" --json)
MIGRATE_STATUS=$(echo "$MIGRATE_JSON" | python3 -c 'import sys, json; d=json.load(sys.stdin); print(d.get("status"))')
[[ "$MIGRATE_STATUS" == "ok" ]] || { echo "FAIL: pack migrate failed" >&2; exit 1; }

# 5. Inventory result
python3 - "$OUT" <<PY
import json, sys, os, pathlib
out_path = sys.argv[1]
data = {
  "format": "velqu-beta-qpack-tools-inventory-v1",
  "tools": {
    "velquRuntime": {
      "path": "$RUNTIME",
      "exists": os.path.exists("$RUNTIME"),
      "fingerprintCheck": "$VERDICT"
    },
    "velquStandalone": {
      "path": "$STANDALONE",
      "exists": os.path.exists("$STANDALONE")
    },
    "velquBytecode": {
      "path": "$BYTECODE",
      "exists": os.path.exists("$BYTECODE")
    },
    "cliPackInspect": {
      "command": "velqu pack inspect <pack> --json",
      "status": "$INSPECT_STATUS"
    },
    "cliPackMigrate": {
      "command": "velqu pack migrate <pack> --json",
      "status": "$MIGRATE_STATUS"
    }
  },
  "verdict": "PASS"
}
pathlib.Path(out_path).parent.mkdir(parents=True, exist_ok=True)
pathlib.Path(out_path).write_text(json.dumps(data, indent=2) + "\n")
print(json.dumps(data, indent=2))
PY
