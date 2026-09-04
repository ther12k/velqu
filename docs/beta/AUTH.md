# Auth: policies, sessions, and the JWT reference package

Authentication in Velqu is route-level **policy**, not ambient middleware.
A policy runs inside the runtime before a handler, receives the request
headers, and yields either a typed failure (a declared problem status such
as 401/403) or a typed session the handler reads. Because the policy is
part of the route declaration, its failure statuses appear in the schema
contract, the OpenAPI artifact, the contract lock, and Treaty client types.

## Identity comes from verified tokens, not headers

The runtime treats ingress headers like `Host` and `X-Forwarded-*` as
ordinary request data — never as client identity. Client identity exists
only after a policy verifies a credential (for example a bearer token).
A reverse proxy cannot grant identity by setting a header.

## The proof fixture: login and a protected route

`examples/proof` ships a small educational auth fixture (credentials are
compile-time fixtures, not a user database). Build and run it:

```bash
bun packages/cli/src/index.ts build --project examples/proof
./target/release/velqu-runtime --pack examples/proof/dist/app.qpack --port 8080
```

Login issues a signed reference token; the protected profile route is
guarded by the JWT-like policy:

```bash
TOKEN=$(curl -sf -X POST http://127.0.0.1:8080/auth/login \
  -H 'content-type: application/json' \
  -d '{"username":"ada","demoSecret":"jwt-reference-demo-secret"}' | jq -r .token)

curl -sf http://127.0.0.1:8080/auth/profile -H "authorization: Bearer $TOKEN"
# → {"scope":"items:read profile:read","userId":"usr_ada"}
```

Declared failures are typed responses, not exceptions:

```bash
curl -s -o /dev/null -w '%{http_code}\n' -X POST http://127.0.0.1:8080/auth/login \
  -H 'content-type: application/json' -d '{"username":"ada","demoSecret":"wrong"}'
# → 401

curl -s -o /dev/null -w '%{http_code}\n' http://127.0.0.1:8080/auth/profile
# → 401  (missing bearer token)
```

## The JWT reference package

`@velqu/capability-auth-jwt` is the first-party bearer-token policy
reference (BETA-005). It is deliberately narrow: **HS256 only** (compact
JWS, `typ: "JWT"`), with five fail-closed verification gates in order —
structure, algorithm (no `none`, no key-type dispatch, so algorithm
confusion is impossible by construction), header field allowlist
(key-injection fields like `jku`/`x5u` reject), timing-safe HMAC-SHA-256
signature, and claims decoding. Expiry is required; `nbf`/`iss`/`aud`
are enforced when configured, with bounded clock skew. See
`packages/capability-auth-jwt/README.md` for the gate-by-gate contract
and `docs/beta/CAPABILITY_AUTHORS.md` for writing your own capability.

Policies declare their failures, so a 401 from a JWT policy is a typed
value in the route's response union — Treaty clients see it at compile
time.

## Notes and limits

- Session/secret handling in the runtime is redacted by construction
  (`SecretString`: `Debug`/`Display` never reveal values).
- The proof fixture is an educational fixture; real deployments must
  supply their own credential storage and key management.
- This is a non-SLA public beta; nothing here is a production-readiness
  claim. Performance is not implied by these examples — measured claims
  require matched raw evidence under `benchmarks/raw/`.

## Verify

From the repository root (after building the proof pack):

```bash
bun test examples/proof
bun run typecheck
```

The proof auth routes and the JWT package are covered by these suites
(gate-by-gate token verification tests live with the package).
