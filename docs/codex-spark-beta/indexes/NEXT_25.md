# Next 25 Dependency-Safe Tasks

The next dependency-ready implementation task is M3-003-C; this short queue lists the first 25 unchecked packets with their issue numbers.

1. [M3-003-C — Throughput initializes configured workers before ready](tasks/06_m3_multi_worker/M3-003-C-throughput-initializes-configured-workers-before-ready.md) — deps: M3-003-B — #386
2. [M3-003-D — Expose profile in inspect/config](tasks/06_m3_multi_worker/M3-003-D-expose-profile-in-inspect-config.md) — deps: M3-003-C — #387
3. [M3-003-V — Verify Implement serverless, service, and throughput profiles](tasks/06_m3_multi_worker/M3-003-V-verify-implement-serverless-service-and-throughput-profiles.md) — deps: M3-003-A, M3-003-B, M3-003-C, M3-003-D — #388
4. [M3-003-Z — Package evidence for Implement serverless, service, and throughput profiles](tasks/06_m3_multi_worker/M3-003-Z-package-evidence-for-implement-serverless-service-and-throughput-profiles.md) — deps: M3-003-V — #389
5. [M3-004-A — Share immutable mapped QPack bytes](tasks/06_m3_multi_worker/M3-004-A-share-immutable-mapped-qpack-bytes.md) — deps: M3-002-Z, M26-GATE — #390
6. [M3-004-B — Create separate QuickJS runtimes/functions/context state](tasks/06_m3_multi_worker/M3-004-B-create-separate-quickjs-runtimes-functions-context-state.md) — deps: M3-004-A — #391
7. [M3-004-C — Validate capability compatibility per worker](tasks/06_m3_multi_worker/M3-004-C-validate-capability-compatibility-per-worker.md) — deps: M3-004-B — #392
8. [M3-004-D — Bound startup parallelism](tasks/06_m3_multi_worker/M3-004-D-bound-startup-parallelism.md) — deps: M3-004-C — #393
9. [M3-004-V — Verify Implement deterministic worker initialization and artifact sharing](tasks/06_m3_multi_worker/M3-004-V-verify-implement-deterministic-worker-initialization-and-artifact-sharing.md) — deps: M3-004-A, M3-004-B, M3-004-C, M3-004-D — #394
10. [M3-004-Z — Package evidence for Implement deterministic worker initialization and artifact sharing](tasks/06_m3_multi_worker/M3-004-Z-package-evidence-for-implement-deterministic-worker-initialization-and-artifact.md) — deps: M3-004-V — #395
11. [M3-005-A — Remove quarantined worker from dispatch](tasks/06_m3_multi_worker/M3-005-A-remove-quarantined-worker-from-dispatch.md) — deps: M3-002-Z, M3-004-Z — #396
12. [M3-005-B — Fail/settle its pending work](tasks/06_m3_multi_worker/M3-005-B-fail-settle-its-pending-work.md) — deps: M3-005-A — #397
13. [M3-005-C — Initialize replacement under bounded policy](tasks/06_m3_multi_worker/M3-005-C-initialize-replacement-under-bounded-policy.md) — deps: M3-005-B — #398
14. [M3-005-D — Aggregate readiness from usable capacity](tasks/06_m3_multi_worker/M3-005-D-aggregate-readiness-from-usable-capacity.md) — deps: M3-005-C — #399
15. [M3-005-V — Verify Implement quarantine, replacement, and readiness aggregation](tasks/06_m3_multi_worker/M3-005-V-verify-implement-quarantine-replacement-and-readiness-aggregation.md) — deps: M3-005-A, M3-005-B, M3-005-C, M3-005-D — #400
16. [M3-005-Z — Package evidence for Implement quarantine, replacement, and readiness aggregation](tasks/06_m3_multi_worker/M3-005-Z-package-evidence-for-implement-quarantine-replacement-and-readiness-aggregation.md) — deps: M3-005-V — #401
17. [M3-006-A — Define thresholds/hysteresis](tasks/06_m3_multi_worker/M3-006-A-define-thresholds-hysteresis.md) — deps: M3-003-Z, M3-005-Z — #402
18. [M3-006-B — Bound min/max workers](tasks/06_m3_multi_worker/M3-006-B-bound-min-max-workers.md) — deps: M3-006-A — #403
19. [M3-006-C — Drain before scale-down](tasks/06_m3_multi_worker/M3-006-C-drain-before-scale-down.md) — deps: M3-006-B — #404
20. [M3-006-D — Avoid oscillation](tasks/06_m3_multi_worker/M3-006-D-avoid-oscillation.md) — deps: M3-006-C — #405
21. [M3-006-V — Verify Implement adaptive scale-up and scale-down](tasks/06_m3_multi_worker/M3-006-V-verify-implement-adaptive-scale-up-and-scale-down.md) — deps: M3-006-A, M3-006-B, M3-006-C, M3-006-D — #406
22. [M3-006-Z — Package evidence for Implement adaptive scale-up and scale-down](tasks/06_m3_multi_worker/M3-006-Z-package-evidence-for-implement-adaptive-scale-up-and-scale-down.md) — deps: M3-006-V — #407
23. [M3-007-A — Track invocation-to-worker ownership](tasks/06_m3_multi_worker/M3-007-A-track-invocation-to-worker-ownership.md) — deps: M3-002-Z, M3-004-Z — #408
24. [M3-007-B — Stop admission on drain](tasks/06_m3_multi_worker/M3-007-B-stop-admission-on-drain.md) — deps: M3-007-A — #409
25. [M3-007-C — Allow bounded in-flight completion](tasks/06_m3_multi_worker/M3-007-C-allow-bounded-in-flight-completion.md) — deps: M3-007-B — #410
