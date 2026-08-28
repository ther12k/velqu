# M28-002-C — DNS / TLS / Pool Behavior of the Selected Stack

Behavioral verification of the M28-002-A-selected stack (hyper 1 +
hyper-util client-legacy + hyper-rustls webpki-roots) before M28-003
implements the production pool. Executed as Rust probes against local
mock origins in the standalone spike workspace
(`benchmarks/stack-spike/spike-hyper/tests/stack_behavior.rs`) — the
Velqu dependency graph is untouched.

## Results — 6/6 PASS

| # | Probe | Verifies | Result |
| --- | --- | --- | --- |
| 1 | `pool_reuses_connection_for_sequential_same_origin_requests` | Pool keepalive: 2 sequential requests to one origin are served over exactly **1 TCP accept** — connection reuse works, the core behavior M28-003 bounds. | PASS |
| 2 | `pool_dials_a_separate_connection_per_origin` | Pool keying: distinct origins get distinct connections (1 accept each). | PASS |
| 3 | `dns_hostname_resolution_reaches_loopback_origin` | DNS: hostname (`localhost`) requests resolve through the system resolver and reach the loopback origin — the resolution path M28-008-A wraps with the ADR-0033 §3 validate-after-resolve pipeline. | PASS |
| 4 | `dns_unresolvable_host_fails_typed_and_fast` | DNS failure: reserved `.invalid` host errors in < 10 s — typed connect error, no hang. | PASS |
| 5 | `tls_self_signed_certificate_is_rejected_fail_closed` | **TLS policy (ADR-0033 §6)**: a live rustls TLS server presenting a self-signed certificate for `127.0.0.1` is **rejected** by the webpki-roots-only client, fast (< 10 s). Root-of-trust validation is mandatory and the policy-shaped connector exposes **no bypass knob** — the exact fail-closed direction the ADR requires. | PASS |
| 6 | `streaming_body_supports_bounded_prefix_and_early_drop` | Backpressure/cancellation: a 1 MiB streamed response can be read as bounded frames and dropped mid-stream; the server observes the cancel. The semantics M28-006 builds on. | PASS |

## Findings for M28-003+

1. **Pool reuse is real but unbounded by default** — the legacy pool
   reuses connections indefinitely; M28-003 must set idle timeout,
   per-host max, and total max explicitly (already required by
   ADR-0033 §7 / M28-003-B).
2. **TLS rejection is structural, not configurable** — with
   `with_webpki_roots()` there is no dangerous-config path; the only way
   to trust a non-webpki CA is a different builder, which the policy
   layer will never construct.
3. **DNS resolution is opaque to the client** — the connector resolves
   internally; M28-008-A's validate-after-resolve pipeline must replace
   the default `HttpConnector` resolve step (custom resolver service) so
   every resolved address passes ADR-0033 §2 classification *before*
   dial, satisfying §3's connect-to-validated rule.
4. **Immediate-close EOF during TLS handshake**: the legacy connector's
   behavior on a server that accepts and instantly closes proved
   non-deterministic in this environment (probe could hang); dropped as
   a spike probe. Transport-error classification gets deterministic
   coverage in M28-003/M28-006 tests with real deadlines and the
   production pool.

## Test dependencies added (spike workspace only)

`rcgen 0.13` (self-signed cert generation), `tokio-rustls 0.26` (ring),
`rustls 0.23` (ring, no defaults), `rustls-pki-types 1` — dev-dependencies
of the standalone spike crate; production dependency graph unchanged
(verified by `./scripts/verify`).

## Reproducing

```bash
cd benchmarks/stack-spike/spike-hyper
cargo test --test stack_behavior
```
