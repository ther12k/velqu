#!/usr/bin/env bash
# BWASM-R-002 — native/browser dispatcher fixture diff.
#
# Runs one route corpus twice against the SAME pack (examples/proof):
#  1. native: velqu-runtime serving the pack over HTTP (netns)
#  2. browser: the R-002 dispatcher + the REAL q-browser-kernel wasm
#     (nodejs glue) in Node
# then diffs status codes and problem ids. Requires the repository
# release runtime, the proof pack, and the kernel glue (regenerated at
# the current commit; see the report for the exact commands).
set -euo pipefail
cd "$(dirname "$0")/../../../.."  # repo root (packages/browser-runtime/test/fixtures → root)

RUNTIME=target/release/velqu-runtime
PACK=examples/proof/dist/app.qpack
GLUE=${GLUE_DIR:-/tmp/r002-glue}
PORT=${PORT:-8377}

[[ -x "$RUNTIME" ]] || { echo "FAIL: build $RUNTIME first" >&2; exit 1; }
[[ -f "$PACK" ]] || { echo "FAIL: build $PACK first" >&2; exit 1; }
[[ -f "$GLUE/q_browser_kernel.js" ]] || { echo "FAIL: generate glue into $GLUE first (see report)" >&2; exit 1; }

# Corpus: method path (mix of real routes, wrong methods, unknown
# paths, trailing slash). Kept aligned with the JS side below.
CORPUS_FILE=$(mktemp)
cat > "$CORPUS_FILE" <<'EOF'
GET /health/live
HEAD /health/live
POST /health/live
GET /health/nope
GET /hello/beta
GET /items
OPTIONS /items
GET /items/
GET /nope/nope
EOF

# Documented expected difference (NOT a corpus row): /health/ready is a
# NATIVE HOST-LEVEL socket probe (examples/proof/src/modules/ops/routes.ts
# comment; the artifact route is /ops/readiness). The browser kernel
# correctly answers 404 for it — native-only surface, per ADR-0037 §1.

# --- native side (runtime AND curls inside one netns) ---
NATIVE_OUT=$(mktemp)
unshare -rn bash -c '
  set -e
  ip link set lo up
  RUNTIME='"$RUNTIME"' PACK='"$PACK"' PORT='"$PORT"' CORPUS='"$CORPUS_FILE"' OUT='"$NATIVE_OUT"'
  "$RUNTIME" --pack "$PACK" --port "$PORT" --proxy-mode direct --log off &
  RT=$!
  for _ in $(seq 1 60); do curl -sf "http://127.0.0.1:$PORT/health/live" >/dev/null 2>&1 && break; sleep 0.25; done
  while read -r method path; do
    status=$(curl -s -o /tmp/r002-body.$$ -w "%{http_code}" -X "$method" "http://127.0.0.1:$PORT$path" || echo 000)
    # Native problems are RFC 9457 with a type URI; derive the registry
    # id from its last path segment (== kernel problemId vocabulary).
    pid=$(grep -oE "\"type\":\"https://velqu.dev/problems/[a-z-]+\"" /tmp/r002-body.$$ 2>/dev/null | sed "s|.*/||; s|\"||" | head -1 || true)
    # Plan-level reduction: 2xx = ROUTE (route exists + admitted);
    # otherwise PROBLEM(status, id). Handler OUTPUTS are out of scope
    # for this diff (browser handler execution is R-003/R-004).
    case "$status" in
      2*) echo "$method $path -> ROUTE" >> "$OUT" ;;
      *) echo "$method $path -> PROBLEM($status) \"problemId\":\"$pid\"" >> "$OUT" ;;
    esac
  done < "$CORPUS"
  kill $RT 2>/dev/null || true
' 

# --- browser side (dispatcher + real kernel) ---
BROWSER_OUT=$(mktemp)
GLUE="$GLUE" PACK="$PACK" CORPUS="$CORPUS_FILE" OUT="$BROWSER_OUT" node "$(dirname "$0")/dispatcher-corpus.mjs"

echo "== native =="; cat "$NATIVE_OUT"
echo "== browser (dispatcher + real kernel) =="; cat "$BROWSER_OUT"
if diff -u "$NATIVE_OUT" "$BROWSER_OUT"; then
  echo "NATIVE-BROWSER-DIFF-OK: statuses and problem ids identical across $(wc -l < "$CORPUS_FILE") corpus entries"
else
  echo "NATIVE-BROWSER-DIFF-MISMATCH" >&2
  exit 1
fi
