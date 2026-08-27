# Next 25 Dependency-Safe Tasks

The next dependency-ready implementation task is M27-006-V; this short queue lists the first 25 unchecked packets with their issue numbers.

1. [M27-006-V — Verify Implement TextEncoder and TextDecoder](tasks/04_m27_capability_linker/M27-006-V-verify-implement-textencoder-and-textdecoder.md) — deps: M27-006-A, M27-006-B, M27-006-C, M27-006-D — #274
2. [M27-006-Z — Package evidence for Implement TextEncoder and TextDecoder](tasks/04_m27_capability_linker/M27-006-Z-package-evidence-for-implement-textencoder-and-textdecoder.md) — deps: M27-006-V — #275
3. [M27-007-A — Define signal state/listeners/reason](tasks/04_m27_capability_linker/M27-007-A-define-signal-state-listeners-reason.md) — deps: M27-001-Z, M27-003-Z — #276
4. [M27-007-B — Bridge route deadline and explicit cancellation](tasks/04_m27_capability_linker/M27-007-B-bridge-route-deadline-and-explicit-cancellation.md) — deps: M27-007-A — #277
5. [M27-007-C — Prevent listener leaks](tasks/04_m27_capability_linker/M27-007-C-prevent-listener-leaks.md) — deps: M27-007-B — #278
6. [M27-007-D — Make cancellation idempotent](tasks/04_m27_capability_linker/M27-007-D-make-cancellation-idempotent.md) — deps: M27-007-C — #279
7. [M27-007-V — Verify Implement AbortController and AbortSignal](tasks/04_m27_capability_linker/M27-007-V-verify-implement-abortcontroller-and-abortsignal.md) — deps: M27-007-A, M27-007-B, M27-007-C, M27-007-D — #280
8. [M27-007-Z — Package evidence for Implement AbortController and AbortSignal](tasks/04_m27_capability_linker/M27-007-Z-package-evidence-for-implement-abortcontroller-and-abortsignal.md) — deps: M27-007-V — #281
9. [M27-008-A — Implement `getRandomValues` and `randomUUID` through OS CSPRNG](tasks/04_m27_capability_linker/M27-008-A-implement-getrandomvalues-and-randomuuid-through-os-csprng.md) — deps: M27-001-Z, M27-003-Z — #282
10. [M27-008-B — Enforce typed-array and size constraints](tasks/04_m27_capability_linker/M27-008-B-enforce-typed-array-and-size-constraints.md) — deps: M27-008-A — #283
11. [M27-008-C — Define unavailable-entropy failure](tasks/04_m27_capability_linker/M27-008-C-define-unavailable-entropy-failure.md) — deps: M27-008-B — #284
12. [M27-008-D — Do not implement custom cryptography](tasks/04_m27_capability_linker/M27-008-D-do-not-implement-custom-cryptography.md) — deps: M27-008-C — #285
13. [M27-008-V — Verify Implement crypto random subset](tasks/04_m27_capability_linker/M27-008-V-verify-implement-crypto-random-subset.md) — deps: M27-008-A, M27-008-B, M27-008-C, M27-008-D — #286
14. [M27-008-Z — Package evidence for Implement crypto random subset](tasks/04_m27_capability_linker/M27-008-Z-package-evidence-for-implement-crypto-random-subset.md) — deps: M27-008-V — #287
15. [M27-009-A — Define Rust-side SDK traits and metadata](tasks/04_m27_capability_linker/M27-009-A-define-rust-side-sdk-traits-and-metadata.md) — deps: M27-001-Z, M27-002-Z — #288
16. [M27-009-B — Provide test harness and example capability](tasks/04_m27_capability_linker/M27-009-B-provide-test-harness-and-example-capability.md) — deps: M27-009-A — #289
17. [M27-009-C — Expose build/inspect diagnostics](tasks/04_m27_capability_linker/M27-009-C-expose-build-inspect-diagnostics.md) — deps: M27-009-B — #290
18. [M27-009-D — Define semver/ABI compatibility](tasks/04_m27_capability_linker/M27-009-D-define-semver-abi-compatibility.md) — deps: M27-009-C — #291
19. [M27-009-V — Verify Publish capability SDK and inspection surface](tasks/04_m27_capability_linker/M27-009-V-verify-publish-capability-sdk-and-inspection-surface.md) — deps: M27-009-A, M27-009-B, M27-009-C, M27-009-D — #292
20. [M27-009-Z — Package evidence for Publish capability SDK and inspection surface](tasks/04_m27_capability_linker/M27-009-Z-package-evidence-for-publish-capability-sdk-and-inspection-surface.md) — deps: M27-009-V — #293
21. [M27-010-A — Pin WPT/WinterTC subsets](tasks/04_m27_capability_linker/M27-010-A-pin-wpt-wintertc-subsets.md) — deps: M27-005-Z, M27-006-Z, M27-007-Z, M27-008-Z — #294
22. [M27-010-B — Record skips and reasons](tasks/04_m27_capability_linker/M27-010-B-record-skips-and-reasons.md) — deps: M27-010-A — #295
23. [M27-010-C — Automate regression reports](tasks/04_m27_capability_linker/M27-010-C-automate-regression-reports.md) — deps: M27-010-B — #296
24. [M27-010-D — Keep unsupported APIs explicit](tasks/04_m27_capability_linker/M27-010-D-keep-unsupported-apis-explicit.md) — deps: M27-010-C — #297
25. [M27-010-V — Verify Establish Web API conformance program](tasks/04_m27_capability_linker/M27-010-V-verify-establish-web-api-conformance-program.md) — deps: M27-010-A, M27-010-B, M27-010-C, M27-010-D — #298
