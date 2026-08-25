#!/usr/bin/env bash
# M26-009-A — shared-mode artifact smoke test (`velqu-runtime` + app.qpack).
#
# Usage: scripts/artifact-smoke.sh [runtime-binary] [pack-file] [port]
#
# Checks, in order:
#   1. artifacts exist (release runtime + verified pack)
#   2. server becomes ready and answers two real routes
#   3. a fingerprint-mismatched pack is rejected BEFORE ready (exit != 0)
#   4. cold-start samples (startupMs from the runtime's own ready line)
#
# Output ends with a machine-readable SMOKE-OK line; any failure exits
# non-zero.
set -euo pipefail

RUNTIME=${1:-target/release/velqu-runtime}
PACK=${2:-examples/proof/dist/app.qpack}
PORT=${3:-3997}

cd "$(dirname "$0")/.."

[[ -x "$RUNTIME" ]] || { echo "FAIL: runtime not found/executable: $RUNTIME (cargo build --release -p velqu-runtime)" >&2; exit 1; }
[[ -f "$PACK" ]] || { echo "FAIL: pack not found: $PACK (bun packages/cli/src/index.ts build --project examples/proof)" >&2; exit 1; }

echo "artifacts:"
echo "  runtime: $RUNTIME ($(wc -c < "$RUNTIME") bytes)"
echo "  pack:    $PACK ($(wc -c < "$PACK") bytes)"

wait_ready() { # $1=port -> waits until TCP answers, max ~5s
  for _ in $(seq 1 50); do
    curl -sf "http://127.0.0.1:$1/health/live" >/dev/null 2>&1 && return 0
    sleep 0.1
  done
  return 1
}

# --- 2. serve + answer real routes -----------------------------------------
"./$RUNTIME" --pack "$PACK" --port "$PORT" --log off >/tmp/smoke-serve.log 2>&1 &
SRV=$!
trap 'kill $SRV 2>/dev/null || true' EXIT
wait_ready "$PORT" || { echo "FAIL: server never became ready" >&2; exit 1; }

HEALTH=$(curl -sf "http://127.0.0.1:$PORT/health/live")
[[ "$HEALTH" == '{"status":"ok"}' ]] || { echo "FAIL: unexpected health body: $HEALTH" >&2; exit 1; }
HELLO=$(curl -sf "http://127.0.0.1:$PORT/hello/smoke")
[[ -n "$HELLO" ]] || { echo "FAIL: empty hello response" >&2; exit 1; }
kill $SRV 2>/dev/null || true
wait $SRV 2>/dev/null || true
echo "serve: /health/live and /hello/:name answered OK"

# --- 3. mismatched-runtime rejection (fail closed before ready) -------------
MISMATCH=/tmp/smoke-mismatch.qpack
python3 - "$PACK" "$MISMATCH" <<'PY'
import json, sys
src, dst = sys.argv[1], sys.argv[2]
d = json.load(open(src))
d["engine"]["version"] = "9.9.9"  # claims a different embedded engine
json.dump(d, open(dst, "w"))
PY
if "./$RUNTIME" --pack "$MISMATCH" --port "$((PORT + 1))" --log off >/tmp/smoke-reject.log 2>&1; then
  echo "FAIL: mismatched-engine pack was accepted" >&2
  exit 1
fi
grep -q "engine mismatch" /tmp/smoke-reject.log ||
  { echo "FAIL: rejection lacked actionable reason:" >&2; cat /tmp/smoke-reject.log >&2; exit 1; }
echo "reject: mismatched-engine pack failed closed before ready (actionable message)"

# --- 4. cold-start samples ---------------------------------------------------
SAMPLES=${SAMPLES:-10}
: > /tmp/smoke-cold.txt
for _ in $(seq 1 "$SAMPLES"); do
  "./$RUNTIME" --pack "$PACK" --port "$((PORT + 2))" --log off >/tmp/smoke-cold-run.log 2>&1 &
  P=$!
  wait_ready "$((PORT + 2))" || { echo "FAIL: cold spawn never ready" >&2; exit 1; }
  grep -o '"startupMs":[0-9.]*' /tmp/smoke-cold-run.log | head -1 | cut -d: -f2 >> /tmp/smoke-cold.txt
  kill $P 2>/dev/null || true
  wait $P 2>/dev/null || true
done
MEDIAN=$(sort -n /tmp/smoke-cold.txt | awk '{a[NR]=$1} END {print a[int((NR+1)/2)]}')
echo "cold-start: $SAMPLES samples, p50 ${MEDIAN}ms (raw: $(tr '\n' ' ' < /tmp/smoke-cold.txt))"

# --- 5. standalone mode: embedded pack, identical serving --------------- (M26-009-B)
STANDALONE=${STANDALONE_BIN:-target/release/velqu-standalone}
if [[ ! -x "$STANDALONE" ]]; then
  echo "building standalone binary (feature standalone, pack $PACK)"
  VELQU_STANDALONE_PACK="$PACK" cargo build --release -p velqu-runtime --features standalone \
    || { echo "FAIL: standalone build failed" >&2; exit 1; }
fi
STANDALONE_PORT=$((PORT + 3))
"./$STANDALONE" --port "$STANDALONE_PORT" --log off >/tmp/smoke-standalone.log 2>&1 &
SA=$!
trap 'kill $SRV $SA 2>/dev/null || true' EXIT
wait_ready "$STANDALONE_PORT" || { echo "FAIL: standalone never became ready" >&2; cat /tmp/smoke-standalone.log >&2; exit 1; }
S_HEALTH=$(curl -sf "http://127.0.0.1:$STANDALONE_PORT/health/live")
[[ "$S_HEALTH" == "$HEALTH" ]] || { echo "FAIL: standalone health body differs: $S_HEALTH" >&2; exit 1; }
S_HELLO=$(curl -sf "http://127.0.0.1:$STANDALONE_PORT/hello/smoke")
[[ "$S_HELLO" == "$HELLO" ]] || { echo "FAIL: standalone hello body differs: $S_HELLO vs $HELLO" >&2; exit 1; }
S_MODE=$(grep -o '"mode":"[a-z]*"' /tmp/smoke-standalone.log | head -1 || true)
[[ "$S_MODE" == '"mode":"standalone"' ]] || { echo "FAIL: standalone ready line mode field: $S_MODE" >&2; exit 1; }
kill $SA 2>/dev/null || true
wait $SA 2>/dev/null || true
echo "standalone: identical /health/live and /hello/:name answers; mode=standalone"

echo "SMOKE-OK shared-mode runtime=$RUNTIME pack=$PACK"
