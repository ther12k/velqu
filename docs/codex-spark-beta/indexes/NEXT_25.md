# Next 25 Dependency-Safe Tasks

The next dependency-ready implementation task is M27-009-V; this short queue lists the first 25 unchecked packets with their issue numbers.

1. [M27-009-V — Verify Publish capability SDK and inspection surface](tasks/04_m27_capability_linker/M27-009-V-verify-publish-capability-sdk-and-inspection-surface.md) — deps: M27-009-A, M27-009-B, M27-009-C, M27-009-D — #292
2. [M27-009-Z — Package evidence for Publish capability SDK and inspection surface](tasks/04_m27_capability_linker/M27-009-Z-package-evidence-for-publish-capability-sdk-and-inspection-surface.md) — deps: M27-009-V — #293
3. [M27-010-A — Pin WPT/WinterTC subsets](tasks/04_m27_capability_linker/M27-010-A-pin-wpt-wintertc-subsets.md) — deps: M27-005-Z, M27-006-Z, M27-007-Z, M27-008-Z — #294
4. [M27-010-B — Record skips and reasons](tasks/04_m27_capability_linker/M27-010-B-record-skips-and-reasons.md) — deps: M27-010-A — #295
5. [M27-010-C — Automate regression reports](tasks/04_m27_capability_linker/M27-010-C-automate-regression-reports.md) — deps: M27-010-B — #296
6. [M27-010-D — Keep unsupported APIs explicit](tasks/04_m27_capability_linker/M27-010-D-keep-unsupported-apis-explicit.md) — deps: M27-010-C — #297
7. [M27-010-V — Verify Establish Web API conformance program](tasks/04_m27_capability_linker/M27-010-V-verify-establish-web-api-conformance-program.md) — deps: M27-010-A, M27-010-B, M27-010-C, M27-010-D — #298
8. [M27-010-Z — Package evidence for Establish Web API conformance program](tasks/04_m27_capability_linker/M27-010-Z-package-evidence-for-establish-web-api-conformance-program.md) — deps: M27-010-V — #299
9. [M27-011-A — Measure core, web-minimal, and all-beta profiles](tasks/04_m27_capability_linker/M27-011-A-measure-core-web-minimal-and-all-beta-profiles.md) — deps: M27-002-Z, M27-010-Z — #300
10. [M27-011-B — Record binary, startup, and idle RSS deltas](tasks/04_m27_capability_linker/M27-011-B-record-binary-startup-and-idle-rss-deltas.md) — deps: M27-011-A — #301
11. [M27-011-C — Identify eager initialization](tasks/04_m27_capability_linker/M27-011-C-identify-eager-initialization.md) — deps: M27-011-B — #302
12. [M27-011-D — Make expensive modules lazy when safe](tasks/04_m27_capability_linker/M27-011-D-make-expensive-modules-lazy-when-safe.md) — deps: M27-011-C — #303
13. [M27-011-V — Verify Close capability cost budgets](tasks/04_m27_capability_linker/M27-011-V-verify-close-capability-cost-budgets.md) — deps: M27-011-A, M27-011-B, M27-011-C, M27-011-D — #304
14. [M27-011-Z — Package evidence for Close capability cost budgets](tasks/04_m27_capability_linker/M27-011-Z-package-evidence-for-close-capability-cost-budgets.md) — deps: M27-011-V — #305
15. [M27-GATE — M2.7 — Capability Linker and Minimal Web Runtime exit gate](gates/M27-GATE.md) — deps: M27-001-Z, M27-002-Z, M27-003-Z, M27-004-Z, M27-005-Z, M27-006-Z, M27-007-Z, M27-008-Z, M27-009-Z, M27-010-Z, M27-011-Z — #630
16. [M28-001-A — Define URL schemes, redirect policy, DNS rebinding controls, proxy behavior, TLS roots, timeout layers, compression, and body limits](tasks/05_m28_native_fetch/M28-001-A-define-url-schemes-redirect-policy-dns-rebinding-controls-proxy-behavior-tls-roo.md) — deps: M27-GATE — #306
17. [M28-001-B — Specify reverse-proxy and outbound trust](tasks/05_m28_native_fetch/M28-001-B-specify-reverse-proxy-and-outbound-trust.md) — deps: M28-001-A — #307
18. [M28-001-C — Define unsupported Web features](tasks/05_m28_native_fetch/M28-001-C-define-unsupported-web-features.md) — deps: M28-001-B — #308
19. [M28-001-D — Document same-process trusted-code assumption](tasks/05_m28_native_fetch/M28-001-D-document-same-process-trusted-code-assumption.md) — deps: M28-001-C — #309
20. [M28-001-V — Verify Accept fetch, TLS, redirect, and SSRF security ADR](tasks/05_m28_native_fetch/M28-001-V-verify-accept-fetch-tls-redirect-and-ssrf-security-adr.md) — deps: M28-001-A, M28-001-B, M28-001-C, M28-001-D — #310
21. [M28-001-Z — Package evidence for Accept fetch, TLS, redirect, and SSRF security ADR](tasks/05_m28_native_fetch/M28-001-Z-package-evidence-for-accept-fetch-tls-redirect-and-ssrf-security-adr.md) — deps: M28-001-V — #311
22. [M28-002-A — Compare reqwest and lower-level Hyper/Rustls approach](tasks/05_m28_native_fetch/M28-002-A-compare-reqwest-and-lower-level-hyper-rustls-approach.md) — deps: M28-001-Z — #312
23. [M28-002-B — Measure dependency/binary/startup cost](tasks/05_m28_native_fetch/M28-002-B-measure-dependency-binary-startup-cost.md) — deps: M28-002-A — #313
24. [M28-002-C — Test DNS/TLS/pool behavior](tasks/05_m28_native_fetch/M28-002-C-test-dns-tls-pool-behavior.md) — deps: M28-002-B — #314
25. [M28-002-D — Record maintenance/security considerations](tasks/05_m28_native_fetch/M28-002-D-record-maintenance-security-considerations.md) — deps: M28-002-C — #315
