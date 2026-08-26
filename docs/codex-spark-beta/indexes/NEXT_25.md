# Next 25 Dependency-Safe Tasks

The next dependency-ready implementation task is M26-GATE; this short queue lists the first 25 unchecked packets with their issue numbers.

1. [M26-GATE — M2.6 — Binary QPack v2 and Reproducible Artifact ABI exit gate](gates/M26-GATE.md) — deps: M26-001-Z, M26-002-Z, M26-003-Z, M26-004-Z, M26-005-Z, M26-006-Z, M26-007-Z, M26-008-Z, M26-009-Z, M26-010-Z — #629
2. [M27-001-A — Accept ADR](tasks/04_m27_capability_linker/M27-001-A-accept-adr.md) — deps: M26-GATE — #240
3. [M27-001-B — Define CapabilityId/version/dependencies](tasks/04_m27_capability_linker/M27-001-B-define-capabilityid-version-dependencies.md) — deps: M27-001-A — #241
4. [M27-001-C — Define native operation owner/deadline state](tasks/04_m27_capability_linker/M27-001-C-define-native-operation-owner-deadline-state.md) — deps: M27-001-B — #242
5. [M27-001-D — Define lifecycle phases and bounded shutdown](tasks/04_m27_capability_linker/M27-001-D-define-lifecycle-phases-and-bounded-shutdown.md) — deps: M27-001-C — #243
6. [M27-001-V — Verify Define capability ABI and lifecycle state machine](tasks/04_m27_capability_linker/M27-001-V-verify-define-capability-abi-and-lifecycle-state-machine.md) — deps: M27-001-A, M27-001-B, M27-001-C, M27-001-D — #244
7. [M27-001-Z — Package evidence for Define capability ABI and lifecycle state machine](tasks/04_m27_capability_linker/M27-001-Z-package-evidence-for-define-capability-abi-and-lifecycle-state-machine.md) — deps: M27-001-V — #245
8. [M27-002-A — Build dependency DAG](tasks/04_m27_capability_linker/M27-002-A-build-dependency-dag.md) — deps: M27-001-Z — #246
9. [M27-002-B — Reject cycles/missing/conflicting versions](tasks/04_m27_capability_linker/M27-002-B-reject-cycles-missing-conflicting-versions.md) — deps: M27-002-A — #247
10. [M27-002-C — Emit capability inventory/hash into QPack](tasks/04_m27_capability_linker/M27-002-C-emit-capability-inventory-hash-into-qpack.md) — deps: M27-002-B — #248
11. [M27-002-D — Remove unused modules](tasks/04_m27_capability_linker/M27-002-D-remove-unused-modules.md) — deps: M27-002-C — #249
12. [M27-002-V — Verify Implement compile-time capability dependency resolver](tasks/04_m27_capability_linker/M27-002-V-verify-implement-compile-time-capability-dependency-resolver.md) — deps: M27-002-A, M27-002-B, M27-002-C, M27-002-D — #250
13. [M27-002-Z — Package evidence for Implement compile-time capability dependency resolver](tasks/04_m27_capability_linker/M27-002-Z-package-evidence-for-implement-compile-time-capability-dependency-resolver.md) — deps: M27-002-V — #251
14. [M27-003-A — Build configurable intrinsic profiles](tasks/04_m27_capability_linker/M27-003-A-build-configurable-intrinsic-profiles.md) — deps: M27-002-Z — #252
15. [M27-003-B — Compile application requirements](tasks/04_m27_capability_linker/M27-003-B-compile-application-requirements.md) — deps: M27-003-A — #253
16. [M27-003-C — Report missing API/intrinsic diagnostics](tasks/04_m27_capability_linker/M27-003-C-report-missing-api-intrinsic-diagnostics.md) — deps: M27-003-B — #254
17. [M27-003-D — Retain full profile for compatibility testing](tasks/04_m27_capability_linker/M27-003-D-retain-full-profile-for-compatibility-testing.md) — deps: M27-003-C — #255
18. [M27-003-V — Verify Introduce custom QuickJS context profiles](tasks/04_m27_capability_linker/M27-003-V-verify-introduce-custom-quickjs-context-profiles.md) — deps: M27-003-A, M27-003-B, M27-003-C, M27-003-D — #256
19. [M27-003-Z — Package evidence for Introduce custom QuickJS context profiles](tasks/04_m27_capability_linker/M27-003-Z-package-evidence-for-introduce-custom-quickjs-context-profiles.md) — deps: M27-003-V — #257
20. [M27-004-A — Port timer cancellation/accounting](tasks/04_m27_capability_linker/M27-004-A-port-timer-cancellation-accounting.md) — deps: M27-001-Z, M27-002-Z — #258
21. [M27-004-B — Define console levels and redaction](tasks/04_m27_capability_linker/M27-004-B-define-console-levels-and-redaction.md) — deps: M27-004-A — #259
22. [M27-004-C — Keep logs asynchronous/bounded](tasks/04_m27_capability_linker/M27-004-C-keep-logs-asynchronous-bounded.md) — deps: M27-004-B — #260
23. [M27-004-D — Support shutdown and quarantine](tasks/04_m27_capability_linker/M27-004-D-support-shutdown-and-quarantine.md) — deps: M27-004-C — #261
24. [M27-004-V — Verify Implement console and timer core capabilities](tasks/04_m27_capability_linker/M27-004-V-verify-implement-console-and-timer-core-capabilities.md) — deps: M27-004-A, M27-004-B, M27-004-C, M27-004-D — #262
25. [M27-004-Z — Package evidence for Implement console and timer core capabilities](tasks/04_m27_capability_linker/M27-004-Z-package-evidence-for-implement-console-and-timer-core-capabilities.md) — deps: M27-004-V — #263
