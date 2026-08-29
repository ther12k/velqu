# M28-GATE Review — Native Outbound Fetch

Milestone exit decision for M2.8 (Native Outbound Fetch).

## Milestone Decision: PASS

All 11 parent tasks (`M28-001` through `M28-011`) are complete, verified with source-backed evidence, and squash-merged to master.

### Parent Task Dependency Closure
1. **M28-001 (Accept Fetch/TLS/Redirect/SSRF Security ADRs)** — PRs #913–#914: ADR-0033 (fetch security policy: scheme allowlist, SSRF classes, rebinding, redirects, TLS, timeouts, compression, body limits), ADR-0034 (forwarded headers never identity; loopback bind default; Host never routes), ADR-0035 (same-process trusted-code assumption, `TRUSTED_CODE_ASSUMPTION` pinned).
2. **M28-002 (Select Native HTTP Client Stack)** — PRs #919–#920: hyper 1 + hyper-util client-legacy + hyper-rustls (webpki-roots, ring, HTTP/1, TLS 1.2) selected from matched spike evidence; stack linked dormant (`fetch_stack.rs`, +915 KB binary, +0.45 ms cold-start, within budget).
3. **M28-003 (Connection Pooling, DNS, TLS)** — PRs #925–#926: lazy `FetchPool` (`OnceLock` client, permit-based active bounds, pool/idle timeouts, webpki TLS mandatory with negative tests, `drain_shutdown` budget).
4. **M28-004 (Request/Response/Headers subset)** — PRs #931–#932: WinterTC/WHATWG fetch subset in the QuickJS prelude — `fetch`, `Request`, `Response`, `Headers`, `Response.json()`, lazy headers, `bodyUsed` enforcement, `clone()`, fail-closed scheme diagnostics.
5. **M28-005 (AbortSignal + Route Deadlines)** — PRs #937–#938: physical Tokio abort via CAS ownership, deterministic terminal-state mapping (timeout→504, capacity→503, engine failure→redacted 500), quarantine accounting.
6. **M28-006 (Streaming + Strict Backpressure)** — PRs #939, #940, #941, #942, #943, #944: `BoundedStream` (1 MiB chunk ceiling, 16 MiB body cap, capacity backpressure), `poll_write`/`write_chunk`/`read_chunk` pump futures, consumer-stop cancellation (`StreamError::Cancelled`), max body helper sizes (`MAX_BODY_HELPER_BYTES` = 16 MiB fail-closed in JS helpers); slow-consumer proof: producer stalls at exactly the 64 KiB capacity on an 8 MiB stream; mid-stream disconnect stops the pump at 192 KiB.
7. **M28-007 (Redirect + Compression Policy)** — PRs #945, #946, #947, #948, #949, #950: `RedirectLimiter` (hop ceiling 20, typed `RedirectLoop`), per-hop SSRF/DNS revalidation (`evaluate_resolved`), credential/header stripping on cross-origin hops, decompression bomb guard (1000:1 ratio + output cap), 17 new policy tests.
8. **M28-008 (SSRF + Network Egress Controls)** — PRs #951, #952, #953, #954, #955, #956: resolve-and-validate connect gate (`resolve_and_validate`, metadata-by-name + address-class denial), atomic `follow_hop` revalidation with pin sets, egress allow/deny configuration (deny wins; metadata not re-enablable), `ProxyMode::Disabled` posture with closed env survey.
9. **M28-009 (Lifecycle, Observability, Shutdown)** — PRs #957, #958, #959, #960, #961, #962: fetch metrics schema (redacted, bounded), thread-safe collector with cumulative/interval sampling, shared-pool drain integrated into the runtime SIGTERM teardown (reported in `shutdown.complete`), quarantined pool rejects new work; measured overhead ~0 ns (plain) / ~22 ns (collector) / structurally 0 disabled.
10. **M28-010 (Fetch Conformance + Fault Testing)** — PRs #963, #964, #965, #966, #967, #968: WPT manifest v1.2.0 (79 pinned vectors, 100% PASS, 23 explicit skips), deterministic fixture library (rebinding/exactly-once DNS, redirect chains, slow bodies, untrusted TLS), 7 property-fuzz suites (3,584 executions/run; found+fixed a fixture race), proxy-isolation and mid-body cancellation proofs.
11. **M28-011 (Controlled Upstream + Fan-out Benchmarks)** — PRs #969, #970, #971, #972, #973, #974: four matched proxy candidates (bun-fetch/hono/elysia2/fastify, pinned lockfile), W4 latency matrix 1/5/10/25ms, fan-out 1/2/4 (parallelism proven), mixed-outcome (typed 200/504/502), concurrency ladder 1/10/50/200 — all cells 0 errors/0 mismatches with enforced structural guardrails; disclosed finding: fastify/Node-fetch c=200/1ms tail not reproducibly bounded on shared hardware.

### Architecture Decision Records (ADRs Accepted)
- **ADR-0033**: Outbound Fetch Security Policy (schemes, SSRF, redirects, TLS, timeouts, compression, bodies)
- **ADR-0034**: Reverse-Proxy and Outbound Trust (forwarded headers, loopback bind, Host never routes)
- **ADR-0035**: Same-Process Trusted Code Assumption

### Standards Conformance & Open Items
- **WPT / WinterTC**: manifest v1.2.0 — 79 pinned vectors passing (100%); 23 explicit skips with machine-readable reason codes.
- **Executor wiring note**: the hyper fetch executor remains dormant by design at this gate (policy/pool/metrics layers complete and DCE-inert; binary `b8296060…` matches the manifest). JS-visible dialing lands with the M3 track per the authorized sequence (ADR-0018).
- **Open Decisions**: `PACK_FORMAT_CURRENT` v1→v2 default flip remains owner-gated (carried from M26, tracked in REVIEW_INDEX openItems). No unauthorized features (WebSockets, SSE, general Node compatibility remain post-beta per ADR-0018 / AGENTS.md constraint 15).
- **Standing CI Disclosure**: CI in this repository fails with zero executed steps on PRs (infrastructure-side since ~#714); local verification passes 100% from the clean candidate commit.
