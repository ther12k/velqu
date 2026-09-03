# BETA-005-A — One Approved JWT Algorithm/Profile (HS256)

Status: **ENFORCED** (profile gates test-enforced).

## The profile

`packages/capability-auth-jwt` (`@velqu/capability-auth-jwt`, new):
HS256 — HMAC-SHA-256 over the compact JWS serialization with
`typ: "JWT"` — is the single approved algorithm. Verification is a
five-gate fail-closed pipeline:

1. **Structure** — three non-empty base64url segments, ≤ 8 KB.
2. **Algorithm** — header `alg` must equal `"HS256"` exactly; `none`,
   case variants, asymmetric algorithms, missing/non-string `alg` all
   reject **before any signature work**.
3. **Header fields** — only `alg` + `typ`; `jku`/`jwk`/`x5u`/`kid`-
   style key-injection fields reject.
4. **Signature** — HMAC-SHA-256, timing-safe comparison.
5. **Claims** — JSON object, ≤ 4 KB.

**Algorithm confusion is impossible by construction**: there is no
key-type dispatch and no verification path other than HMAC-SHA-256 —
the algorithm gate exists to reject, not to choose an implementation.

## Security tests (14, deterministic)

- RFC 4231 TC2 HMAC vector (base64url) matches.
- `none` / lowercase / case-variant / `RS256` / `ES256` / `PS256` /
  missing / non-string `alg` — all rejected
  `algorithm-not-approved`.
- `jku`/`jwk`/`x5u`/`kid` header injection — `header-field-not-allowed`.
- `typ` ≠ `JWT` — `typ-not-jwt`.
- Tampered payload, wrong secret — `signature-mismatch`.
- Segment-count/empty-segment — `malformed`; non-object claims —
  `claims-not-object`.
- Approved profile round-trips (sign → verify).

## Invalid tokens fail closed

Every gate returns a closed-set typed reason; nothing degrades to a
warning path, a claim-level pass-through, or a best-effort decode.

## Treaty contract

Policy failures are declared through the policy `declares` field
(`401: "unauthorized"`), which is the mechanism the Treaty contract
surface renders (covered by the existing treaty conformance suite:
declared statuses appear in contracts).

## Performance/caching posture

Verification is one HMAC over the token — O(token length), constant-
shape work on failure paths (structural/algorithm gates run before
signing and are cheaper than the MAC). No verification cache exists in
this profile; a bounded hash-keyed cache would be a documented
BETA-005-C decision, never implicit.

## W1/W2/W3

The proof application's `users.get` route (W1) is the JWT policy
consumer; the reference primitives match the RFC 4231 vectors the proof
policy tests pin. Full W1/W2/W3 load runs remain BETA-013/014 scope.
