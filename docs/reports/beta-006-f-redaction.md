# BETA-006-F — Redaction (observability)

Status: **ENFORCED** (the log allowlist is now code, tested).

## What was built

`completion_log_json` — the structured `request.complete` document is
built by a pure function whose field set **is** the allowlist:

`level, event, requestId, routeId, method, path, status, bodyBytes,
stage, durationMs, traceId (optional)`.

- **No field exists** for header values, query strings, claim material,
  request bodies, or client addresses — they cannot appear because
  there is nowhere to put them.
- **Query-string defense**: `path` is re-stripped of any `?…` suffix
  inside the builder (the caller already passes `uri.path()`); a test
  proves a secret-bearing query string never reaches the document.
- **Trace id** (BETA-006-E) is bounded printable ASCII validated at
  extraction; omission omits the field.

## Redaction audit (test-enforced)

4 tests: field allowlist (exact key set, sorted), query-strip defense
(a `?token=SECRET` path never leaks the material), trace-id absence
semantics, status-driven level. Combined with BETA-005-E's no-secret
sweep on the auth capability and the metrics schema carrying only
ids/classes/durations (BETA-006-A), the observability surface is
redaction-audited end to end.

## Gates

- `cargo test -p velqu-runtime` -> 63 pass (4 new redaction tests)
- fmt / clippy (`-D warnings`) -> clean
- `./scripts/verify` -> ALL PASS; `bun test` -> 434+ pass / 0 fail
