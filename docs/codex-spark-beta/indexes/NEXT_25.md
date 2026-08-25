# Next 25 Dependency-Safe Tasks

The next dependency-ready implementation task is M26-009-D; this short queue lists the first 25 unchecked packets with their issue numbers.

1. [M26-009-D — Define source-map/debug sidecars](tasks/03_m26_qpack_v2/M26-009-D-define-source-map-debug-sidecars.md) — deps: M26-009-C — #231
2. [M26-009-V — Verify Build shared-runtime and standalone deployment artifacts](tasks/03_m26_qpack_v2/M26-009-V-verify-build-shared-runtime-and-standalone-deployment-artifacts.md) — deps: M26-009-A, M26-009-B, M26-009-C, M26-009-D — #232
3. [M26-009-Z — Package evidence for Build shared-runtime and standalone deployment artifacts](tasks/03_m26_qpack_v2/M26-009-Z-package-evidence-for-build-shared-runtime-and-standalone-deployment-artifacts.md) — deps: M26-009-V — #233
4. [M26-010-A — Measure 25/100/1,000/5,000/10,000 routes](tasks/03_m26_qpack_v2/M26-010-A-measure-25-100-1-000-5-000-10-000-routes.md) — deps: M26-004-Z, M26-005-Z, M26-009-Z — #234
5. [M26-010-B — At least 100 fresh processes for release evidence](tasks/03_m26_qpack_v2/M26-010-B-at-least-100-fresh-processes-for-release-evidence.md) — deps: M26-010-A — #235
6. [M26-010-C — Randomize source/bytecode/competitor order](tasks/03_m26_qpack_v2/M26-010-C-randomize-source-bytecode-competitor-order.md) — deps: M26-010-B — #236
7. [M26-010-D — Record p50/p95/p99, RSS, stage timings, and hashes](tasks/03_m26_qpack_v2/M26-010-D-record-p50-p95-p99-rss-stage-timings-and-hashes.md) — deps: M26-010-C — #237
8. [M26-010-V — Verify Close route-count cold-start evidence](tasks/03_m26_qpack_v2/M26-010-V-verify-close-route-count-cold-start-evidence.md) — deps: M26-010-A, M26-010-B, M26-010-C, M26-010-D — #238
9. [M26-010-Z — Package evidence for Close route-count cold-start evidence](tasks/03_m26_qpack_v2/M26-010-Z-package-evidence-for-close-route-count-cold-start-evidence.md) — deps: M26-010-V — #239
10. [M26-GATE — M2.6 — Binary QPack v2 and Reproducible Artifact ABI exit gate](gates/M26-GATE.md) — deps: M26-001-Z, M26-002-Z, M26-003-Z, M26-004-Z, M26-005-Z, M26-006-Z, M26-007-Z, M26-008-Z, M26-009-Z, M26-010-Z — #629
11. [M27-001-A — Accept ADR](tasks/04_m27_capability_linker/M27-001-A-accept-adr.md) — deps: M26-GATE — #240
12. [M27-001-B — Define CapabilityId/version/dependencies](tasks/04_m27_capability_linker/M27-001-B-define-capabilityid-version-dependencies.md) — deps: M27-001-A — #241
13. [M27-001-C — Define native operation owner/deadline state](tasks/04_m27_capability_linker/M27-001-C-define-native-operation-owner-deadline-state.md) — deps: M27-001-B — #242
14. [M27-001-D — Define lifecycle phases and bounded shutdown](tasks/04_m27_capability_linker/M27-001-D-define-lifecycle-phases-and-bounded-shutdown.md) — deps: M27-001-C — #243
15. [M27-001-V — Verify Define capability ABI and lifecycle state machine](tasks/04_m27_capability_linker/M27-001-V-verify-define-capability-abi-and-lifecycle-state-machine.md) — deps: M27-001-A, M27-001-B, M27-001-C, M27-001-D — #244
16. [M27-001-Z — Package evidence for Define capability ABI and lifecycle state machine](tasks/04_m27_capability_linker/M27-001-Z-package-evidence-for-define-capability-abi-and-lifecycle-state-machine.md) — deps: M27-001-V — #245
17. [M27-002-A — Build dependency DAG](tasks/04_m27_capability_linker/M27-002-A-build-dependency-dag.md) — deps: M27-001-Z — #246
18. [M27-002-B — Reject cycles/missing/conflicting versions](tasks/04_m27_capability_linker/M27-002-B-reject-cycles-missing-conflicting-versions.md) — deps: M27-002-A — #247
19. [M27-002-C — Emit capability inventory/hash into QPack](tasks/04_m27_capability_linker/M27-002-C-emit-capability-inventory-hash-into-qpack.md) — deps: M27-002-B — #248
20. [M27-002-D — Remove unused modules](tasks/04_m27_capability_linker/M27-002-D-remove-unused-modules.md) — deps: M27-002-C — #249
21. [M27-002-V — Verify Implement compile-time capability dependency resolver](tasks/04_m27_capability_linker/M27-002-V-verify-implement-compile-time-capability-dependency-resolver.md) — deps: M27-002-A, M27-002-B, M27-002-C, M27-002-D — #250
22. [M27-002-Z — Package evidence for Implement compile-time capability dependency resolver](tasks/04_m27_capability_linker/M27-002-Z-package-evidence-for-implement-compile-time-capability-dependency-resolver.md) — deps: M27-002-V — #251
23. [M27-003-A — Build configurable intrinsic profiles](tasks/04_m27_capability_linker/M27-003-A-build-configurable-intrinsic-profiles.md) — deps: M27-002-Z — #252
24. [M27-003-B — Compile application requirements](tasks/04_m27_capability_linker/M27-003-B-compile-application-requirements.md) — deps: M27-003-A — #253
25. [M27-003-C — Report missing API/intrinsic diagnostics](tasks/04_m27_capability_linker/M27-003-C-report-missing-api-intrinsic-diagnostics.md) — deps: M27-003-B — #254
