#!/usr/bin/env bash
# #1400 — external consumer relay service, full journey on PUBLISHED
# artifacts: registry install -> check -> test -> build -> serve on the
# real Rust runtime -> assert happy paths AND typed failures -> transcript.
#
# Differs from the beta-016 scripts on purpose: those run inside the
# fresh beta environment; this one runs on a dev host (or CI) that has
#   - bun 1.4.0 exactly on PATH (or BUN=... override),
#   - a release velqu-runtime (VELQU_RUNTIME=... or ./target/release/),
#   - network access to registry.npmjs.org (bun install step).
#
# Usage: scripts/beta-external/relay-service.sh [workdir]
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
BUN="${BUN:-bun}"
APP="${1:-$(mktemp -d /tmp/velqu-relay-XXXXXX)}/relay-service"
RUNTIME="${VELQU_RUNTIME:-$ROOT/target/release/velqu-runtime}"
UPSTREAM_PORT=18971
TOKEN="q-relay-demo-token"

STEP=0
step() { STEP=$((STEP+1)); echo; echo "== [$STEP] $*"; }
fail() { echo "RELAY-FAIL at step $STEP: $*" >&2; exit 1; }
cleanup() {
  [ -n "${RT_PID:-}" ] && kill "$RT_PID" 2>/dev/null || true
  [ -n "${UP_PID:-}" ] && kill "$UP_PID" 2>/dev/null || true
}
trap cleanup EXIT

command -v "$BUN" >/dev/null 2>&1 || fail "bun not found (set BUN=...)"
BUN_VER="$("$BUN" --version)"
[ "$BUN_VER" = "1.4.0" ] || fail "bun 1.4.0 required (pinned toolchain), found $BUN_VER"
[ -x "$RUNTIME" ] || fail "runtime not found at $RUNTIME (cargo build --release -p velqu-runtime, or set VELQU_RUNTIME)"

free_port() {
  "$BUN" -e 'const s=Bun.listen({hostname:"127.0.0.1",port:0,socket:{data(){}}});console.log(s.port);s.stop(true);'
}
# bun -e helper: perform one HTTP request, exit non-zero unless every
# listed expectation holds. Usage: http <method> <url> [json-body] [expect...]
# where each expect is status:<n> | ctype:<substr> | body:<substr>
http() {
  local method="$1" url="$2" body="${3:-}" expect
  shift $(( $# > 3 ? 3 : $# ))
  "$BUN" -e '
    const [method, url, body, ...expects] = process.argv.slice(1);
    const res = await fetch(url, {
      method,
      headers: body ? { "content-type": "application/json", authorization: `Bearer ${process.env.TOKEN ?? ""}` } : {},
      body: body || undefined,
    });
    const text = await res.text();
    let ok = true;
    for (const e of expects) {
      const [k, v] = [e.slice(0, e.indexOf(":")), e.slice(e.indexOf(":") + 1)];
      if (k === "status" && res.status !== Number(v)) { ok = false; console.error(`status ${res.status} != ${v}`); }
      if (k === "ctype" && !(res.headers.get("content-type") ?? "").includes(v)) { ok = false; console.error(`ctype ${res.headers.get("content-type")} lacks ${v}`); }
      if (k === "body" && !text.includes(v)) { ok = false; console.error(`body lacks ${v}: ${text.slice(0, 300)}`); }
    }
    console.log(`${method} ${url} -> ${res.status} ${text.slice(0, 200)}`);
    process.exit(ok ? 0 : 1);
  ' "$method" "$url" "$body" "$@"
}

echo "== relay-service external consumer transcript =="
echo "bun=$BUN_VER runtime=$RUNTIME app=$APP"

step "copy template to $APP (clean consumer tree)"
mkdir -p "$(dirname "$APP")"
cp -r "$ROOT/scripts/beta-external/relay-service/." "$APP"/
rm -rf "$APP/node_modules" "$APP/dist"

step "bun install (public registry — no workspace links)"
(cd "$APP" && "$BUN" install) || fail "registry install failed"

step "velqu check (via the scaffold-style velqu bin script)"
OUT="$(cd "$APP" && "$BUN" run check)" || fail "check failed: $OUT"
echo "$OUT"
echo "$OUT" | grep -q "4 routes" || fail "check did not report 4 routes: $OUT"

step "bun test (unit green; runtime-local client tests skip without a server)"
FREE_TEST_PORT="$(free_port)"
(cd "$APP" && VELQU_DEV_PORT="$FREE_TEST_PORT" "$BUN" test) || fail "tests failed"

step "velqu build -> dist/app.qpack"
(cd "$APP" && "$BUN" run build) || fail "build failed"
[ -f "$APP/dist/app.qpack" ] || fail "dist/app.qpack missing"

step "start loopback upstream stub on :$UPSTREAM_PORT"
"$BUN" -e '
  Bun.serve({
    port: Number(process.argv[2] ?? 18971),
    fetch: (req) => new Response(JSON.stringify({ enrichment: "stub-enrichment", path: new URL(req.url).pathname }), { headers: { "content-type": "application/json" } }),
  });
  setInterval(() => {}, 60_000);
' x "$UPSTREAM_PORT" &
UP_PID=$!
sleep 0.3

step "spawn velqu-runtime with the built pack"
SVC_PORT="$(free_port)"
"$RUNTIME" --pack "$APP/dist/app.qpack" --port "$SVC_PORT" --log errors &
RT_PID=$!
for i in $(seq 1 50); do
  if http GET "http://127.0.0.1:$SVC_PORT/health/live" "" status:200 body:ok >/dev/null 2>&1; then break; fi
  sleep 0.2
  [ "$i" = 50 ] && fail "runtime never became ready"
done
echo "runtime ready on :$SVC_PORT (pid $RT_PID)"

step "typed failure: intake without token -> 401 problem+json"
http POST "http://127.0.0.1:$SVC_PORT/hooks" \
  '{"source":"generic","deliveryId":"d12345678","eventType":"ping","payloadText":"{}"}' \
  status:401 ctype:problem+json body:unauthorized || fail "401 policy failure shape wrong"

step "typed failure: invalid deliveryId -> 422 validation"
TOKEN="$TOKEN" http POST "http://127.0.0.1:$SVC_PORT/hooks" \
  '{"source":"generic","deliveryId":"short","eventType":"ping","payloadText":"{}"}' \
  status:422 || fail "422 validation failure shape wrong"

step "happy path: authenticated intake -> 201"
TOKEN="$TOKEN" http POST "http://127.0.0.1:$SVC_PORT/hooks" \
  '{"source":"github","deliveryId":"dlv-10000001","eventType":"push","payloadText":"{\"repo\":\"demo\"}"}' \
  status:201 body:dlv-10000001 || fail "intake failed"

step "happy path: enrichment via loopback fetch -> 200 stub-enrichment"
TOKEN="$TOKEN" http POST "http://127.0.0.1:$SVC_PORT/hooks/dlv-10000001/enrich" \
  "" status:200 body:stub-enrichment || fail "enrichment failed"

step "read-back shows enrichment (was pending)"
http GET "http://127.0.0.1:$SVC_PORT/hooks/dlv-10000001" "" status:200 body:stub-enrichment \
  || fail "read-back failed"

step "typed failure: unknown delivery -> 404 problem"
http GET "http://127.0.0.1:$SVC_PORT/hooks/missing-00001" "" status:404 ctype:problem+json \
  || fail "404 shape wrong"

step "typed failure: upstream loss -> 502 typed error"
kill "$UP_PID" 2>/dev/null || true
sleep 0.3
TOKEN="$TOKEN" http POST "http://127.0.0.1:$SVC_PORT/hooks" \
  '{"source":"generic","deliveryId":"dlv-20000002","eventType":"ping","payloadText":"{}"}' \
  status:201 body:dlv-20000002 >/dev/null || fail "second intake failed"
TOKEN="$TOKEN" http POST "http://127.0.0.1:$SVC_PORT/hooks/dlv-20000002/enrich" \
  "" status:502 body:upstream || fail "502 upstream-loss shape wrong"

step "graceful stop"
kill -TERM "$RT_PID"
wait "$RT_PID" 2>/dev/null || true
RT_PID=""

echo
echo "RELAY-OK"
