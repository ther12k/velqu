# Next 25 Dependency-Safe Tasks

The next dependency-ready implementation task is M27-008-V; this short queue lists the first 25 unchecked packets with their issue numbers.

1. [M27-008-V — Verify Implement crypto random subset](tasks/04_m27_capability_linker/M27-008-V-verify-implement-crypto-random-subset.md) — deps: M27-008-A, M27-008-B, M27-008-C, M27-008-D — #286
2. [M27-008-Z — Package evidence for Implement crypto random subset](tasks/04_m27_capability_linker/M27-008-Z-package-evidence-for-implement-crypto-random-subset.md) — deps: M27-008-V — #287
3. [M27-009-A — Define Rust-side SDK traits and metadata](tasks/04_m27_capability_linker/M27-009-A-define-rust-side-sdk-traits-and-metadata.md) — deps: M27-001-Z, M27-002-Z — #288
4. [M27-009-B — Provide test harness and example capability](tasks/04_m27_capability_linker/M27-009-B-provide-test-harness-and-example-capability.md) — deps: M27-009-A — #289
5. [M27-009-C — Expose build/inspect diagnostics](tasks/04_m27_capability_linker/M27-009-C-expose-build-inspect-diagnostics.md) — deps: M27-009-B — #290
6. [M27-009-D — Define semver/ABI compatibility](tasks/04_m27_capability_linker/M27-009-D-define-semver-abi-compatibility.md) — deps: M27-009-C — #291
7. [M27-009-V — Verify Publish capability SDK and inspection surface](tasks/04_m27_capability_linker/M27-009-V-verify-publish-capability-sdk-and-inspection-surface.md) — deps: M27-009-A, M27-009-B, M27-009-C, M27-009-D — #292
8. [M27-009-Z — Package evidence for Publish capability SDK and inspection surface](tasks/04_m27_capability_linker/M27-009-Z-package-evidence-for-publish-capability-sdk-and-inspection-surface.md) — deps: M27-009-V — #293
9. [M27-010-A — Pin WPT/WinterTC subsets](tasks/04_m27_capability_linker/M27-010-A-pin-wpt-wintertc-subsets.md) — deps: M27-005-Z, M27-006-Z, M27-007-Z, M27-008-Z — #294
10. [M27-010-B — Record skips and reasons](tasks/04_m27_capability_linker/M27-010-B-record-skips-and-reasons.md) — deps: M27-010-A — #295
11. [M27-010-C — Automate regression reports](tasks/04_m27_capability_linker/M27-010-C-automate-regression-reports.md) — deps: M27-010-B — #296
12. [M27-010-D — Keep unsupported APIs explicit](tasks/04_m27_capability_linker/M27-010-D-keep-unsupported-apis-explicit.md) — deps: M27-010-C — #297
13. [M27-010-V — Verify Establish Web API conformance program](tasks/04_m27_capability_linker/M27-010-V-verify-establish-web-api-conformance-program.md) — deps: M27-010-A, M27-010-B, M27-010-C, M27-010-D — #298
14. [M27-010-Z — Package evidence for Establish Web API conformance program](tasks/04_m27_capability_linker/M27-010-Z-package-evidence-for-establish-web-api-conformance-program.md) — deps: M27-010-V — #299
15. [M27-011-A — Measure core, web-minimal, and all-beta profiles](tasks/04_m27_capability_linker/M27-011-A-measure-core-web-minimal-and-all-beta-profiles.md) — deps: M27-002-Z, M27-010-Z — #300
16. [M27-011-B — Record binary, startup, and idle RSS deltas](tasks/04_m27_capability_linker/M27-011-B-record-binary-startup-and-idle-rss-deltas.md) — deps: M27-011-A — #301
17. [M27-011-C — Identify eager initialization](tasks/04_m27_capability_linker/M27-011-C-identify-eager-initialization.md) — deps: M27-011-B — #302
18. [M27-011-D — Make expensive modules lazy when safe](tasks/04_m27_capability_linker/M27-011-D-make-expensive-modules-lazy-when-safe.md) — deps: M27-011-C — #303
19. [M27-011-V — Verify Close capability cost budgets](tasks/04_m27_capability_linker/M27-011-V-verify-close-capability-cost-budgets.md) — deps: M27-011-A, M27-011-B, M27-011-C, M27-011-D — #304
20. [M27-011-Z — Package evidence for Close capability cost budgets](tasks/04_m27_capability_linker/M27-011-Z-package-evidence-for-close-capability-cost-budgets.md) — deps: M27-011-V — #305
21. [M27-GATE — M2.7 — Capability Linker and Minimal Web Runtime exit gate](gates/M27-GATE.md) — deps: M27-001-Z, M27-002-Z, M27-003-Z, M27-004-Z, M27-005-Z, M27-006-Z, M27-007-Z, M27-008-Z, M27-009-Z, M27-010-Z, M27-011-Z — #630
22. [M28-001-A — Define URL schemes, redirect policy, DNS rebinding controls, proxy behavior, TLS roots, timeout layers, compression, and body limits](tasks/05_m28_native_fetch/M28-001-A-define-url-schemes-redirect-policy-dns-rebinding-controls-proxy-behavior-tls-roo.md) — deps: M27-GATE — #306
23. [M28-001-B — Specify reverse-proxy and outbound trust](tasks/05_m28_native_fetch/M28-001-B-specify-reverse-proxy-and-outbound-trust.md) — deps: M28-001-A — #307
24. [M28-001-C — Define unsupported Web features](tasks/05_m28_native_fetch/M28-001-C-define-unsupported-web-features.md) — deps: M28-001-B — #308
25. [M28-001-D — Document same-process trusted-code assumption](tasks/05_m28_native_fetch/M28-001-D-document-same-process-trusted-code-assumption.md) — deps: M28-001-C — #309
