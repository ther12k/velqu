# Next 25 Dependency-Safe Tasks

The next dependency-ready implementation task is M3-001-B; this short queue lists the first 25 unchecked packets with their issue numbers.

1. [M3-001-B — Document module-level state replication](tasks/06_m3_multi_worker/M3-001-B-document-module-level-state-replication.md) — deps: M3-001-A — #373
2. [M3-001-C — Forbid JSValue sharing](tasks/06_m3_multi_worker/M3-001-C-forbid-jsvalue-sharing.md) — deps: M3-001-B — #374
3. [M3-001-D — Define service/capability shared handles and thread safety](tasks/06_m3_multi_worker/M3-001-D-define-service-capability-shared-handles-and-thread-safety.md) — deps: M3-001-C — #375
4. [M3-001-V — Verify Freeze independent-worker state semantics](tasks/06_m3_multi_worker/M3-001-V-verify-freeze-independent-worker-state-semantics.md) — deps: M3-001-A, M3-001-B, M3-001-C, M3-001-D — #376
5. [M3-001-Z — Package evidence for Freeze independent-worker state semantics](tasks/06_m3_multi_worker/M3-001-Z-package-evidence-for-freeze-independent-worker-state-semantics.md) — deps: M3-001-V — #377
6. [M3-002-A — Use bounded per-worker queues](tasks/06_m3_multi_worker/M3-002-A-use-bounded-per-worker-queues.md) — deps: M3-001-Z — #378
7. [M3-002-B — Select worker using outstanding-load strategy](tasks/06_m3_multi_worker/M3-002-B-select-worker-using-outstanding-load-strategy.md) — deps: M3-002-A — #379
8. [M3-002-C — Define admission and overload response](tasks/06_m3_multi_worker/M3-002-C-define-admission-and-overload-response.md) — deps: M3-002-B — #380
9. [M3-002-D — Preserve RouteId/RoutePlan before dispatch](tasks/06_m3_multi_worker/M3-002-D-preserve-routeid-routeplan-before-dispatch.md) — deps: M3-002-C — #381
10. [M3-002-V — Verify Implement bounded worker dispatcher](tasks/06_m3_multi_worker/M3-002-V-verify-implement-bounded-worker-dispatcher.md) — deps: M3-002-A, M3-002-B, M3-002-C, M3-002-D — #382
11. [M3-002-Z — Package evidence for Implement bounded worker dispatcher](tasks/06_m3_multi_worker/M3-002-Z-package-evidence-for-implement-bounded-worker-dispatcher.md) — deps: M3-002-V — #383
12. [M3-003-A — Serverless starts one worker only](tasks/06_m3_multi_worker/M3-003-A-serverless-starts-one-worker-only.md) — deps: M3-002-Z — #384
13. [M3-003-B — Service marks ready after worker 0 and adds workers adaptively](tasks/06_m3_multi_worker/M3-003-B-service-marks-ready-after-worker-0-and-adds-workers-adaptively.md) — deps: M3-003-A — #385
14. [M3-003-C — Throughput initializes configured workers before ready](tasks/06_m3_multi_worker/M3-003-C-throughput-initializes-configured-workers-before-ready.md) — deps: M3-003-B — #386
15. [M3-003-D — Expose profile in inspect/config](tasks/06_m3_multi_worker/M3-003-D-expose-profile-in-inspect-config.md) — deps: M3-003-C — #387
16. [M3-003-V — Verify Implement serverless, service, and throughput profiles](tasks/06_m3_multi_worker/M3-003-V-verify-implement-serverless-service-and-throughput-profiles.md) — deps: M3-003-A, M3-003-B, M3-003-C, M3-003-D — #388
17. [M3-003-Z — Package evidence for Implement serverless, service, and throughput profiles](tasks/06_m3_multi_worker/M3-003-Z-package-evidence-for-implement-serverless-service-and-throughput-profiles.md) — deps: M3-003-V — #389
18. [M3-004-A — Share immutable mapped QPack bytes](tasks/06_m3_multi_worker/M3-004-A-share-immutable-mapped-qpack-bytes.md) — deps: M3-002-Z, M26-GATE — #390
19. [M3-004-B — Create separate QuickJS runtimes/functions/context state](tasks/06_m3_multi_worker/M3-004-B-create-separate-quickjs-runtimes-functions-context-state.md) — deps: M3-004-A — #391
20. [M3-004-C — Validate capability compatibility per worker](tasks/06_m3_multi_worker/M3-004-C-validate-capability-compatibility-per-worker.md) — deps: M3-004-B — #392
21. [M3-004-D — Bound startup parallelism](tasks/06_m3_multi_worker/M3-004-D-bound-startup-parallelism.md) — deps: M3-004-C — #393
22. [M3-004-V — Verify Implement deterministic worker initialization and artifact sharing](tasks/06_m3_multi_worker/M3-004-V-verify-implement-deterministic-worker-initialization-and-artifact-sharing.md) — deps: M3-004-A, M3-004-B, M3-004-C, M3-004-D — #394
23. [M3-004-Z — Package evidence for Implement deterministic worker initialization and artifact sharing](tasks/06_m3_multi_worker/M3-004-Z-package-evidence-for-implement-deterministic-worker-initialization-and-artifact.md) — deps: M3-004-V — #395
24. [M3-005-A — Remove quarantined worker from dispatch](tasks/06_m3_multi_worker/M3-005-A-remove-quarantined-worker-from-dispatch.md) — deps: M3-002-Z, M3-004-Z — #396
25. [M3-005-B — Fail/settle its pending work](tasks/06_m3_multi_worker/M3-005-B-fail-settle-its-pending-work.md) — deps: M3-005-A — #397
