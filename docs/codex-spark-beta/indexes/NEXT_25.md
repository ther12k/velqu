# Next 25 Dependency-Safe Tasks

The next dependency-ready implementation task is M3-009-V; this short queue lists the first 25 unchecked packets with their issue numbers.

1. [M3-009-V — Verify Close multi-worker scaling and memory evidence](tasks/06_m3_multi_worker/M3-009-V-verify-close-multi-worker-scaling-and-memory-evidence.md) — deps: M3-009-A, M3-009-B, M3-009-C, M3-009-D — #424
2. [M3-009-Z — Package evidence for Close multi-worker scaling and memory evidence](tasks/06_m3_multi_worker/M3-009-Z-package-evidence-for-close-multi-worker-scaling-and-memory-evidence.md) — deps: M3-009-V — #425
3. [M3-010-A — Run multi-hour mixed load](tasks/06_m3_multi_worker/M3-010-A-run-multi-hour-mixed-load.md) — deps: M3-005-Z, M3-007-Z, M3-009-Z — #426
4. [M3-010-B — Inject worker poison, upstream timeout, disconnect, and shutdown](tasks/06_m3_multi_worker/M3-010-B-inject-worker-poison-upstream-timeout-disconnect-and-shutdown.md) — deps: M3-010-A — #427
5. [M3-010-C — Track retained memory and task/slot counts](tasks/06_m3_multi_worker/M3-010-C-track-retained-memory-and-task-slot-counts.md) — deps: M3-010-B — #428
6. [M3-010-D — Verify recovery](tasks/06_m3_multi_worker/M3-010-D-verify-recovery.md) — deps: M3-010-C — #429
7. [M3-010-V — Verify Run multi-worker soak and recovery](tasks/06_m3_multi_worker/M3-010-V-verify-run-multi-worker-soak-and-recovery.md) — deps: M3-010-A, M3-010-B, M3-010-C, M3-010-D — #430
8. [M3-010-Z — Package evidence for Run multi-worker soak and recovery](tasks/06_m3_multi_worker/M3-010-Z-package-evidence-for-run-multi-worker-soak-and-recovery.md) — deps: M3-010-V — #431
9. [M3-GATE — M3 — Multi-Worker Service Runtime exit gate](gates/M3-GATE.md) — deps: M3-001-Z, M3-002-Z, M3-003-Z, M3-004-Z, M3-005-Z, M3-006-Z, M3-007-Z, M3-008-Z, M3-009-Z, M3-010-Z — #632
10. [M4A-001-A — Watch source and contracts](tasks/07_m4a_developer_preview/M4A-001-A-watch-source-and-contracts.md) — deps: M3-GATE — #432
11. [M4A-001-B — Build incremental temporary QPack](tasks/07_m4a_developer_preview/M4A-001-B-build-incremental-temporary-qpack.md) — deps: M4A-001-A — #433
12. [M4A-001-C — Load new worker before switching traffic](tasks/07_m4a_developer_preview/M4A-001-C-load-new-worker-before-switching-traffic.md) — deps: M4A-001-B — #434
13. [M4A-001-D — Drain old worker and surface compile/runtime errors](tasks/07_m4a_developer_preview/M4A-001-D-drain-old-worker-and-surface-compile-runtime-errors.md) — deps: M4A-001-C — #435
14. [M4A-001-V — Verify Implement actual-runtime `velqu dev` loop](tasks/07_m4a_developer_preview/M4A-001-V-verify-implement-actual-runtime-velqu-dev-loop.md) — deps: M4A-001-A, M4A-001-B, M4A-001-C, M4A-001-D — #436
15. [M4A-001-Z — Package evidence for Implement actual-runtime `velqu dev` loop](tasks/07_m4a_developer_preview/M4A-001-Z-package-evidence-for-implement-actual-runtime-velqu-dev-loop.md) — deps: M4A-001-V — #437
16. [M4A-002-A — Implement and document `velqu dev`, `build`, `inspect`, `contract diff`, `test`, `pack inspect/migrate`, and diagnostics](tasks/07_m4a_developer_preview/M4A-002-A-implement-and-document-velqu-dev-build-inspect-contract-diff-test-pack-inspect-m.md) — deps: M4A-001-Z, M26-GATE — #438
17. [M4A-002-B — Stable exit codes](tasks/07_m4a_developer_preview/M4A-002-B-stable-exit-codes.md) — deps: M4A-002-A — #439
18. [M4A-002-C — Machine-readable output option](tasks/07_m4a_developer_preview/M4A-002-C-machine-readable-output-option.md) — deps: M4A-002-B — #440
19. [M4A-002-D — Helpful actionable errors](tasks/07_m4a_developer_preview/M4A-002-D-helpful-actionable-errors.md) — deps: M4A-002-C — #441
20. [M4A-002-V — Verify Complete CLI command surface](tasks/07_m4a_developer_preview/M4A-002-V-verify-complete-cli-command-surface.md) — deps: M4A-002-A, M4A-002-B, M4A-002-C, M4A-002-D — #442
21. [M4A-002-Z — Package evidence for Complete CLI command surface](tasks/07_m4a_developer_preview/M4A-002-Z-package-evidence-for-complete-cli-command-surface.md) — deps: M4A-002-V — #443
22. [M4A-003-A — Starter API](tasks/07_m4a_developer_preview/M4A-003-A-starter-api.md) — deps: M4A-002-Z — #444
23. [M4A-003-B — Treaty client example](tasks/07_m4a_developer_preview/M4A-003-B-treaty-client-example.md) — deps: M4A-003-A — #445
24. [M4A-003-C — Testing setup](tasks/07_m4a_developer_preview/M4A-003-C-testing-setup.md) — deps: M4A-003-B — #446
25. [M4A-003-D — Optional fetch/profile choices](tasks/07_m4a_developer_preview/M4A-003-D-optional-fetch-profile-choices.md) — deps: M4A-003-C — #447
