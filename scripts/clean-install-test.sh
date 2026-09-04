#!/usr/bin/env bash
# BETA-010-E — clean environment installation and execution test.
# Verifies that `velqu-runtime` + `app.qpack` function in a pristine,
# isolated directory without node_modules, source trees, or developer tooling.
set -euo pipefail

ROOT=$(cd "$(dirname "$0")/.." && pwd)
RUNTIME=${RUNTIME_BIN:-"$ROOT/target/release/velqu-runtime"}
PACK=${PACK_FILE:-"$ROOT/examples/proof/dist/app.qpack"}
PORT=${PORT:-3996}

[[ -x "$RUNTIME" ]] || { echo "FAIL: runtime binary not found: $RUNTIME" >&2; exit 1; }
[[ -f "$PACK" ]] || { echo "FAIL: pack artifact not found: $PACK" >&2; exit 1; }

CLEAN_DIR=$(mktemp -d /tmp/velqu-clean-env-XXXXXX)
LOG="$CLEAN_DIR/runtime.log"
cleanup() {
  if [[ -n "${PID:-}" ]] && kill -0 "$PID" 2>/dev/null; then
    kill -TERM "$PID" 2>/dev/null || true
    wait "$PID" 2>/dev/null || true
  fi
  rm -rf "$CLEAN_DIR"
}
trap cleanup EXIT

# 1. Install to pristine environment
cp "$RUNTIME" "$CLEAN_DIR/velqu-runtime"
cp "$PACK" "$CLEAN_DIR/app.qpack"
chmod +x "$CLEAN_DIR/velqu-runtime"

cd "$CLEAN_DIR"

# 2. Verify fingerprint and pack compatibility without serving
FP_JSON=$(./velqu-runtime --fingerprint --pack app.qpack)
echo "$FP_JSON" | grep -Fq '"verdict":"compatible"' || { echo "FAIL: fingerprint compatibility check failed in clean env" >&2; exit 1; }

# 3. Serve in clean environment
./velqu-runtime --pack app.qpack --port "$PORT" --log off >"$LOG" 2>&1 &
PID=$!

for _ in $(seq 1 100); do
  if curl -sf "http://127.0.0.1:$PORT/health/live" >/dev/null 2>&1; then break; fi
  sleep 0.05
done

# 4. Verify endpoints
curl -sf "http://127.0.0.1:$PORT/health/live" | grep -Fq '{"status":"ok"}' || { echo "FAIL: health live failed" >&2; exit 1; }
curl -sf "http://127.0.0.1:$PORT/hello/clean-env" | grep -Fq 'clean-env' || { echo "FAIL: route hello failed" >&2; exit 1; }

# 5. Verify clean shutdown on SIGTERM
kill -TERM "$PID"
for _ in $(seq 1 100); do
  if ! kill -0 "$PID" 2>/dev/null; then break; fi
  sleep 0.05
done
wait "$PID" 2>/dev/null || true
PID=""

# 6. Verify fail-closed behavior on missing or corrupt pack
set +e
./velqu-runtime --pack nonexistent.qpack --port "$PORT" >/dev/null 2>&1
MISSING_CODE=$?
printf 'corrupted-data' > corrupt.qpack
./velqu-runtime --pack corrupt.qpack --port "$PORT" >/dev/null 2>&1
CORRUPT_CODE=$?
set -e

[[ "$MISSING_CODE" -ne 0 ]] || { echo "FAIL: missing pack must exit non-zero" >&2; exit 1; }
[[ "$CORRUPT_CODE" -ne 0 ]] || { echo "FAIL: corrupt pack must exit non-zero" >&2; exit 1; }

echo "CLEAN-INSTALL-TEST-OK: pristine directory runtime+pack verified, served, and shut down cleanly"
