# Next 25 Dependency-Safe Tasks

The next dependency-ready implementation task is M3-007-D; this short queue lists the first 25 unchecked packets with their issue numbers.

1. [M3-007-D — Abort after shutdown deadline](tasks/06_m3_multi_worker/M3-007-D-abort-after-shutdown-deadline.md) — deps: M3-007-C — #411
2. [M3-007-V — Verify Implement multi-worker cancellation and graceful shutdown](tasks/06_m3_multi_worker/M3-007-V-verify-implement-multi-worker-cancellation-and-graceful-shutdown.md) — deps: M3-007-A, M3-007-B, M3-007-C, M3-007-D — #412
3. [M3-007-Z — Package evidence for Implement multi-worker cancellation and graceful shutdown](tasks/06_m3_multi_worker/M3-007-Z-package-evidence-for-implement-multi-worker-cancellation-and-graceful-shutdown.md) — deps: M3-007-V — #413
4. [M3-008-A — Add route/global queue limits or weighted admission](tasks/06_m3_multi_worker/M3-008-A-add-route-global-queue-limits-or-weighted-admission.md) — deps: M3-002-Z, M3-006-Z — #414
5. [M3-008-B — Define long-running JS policy](tasks/06_m3_multi_worker/M3-008-B-define-long-running-js-policy.md) — deps: M3-008-A — #415
6. [M3-008-C — Expose load-shed reasons](tasks/06_m3_multi_worker/M3-008-C-expose-load-shed-reasons.md) — deps: M3-008-B — #416
7. [M3-008-D — Test mixed workloads](tasks/06_m3_multi_worker/M3-008-D-test-mixed-workloads.md) — deps: M3-008-C — #417
8. [M3-008-V — Verify Add fairness and overload controls](tasks/06_m3_multi_worker/M3-008-V-verify-add-fairness-and-overload-controls.md) — deps: M3-008-A, M3-008-B, M3-008-C, M3-008-D — #418
9. [M3-008-Z — Package evidence for Add fairness and overload controls](tasks/06_m3_multi_worker/M3-008-Z-package-evidence-for-add-fairness-and-overload-controls.md) — deps: M3-008-V — #419
10. [M3-009-A — Measure 1/2/4 workers](tasks/06_m3_multi_worker/M3-009-A-measure-1-2-4-workers.md) — deps: M3-003-Z, M3-006-Z, M3-008-Z — #420
11. [M3-009-B — Report throughput, p50/p95/p99, queue time, CPU, RSS, errors](tasks/06_m3_multi_worker/M3-009-B-report-throughput-p50-p95-p99-queue-time-cpu-rss-errors.md) — deps: M3-009-A — #421
12. [M3-009-C — Run C1/C2/C3 and controlled I/O](tasks/06_m3_multi_worker/M3-009-C-run-c1-c2-c3-and-controlled-i-o.md) — deps: M3-009-B — #422
13. [M3-009-D — Record physical core topology](tasks/06_m3_multi_worker/M3-009-D-record-physical-core-topology.md) — deps: M3-009-C — #423
14. [M3-009-V — Verify Close multi-worker scaling and memory evidence](tasks/06_m3_multi_worker/M3-009-V-verify-close-multi-worker-scaling-and-memory-evidence.md) — deps: M3-009-A, M3-009-B, M3-009-C, M3-009-D — #424
15. [M3-009-Z — Package evidence for Close multi-worker scaling and memory evidence](tasks/06_m3_multi_worker/M3-009-Z-package-evidence-for-close-multi-worker-scaling-and-memory-evidence.md) — deps: M3-009-V — #425
16. [M3-010-A — Run multi-hour mixed load](tasks/06_m3_multi_worker/M3-010-A-run-multi-hour-mixed-load.md) — deps: M3-005-Z, M3-007-Z, M3-009-Z — #426
17. [M3-010-B — Inject worker poison, upstream timeout, disconnect, and shutdown](tasks/06_m3_multi_worker/M3-010-B-inject-worker-poison-upstream-timeout-disconnect-and-shutdown.md) — deps: M3-010-A — #427
18. [M3-010-C — Track retained memory and task/slot counts](tasks/06_m3_multi_worker/M3-010-C-track-retained-memory-and-task-slot-counts.md) — deps: M3-010-B — #428
19. [M3-010-D — Verify recovery](tasks/06_m3_multi_worker/M3-010-D-verify-recovery.md) — deps: M3-010-C — #429
20. [M3-010-V — Verify Run multi-worker soak and recovery](tasks/06_m3_multi_worker/M3-010-V-verify-run-multi-worker-soak-and-recovery.md) — deps: M3-010-A, M3-010-B, M3-010-C, M3-010-D — #430
21. [M3-010-Z — Package evidence for Run multi-worker soak and recovery](tasks/06_m3_multi_worker/M3-010-Z-package-evidence-for-run-multi-worker-soak-and-recovery.md) — deps: M3-010-V — #431
22. [M3-GATE — M3 — Multi-Worker Service Runtime exit gate](gates/M3-GATE.md) — deps: M3-001-Z, M3-002-Z, M3-003-Z, M3-004-Z, M3-005-Z, M3-006-Z, M3-007-Z, M3-008-Z, M3-009-Z, M3-010-Z — #632
23. [M4A-001-A — Watch source and contracts](tasks/07_m4a_developer_preview/M4A-001-A-watch-source-and-contracts.md) — deps: M3-GATE — #432
24. [M4A-001-B — Build incremental temporary QPack](tasks/07_m4a_developer_preview/M4A-001-B-build-incremental-temporary-qpack.md) — deps: M4A-001-A — #433
25. [M4A-001-C — Load new worker before switching traffic](tasks/07_m4a_developer_preview/M4A-001-C-load-new-worker-before-switching-traffic.md) — deps: M4A-001-B — #434
