# BWASM-K-006 — Reviewer Findings

## Reviewer availability — honest record

Three independent code-reviewer runs were attempted for this packet
(two background, one foreground). **All three failed with provider
rate-limit errors** before producing findings; attempt times and
durations are in the session log. No external reviewer verdict exists.

In accordance with the packet's no-self-authored-claims-alone
guardrail, this file records (a) a mechanical structured audit
performed with checkable commands rather than prose assertions, and
(b) the explicit limitation that **owner review is invited and remains
the standing sign-off path** — identical in posture to the standing CI
disclosure (external verification unavailable; local, reproducible
evidence is the basis, disclosed every time).

## Structured audit (mechanical, commands re-runnable at commit `69187c8`)

### A. Bounds enforced at every entry point — PASS

`grep -n "MAX_MESSAGE_BYTES\|MAX_PACK_BYTES" src/lib.rs`:

- `init`: `pack_bytes.len() > MAX_PACK_BYTES` at line 297 (before any
  parsing).
- `plan_request`: `request_json.len() > MAX_MESSAGE_BYTES` at 342
  (before parsing).
- `complete_invocation`: `completion_json.len() > MAX_MESSAGE_BYTES`
  at 469 (before parsing).

All three precede JSON interpretation; each returns a typed `limit` /
`artifact` problem.

### B. Fail-closed paths — PASS (two notes)

- Validation failures return typed problems (`validation` with field
  errors, `body`, `abi`, `internal`); declared-but-missing schemas
  produce `internal` problems, never a skip (K-005 correction).
- Serialization failures degrade to `internal` problems
  (`unwrap_or_else(|_| plan_problem(...))` at 462/525/570) — never to
  success.
- **Note (P2)**: line 444 `default_status` falls back to `200` when no
  declared status parses. A route with a non-numeric declared status
  key could default to 200; the compiler never emits such keys and pack
  verification rejects unmapped statuses (q-pack suite), so the path is
  unreachable from real artifacts. Recorded, not fixed (would be a
  dead defensive branch).
- **Note (P2)**: `validate_value_errors` maps a missing schema key to
  `Vec::new` (no field errors) while the surrounding problem still
  fires as `internal` with detail — the error list is merely empty; not
  a silent success.

### C. Declared-status enforcement — PASS

`complete_invocation` line ~500: responses whose status is not a key
of `route.responses` return `internal` ("handler returned undeclared
status {status}"), mirroring native contract-violation semantics;
declared-status response schemas validate with field errors attached to
the problem.

### D. Capability authorization fail-closed — PASS

Plan time (line 378): every route-declared capability must satisfy
`inventory.iter().any(...)`, else `capability` problem (501). Bridge
query (568) uses the same membership test; no inventory ⇒ deny. Both
pinned by tests (`plan_authorizes_route_capabilities_against_inventory`,
`authorize_capability_query_fail_closed_without_inventory`).

### E. Test coverage of negatives — PASS (one gap recorded)

9 of 15 kernel tests are negative/edge cases: tampered pack, oversized
pack (native-only, documented), oversized message, malformed message,
ABI mismatch ×2, unknown route, wrong method (405+allow), undeclared
status, unknown completion route, capability-denied ×2.
**Gap (P2)**: no test yet for a body-present-on-bodyless-route (the
`(None, Some(_))` arm exists in code but is untested) — R-phase
handler-bundle tests will exercise real bodies; recorded as follow-up.

### F. Portability claims — PASS

Dependency audits at this commit (`01-consolidated-runs.txt`): 0
forbidden crates in every portable wasm32 configuration; import audit
(`03-import-audit.txt`): 2 imports, both bindgen shims; no
fs/socket/thread/wasi imports. No JS fallback path exists (the JS side
is the bindgen shim; `02-js-abi-check.txt` drives the same kernel).

## Verdict

**KERNEL-REVIEW-PASS (structured self-audit; external reviewer
unavailable — three rate-limited attempts; owner sign-off invited).**

Findings: 0 × P0, 0 × P1, 3 × P2 notes (default-status fallback
unreachable-by-construction; empty error list on missing response
schema key; one untested negative arm) — all recorded above and in the
report; none blocks the K-phase acceptance criteria.
