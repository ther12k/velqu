# Next 25 Dependency-Safe Tasks

The next dependency-ready implementation task is M3-005-V; this short queue lists the first 25 unchecked packets with their issue numbers.

1. [M3-005-V — Verify Implement quarantine, replacement, and readiness aggregation](tasks/06_m3_multi_worker/M3-005-V-verify-implement-quarantine-replacement-and-readiness-aggregation.md) — deps: M3-005-A, M3-005-B, M3-005-C, M3-005-D — #400
2. [M3-005-Z — Package evidence for Implement quarantine, replacement, and readiness aggregation](tasks/06_m3_multi_worker/M3-005-Z-package-evidence-for-implement-quarantine-replacement-and-readiness-aggregation.md) — deps: M3-005-V — #401
3. [M3-006-A — Define thresholds/hysteresis](tasks/06_m3_multi_worker/M3-006-A-define-thresholds-hysteresis.md) — deps: M3-003-Z, M3-005-Z — #402
4. [M3-006-B — Bound min/max workers](tasks/06_m3_multi_worker/M3-006-B-bound-min-max-workers.md) — deps: M3-006-A — #403
5. [M3-006-C — Drain before scale-down](tasks/06_m3_multi_worker/M3-006-C-drain-before-scale-down.md) — deps: M3-006-B — #404
6. [M3-006-D — Avoid oscillation](tasks/06_m3_multi_worker/M3-006-D-avoid-oscillation.md) — deps: M3-006-C — #405
7. [M3-006-V — Verify Implement adaptive scale-up and scale-down](tasks/06_m3_multi_worker/M3-006-V-verify-implement-adaptive-scale-up-and-scale-down.md) — deps: M3-006-A, M3-006-B, M3-006-C, M3-006-D — #406
8. [M3-006-Z — Package evidence for Implement adaptive scale-up and scale-down](tasks/06_m3_multi_worker/M3-006-Z-package-evidence-for-implement-adaptive-scale-up-and-scale-down.md) — deps: M3-006-V — #407
9. [M3-007-A — Track invocation-to-worker ownership](tasks/06_m3_multi_worker/M3-007-A-track-invocation-to-worker-ownership.md) — deps: M3-002-Z, M3-004-Z — #408
10. [M3-007-B — Stop admission on drain](tasks/06_m3_multi_worker/M3-007-B-stop-admission-on-drain.md) — deps: M3-007-A — #409
11. [M3-007-C — Allow bounded in-flight completion](tasks/06_m3_multi_worker/M3-007-C-allow-bounded-in-flight-completion.md) — deps: M3-007-B — #410
12. [M3-007-D — Abort after shutdown deadline](tasks/06_m3_multi_worker/M3-007-D-abort-after-shutdown-deadline.md) — deps: M3-007-C — #411
13. [M3-007-V — Verify Implement multi-worker cancellation and graceful shutdown](tasks/06_m3_multi_worker/M3-007-V-verify-implement-multi-worker-cancellation-and-graceful-shutdown.md) — deps: M3-007-A, M3-007-B, M3-007-C, M3-007-D — #412
14. [M3-007-Z — Package evidence for Implement multi-worker cancellation and graceful shutdown](tasks/06_m3_multi_worker/M3-007-Z-package-evidence-for-implement-multi-worker-cancellation-and-graceful-shutdown.md) — deps: M3-007-V — #413
15. [M3-008-A — Add route/global queue limits or weighted admission](tasks/06_m3_multi_worker/M3-008-A-add-route-global-queue-limits-or-weighted-admission.md) — deps: M3-002-Z, M3-006-Z — #414
16. [M3-008-B — Define long-running JS policy](tasks/06_m3_multi_worker/M3-008-B-define-long-running-js-policy.md) — deps: M3-008-A — #415
17. [M3-008-C — Expose load-shed reasons](tasks/06_m3_multi_worker/M3-008-C-expose-load-shed-reasons.md) — deps: M3-008-B — #416
18. [M3-008-D — Test mixed workloads](tasks/06_m3_multi_worker/M3-008-D-test-mixed-workloads.md) — deps: M3-008-C — #417
19. [M3-008-V — Verify Add fairness and overload controls](tasks/06_m3_multi_worker/M3-008-V-verify-add-fairness-and-overload-controls.md) — deps: M3-008-A, M3-008-B, M3-008-C, M3-008-D — #418
20. [M3-008-Z — Package evidence for Add fairness and overload controls](tasks/06_m3_multi_worker/M3-008-Z-package-evidence-for-add-fairness-and-overload-controls.md) — deps: M3-008-V — #419
21. [M3-009-A — Measure 1/2/4 workers](tasks/06_m3_multi_worker/M3-009-A-measure-1-2-4-workers.md) — deps: M3-003-Z, M3-006-Z, M3-008-Z — #420
22. [M3-009-B — Report throughput, p50/p95/p99, queue time, CPU, RSS, errors](tasks/06_m3_multi_worker/M3-009-B-report-throughput-p50-p95-p99-queue-time-cpu-rss-errors.md) — deps: M3-009-A — #421
23. [M3-009-C — Run C1/C2/C3 and controlled I/O](tasks/06_m3_multi_worker/M3-009-C-run-c1-c2-c3-and-controlled-i-o.md) — deps: M3-009-B — #422
24. [M3-009-D — Record physical core topology](tasks/06_m3_multi_worker/M3-009-D-record-physical-core-topology.md) — deps: M3-009-C — #423
25. [M3-009-V — Verify Close multi-worker scaling and memory evidence](tasks/06_m3_multi_worker/M3-009-V-verify-close-multi-worker-scaling-and-memory-evidence.md) — deps: M3-009-A, M3-009-B, M3-009-C, M3-009-D — #424
