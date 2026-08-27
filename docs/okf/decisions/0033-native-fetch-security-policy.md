---
type: Architecture Decision Record
title: ADR-0033 Native Fetch — Schemes, Redirects, SSRF Controls, Proxy, TLS, Timeouts, Compression, Body Limits
status: accepted
date: 2026-08-28
implements: ADR-0018 (M2.8 native fetch), ADR-0010 (capability principle), ADR-0012 (evidence before performance claims)
---

# ADR-0033: Native Fetch Security Policy — Schemes, Redirects, SSRF, Proxy, TLS, Timeouts, Compression, Body Limits

## Context

M2.8 introduces native outbound `fetch` executing inside the single QuickJS
worker. The application bundle is trusted code (ADR-0003: no Node
compatibility; ADR-0010: capability model), but the *network* is not: a fetch
capability turns every handler into a potential pivot into the host network.
Before any client stack is selected (M28-002) or any connection is dialed
(M28-003), the trust boundary, defaults, and non-goals must be frozen so the
implementation is constrained by policy rather than by afterthought.

This ADR defines the security policy object
(`q_capabilities::fetch_policy`) that every later M28 packet consumes:
scheme allowlist, SSRF address classification, DNS-rebinding controls,
redirect revalidation, proxy posture, TLS roots, timeout layers,
compression behavior, and body limits.

## Decision

### 1. URL schemes: closed allowlist, fail closed

Only `http:` and `https:` (case-insensitive) are fetchable. Everything
else — `file:`, `data:`, `ftp:`, `ws:`, `wss:`, `gopher:`, unknown and
empty schemes — is a typed rejection before any DNS resolution or socket
work. The allowlist is a compile-time constant, not configuration; widening
it is an ADR-level change. WebSocket upgrade is explicitly out of scope for
beta (AGENTS.md constraint 15).

### 2. SSRF: address-class deny-by-default

Every host the runtime may dial is classified by its **resolved IP
address**, never by hostname string:

| Class | Examples | Default |
| --- | --- | --- |
| Public | `8.8.8.8`, `2001:4860::8888` | allowed |
| Private (RFC 1918 / ULA) | `10.0.0.0/8`, `172.16/12`, `192.168/16`, `fd00::/8` | denied |
| Loopback | `127.0.0.0/8`, `::1` | denied |
| Link-local | `169.254.0.0/16`, `fe80::/10` | denied |
| Cloud metadata | `169.254.169.254`, `fd00:ec2::235` | denied (listed explicitly, distinct error) |
| Unspecified / reserved / multicast | `0.0.0.0`, `224.0.0.0/4`, `::`, `100.64/10` | denied |

IPv4-mapped IPv6 addresses (`::ffff:10.0.0.1`) are normalized to IPv4
before classification — mapped forms must not evade the classifier.
Denial is a typed, loggable error naming the class; there is no silent
fallback to "best effort".

**Override posture**: the default constructor offers no escape hatch. A
`FetchPolicy::trusted_loopback_explicit()` exists for opt-in, explicit
local testing (e.g., integration tests dialing a local mock origin) and is
auditable by name at the call site; production serving constructs use
`FetchPolicy::default()` (deny). Private/link-local/metadata behavior is
explicit in both code and errors.

### 3. DNS rebinding: validate-after-resolve, connect-to-validated

TOCTOU rebinding (DNS answers a public IP to the validator, a private IP to
the dialer) is mitigated structurally:

1. resolve the hostname;
2. **every** returned A/AAAA record must pass classification — one bad
   address fails the whole fetch (no partial retry);
3. the connection is dialed only to a **previously validated** address,
   carrying the hostname for TLS SNI/verification. The dialer never
   re-resolves.

M28-008-A implements the resolve-validate-connect pipeline against this
rule; this ADR freezes the policy the pipeline must call.

### 4. Redirects: full revalidation per hop

Redirects are followed only under `RedirectPolicy::Follow { max_hops }`
(max 20, aligned with browser behavior; zero or >20 rejected at
construction). For **every** hop:

- the `Location` target is resolved and classified again from scratch
  (redirect revalidation is required — a public origin redirecting to
  `169.254.169.254` is rejected exactly like a direct request);
- scheme downgrades `https → http` are rejected;
- the hop consumes the same total deadline;
- credentials (`Authorization`, `Cookie`) are stripped on cross-origin
  redirects (origin = scheme + host + port comparison).

`RedirectPolicy::Manual` returns the 3xx response to the caller without
following. Redirect count beyond `max_hops` is a typed failure, never a
silent truncation.

### 5. Proxy: no ambient trust

The runtime reads **no** proxy environment variables (`HTTP_PROXY`,
`ALL_PROXY`, …) by default — ambient proxy trust silently moves the trust
boundary onto an unaudited middlebox and defeats the address classifier
(the proxy, not the policy, chooses the dialed IP). If a proxy is ever
configured it must be an explicit, validated, policy-owned setting; reverse
proxy and outbound trust posture is specified separately in M28-001-B. No
proxy support ships in beta.

### 6. TLS: verified roots, mandatory hostname validation

- TLS 1.2 minimum; TLS 1.3 preferred.
- Root store: bundled **webpki-roots** (Mozilla set) — not the system
  store, keeping behavior identical across containers and reproducible.
- Hostname validation is mandatory and non-disabling; there is no
  `danger_accept_invalid_certs` equivalent anywhere in the policy surface.
- Plain `http:` remains allowed (the public internet still includes it);
  downgrade protection is the redirect rule above.
- Client certificates are out of scope for beta.

### 7. Timeouts: layered, bounded, one total budget

| Layer | Default | Bound |
| --- | --- | --- |
| Total fetch deadline | 30 s | `MAX_FETCH_DEADLINE_MS` = 300 000 (matches `MAX_OP_DEADLINE_MS`, ADR-0030) |
| Connect (TCP) | 10 s | ≤ total |
| TLS handshake | 10 s | ≤ total |
| Redirect budget | shared | total deadline covers all hops |

Zero or over-ceiling deadlines are typed rejections at policy construction
(fail closed before any I/O). The fetch operation is a cancellable native
operation under the ADR-0030 model: abort/deadline propagation is the same
slot/generation machinery timers use.

### 8. Compression: explicit, bounded

Requests send `Accept-Encoding: gzip` only when the caller enables it
(default off — no ambient negotiation). Responses are decompressed only
with a bounded budget: decompressed size is capped at the response body
limit below; over-budget streams abort as `413`-class typed errors, never
OOM (`decompression bomb` non-goal). `br`/`zstd` are out of scope for beta.

### 9. Body limits: bounded both directions

- Request bodies: `MAX_FETCH_REQUEST_BODY_BYTES` = 16 MiB (matches the
  text-encoding buffer bound).
- Response bodies: `MAX_FETCH_RESPONSE_BODY_BYTES` = 16 MiB by default;
  streaming consumption (M28-006) reads bounded chunks and never buffers
  more than the cap regardless of `Content-Length` claims — chunked
  transfers are enforced against the same cap while streaming.

Limits are policy constants; lowering them per-application is allowed via
explicit policy construction, raising them above the compile-time ceilings
is not.

## Threat model (summary)

| Threat | Mitigation (section) |
| --- | --- |
| SSRF into private network | §2 deny-by-default classification |
| Cloud metadata theft | §2 explicit metadata deny list |
| DNS rebinding TOCTOU | §3 validate-after-resolve, connect-to-validated |
| Redirect SSRF pivot | §4 per-hop full revalidation |
| Credential leakage via redirect | §4 cross-origin credential strip |
| Ambient proxy hijack | §5 no environment proxy trust |
| MITM / hostile cert | §6 verified roots + hostname validation |
| Slowloris / hung origin | §7 layered deadlines, one budget |
| Decompression bomb | §8 bounded decompression |
| Memory exhaustion via response | §9 bounded bodies, enforced while streaming |
| Scheme confusion (`file:`, `data:`) | §1 closed allowlist |
| IPv4-mapped evasion | §2 normalization before classification |

Trusted-code assumption: the JS application is *not* the adversary for
memory-safety purposes (ADR-0003 posture, same-process QuickJS runs trusted
application code only — AGENTS.md constraint 14); the network is. This
packet's controls protect the **host network**, not the process interior.

## Non-goals

- WebSockets / SSE (post-beta, AGENTS.md constraint 15).
- Client certificates, mutual TLS.
- HTTP/2 upstream, `br`/`zstd` decompression.
- SOCKS or environment-proxy support.
- Caching, conditional requests (`ETag`) — none in beta.

## Consequences

- `q_capabilities::fetch_policy` is the single policy source of truth;
  M28-002 (stack), M28-003 (pooling/DNS/TLS), M28-006 (streaming),
  M28-007 (redirects), M28-008 (address validation) consume it, never
  re-derive it.
- The security test matrix in `fetch_policy.rs` pins every rule above as
  unit tests; the M28-001-V verify packet maps guardrails to them.
- Any future widening (schemes, proxy, limits) is an ADR-level decision
  with a new security review — never a constant flip inside a packet.

## Status

Accepted (M28-001-A). Tests in
`crates/q-capabilities/src/fetch_policy.rs`.
