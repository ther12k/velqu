# Next 25 Dependency-Safe Tasks

The next dependency-ready implementation task is M28-008-V; this short queue lists the first 25 unchecked packets with their issue numbers.

1. [M28-008-V — Verify Implement SSRF and network egress controls](tasks/05_m28_native_fetch/M28-008-V-verify-implement-ssrf-and-network-egress-controls.md) — deps: M28-008-A, M28-008-B, M28-008-C, M28-008-D — #352
2. [M28-008-Z — Package evidence for Implement SSRF and network egress controls](tasks/05_m28_native_fetch/M28-008-Z-package-evidence-for-implement-ssrf-and-network-egress-controls.md) — deps: M28-008-V — #353
3. [M28-009-A — Expose pool wait, DNS, connect, TLS, TTFB, body, errors, cancellations](tasks/05_m28_native_fetch/M28-009-A-expose-pool-wait-dns-connect-tls-ttfb-body-errors-cancellations.md) — deps: M28-003-Z, M28-005-Z, M28-006-Z — #354
4. [M28-009-B — Sample/aggregate metrics](tasks/05_m28_native_fetch/M28-009-B-sample-aggregate-metrics.md) — deps: M28-009-A — #355
5. [M28-009-C — Drain pool on shutdown](tasks/05_m28_native_fetch/M28-009-C-drain-pool-on-shutdown.md) — deps: M28-009-B — #356
6. [M28-009-D — Quarantine rejects new work](tasks/05_m28_native_fetch/M28-009-D-quarantine-rejects-new-work.md) — deps: M28-009-C — #357
7. [M28-009-V — Verify Integrate lifecycle, observability, and shutdown](tasks/05_m28_native_fetch/M28-009-V-verify-integrate-lifecycle-observability-and-shutdown.md) — deps: M28-009-A, M28-009-B, M28-009-C, M28-009-D — #358
8. [M28-009-Z — Package evidence for Integrate lifecycle, observability, and shutdown](tasks/05_m28_native_fetch/M28-009-Z-package-evidence-for-integrate-lifecycle-observability-and-shutdown.md) — deps: M28-009-V — #359
9. [M28-010-A — Run selected WPT cases](tasks/05_m28_native_fetch/M28-010-A-run-selected-wpt-cases.md) — deps: M28-004-Z, M28-005-Z, M28-006-Z, M28-007-Z, M28-008-Z — #360
10. [M28-010-B — Create deterministic DNS/TLS/redirect/slow/body fixtures](tasks/05_m28_native_fetch/M28-010-B-create-deterministic-dns-tls-redirect-slow-body-fixtures.md) — deps: M28-010-A — #361
11. [M28-010-C — Fuzz headers and URLs](tasks/05_m28_native_fetch/M28-010-C-fuzz-headers-and-urls.md) — deps: M28-010-B — #362
12. [M28-010-D — Test proxy and cancellation](tasks/05_m28_native_fetch/M28-010-D-test-proxy-and-cancellation.md) — deps: M28-010-C — #363
13. [M28-010-V — Verify Complete fetch conformance and fault testing](tasks/05_m28_native_fetch/M28-010-V-verify-complete-fetch-conformance-and-fault-testing.md) — deps: M28-010-A, M28-010-B, M28-010-C, M28-010-D — #364
14. [M28-010-Z — Package evidence for Complete fetch conformance and fault testing](tasks/05_m28_native_fetch/M28-010-Z-package-evidence-for-complete-fetch-conformance-and-fault-testing.md) — deps: M28-010-V — #365
15. [M28-011-A — Run 1/5/10/25ms upstream latency](tasks/05_m28_native_fetch/M28-011-A-run-1-5-10-25ms-upstream-latency.md) — deps: M28-009-Z, M28-010-Z — #366
16. [M28-011-B — Run one, two, and four parallel calls](tasks/05_m28_native_fetch/M28-011-B-run-one-two-and-four-parallel-calls.md) — deps: M28-011-A — #367
17. [M28-011-C — Mix timeout/success/malformed responses](tasks/05_m28_native_fetch/M28-011-C-mix-timeout-success-malformed-responses.md) — deps: M28-011-B — #368
18. [M28-011-D — Test concurrency 1/10/50/200](tasks/05_m28_native_fetch/M28-011-D-test-concurrency-1-10-50-200.md) — deps: M28-011-C — #369
19. [M28-011-V — Verify Run controlled upstream and fan-out benchmarks](tasks/05_m28_native_fetch/M28-011-V-verify-run-controlled-upstream-and-fan-out-benchmarks.md) — deps: M28-011-A, M28-011-B, M28-011-C, M28-011-D — #370
20. [M28-011-Z — Package evidence for Run controlled upstream and fan-out benchmarks](tasks/05_m28_native_fetch/M28-011-Z-package-evidence-for-run-controlled-upstream-and-fan-out-benchmarks.md) — deps: M28-011-V — #371
21. [M28-GATE — M2.8 — Native Outbound Fetch exit gate](gates/M28-GATE.md) — deps: M28-001-Z, M28-002-Z, M28-003-Z, M28-004-Z, M28-005-Z, M28-006-Z, M28-007-Z, M28-008-Z, M28-009-Z, M28-010-Z, M28-011-Z — #631
22. [M3-001-A — Accept ADR](tasks/06_m3_multi_worker/M3-001-A-accept-adr.md) — deps: M28-GATE — #372
23. [M3-001-B — Document module-level state replication](tasks/06_m3_multi_worker/M3-001-B-document-module-level-state-replication.md) — deps: M3-001-A — #373
24. [M3-001-C — Forbid JSValue sharing](tasks/06_m3_multi_worker/M3-001-C-forbid-jsvalue-sharing.md) — deps: M3-001-B — #374
25. [M3-001-D — Define service/capability shared handles and thread safety](tasks/06_m3_multi_worker/M3-001-D-define-service-capability-shared-handles-and-thread-safety.md) — deps: M3-001-C — #375
