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

## Performance posture

Verification is a single HMAC over the token — O(token length), no
verification cache in this profile. Caching (bounded, hash-keyed) is a
documented future decision (BETA-005-C), never an implicit one.
