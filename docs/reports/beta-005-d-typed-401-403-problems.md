# BETA-005-D — Typed 401/403 Problems (JWT capability)

Status: **ENFORCED** (total failure mapping, deterministic tests).

## What was built

`packages/capability-auth-jwt/src/problems.ts`:

- **`authProblem(reason)`** — total mapping over the closed-set failure
  reasons to RFC 9457 problem documents:
  - 401 for all authentication failures (missing/malformed token,
    unapproved algorithm, header injection, signature mismatch, expiry,
    issuer/audience failures) with
    `WWW-Authenticate: Bearer error="invalid_token"` (RFC 6750 §3).
  - 403 for the authorization failure `insufficient-scope` with
    `WWW-Authenticate: Bearer error="insufficient_scope"` (RFC 6750
    §3.1).
  - Unknown reasons collapse into the generic invalid-token 401 — the
    set is closed; no invented types.
- **`authenticateBearer(header, secret, options)`** — the whole bearer
  flow: missing token → typed 401; profile gates → typed 401; claims
  checks → typed 401; success returns claims.
- **`requireScope(claims, needed)`** — explicit authorization step: a
  valid-but-underprivileged token yields **403** `insufficient-scope`
  (space-delimited scope per RFC 8693), never 401 — the two failure
  classes are deliberately distinct.

Token material never appears in a problem document (static details
only; E owns redaction end to end).

## Tests (9 new, deterministic)

401 mapping with WWW-Authenticate + typed URIs across six reasons; 403
insufficient-scope mapping; closed-set collapse probe; full bearer flow
(valid/missing/malformed/expired); algorithm-confused token → 401
algorithm-not-approved; scope pass (single/among-many) and 403 failures
(insufficient, missing claim). Package total 44 pass.

## Gates

- `bun test packages/capability-auth-jwt` -> 44 pass / 0 fail
- `bun test` -> 428 pass / 0 fail (66 files)
- typecheck / fmt / clippy -> clean
- `./scripts/verify` -> ALL PASS
