# Next 25 Dependency-Safe Tasks

The next dependency-ready implementation task is M27-011-C; this short queue lists the first 25 unchecked packets with their issue numbers.

1. [M27-011-C — Identify eager initialization](tasks/04_m27_capability_linker/M27-011-C-identify-eager-initialization.md) — deps: M27-011-B — #302
2. [M27-011-D — Make expensive modules lazy when safe](tasks/04_m27_capability_linker/M27-011-D-make-expensive-modules-lazy-when-safe.md) — deps: M27-011-C — #303
3. [M27-011-V — Verify Close capability cost budgets](tasks/04_m27_capability_linker/M27-011-V-verify-close-capability-cost-budgets.md) — deps: M27-011-A, M27-011-B, M27-011-C, M27-011-D — #304
4. [M27-011-Z — Package evidence for Close capability cost budgets](tasks/04_m27_capability_linker/M27-011-Z-package-evidence-for-close-capability-cost-budgets.md) — deps: M27-011-V — #305
5. [M27-GATE — M2.7 — Capability Linker and Minimal Web Runtime exit gate](gates/M27-GATE.md) — deps: M27-001-Z, M27-002-Z, M27-003-Z, M27-004-Z, M27-005-Z, M27-006-Z, M27-007-Z, M27-008-Z, M27-009-Z, M27-010-Z, M27-011-Z — #630
6. [M28-001-A — Define URL schemes, redirect policy, DNS rebinding controls, proxy behavior, TLS roots, timeout layers, compression, and body limits](tasks/05_m28_native_fetch/M28-001-A-define-url-schemes-redirect-policy-dns-rebinding-controls-proxy-behavior-tls-roo.md) — deps: M27-GATE — #306
7. [M28-001-B — Specify reverse-proxy and outbound trust](tasks/05_m28_native_fetch/M28-001-B-specify-reverse-proxy-and-outbound-trust.md) — deps: M28-001-A — #307
8. [M28-001-C — Define unsupported Web features](tasks/05_m28_native_fetch/M28-001-C-define-unsupported-web-features.md) — deps: M28-001-B — #308
9. [M28-001-D — Document same-process trusted-code assumption](tasks/05_m28_native_fetch/M28-001-D-document-same-process-trusted-code-assumption.md) — deps: M28-001-C — #309
10. [M28-001-V — Verify Accept fetch, TLS, redirect, and SSRF security ADR](tasks/05_m28_native_fetch/M28-001-V-verify-accept-fetch-tls-redirect-and-ssrf-security-adr.md) — deps: M28-001-A, M28-001-B, M28-001-C, M28-001-D — #310
11. [M28-001-Z — Package evidence for Accept fetch, TLS, redirect, and SSRF security ADR](tasks/05_m28_native_fetch/M28-001-Z-package-evidence-for-accept-fetch-tls-redirect-and-ssrf-security-adr.md) — deps: M28-001-V — #311
12. [M28-002-A — Compare reqwest and lower-level Hyper/Rustls approach](tasks/05_m28_native_fetch/M28-002-A-compare-reqwest-and-lower-level-hyper-rustls-approach.md) — deps: M28-001-Z — #312
13. [M28-002-B — Measure dependency/binary/startup cost](tasks/05_m28_native_fetch/M28-002-B-measure-dependency-binary-startup-cost.md) — deps: M28-002-A — #313
14. [M28-002-C — Test DNS/TLS/pool behavior](tasks/05_m28_native_fetch/M28-002-C-test-dns-tls-pool-behavior.md) — deps: M28-002-B — #314
15. [M28-002-D — Record maintenance/security considerations](tasks/05_m28_native_fetch/M28-002-D-record-maintenance-security-considerations.md) — deps: M28-002-C — #315
16. [M28-002-V — Verify Select native HTTP client stack from evidence](tasks/05_m28_native_fetch/M28-002-V-verify-select-native-http-client-stack-from-evidence.md) — deps: M28-002-A, M28-002-B, M28-002-C, M28-002-D — #316
17. [M28-002-Z — Package evidence for Select native HTTP client stack from evidence](tasks/05_m28_native_fetch/M28-002-Z-package-evidence-for-select-native-http-client-stack-from-evidence.md) — deps: M28-002-V — #317
18. [M28-003-A — Lazy pool initialization](tasks/05_m28_native_fetch/M28-003-A-lazy-pool-initialization.md) — deps: M28-002-Z — #318
19. [M28-003-B — Bound idle/active connections and DNS cache](tasks/05_m28_native_fetch/M28-003-B-bound-idle-active-connections-and-dns-cache.md) — deps: M28-003-A — #319
20. [M28-003-C — Use verified TLS roots and hostname validation](tasks/05_m28_native_fetch/M28-003-C-use-verified-tls-roots-and-hostname-validation.md) — deps: M28-003-B — #320
21. [M28-003-D — Define keepalive and shutdown](tasks/05_m28_native_fetch/M28-003-D-define-keepalive-and-shutdown.md) — deps: M28-003-C — #321
22. [M28-003-V — Verify Implement connection pooling, DNS, and TLS](tasks/05_m28_native_fetch/M28-003-V-verify-implement-connection-pooling-dns-and-tls.md) — deps: M28-003-A, M28-003-B, M28-003-C, M28-003-D — #322
23. [M28-003-Z — Package evidence for Implement connection pooling, DNS, and TLS](tasks/05_m28_native_fetch/M28-003-Z-package-evidence-for-implement-connection-pooling-dns-and-tls.md) — deps: M28-003-V — #323
24. [M28-004-A — Implement method, URL, selected headers, body types, status, and response methods](tasks/05_m28_native_fetch/M28-004-A-implement-method-url-selected-headers-body-types-status-and-response-methods.md) — deps: M28-003-Z, M27-005-Z, M27-006-Z — #324
25. [M28-004-B — Use lazy native-backed objects](tasks/05_m28_native_fetch/M28-004-B-use-lazy-native-backed-objects.md) — deps: M28-004-A — #325
