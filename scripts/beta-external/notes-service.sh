#!/usr/bin/env bash
# #1404 — second external consumer: notes CRUD on the Postgres
# capability, generated Treaty contract, ephemeral-DB E2E on PUBLISHED
# artifacts (registry install -> check -> build -> test -> serve on the
# real Rust runtime with VELQU_DATABASE_URL -> happy paths AND typed
# failures incl. live database loss -> transcript).
#
# Requires: bun 1.4.0 (or BUN=...), a release velqu-runtime
# (VELQU_RUNTIME=... or ./target/release/), a running docker daemon
# (or DOCKER=...), registry.npmjs.org access.
#
# Usage: scripts/beta-external/notes-service.sh [workdir]
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
BUN="${BUN:-bun}"
DOCKER="${DOCKER:-docker}"
APP="${1:-$(mktemp -d /tmp/velqu-notes-XXXXXX)}/notes-service"
RUNTIME="${VELQU_RUNTIME:-$ROOT/target/release/velqu-runtime}"
TOKEN="q-notes-demo-token"
PG_NAME="velqu-notes-e2e-$$"
PG_IMAGE="postgres:16-alpine"

STEP=0
step() { STEP=$((STEP+1)); echo; echo "== [$STEP] $*"; }
fail() { echo "NOTES-FAIL at step $STEP: $*" >&2; exit 1; }
cleanup() {
  [ -n "${RT_PID:-}" ] && kill "$RT_PID" 2>/dev/null || true
  "$DOCKER" rm -f "$PG_NAME" >/dev/null 2>&1 || true
}
trap cleanup EXIT

command -v "$BUN" >/dev/null 2>&1 || fail "bun not found (set BUN=...)"
BUN_VER="$("$BUN" --version)"
[ "$BUN_VER" = "1.4.0" ] || fail "bun 1.4.0 required (pinned toolchain), found $BUN_VER"
[ -x "$RUNTIME" ] || fail "runtime not found at $RUNTIME (cargo build --release -p velqu-runtime, or set VELQU_RUNTIME)"
command -v "$DOCKER" >/dev/null 2>&1 || fail "docker not found (set DOCKER=...)"
"$DOCKER" info >/dev/null 2>&1 || fail "docker daemon not reachable"

free_port() {
  "$BUN" -e 'const s=Bun.listen({hostname:"127.0.0.1",port:0,socket:{data(){}}});console.log(s.port);s.stop(true);'
}
# bun -e helper: one HTTP request; exit non-zero unless every expectation
# holds. Usage: http <method> <url> [json-body] [expect...] with
# status:<n> | ctype:<substr> | body:<substr>
http() {
  local method="$1" url="$2" body="${3:-}" expect
  shift $(( $# > 3 ? 3 : $# ))
  "$BUN" -e '
    const [method, url, body, ...expects] = process.argv.slice(1);
    const headers = {
      ...(body ? { "content-type": "application/json" } : {}),
      ...(process.env.TOKEN ? { authorization: `Bearer ${process.env.TOKEN}` } : {}),
    };
    const res = await fetch(url, {
      method,
      headers,
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
    console.log(`${method} ${url} -> ${res.status} ${text.slice(0, 220)}`);
    process.exit(ok ? 0 : 1);
  ' "$method" "$url" "$body" "$@"
}

echo "== notes-service external consumer transcript =="
echo "bun=$BUN_VER runtime=$RUNTIME docker-image=$PG_IMAGE app=$APP"

step "copy template to $APP (clean consumer tree)"
mkdir -p "$(dirname "$APP")"
cp -r "$ROOT/scripts/beta-external/notes-service/." "$APP"/
rm -rf "$APP/node_modules" "$APP/dist"

step "bun install (public registry — no workspace links)"
(cd "$APP" && "$BUN" install) || fail "registry install failed"

step "velqu check"
OUT="$(cd "$APP" && "$BUN" run check)" || fail "check failed: $OUT"
echo "$OUT"
echo "$OUT" | grep -q "5 routes" || fail "check did not report 5 routes: $OUT"

step "velqu build -> dist/app.qpack (+ generated contract artifacts)"
(cd "$APP" && "$BUN" run build) || fail "build failed"
[ -f "$APP/dist/app.qpack" ] || fail "dist/app.qpack missing"
[ -f "$APP/dist/contract.json" ] || fail "generated contract.json missing"

step "bun test (build ran first — the client imports generated artifacts)"
FREE_TEST_PORT="$(free_port)"
(cd "$APP" && VELQU_DEV_PORT="$FREE_TEST_PORT" "$BUN" test) || fail "tests failed"

step "boot ephemeral Postgres ($PG_IMAGE, loopback-published, removed on exit)"
"$DOCKER" run --rm -d --name "$PG_NAME" -p 127.0.0.1::5432 \
  -e POSTGRES_PASSWORD=e2e -e POSTGRES_DB=velqu_e2e "$PG_IMAGE" >/dev/null \
  || fail "postgres container failed to start"
PG_DIGEST="$("$DOCKER" inspect --format '{{index .RepoDigests 0}}' "$PG_IMAGE" 2>/dev/null || echo "$PG_IMAGE (digest unavailable)")"
for i in $(seq 1 60); do
  if "$DOCKER" exec "$PG_NAME" pg_isready -U postgres -d velqu_e2e >/dev/null 2>&1; then break; fi
  sleep 0.5
  [ "$i" = 60 ] && fail "postgres never became ready"
done
PG_PORT="$("$DOCKER" port "$PG_NAME" 5432/tcp | head -1 | sed 's/.*://')"
echo "postgres ready: 127.0.0.1:$PG_PORT/velqu_e2e image=$PG_DIGEST"

step "fail-closed proof: the pack's postgres grant rejects startup without VELQU_DATABASE_URL"
if env -u VELQU_DATABASE_URL "$RUNTIME" --pack "$APP/dist/app.qpack" --port "$(free_port)" --log errors >/tmp/velqu-notes-noUrl-$$ 2>&1; then
  fail "runtime started without a database URL — the postgres grant did not reach the pack"
fi
grep -qi "database" /tmp/velqu-notes-noUrl-$$ && echo "startup rejected (typed): $(head -2 /tmp/velqu-notes-noUrl-$$ | tail -1)" \
  || fail "startup failed without a database-related diagnostic"
rm -f /tmp/velqu-notes-noUrl-$$

step "spawn velqu-runtime with the built pack + VELQU_DATABASE_URL"
SVC_PORT="$(free_port)"
VELQU_DATABASE_URL="postgres://postgres:e2e@127.0.0.1:$PG_PORT/velqu_e2e" \
  "$RUNTIME" --pack "$APP/dist/app.qpack" --port "$SVC_PORT" --log errors &
RT_PID=$!
for i in $(seq 1 50); do
  if http GET "http://127.0.0.1:$SVC_PORT/health/live" "" status:200 body:ok >/dev/null 2>&1; then break; fi
  sleep 0.2
  [ "$i" = 50 ] && fail "runtime never became ready"
done
echo "runtime ready on :$SVC_PORT (pid $RT_PID)"

step "typed failure: create without token -> 401 problem+json"
http POST "http://127.0.0.1:$SVC_PORT/notes" \
  '{"title":"nope"}' status:401 ctype:problem+json body:unauthorized || fail "401 policy failure shape wrong"

step "typed failure: out-of-range pageSize -> 422 validation"
http GET "http://127.0.0.1:$SVC_PORT/notes?pageSize=999" "" status:422 || fail "422 query validation shape wrong"

step "typed failure: malformed id -> 422 params validation"
http GET "http://127.0.0.1:$SVC_PORT/notes/not-an-id" "" status:422 || fail "422 params validation shape wrong"

step "happy path: create with SQL metacharacters in the title -> 201 (parameterized)"
CREATED="$(TOKEN="$TOKEN" "$BUN" -e '
    const res = await fetch(process.argv[3], { method: "POST", headers: { "content-type": "application/json", authorization: `Bearer ${process.env.TOKEN}` }, body: process.argv[4] });
    const j = await res.json();
    console.log(j.id ?? "");
    if (res.status !== 201) { console.error(`status ${res.status}: ${JSON.stringify(j)}`); process.exit(1); }
  ' x x "http://127.0.0.1:$SVC_PORT/notes" '{"title":"O'"'"'Brien; DROP TABLE notes;--","body":"injection probe"}')" \
  || fail "create failed"
echo "created id: $CREATED"
[ -n "$CREATED" ] || fail "create returned no id"

step "happy path: list page 1 sees the row (pagination round trip)"
http GET "http://127.0.0.1:$SVC_PORT/notes?page=1&pageSize=10" "" status:200 body:'"total":1' \
  || fail "list failed"

step "happy path: read-back shows the title VERBATIM (injection stored safely)"
http GET "http://127.0.0.1:$SVC_PORT/notes/$CREATED" "" status:200 body:'DROP TABLE notes' \
  || fail "read-back failed"

step "typed failure: unknown (pattern-valid) id -> 404 problem"
http GET "http://127.0.0.1:$SVC_PORT/notes/nts_000000000000" "" status:404 ctype:problem+json \
  || fail "404 shape wrong"

step "typed failure: delete with wrong token -> 401"
TOKEN="wrong-token" http DELETE "http://127.0.0.1:$SVC_PORT/notes/$CREATED" "" status:401 \
  || fail "delete 401 shape wrong"

step "happy path: delete -> 200, then re-get -> 404"
TOKEN="$TOKEN" http DELETE "http://127.0.0.1:$SVC_PORT/notes/$CREATED" "" status:200 body:'"deleted":true' \
  || fail "delete failed"
http GET "http://127.0.0.1:$SVC_PORT/notes/$CREATED" "" status:404 || fail "re-get after delete not 404"

step "typed failure: live database loss -> declared 503 (fail closed, no leak)"
"$DOCKER" stop -t 1 "$PG_NAME" >/dev/null 2>&1 || true
sleep 0.5
TOKEN="$TOKEN" http POST "http://127.0.0.1:$SVC_PORT/notes" \
  '{"title":"during outage"}' status:503 body:'database unavailable' \
  || fail "db-loss 503 shape wrong"

step "graceful stop"
kill -TERM "$RT_PID"
wait "$RT_PID" 2>/dev/null || true
RT_PID=""

echo
echo "NOTES-OK"
