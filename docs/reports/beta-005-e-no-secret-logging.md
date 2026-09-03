# BETA-005-E — No Secret Logging (JWT capability)

Status: **ENFORCED** (sweep test + redaction affordances).

## Guarantee

No string the capability emits — typed errors, RFC 9457 problem
documents, keyring snapshots, verification results — contains token or
secret material. Enforced by a sweep test that constructs every typed
failure with a distinctive secret and asserts neither the secret nor a
signed token appears anywhere in the emitted strings.

## Redaction affordances (`src/redaction.ts`)

For the caller's logging path (safe logging as a one-call affordance,
not a judgment call):

- `redactToken(token)` / `redactAuthorizationHeader(header)` —
  constant-shape markers carrying only shapes and sizes
  (`<jwt redacted; segments=3; bytes=712>`); no prefixes, suffixes, or
  partial material.
- `scrub(text, secrets)` — defense in depth: removes every occurrence
  of supplied secret material from a line.
- `secretFingerprint(secret)` — stable 12-hex, keyed, non-reversible id
  for correlating which configured key was in play without revealing it.

## Tests (6 new, deterministic)

The enforcement sweep (all typed failures × distinctive secret — no
leak); typed errors carry reasons only; token/header/scope marker
shapes; scrub removes all occurrences; fingerprint stability, format,
and divergence. Package total 50 pass.

## Gates

- `bun test packages/capability-auth-jwt` -> 50 pass / 0 fail
- `bun test` -> 434 pass / 0 fail (67 files)
- typecheck / fmt / clippy -> clean
- `./scripts/verify` -> ALL PASS
