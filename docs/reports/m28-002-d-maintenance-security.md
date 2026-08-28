# M28-002-D — Maintenance & Security Considerations for the Outbound Fetch Stack

Standing maintenance and security posture for the M28-002-A-selected
outbound stack (hyper 1 + hyper-util 0.1 client-legacy + hyper-rustls
0.27 with webpki-roots/ring), which M28-002-B linked into the production
binary (dormant until M28-003). This document is the maintained record;
future M28 packets update it when they change the stack's surface.

## Ownership & review boundaries

| Concern | Owner | Notes |
| --- | --- | --- |
| Outbound policy (schemes, SSRF, TLS, redirects, timeouts, bodies) | `q_capabilities::fetch_policy` (ADR-0033) — single source of truth | No dial bypasses it; widening is an ADR-level decision |
| Trust model (ingress headers, capability grant) | ADR-0034 | Fetch = `runtime:fetch@1`, compiler-granted |
| Pool bounds (idle, per-host, total) | M28-003 (implements), bounded by ADR-0033 §7 | Default pool is unbounded until M28-003-B lands |
| Address validation pipeline | M28-008-A (implements ADR-0033 §3) | Custom resolver replaces the connector's internal resolve |

## Dependency maintenance policy

- **Pinned, minimal feature sets.** `hyper-rustls` is
  `default-features = false` with `http1`, `ring`, `webpki-tokio`,
  `tls12`; `tokio-rustls`/`rustls` (spike-only dev-deps) are ring-only
  with defaults off. No ambient TLS, no aws-lc-rs (single provider =
  one audit surface). The spike's TLS-probe dependency confusion (two
  providers → process-level panic) demonstrated why: features are
  unified per graph, so both sides must stay ring-only.
- **Upgrade rule.** hyper / hyper-util / hyper-rustls / rustls / ring
  upgrades happen in dedicated packets that re-run: full
  `./scripts/verify`, the M28-002-C behavior probes, and (for rustls or
  ring) a fresh binary-size/startup measurement against the M28-002-B
  baseline. Blind `cargo update` across the workspace is not a
  maintenance action.
- **CVE posture.** rustls and ring have historically fast,
  well-documented advisory flow (RUSTSEC). Reviewer checklist on any
  security advisory touching rustls/ring/hyper: (1) is the affected
  path reachable given our policy (most rustls advisories concern
  features we do not enable — e.g., client-cert auth, dangerous
  verifier configs), (2) if reachable, treat as P0 and ship the pinned
  bump as its own packet with the C-probes re-run, (3) record the
  verdict here.
- **webpki-roots refresh.** The bundled Mozilla root store is
  compile-time data; refresh rides rustls/hyper-rustls version bumps
  (the crate re-exports a refreshed set). Staleness risk is documented
  rather than hidden: if a root CA rotation outruns our pin, HTTPS to
  affected hosts fails closed (typed TLS error) — never silently
  downgraded.

## Security considerations

1. **No ambient configuration.** The stack reads no environment
   variables (no `SSL_CERT_FILE`, no proxies) — deployment cannot
   accidentally change trust roots or add a middlebox (ADR-0033 §5,
   ADR-0034 §3).
2. **No bypass surface.** `with_webpki_roots()` has no
   dangerous-verifier alternative in the constructed path; hostname
   validation is unconditional (M28-002-C probe 5 proves rejection
   end-to-end).
3. **Trust boundary is the network, not the process.** Same-process
   QuickJS runs trusted application code only (ADR-0035); fetch policy
   protects the host network. This document never treats the engine as
   a sandbox.
4. **Dormant-until-wired.** Between M28-002-B and M28-003 nothing dials;
   the linked code path is construction-only (`--fetch-stack-info`).
   Attack surface exists only after M28-003 gates dialing behind the
   capability grant and policy object.
5. **Known limitation (disclosed).** hyper-util's legacy connector may
   retry transport-class failures (observed as non-deterministic
   immediate-close handling during the C-probes). M28-003/M28-006 must
   make retry behavior explicit and bounded in the production client
   (single dial attempt per validated address; retries are policy
   decisions, not connector accidents).

## Maintenance checklist (per stack-touching packet)

- [ ] `./scripts/verify` ALL PASS
- [ ] `cargo test --test stack_behavior` (spike) 6/6
- [ ] Binary size + cold-start delta recorded against M28-002-B baseline
- [ ] No new feature flags on hyper-rustls/rustls/tokio-rustls without an ADR note here
- [ ] RUSTSEC advisories for rustls/ring/hyper reviewed; verdict recorded
