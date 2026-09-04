# BETA-008-B — Forwarded Header Policy

## What changed

The runtime now makes the ingress trust boundary explicit in `q-http`:

- `UNTRUSTED_INGRESS_HEADERS` is a closed list of `X-Forwarded-For`,
  `X-Forwarded-Proto`, `X-Forwarded-Host`, `X-Forwarded-Port`,
  `X-Forwarded-All`, RFC 7239 `Forwarded`, and `Host`.
- `is_untrusted_ingress_header` is case-insensitive and identifies headers
  that must never become identity, authentication, authorization, scheme, or
  routing input. They remain readable as route-declared application data.
- The TCP peer returned by `TcpListener::accept` is carried on
  `NativeRequest`; `connection_peer` has no header-derived fallback. The
  runtime pipeline deliberately does not put peer identity into the JS
  request ABI.
- Existing route selection remains method + path only. Existing header
  materialization continues to honor route declarations, so this packet does
  not silently remove application data or pull readiness/drain work forward.
- ADR-0034 and reverse-proxy deployment docs now state the policy and signed
  application-token alternative.

## Tests

- `q-http::tests::forwarded_headers_are_data_not_identity` checks all seven
  case-insensitive distrust names, ordinary headers remain ordinary, and a
  forged `x-forwarded-for`/`host` pair cannot change the connection peer.
- `q-runtime::tests::forwarded_headers_are_ordinary_data_and_never_identity`
  is a black-box runtime test: forged forwarded metadata cannot authenticate
  the protected `/users/usr_1` route; a declared Authorization token still
  authenticates regardless of those forged headers.
- Existing `q-capabilities` test
  `forwarded_headers_are_never_trusted_identity` remains green and pins the
  six ADR-0034 forwarding names used by capability policy.

## Runbook / examples

- `docs/beta/DEPLOYMENT-REVERSE-PROXY.md` has the BETA-008-B policy section,
  including peer-vs-header identity, Host non-routing, and signed token guidance.
- The BETA-008-A runbook remains the deployment checklist; this packet adds no
  native TLS, header normalization, or readiness/drain behavior.

## Gates

- `cargo test -p q-http` — pass (8 lib tests)
- `cargo test -p q-bridge` — pass (11 lib tests)
- `cargo test -p velqu-runtime` — pass (101 lib + 36 runtime conformance)
- `cargo fmt --all --check` — pass
- `cargo clippy --workspace --all-targets -- -D warnings` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS
- `bun test` — 434 pass / 0 fail
- `bun run typecheck` — pass

## Disclosures

- Forwarded headers remain readable as data when explicitly declared. This is
  not trust; applications must validate/authorize their own declared values.
- The connection peer is captured natively but intentionally not exposed as a
  general-purpose JS identity field. Signed application-layer identity remains
  the supported proxy boundary mechanism.
- Standing CI disclosure: repository verify workflows have stalled/failed with
  zero executed steps at PR creation since roughly #714; local gates above are
  the acceptance evidence.
