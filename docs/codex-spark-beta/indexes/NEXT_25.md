# Next 25 Dependency-Safe Tasks

The next dependency-ready implementation task is M27-003-B; this short queue lists the first 25 unchecked packets with their issue numbers.

1. [M27-003-B — Compile application requirements](tasks/04_m27_capability_linker/M27-003-B-compile-application-requirements.md) — deps: M27-003-A — #253
2. [M27-003-C — Report missing API/intrinsic diagnostics](tasks/04_m27_capability_linker/M27-003-C-report-missing-api-intrinsic-diagnostics.md) — deps: M27-003-B — #254
3. [M27-003-D — Retain full profile for compatibility testing](tasks/04_m27_capability_linker/M27-003-D-retain-full-profile-for-compatibility-testing.md) — deps: M27-003-C — #255
4. [M27-003-V — Verify Introduce custom QuickJS context profiles](tasks/04_m27_capability_linker/M27-003-V-verify-introduce-custom-quickjs-context-profiles.md) — deps: M27-003-A, M27-003-B, M27-003-C, M27-003-D — #256
5. [M27-003-Z — Package evidence for Introduce custom QuickJS context profiles](tasks/04_m27_capability_linker/M27-003-Z-package-evidence-for-introduce-custom-quickjs-context-profiles.md) — deps: M27-003-V — #257
6. [M27-004-A — Port timer cancellation/accounting](tasks/04_m27_capability_linker/M27-004-A-port-timer-cancellation-accounting.md) — deps: M27-001-Z, M27-002-Z — #258
7. [M27-004-B — Define console levels and redaction](tasks/04_m27_capability_linker/M27-004-B-define-console-levels-and-redaction.md) — deps: M27-004-A — #259
8. [M27-004-C — Keep logs asynchronous/bounded](tasks/04_m27_capability_linker/M27-004-C-keep-logs-asynchronous-bounded.md) — deps: M27-004-B — #260
9. [M27-004-D — Support shutdown and quarantine](tasks/04_m27_capability_linker/M27-004-D-support-shutdown-and-quarantine.md) — deps: M27-004-C — #261
10. [M27-004-V — Verify Implement console and timer core capabilities](tasks/04_m27_capability_linker/M27-004-V-verify-implement-console-and-timer-core-capabilities.md) — deps: M27-004-A, M27-004-B, M27-004-C, M27-004-D — #262
11. [M27-004-Z — Package evidence for Implement console and timer core capabilities](tasks/04_m27_capability_linker/M27-004-Z-package-evidence-for-implement-console-and-timer-core-capabilities.md) — deps: M27-004-V — #263
12. [M27-005-A — Adopt or adapt a proven implementation](tasks/04_m27_capability_linker/M27-005-A-adopt-or-adapt-a-proven-implementation.md) — deps: M27-001-Z, M27-003-Z — #264
13. [M27-005-B — Run selected WPT/WinterTC cases](tasks/04_m27_capability_linker/M27-005-B-run-selected-wpt-wintertc-cases.md) — deps: M27-005-A — #265
14. [M27-005-C — Define host/path encoding behavior](tasks/04_m27_capability_linker/M27-005-C-define-host-path-encoding-behavior.md) — deps: M27-005-B — #266
15. [M27-005-D — Keep parser limits explicit](tasks/04_m27_capability_linker/M27-005-D-keep-parser-limits-explicit.md) — deps: M27-005-C — #267
16. [M27-005-V — Verify Implement URL and URLSearchParams](tasks/04_m27_capability_linker/M27-005-V-verify-implement-url-and-urlsearchparams.md) — deps: M27-005-A, M27-005-B, M27-005-C, M27-005-D — #268
17. [M27-005-Z — Package evidence for Implement URL and URLSearchParams](tasks/04_m27_capability_linker/M27-005-Z-package-evidence-for-implement-url-and-urlsearchparams.md) — deps: M27-005-V — #269
18. [M27-006-A — Support UTF-8 baseline](tasks/04_m27_capability_linker/M27-006-A-support-utf-8-baseline.md) — deps: M27-001-Z, M27-003-Z — #270
19. [M27-006-B — Define invalid sequence/replacement behavior](tasks/04_m27_capability_linker/M27-006-B-define-invalid-sequence-replacement-behavior.md) — deps: M27-006-A — #271
20. [M27-006-C — Integrate TypedArray ownership](tasks/04_m27_capability_linker/M27-006-C-integrate-typedarray-ownership.md) — deps: M27-006-B — #272
21. [M27-006-D — Run WPT subset](tasks/04_m27_capability_linker/M27-006-D-run-wpt-subset.md) — deps: M27-006-C — #273
22. [M27-006-V — Verify Implement TextEncoder and TextDecoder](tasks/04_m27_capability_linker/M27-006-V-verify-implement-textencoder-and-textdecoder.md) — deps: M27-006-A, M27-006-B, M27-006-C, M27-006-D — #274
23. [M27-006-Z — Package evidence for Implement TextEncoder and TextDecoder](tasks/04_m27_capability_linker/M27-006-Z-package-evidence-for-implement-textencoder-and-textdecoder.md) — deps: M27-006-V — #275
24. [M27-007-A — Define signal state/listeners/reason](tasks/04_m27_capability_linker/M27-007-A-define-signal-state-listeners-reason.md) — deps: M27-001-Z, M27-003-Z — #276
25. [M27-007-B — Bridge route deadline and explicit cancellation](tasks/04_m27_capability_linker/M27-007-B-bridge-route-deadline-and-explicit-cancellation.md) — deps: M27-007-A — #277
