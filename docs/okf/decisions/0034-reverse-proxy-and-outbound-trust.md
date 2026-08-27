---
type: Architecture Decision Record
title: ADR-0034 Reverse-Proxy Ingress and Outbound Fetch Trust
status: accepted
date: 2026-08-28
implements: ADR-0033 (native fetch security policy), ADR-0010 (capability model), ADR-0029 (capability identity)
---

# ADR-0034: Reverse-Proxy Ingress and Outbound Fetch Trust

## Context

ADR-0033 froze the per-request outbound policy (schemes, SSRF, TLS,
redirects, timeouts, bodies). Two trust boundaries remain unspecified and
must be frozen before any fetch client ships:

1. **Outbound**: when is application code allowed to open network
   connections at all, and whose policy governs the dial?
2. **Ingress**: the runtime is commonly deployed behind a reverse proxy
   (TLS termination, load balancing). Which inbound signals does the
   runtime trust, and which are attacker-controlled headers?

## Decision

### 1. Outbound fetch is a declared capability, not an ambient global

`fetch` is the capability `runtime:fetch@1` under the ADR-0029 identity
system. It follows the same rules as every M27 capability:

- The compiler grants `fetch` only to routes that actually reference it
  (the M27-002-D detection model: `ctx.native.fetch`, destructured
  `fetch`, aliased forms); unknown grants fail the build.
- The pack inventory records `runtime:fetch@1` only when some route
  declares it; pruned packs carry no fetch at all.
- Routes without the grant see **no** fetch surface — `ctx.native.fetch`
  is `undefined` and (until the pack declares fetch) the global `fetch`
  stays `undefined`, exactly as pinned by the M27-010-D absent-API test.
- There is no ambient global `fetch` for undeclared routes, ever.

### 2. Outbound trust is runtime-owned, not application-owned

Every dial flows through one `FetchPolicy::default()` (ADR-0033)
constructed by the host. Applications **cannot** widen trust in beta:
no policy configuration is exposed to JS, so no handler can whitelist a
private range, disable hostname validation, or raise body ceilings.
Tightening per deployment is a host/runtime concern, not a pack concern.

### 3. Reverse proxy: forwarded headers are never identity

The runtime does not trust `X-Forwarded-For`, `X-Forwarded-Proto`,
`X-Forwarded-Host`, `X-Forwarded-Port`, `X-Forwarded-All`, or the RFC 7239
`Forwarded` header for **any** decision:

- They are ordinary request headers — readable as data, never used as
  identity, authentication, or authorization input.
- If a route wants the peer address, it gets the **connection** peer, not
  a header claim. (The reverse proxy may know the real client; the
  runtime does not take the proxy's word for it.)
- A proxy that needs identity forwarding must use signed tokens at the
  application layer — out of runtime scope.

### 4. TLS termination is a deployment concern

The runtime serves plain HTTP on its listener; HTTPS at the edge is the
reverse proxy's job. The runtime neither inspects `X-Forwarded-Proto` to
"recover" a scheme (§3) nor terminates TLS itself in beta. Outbound TLS
(ADR-0033 §6) is unchanged: verified roots, mandatory hostname checks.

### 5. Default loopback bind

`velqu-runtime` binds `127.0.0.1` by default (`--addr` default). Exposing
the listener beyond the host is an explicit operator action; combined
with §3–§4 the expected deployment is: edge proxy (TLS, access control)
→ loopback Velqu runtime. Direct external exposure is supported but
takes on header-trust and TLS consequences the operator owns.

### 6. Host header is untrusted routing input

Routing selects by method + path only; `Host` never selects a route,
authority, or virtual host. A forged `Host` cannot redirect traffic to a
different application surface because no decision depends on it.

## Threat model additions

| Threat | Mitigation |
| --- | --- |
| Undeclared route opens sockets | §1 capability grant; pruned inventory; absent-API fail closed |
| Handler widens own network trust | §2 runtime-owned policy, no JS config surface |
| `X-Forwarded-For` spoofing for authz | §3 headers never identity; peer = connection |
| Proxy scheme confusion (`X-Forwarded-Proto`) | §3/§4 not trusted; TLS is edge concern |
| Direct exposure of listener | §5 loopback default, explicit opt-in |
| Host-header routing confusion / cache poisoning | §6 Host never routes |

## Non-goals

- Signed identity forwarding between proxy and runtime.
- Runtime TLS termination (GA-track discussion, not beta).
- `X-Forwarded-*` normalization or logging helpers.
- Per-route fetch policy overrides.

## Consequences

- `q_capabilities::fetch_policy` pins the capability identity
  (`FETCH_CAPABILITY`) and the forwarded-header distrust list as tested
  data; M28-004 (fetch surface) consumes the identity, and ingress code
  reviews check the distrust list.
- Compiler capability detection gains a `fetch` grant form in M28-004;
  until then the absent-API pins hold (no fetch global).
- The M28-001-V verify packet maps these sections to the tests below.

## Status

Accepted (M28-001-B). Tests in
`crates/q-capabilities/src/fetch_policy.rs`.
