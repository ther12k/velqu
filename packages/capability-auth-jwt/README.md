# @velqu/capability-auth-jwt

JWT bearer policy reference — **one approved algorithm profile**:
HS256 (HMAC-SHA-256, compact JWS serialization, `typ: "JWT"`).

## Fail-closed verification gates (in order)

1. **Structure** — exactly three non-empty base64url segments, ≤ 8 KB.
2. **Algorithm** — decoded header must declare exactly `"alg": "HS256"`.
   `none`, case variants, asymmetric algorithms, and missing `alg` are
   rejected before any signature work: there is no key-type dispatch and
   no verification path other than HMAC-SHA-256, so **algorithm confusion
   is impossible by construction**.
3. **Header fields** — only `alg` and `typ` (=`"JWT"`) are allowed;
   `jku`/`jwk`/`x5u`/`kid`-style key-injection fields reject the token.
4. **Signature** — HMAC-SHA-256, timing-safe comparison.
5. **Claims** — must decode to a JSON object (≤ 4 KB).

Expiry/audience/issuer validation and typed 401/403 problem mapping are
layered on top (BETA-005-B/C). The declared policy failure (401
`unauthorized`) appears in the Treaty contract via the policy's
`declares` field.

## Claims validation: expiry / audience / issuer (BETA-005-C)

```ts
import { verifyJwtWithClaims } from "./src/claims";

const res = verifyJwtWithClaims(token, secret, {
  expectedIssuer: "https://issuer.example",
  expectedAudience: "api://demo",
  clockSkewMs: 5_000,       // default 5s, ceiling 60s
  now: Date.now(),          // injectable clock (deterministic tests)
});
```

- `exp` is **required** (RFC 7519 NumericDate, seconds) — eternal
  tokens do not pass; expiry is skew-tolerant (default 5s).
- `nbf`, when present, is enforced with the same skew.
- `iss`/`aud` are enforced only when the caller configures expected
  values — and a token that *omits* the claim then fails typed
  (`missing-iss` / `missing-aud`); configured expectations cannot be
  skipped by claim omission.
- All failures are typed (`token-expired`, `issuer-mismatch`,
  `audience-mismatch`, ...); the clock is injectable for deterministic
  tests.

## Typed 401/403 problems (BETA-005-D)

Every auth failure maps to an RFC 9457 problem document with a
closed-set `type` URI and the right status class:

- **401** — authentication failures (missing/malformed token, unapproved
  algorithm, signature mismatch, expired, wrong issuer/audience), each
  with `WWW-Authenticate: Bearer error="invalid_token"` (RFC 6750).
- **403** — authorization failure (`requireScope`): a valid token
  without the needed scope, with
  `WWW-Authenticate: Bearer error="insufficient_scope"`.

`authenticateBearer(header, secret, claimsOptions)` composes the whole
flow; unknown reasons collapse into the generic invalid-token 401
(closed set — no invented types). Token material never appears in a
problem document.

## Key loading and rotation (BETA-005-B)

`JwtKeyring` manages HS256 secrets with caller-driven hooks — the
capability never fetches secrets itself and never logs them:

```ts
const ring = await JwtKeyring.load(loadKeysFromSecretStore); // loading hook
ring.rotate({ id: "key-2026-02", secret: "..." }); // admit new signing key
// old keys keep verifying during the overlap window (no kid in tokens —
// verification tries each active key, bounded by MAX_KEYRING_KEYS = 8)
ring.retire("key-2026-01");                       // stop verifying an old key
await ring.refresh(loadKeysFromSecretStore);      // atomic full reload
```

Loading and refresh are validated fail-closed (empty sets, duplicate
ids, oversize rings, malformed shapes are typed rejections); a failed
refresh leaves the previous ring untouched. Signing always uses the
current key; snapshots expose **ids only** — secrets never appear in
errors, snapshots, or logs.

## Performance posture

Verification is a single HMAC over the token — O(token length), no
verification cache in this profile. Caching (bounded, hash-keyed) is a
documented future decision (BETA-005-C), never an implicit one.
