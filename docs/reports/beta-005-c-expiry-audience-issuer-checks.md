# BETA-005-C — Expiry/Audience/Issuer Checks (JWT capability)

Status: **ENFORCED** (typed claims validation, deterministic tests).

## What was built

`packages/capability-auth-jwt/src/claims.ts`:

- **`exp` required** (RFC 7519 NumericDate, seconds): a token without a
  numeric `exp` fails typed (`missing-exp` / `exp-not-number`) —
  eternal tokens do not pass. Expiry is skew-tolerant (default 5s,
  ceiling 60s; skew outside bounds is itself a typed rejection).
- **`nbf`** enforced when present (numeric, skew-aware).
- **`iss`** enforced only when the caller configures an expectation;
  then a token *omitting* `iss` fails typed (`missing-iss`) — a
  configured expectation cannot be skipped by claim omission.
  Mismatch → `issuer-mismatch`.
- **`aud`** same policy for `expectedAudience`: string or array-of-
  strings containment; omission → `missing-aud`; mismatch →
  `audience-mismatch`.
- **Injectable clock** (`now`) — deterministic tests; production
  callers use `Date.now()`.
- **Composition**: `verifyJwtWithClaims(token, secret, options)` —
  profile gates (A) first, claims validation (this packet) second;
  signature failures still take precedence.

## Security posture

Typed, closed-set failures; token material never appears in errors.
The skew default (5s) and ceiling (60s) are documented; both are
caller-visible configuration, never implicit state.

## Tests (8 new, deterministic)

Valid/expired exp, missing/non-numeric `exp`, skew tolerance and its
ceiling, `nbf` future/malformed, `iss` match/mismatch/missing/
unconfigured, `aud` string/array/mismatch/missing, composition with
profile gates (fresh passes, expired fails typed, forged signature
wins). Package total 35 pass.

## Gates

- `bun test packages/capability-auth-jwt` -> 35 pass / 0 fail
- `bun test` -> 419 pass / 0 fail (65 files)
- typecheck / fmt / clippy -> clean
- `./scripts/verify` -> ALL PASS
