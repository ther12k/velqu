# Dependency-Safe Execution Queue

All PASS packets are omitted. The first unchecked dependency-ready task is M28-005-B; future milestone tasks remain TODO until implemented and evidenced.

1. [M28-005-B — Use one terminal state for each operation](tasks/05_m28_native_fetch/M28-005-B-use-one-terminal-state-for-each-operation.md) — deps: M28-005-A — #331
2. [M28-005-C — Cancel DNS/connect/body streaming](tasks/05_m28_native_fetch/M28-005-C-cancel-dns-connect-body-streaming.md) — deps: M28-005-B — #332
3. [M28-005-D — Map failures deterministically](tasks/05_m28_native_fetch/M28-005-D-map-failures-deterministically.md) — deps: M28-005-C — #333
4. [M28-005-V — Verify Propagate AbortSignal and route deadlines](tasks/05_m28_native_fetch/M28-005-V-verify-propagate-abortsignal-and-route-deadlines.md) — deps: M28-005-A, M28-005-B, M28-005-C, M28-005-D — #334
5. [M28-005-Z — Package evidence for Propagate AbortSignal and route deadlines](tasks/05_m28_native_fetch/M28-005-Z-package-evidence-for-propagate-abortsignal-and-route-deadlines.md) — deps: M28-005-V — #335
6. [M28-006-A — Bound read/write buffers](tasks/05_m28_native_fetch/M28-006-A-bound-read-write-buffers.md) — deps: M28-004-Z, M28-005-Z — #336
7. [M28-006-B — Propagate downstream backpressure](tasks/05_m28_native_fetch/M28-006-B-propagate-downstream-backpressure.md) — deps: M28-006-A — #337
8. [M28-006-C — Cancel on consumer stop/disconnect](tasks/05_m28_native_fetch/M28-006-C-cancel-on-consumer-stop-disconnect.md) — deps: M28-006-B — #338
9. [M28-006-D — Define maximum body helper sizes](tasks/05_m28_native_fetch/M28-006-D-define-maximum-body-helper-sizes.md) — deps: M28-006-C — #339
10. [M28-006-V — Verify Implement streaming and strict backpressure](tasks/05_m28_native_fetch/M28-006-V-verify-implement-streaming-and-strict-backpressure.md) — deps: M28-006-A, M28-006-B, M28-006-C, M28-006-D — #340
11. [M28-006-Z — Package evidence for Implement streaming and strict backpressure](tasks/05_m28_native_fetch/M28-006-Z-package-evidence-for-implement-streaming-and-strict-backpressure.md) — deps: M28-006-V — #341
12. [M28-007-A — Limit redirect count](tasks/05_m28_native_fetch/M28-007-A-limit-redirect-count.md) — deps: M28-003-Z, M28-004-Z — #342
13. [M28-007-B — Reapply SSRF/DNS policy on every hop](tasks/05_m28_native_fetch/M28-007-B-reapply-ssrf-dns-policy-on-every-hop.md) — deps: M28-007-A — #343
14. [M28-007-C — Define credential/header stripping](tasks/05_m28_native_fetch/M28-007-C-define-credential-header-stripping.md) — deps: M28-007-B — #344
15. [M28-007-D — Bound decompression ratio and output](tasks/05_m28_native_fetch/M28-007-D-bound-decompression-ratio-and-output.md) — deps: M28-007-C — #345
16. [M28-007-V — Verify Implement redirect and compression policy](tasks/05_m28_native_fetch/M28-007-V-verify-implement-redirect-and-compression-policy.md) — deps: M28-007-A, M28-007-B, M28-007-C, M28-007-D — #346
17. [M28-007-Z — Package evidence for Implement redirect and compression policy](tasks/05_m28_native_fetch/M28-007-Z-package-evidence-for-implement-redirect-and-compression-policy.md) — deps: M28-007-V — #347
18. [M28-008-A — Resolve and validate addresses before connect](tasks/05_m28_native_fetch/M28-008-A-resolve-and-validate-addresses-before-connect.md) — deps: M28-001-Z, M28-003-Z, M28-007-Z — #348
19. [M28-008-B — Revalidate redirects and connection targets](tasks/05_m28_native_fetch/M28-008-B-revalidate-redirects-and-connection-targets.md) — deps: M28-008-A — #349
20. [M28-008-C — Support allow/deny configuration](tasks/05_m28_native_fetch/M28-008-C-support-allow-deny-configuration.md) — deps: M28-008-B — #350
21. [M28-008-D — Define proxy interaction](tasks/05_m28_native_fetch/M28-008-D-define-proxy-interaction.md) — deps: M28-008-C — #351
22. [M28-008-V — Verify Implement SSRF and network egress controls](tasks/05_m28_native_fetch/M28-008-V-verify-implement-ssrf-and-network-egress-controls.md) — deps: M28-008-A, M28-008-B, M28-008-C, M28-008-D — #352
23. [M28-008-Z — Package evidence for Implement SSRF and network egress controls](tasks/05_m28_native_fetch/M28-008-Z-package-evidence-for-implement-ssrf-and-network-egress-controls.md) — deps: M28-008-V — #353
24. [M28-009-A — Expose pool wait, DNS, connect, TLS, TTFB, body, errors, cancellations](tasks/05_m28_native_fetch/M28-009-A-expose-pool-wait-dns-connect-tls-ttfb-body-errors-cancellations.md) — deps: M28-003-Z, M28-005-Z, M28-006-Z — #354
25. [M28-009-B — Sample/aggregate metrics](tasks/05_m28_native_fetch/M28-009-B-sample-aggregate-metrics.md) — deps: M28-009-A — #355
26. [M28-009-C — Drain pool on shutdown](tasks/05_m28_native_fetch/M28-009-C-drain-pool-on-shutdown.md) — deps: M28-009-B — #356
27. [M28-009-D — Quarantine rejects new work](tasks/05_m28_native_fetch/M28-009-D-quarantine-rejects-new-work.md) — deps: M28-009-C — #357
28. [M28-009-V — Verify Integrate lifecycle, observability, and shutdown](tasks/05_m28_native_fetch/M28-009-V-verify-integrate-lifecycle-observability-and-shutdown.md) — deps: M28-009-A, M28-009-B, M28-009-C, M28-009-D — #358
29. [M28-009-Z — Package evidence for Integrate lifecycle, observability, and shutdown](tasks/05_m28_native_fetch/M28-009-Z-package-evidence-for-integrate-lifecycle-observability-and-shutdown.md) — deps: M28-009-V — #359
30. [M28-010-A — Run selected WPT cases](tasks/05_m28_native_fetch/M28-010-A-run-selected-wpt-cases.md) — deps: M28-004-Z, M28-005-Z, M28-006-Z, M28-007-Z, M28-008-Z — #360
31. [M28-010-B — Create deterministic DNS/TLS/redirect/slow/body fixtures](tasks/05_m28_native_fetch/M28-010-B-create-deterministic-dns-tls-redirect-slow-body-fixtures.md) — deps: M28-010-A — #361
32. [M28-010-C — Fuzz headers and URLs](tasks/05_m28_native_fetch/M28-010-C-fuzz-headers-and-urls.md) — deps: M28-010-B — #362
33. [M28-010-D — Test proxy and cancellation](tasks/05_m28_native_fetch/M28-010-D-test-proxy-and-cancellation.md) — deps: M28-010-C — #363
34. [M28-010-V — Verify Complete fetch conformance and fault testing](tasks/05_m28_native_fetch/M28-010-V-verify-complete-fetch-conformance-and-fault-testing.md) — deps: M28-010-A, M28-010-B, M28-010-C, M28-010-D — #364
35. [M28-010-Z — Package evidence for Complete fetch conformance and fault testing](tasks/05_m28_native_fetch/M28-010-Z-package-evidence-for-complete-fetch-conformance-and-fault-testing.md) — deps: M28-010-V — #365
36. [M28-011-A — Run 1/5/10/25ms upstream latency](tasks/05_m28_native_fetch/M28-011-A-run-1-5-10-25ms-upstream-latency.md) — deps: M28-009-Z, M28-010-Z — #366
37. [M28-011-B — Run one, two, and four parallel calls](tasks/05_m28_native_fetch/M28-011-B-run-one-two-and-four-parallel-calls.md) — deps: M28-011-A — #367
38. [M28-011-C — Mix timeout/success/malformed responses](tasks/05_m28_native_fetch/M28-011-C-mix-timeout-success-malformed-responses.md) — deps: M28-011-B — #368
39. [M28-011-D — Test concurrency 1/10/50/200](tasks/05_m28_native_fetch/M28-011-D-test-concurrency-1-10-50-200.md) — deps: M28-011-C — #369
40. [M28-011-V — Verify Run controlled upstream and fan-out benchmarks](tasks/05_m28_native_fetch/M28-011-V-verify-run-controlled-upstream-and-fan-out-benchmarks.md) — deps: M28-011-A, M28-011-B, M28-011-C, M28-011-D — #370
41. [M28-011-Z — Package evidence for Run controlled upstream and fan-out benchmarks](tasks/05_m28_native_fetch/M28-011-Z-package-evidence-for-run-controlled-upstream-and-fan-out-benchmarks.md) — deps: M28-011-V — #371
42. [M28-GATE — M2.8 — Native Outbound Fetch exit gate](gates/M28-GATE.md) — deps: M28-001-Z, M28-002-Z, M28-003-Z, M28-004-Z, M28-005-Z, M28-006-Z, M28-007-Z, M28-008-Z, M28-009-Z, M28-010-Z, M28-011-Z — #631
43. [M3-001-A — Accept ADR](tasks/06_m3_multi_worker/M3-001-A-accept-adr.md) — deps: M28-GATE — #372
44. [M3-001-B — Document module-level state replication](tasks/06_m3_multi_worker/M3-001-B-document-module-level-state-replication.md) — deps: M3-001-A — #373
45. [M3-001-C — Forbid JSValue sharing](tasks/06_m3_multi_worker/M3-001-C-forbid-jsvalue-sharing.md) — deps: M3-001-B — #374
46. [M3-001-D — Define service/capability shared handles and thread safety](tasks/06_m3_multi_worker/M3-001-D-define-service-capability-shared-handles-and-thread-safety.md) — deps: M3-001-C — #375
47. [M3-001-V — Verify Freeze independent-worker state semantics](tasks/06_m3_multi_worker/M3-001-V-verify-freeze-independent-worker-state-semantics.md) — deps: M3-001-A, M3-001-B, M3-001-C, M3-001-D — #376
48. [M3-001-Z — Package evidence for Freeze independent-worker state semantics](tasks/06_m3_multi_worker/M3-001-Z-package-evidence-for-freeze-independent-worker-state-semantics.md) — deps: M3-001-V — #377
49. [M3-002-A — Use bounded per-worker queues](tasks/06_m3_multi_worker/M3-002-A-use-bounded-per-worker-queues.md) — deps: M3-001-Z — #378
50. [M3-002-B — Select worker using outstanding-load strategy](tasks/06_m3_multi_worker/M3-002-B-select-worker-using-outstanding-load-strategy.md) — deps: M3-002-A — #379
51. [M3-002-C — Define admission and overload response](tasks/06_m3_multi_worker/M3-002-C-define-admission-and-overload-response.md) — deps: M3-002-B — #380
52. [M3-002-D — Preserve RouteId/RoutePlan before dispatch](tasks/06_m3_multi_worker/M3-002-D-preserve-routeid-routeplan-before-dispatch.md) — deps: M3-002-C — #381
53. [M3-002-V — Verify Implement bounded worker dispatcher](tasks/06_m3_multi_worker/M3-002-V-verify-implement-bounded-worker-dispatcher.md) — deps: M3-002-A, M3-002-B, M3-002-C, M3-002-D — #382
54. [M3-002-Z — Package evidence for Implement bounded worker dispatcher](tasks/06_m3_multi_worker/M3-002-Z-package-evidence-for-implement-bounded-worker-dispatcher.md) — deps: M3-002-V — #383
55. [M3-003-A — Serverless starts one worker only](tasks/06_m3_multi_worker/M3-003-A-serverless-starts-one-worker-only.md) — deps: M3-002-Z — #384
56. [M3-003-B — Service marks ready after worker 0 and adds workers adaptively](tasks/06_m3_multi_worker/M3-003-B-service-marks-ready-after-worker-0-and-adds-workers-adaptively.md) — deps: M3-003-A — #385
57. [M3-003-C — Throughput initializes configured workers before ready](tasks/06_m3_multi_worker/M3-003-C-throughput-initializes-configured-workers-before-ready.md) — deps: M3-003-B — #386
58. [M3-003-D — Expose profile in inspect/config](tasks/06_m3_multi_worker/M3-003-D-expose-profile-in-inspect-config.md) — deps: M3-003-C — #387
59. [M3-003-V — Verify Implement serverless, service, and throughput profiles](tasks/06_m3_multi_worker/M3-003-V-verify-implement-serverless-service-and-throughput-profiles.md) — deps: M3-003-A, M3-003-B, M3-003-C, M3-003-D — #388
60. [M3-003-Z — Package evidence for Implement serverless, service, and throughput profiles](tasks/06_m3_multi_worker/M3-003-Z-package-evidence-for-implement-serverless-service-and-throughput-profiles.md) — deps: M3-003-V — #389
61. [M3-004-A — Share immutable mapped QPack bytes](tasks/06_m3_multi_worker/M3-004-A-share-immutable-mapped-qpack-bytes.md) — deps: M3-002-Z, M26-GATE — #390
62. [M3-004-B — Create separate QuickJS runtimes/functions/context state](tasks/06_m3_multi_worker/M3-004-B-create-separate-quickjs-runtimes-functions-context-state.md) — deps: M3-004-A — #391
63. [M3-004-C — Validate capability compatibility per worker](tasks/06_m3_multi_worker/M3-004-C-validate-capability-compatibility-per-worker.md) — deps: M3-004-B — #392
64. [M3-004-D — Bound startup parallelism](tasks/06_m3_multi_worker/M3-004-D-bound-startup-parallelism.md) — deps: M3-004-C — #393
65. [M3-004-V — Verify Implement deterministic worker initialization and artifact sharing](tasks/06_m3_multi_worker/M3-004-V-verify-implement-deterministic-worker-initialization-and-artifact-sharing.md) — deps: M3-004-A, M3-004-B, M3-004-C, M3-004-D — #394
66. [M3-004-Z — Package evidence for Implement deterministic worker initialization and artifact sharing](tasks/06_m3_multi_worker/M3-004-Z-package-evidence-for-implement-deterministic-worker-initialization-and-artifact.md) — deps: M3-004-V — #395
67. [M3-005-A — Remove quarantined worker from dispatch](tasks/06_m3_multi_worker/M3-005-A-remove-quarantined-worker-from-dispatch.md) — deps: M3-002-Z, M3-004-Z — #396
68. [M3-005-B — Fail/settle its pending work](tasks/06_m3_multi_worker/M3-005-B-fail-settle-its-pending-work.md) — deps: M3-005-A — #397
69. [M3-005-C — Initialize replacement under bounded policy](tasks/06_m3_multi_worker/M3-005-C-initialize-replacement-under-bounded-policy.md) — deps: M3-005-B — #398
70. [M3-005-D — Aggregate readiness from usable capacity](tasks/06_m3_multi_worker/M3-005-D-aggregate-readiness-from-usable-capacity.md) — deps: M3-005-C — #399
71. [M3-005-V — Verify Implement quarantine, replacement, and readiness aggregation](tasks/06_m3_multi_worker/M3-005-V-verify-implement-quarantine-replacement-and-readiness-aggregation.md) — deps: M3-005-A, M3-005-B, M3-005-C, M3-005-D — #400
72. [M3-005-Z — Package evidence for Implement quarantine, replacement, and readiness aggregation](tasks/06_m3_multi_worker/M3-005-Z-package-evidence-for-implement-quarantine-replacement-and-readiness-aggregation.md) — deps: M3-005-V — #401
73. [M3-006-A — Define thresholds/hysteresis](tasks/06_m3_multi_worker/M3-006-A-define-thresholds-hysteresis.md) — deps: M3-003-Z, M3-005-Z — #402
74. [M3-006-B — Bound min/max workers](tasks/06_m3_multi_worker/M3-006-B-bound-min-max-workers.md) — deps: M3-006-A — #403
75. [M3-006-C — Drain before scale-down](tasks/06_m3_multi_worker/M3-006-C-drain-before-scale-down.md) — deps: M3-006-B — #404
76. [M3-006-D — Avoid oscillation](tasks/06_m3_multi_worker/M3-006-D-avoid-oscillation.md) — deps: M3-006-C — #405
77. [M3-006-V — Verify Implement adaptive scale-up and scale-down](tasks/06_m3_multi_worker/M3-006-V-verify-implement-adaptive-scale-up-and-scale-down.md) — deps: M3-006-A, M3-006-B, M3-006-C, M3-006-D — #406
78. [M3-006-Z — Package evidence for Implement adaptive scale-up and scale-down](tasks/06_m3_multi_worker/M3-006-Z-package-evidence-for-implement-adaptive-scale-up-and-scale-down.md) — deps: M3-006-V — #407
79. [M3-007-A — Track invocation-to-worker ownership](tasks/06_m3_multi_worker/M3-007-A-track-invocation-to-worker-ownership.md) — deps: M3-002-Z, M3-004-Z — #408
80. [M3-007-B — Stop admission on drain](tasks/06_m3_multi_worker/M3-007-B-stop-admission-on-drain.md) — deps: M3-007-A — #409
81. [M3-007-C — Allow bounded in-flight completion](tasks/06_m3_multi_worker/M3-007-C-allow-bounded-in-flight-completion.md) — deps: M3-007-B — #410
82. [M3-007-D — Abort after shutdown deadline](tasks/06_m3_multi_worker/M3-007-D-abort-after-shutdown-deadline.md) — deps: M3-007-C — #411
83. [M3-007-V — Verify Implement multi-worker cancellation and graceful shutdown](tasks/06_m3_multi_worker/M3-007-V-verify-implement-multi-worker-cancellation-and-graceful-shutdown.md) — deps: M3-007-A, M3-007-B, M3-007-C, M3-007-D — #412
84. [M3-007-Z — Package evidence for Implement multi-worker cancellation and graceful shutdown](tasks/06_m3_multi_worker/M3-007-Z-package-evidence-for-implement-multi-worker-cancellation-and-graceful-shutdown.md) — deps: M3-007-V — #413
85. [M3-008-A — Add route/global queue limits or weighted admission](tasks/06_m3_multi_worker/M3-008-A-add-route-global-queue-limits-or-weighted-admission.md) — deps: M3-002-Z, M3-006-Z — #414
86. [M3-008-B — Define long-running JS policy](tasks/06_m3_multi_worker/M3-008-B-define-long-running-js-policy.md) — deps: M3-008-A — #415
87. [M3-008-C — Expose load-shed reasons](tasks/06_m3_multi_worker/M3-008-C-expose-load-shed-reasons.md) — deps: M3-008-B — #416
88. [M3-008-D — Test mixed workloads](tasks/06_m3_multi_worker/M3-008-D-test-mixed-workloads.md) — deps: M3-008-C — #417
89. [M3-008-V — Verify Add fairness and overload controls](tasks/06_m3_multi_worker/M3-008-V-verify-add-fairness-and-overload-controls.md) — deps: M3-008-A, M3-008-B, M3-008-C, M3-008-D — #418
90. [M3-008-Z — Package evidence for Add fairness and overload controls](tasks/06_m3_multi_worker/M3-008-Z-package-evidence-for-add-fairness-and-overload-controls.md) — deps: M3-008-V — #419
91. [M3-009-A — Measure 1/2/4 workers](tasks/06_m3_multi_worker/M3-009-A-measure-1-2-4-workers.md) — deps: M3-003-Z, M3-006-Z, M3-008-Z — #420
92. [M3-009-B — Report throughput, p50/p95/p99, queue time, CPU, RSS, errors](tasks/06_m3_multi_worker/M3-009-B-report-throughput-p50-p95-p99-queue-time-cpu-rss-errors.md) — deps: M3-009-A — #421
93. [M3-009-C — Run C1/C2/C3 and controlled I/O](tasks/06_m3_multi_worker/M3-009-C-run-c1-c2-c3-and-controlled-i-o.md) — deps: M3-009-B — #422
94. [M3-009-D — Record physical core topology](tasks/06_m3_multi_worker/M3-009-D-record-physical-core-topology.md) — deps: M3-009-C — #423
95. [M3-009-V — Verify Close multi-worker scaling and memory evidence](tasks/06_m3_multi_worker/M3-009-V-verify-close-multi-worker-scaling-and-memory-evidence.md) — deps: M3-009-A, M3-009-B, M3-009-C, M3-009-D — #424
96. [M3-009-Z — Package evidence for Close multi-worker scaling and memory evidence](tasks/06_m3_multi_worker/M3-009-Z-package-evidence-for-close-multi-worker-scaling-and-memory-evidence.md) — deps: M3-009-V — #425
97. [M3-010-A — Run multi-hour mixed load](tasks/06_m3_multi_worker/M3-010-A-run-multi-hour-mixed-load.md) — deps: M3-005-Z, M3-007-Z, M3-009-Z — #426
98. [M3-010-B — Inject worker poison, upstream timeout, disconnect, and shutdown](tasks/06_m3_multi_worker/M3-010-B-inject-worker-poison-upstream-timeout-disconnect-and-shutdown.md) — deps: M3-010-A — #427
99. [M3-010-C — Track retained memory and task/slot counts](tasks/06_m3_multi_worker/M3-010-C-track-retained-memory-and-task-slot-counts.md) — deps: M3-010-B — #428
100. [M3-010-D — Verify recovery](tasks/06_m3_multi_worker/M3-010-D-verify-recovery.md) — deps: M3-010-C — #429
101. [M3-010-V — Verify Run multi-worker soak and recovery](tasks/06_m3_multi_worker/M3-010-V-verify-run-multi-worker-soak-and-recovery.md) — deps: M3-010-A, M3-010-B, M3-010-C, M3-010-D — #430
102. [M3-010-Z — Package evidence for Run multi-worker soak and recovery](tasks/06_m3_multi_worker/M3-010-Z-package-evidence-for-run-multi-worker-soak-and-recovery.md) — deps: M3-010-V — #431
103. [M3-GATE — M3 — Multi-Worker Service Runtime exit gate](gates/M3-GATE.md) — deps: M3-001-Z, M3-002-Z, M3-003-Z, M3-004-Z, M3-005-Z, M3-006-Z, M3-007-Z, M3-008-Z, M3-009-Z, M3-010-Z — #632
104. [M4A-001-A — Watch source and contracts](tasks/07_m4a_developer_preview/M4A-001-A-watch-source-and-contracts.md) — deps: M3-GATE — #432
105. [M4A-001-B — Build incremental temporary QPack](tasks/07_m4a_developer_preview/M4A-001-B-build-incremental-temporary-qpack.md) — deps: M4A-001-A — #433
106. [M4A-001-C — Load new worker before switching traffic](tasks/07_m4a_developer_preview/M4A-001-C-load-new-worker-before-switching-traffic.md) — deps: M4A-001-B — #434
107. [M4A-001-D — Drain old worker and surface compile/runtime errors](tasks/07_m4a_developer_preview/M4A-001-D-drain-old-worker-and-surface-compile-runtime-errors.md) — deps: M4A-001-C — #435
108. [M4A-001-V — Verify Implement actual-runtime `velqu dev` loop](tasks/07_m4a_developer_preview/M4A-001-V-verify-implement-actual-runtime-velqu-dev-loop.md) — deps: M4A-001-A, M4A-001-B, M4A-001-C, M4A-001-D — #436
109. [M4A-001-Z — Package evidence for Implement actual-runtime `velqu dev` loop](tasks/07_m4a_developer_preview/M4A-001-Z-package-evidence-for-implement-actual-runtime-velqu-dev-loop.md) — deps: M4A-001-V — #437
110. [M4A-002-A — Implement and document `velqu dev`, `build`, `inspect`, `contract diff`, `test`, `pack inspect/migrate`, and diagnostics](tasks/07_m4a_developer_preview/M4A-002-A-implement-and-document-velqu-dev-build-inspect-contract-diff-test-pack-inspect-m.md) — deps: M4A-001-Z, M26-GATE — #438
111. [M4A-002-B — Stable exit codes](tasks/07_m4a_developer_preview/M4A-002-B-stable-exit-codes.md) — deps: M4A-002-A — #439
112. [M4A-002-C — Machine-readable output option](tasks/07_m4a_developer_preview/M4A-002-C-machine-readable-output-option.md) — deps: M4A-002-B — #440
113. [M4A-002-D — Helpful actionable errors](tasks/07_m4a_developer_preview/M4A-002-D-helpful-actionable-errors.md) — deps: M4A-002-C — #441
114. [M4A-002-V — Verify Complete CLI command surface](tasks/07_m4a_developer_preview/M4A-002-V-verify-complete-cli-command-surface.md) — deps: M4A-002-A, M4A-002-B, M4A-002-C, M4A-002-D — #442
115. [M4A-002-Z — Package evidence for Complete CLI command surface](tasks/07_m4a_developer_preview/M4A-002-Z-package-evidence-for-complete-cli-command-surface.md) — deps: M4A-002-V — #443
116. [M4A-003-A — Starter API](tasks/07_m4a_developer_preview/M4A-003-A-starter-api.md) — deps: M4A-002-Z — #444
117. [M4A-003-B — Treaty client example](tasks/07_m4a_developer_preview/M4A-003-B-treaty-client-example.md) — deps: M4A-003-A — #445
118. [M4A-003-C — Testing setup](tasks/07_m4a_developer_preview/M4A-003-C-testing-setup.md) — deps: M4A-003-B — #446
119. [M4A-003-D — Optional fetch/profile choices](tasks/07_m4a_developer_preview/M4A-003-D-optional-fetch-profile-choices.md) — deps: M4A-003-C — #447
120. [M4A-003-V — Verify Implement project scaffolding](tasks/07_m4a_developer_preview/M4A-003-V-verify-implement-project-scaffolding.md) — deps: M4A-003-A, M4A-003-B, M4A-003-C, M4A-003-D — #448
121. [M4A-003-Z — Package evidence for Implement project scaffolding](tasks/07_m4a_developer_preview/M4A-003-Z-package-evidence-for-implement-project-scaffolding.md) — deps: M4A-003-V — #449
122. [M4A-004-A — Unit-local direct generated dispatcher](tasks/07_m4a_developer_preview/M4A-004-A-unit-local-direct-generated-dispatcher.md) — deps: M25-GATE, M4A-001-Z — #450
123. [M4A-004-B — Runtime-local actual Rust/QuickJS process](tasks/07_m4a_developer_preview/M4A-004-B-runtime-local-actual-rust-quickjs-process.md) — deps: M4A-004-A — #451
124. [M4A-004-C — Remote fetch client](tasks/07_m4a_developer_preview/M4A-004-C-remote-fetch-client.md) — deps: M4A-004-B — #452
125. [M4A-004-D — Exact method/body/query/status/problem typing](tasks/07_m4a_developer_preview/M4A-004-D-exact-method-body-query-status-problem-typing.md) — deps: M4A-004-C — #453
126. [M4A-004-V — Verify Complete Treaty unit-local, runtime-local, and remote modes](tasks/07_m4a_developer_preview/M4A-004-V-verify-complete-treaty-unit-local-runtime-local-and-remote-modes.md) — deps: M4A-004-A, M4A-004-B, M4A-004-C, M4A-004-D — #454
127. [M4A-004-Z — Package evidence for Complete Treaty unit-local, runtime-local, and remote modes](tasks/07_m4a_developer_preview/M4A-004-Z-package-evidence-for-complete-treaty-unit-local-runtime-local-and-remote-modes.md) — deps: M4A-004-V — #455
128. [M4A-005-A — Generate d.ts/client/OpenAPI/contract lock](tasks/07_m4a_developer_preview/M4A-005-A-generate-d-ts-client-openapi-contract-lock.md) — deps: M4A-004-Z — #456
129. [M4A-005-B — Tree-shakable client](tasks/07_m4a_developer_preview/M4A-005-B-tree-shakable-client.md) — deps: M4A-005-A — #457
130. [M4A-005-C — Version and public contract hash](tasks/07_m4a_developer_preview/M4A-005-C-version-and-public-contract-hash.md) — deps: M4A-005-B — #458
131. [M4A-005-D — Package verification](tasks/07_m4a_developer_preview/M4A-005-D-package-verification.md) — deps: M4A-005-C — #459
132. [M4A-005-V — Verify Publish compact contract and SDK artifacts](tasks/07_m4a_developer_preview/M4A-005-V-verify-publish-compact-contract-and-sdk-artifacts.md) — deps: M4A-005-A, M4A-005-B, M4A-005-C, M4A-005-D — #460
133. [M4A-005-Z — Package evidence for Publish compact contract and SDK artifacts](tasks/07_m4a_developer_preview/M4A-005-Z-package-evidence-for-publish-compact-contract-and-sdk-artifacts.md) — deps: M4A-005-V — #461
134. [M4A-006-A — Structured diagnostic codes](tasks/07_m4a_developer_preview/M4A-006-A-structured-diagnostic-codes.md) — deps: M4A-001-Z, M4A-002-Z — #462
135. [M4A-006-B — Source-map-aware stacks](tasks/07_m4a_developer_preview/M4A-006-B-source-map-aware-stacks.md) — deps: M4A-006-A — #463
136. [M4A-006-C — Redaction policy](tasks/07_m4a_developer_preview/M4A-006-C-redaction-policy.md) — deps: M4A-006-B — #464
137. [M4A-006-D — Inspect route plan, fields, codecs, capabilities, crossings, and debug names](tasks/07_m4a_developer_preview/M4A-006-D-inspect-route-plan-fields-codecs-capabilities-crossings-and-debug-names.md) — deps: M4A-006-C — #465
138. [M4A-006-V — Verify Finalize diagnostics, source maps, and inspect output](tasks/07_m4a_developer_preview/M4A-006-V-verify-finalize-diagnostics-source-maps-and-inspect-output.md) — deps: M4A-006-A, M4A-006-B, M4A-006-C, M4A-006-D — #466
139. [M4A-006-Z — Package evidence for Finalize diagnostics, source maps, and inspect output](tasks/07_m4a_developer_preview/M4A-006-Z-package-evidence-for-finalize-diagnostics-source-maps-and-inspect-output.md) — deps: M4A-006-V — #467
140. [M4A-007-A — Define deferred owner, queue, deadline, cancellation, shutdown](tasks/07_m4a_developer_preview/M4A-007-A-define-deferred-owner-queue-deadline-cancellation-shutdown.md) — deps: M27-GATE, M3-GATE — #468
141. [M4A-007-B — Separate cleanup from best-effort work](tasks/07_m4a_developer_preview/M4A-007-B-separate-cleanup-from-best-effort-work.md) — deps: M4A-007-A — #469
142. [M4A-007-C — Expose metrics](tasks/07_m4a_developer_preview/M4A-007-C-expose-metrics.md) — deps: M4A-007-B — #470
143. [M4A-007-D — Forbid unbounded recursive spawning](tasks/07_m4a_developer_preview/M4A-007-D-forbid-unbounded-recursive-spawning.md) — deps: M4A-007-C — #471
144. [M4A-007-V — Verify Implement bounded `defer` and lifecycle hooks](tasks/07_m4a_developer_preview/M4A-007-V-verify-implement-bounded-defer-and-lifecycle-hooks.md) — deps: M4A-007-A, M4A-007-B, M4A-007-C, M4A-007-D — #472
145. [M4A-007-Z — Package evidence for Implement bounded `defer` and lifecycle hooks](tasks/07_m4a_developer_preview/M4A-007-Z-package-evidence-for-implement-bounded-defer-and-lifecycle-hooks.md) — deps: M4A-007-V — #473
146. [M4A-008-A — Quickstart](tasks/07_m4a_developer_preview/M4A-008-A-quickstart.md) — deps: M4A-002-Z, M4A-004-Z, M4A-006-Z — #474
147. [M4A-008-B — Routes/schemas/policies/services](tasks/07_m4a_developer_preview/M4A-008-B-routes-schemas-policies-services.md) — deps: M4A-008-A — #475
148. [M4A-008-C — Treaty](tasks/07_m4a_developer_preview/M4A-008-C-treaty.md) — deps: M4A-008-B — #476
149. [M4A-008-D — Fetch/capabilities](tasks/07_m4a_developer_preview/M4A-008-D-fetch-capabilities.md) — deps: M4A-008-C — #477
150. [M4A-008-E — Runtime profiles](tasks/07_m4a_developer_preview/M4A-008-E-runtime-profiles.md) — deps: M4A-008-D — #478
151. [M4A-008-F — Deployment behind reverse proxy](tasks/07_m4a_developer_preview/M4A-008-F-deployment-behind-reverse-proxy.md) — deps: M4A-008-E — #479
152. [M4A-008-G — Limits and non-goals](tasks/07_m4a_developer_preview/M4A-008-G-limits-and-non-goals.md) — deps: M4A-008-F — #480
153. [M4A-008-V — Verify Build documentation and examples](tasks/07_m4a_developer_preview/M4A-008-V-verify-build-documentation-and-examples.md) — deps: M4A-008-A, M4A-008-B, M4A-008-C, M4A-008-D, M4A-008-E, M4A-008-F, M4A-008-G — #481
154. [M4A-008-Z — Package evidence for Build documentation and examples](tasks/07_m4a_developer_preview/M4A-008-Z-package-evidence-for-build-documentation-and-examples.md) — deps: M4A-008-V — #482
155. [M4A-009-A — Feature modules](tasks/07_m4a_developer_preview/M4A-009-A-feature-modules.md) — deps: M4A-004-Z, M4A-007-Z, M28-GATE — #483
156. [M4A-009-B — JWT-like policy reference](tasks/07_m4a_developer_preview/M4A-009-B-jwt-like-policy-reference.md) — deps: M4A-009-A — #484
157. [M4A-009-C — Controlled upstream](tasks/07_m4a_developer_preview/M4A-009-C-controlled-upstream.md) — deps: M4A-009-B — #485
158. [M4A-009-D — Metrics/readiness/shutdown](tasks/07_m4a_developer_preview/M4A-009-D-metrics-readiness-shutdown.md) — deps: M4A-009-C — #486
159. [M4A-009-E — Treaty client](tasks/07_m4a_developer_preview/M4A-009-E-treaty-client.md) — deps: M4A-009-D — #487
160. [M4A-009-V — Verify Build realistic private-alpha proof service](tasks/07_m4a_developer_preview/M4A-009-V-verify-build-realistic-private-alpha-proof-service.md) — deps: M4A-009-A, M4A-009-B, M4A-009-C, M4A-009-D, M4A-009-E — #488
161. [M4A-009-Z — Package evidence for Build realistic private-alpha proof service](tasks/07_m4a_developer_preview/M4A-009-Z-package-evidence-for-build-realistic-private-alpha-proof-service.md) — deps: M4A-009-V — #489
162. [M4A-010-A — Provide clean install packet](tasks/07_m4a_developer_preview/M4A-010-A-provide-clean-install-packet.md) — deps: M4A-003-Z, M4A-008-Z, M4A-009-Z — #490
163. [M4A-010-B — Collect task-based feedback](tasks/07_m4a_developer_preview/M4A-010-B-collect-task-based-feedback.md) — deps: M4A-010-A — #491
164. [M4A-010-C — Classify P0/P1/P2](tasks/07_m4a_developer_preview/M4A-010-C-classify-p0-p1-p2.md) — deps: M4A-010-B — #492
165. [M4A-010-D — Fix beta-blocking findings and publish limitations](tasks/07_m4a_developer_preview/M4A-010-D-fix-beta-blocking-findings-and-publish-limitations.md) — deps: M4A-010-C — #493
166. [M4A-010-V — Verify Run invited developer alpha and close P0/P1 feedback](tasks/07_m4a_developer_preview/M4A-010-V-verify-run-invited-developer-alpha-and-close-p0-p1-feedback.md) — deps: M4A-010-A, M4A-010-B, M4A-010-C, M4A-010-D — #494
167. [M4A-010-Z — Package evidence for Run invited developer alpha and close P0/P1 feedback](tasks/07_m4a_developer_preview/M4A-010-Z-package-evidence-for-run-invited-developer-alpha-and-close-p0-p1-feedback.md) — deps: M4A-010-V — #495
168. [M4A-GATE — M4A — Developer Preview and Private Alpha exit gate](gates/M4A-GATE.md) — deps: M4A-001-Z, M4A-002-Z, M4A-003-Z, M4A-004-Z, M4A-005-Z, M4A-006-Z, M4A-007-Z, M4A-008-Z, M4A-009-Z, M4A-010-Z — #633
169. [BETA-001-V — Verify Make the real-world benchmark harness executable](tasks/08_public_beta/BETA-001-V-verify-make-the-real-world-benchmark-harness-executable.md) — deps: BETA-001-A, BETA-001-B, BETA-001-C, BETA-001-D — #500
170. [BETA-001-Z — Package evidence for Make the real-world benchmark harness executable](tasks/08_public_beta/BETA-001-Z-package-evidence-for-make-the-real-world-benchmark-harness-executable.md) — deps: BETA-001-V — #503
171. [BETA-002-A — Match SQL, pool, JWT, timeouts, logging, responses, compression, and deployment limits](tasks/08_public_beta/BETA-002-A-match-sql-pool-jwt-timeouts-logging-responses-compression-and-deployment-limits.md) — deps: BETA-001-Z — #504
172. [BETA-002-B — Pin versions](tasks/08_public_beta/BETA-002-B-pin-versions.md) — deps: BETA-002-A — #505
173. [BETA-002-C — Add contract-response verification](tasks/08_public_beta/BETA-002-C-add-contract-response-verification.md) — deps: BETA-002-B — #506
174. [BETA-002-D — Document unavoidable differences](tasks/08_public_beta/BETA-002-D-document-unavoidable-differences.md) — deps: BETA-002-C — #507
175. [BETA-002-V — Verify Implement matched competitor candidates](tasks/08_public_beta/BETA-002-V-verify-implement-matched-competitor-candidates.md) — deps: BETA-002-A, BETA-002-B, BETA-002-C, BETA-002-D — #508
176. [BETA-002-Z — Package evidence for Implement matched competitor candidates](tasks/08_public_beta/BETA-002-Z-package-evidence-for-implement-matched-competitor-candidates.md) — deps: BETA-002-V — #509
177. [BETA-003-A — Run 0/1/5/10/25ms I/O, payload matrices, and CPU operation levels](tasks/08_public_beta/BETA-003-A-run-0-1-5-10-25ms-i-o-payload-matrices-and-cpu-operation-levels.md) — deps: BETA-001-Z, M28-GATE, M3-GATE — #510
178. [BETA-003-B — Measure first request through steady state](tasks/08_public_beta/BETA-003-B-measure-first-request-through-steady-state.md) — deps: BETA-003-A — #511
179. [BETA-003-C — Calculate cumulative crossover request counts](tasks/08_public_beta/BETA-003-C-calculate-cumulative-crossover-request-counts.md) — deps: BETA-003-B — #512
180. [BETA-003-D — Report losses honestly](tasks/08_public_beta/BETA-003-D-report-losses-honestly.md) — deps: BETA-003-C — #513
181. [BETA-003-V — Verify Run controlled I/O and CPU/JIT crossover suites](tasks/08_public_beta/BETA-003-V-verify-run-controlled-i-o-and-cpu-jit-crossover-suites.md) — deps: BETA-003-A, BETA-003-B, BETA-003-C, BETA-003-D — #514
182. [BETA-003-Z — Package evidence for Run controlled I/O and CPU/JIT crossover suites](tasks/08_public_beta/BETA-003-Z-package-evidence-for-run-controlled-i-o-and-cpu-jit-crossover-suites.md) — deps: BETA-003-V — #515
183. [BETA-004-A — Use capability ABI](tasks/08_public_beta/BETA-004-A-use-capability-abi.md) — deps: M27-GATE, BETA-001-Z — #516
184. [BETA-004-B — Lazy pool](tasks/08_public_beta/BETA-004-B-lazy-pool.md) — deps: BETA-004-A — #517
185. [BETA-004-C — Parameterized queries/transactions](tasks/08_public_beta/BETA-004-C-parameterized-queries-transactions.md) — deps: BETA-004-B — #518
186. [BETA-004-D — Deadline/cancellation/shutdown](tasks/08_public_beta/BETA-004-D-deadline-cancellation-shutdown.md) — deps: BETA-004-C — #519
187. [BETA-004-E — Pool limits and observability](tasks/08_public_beta/BETA-004-E-pool-limits-and-observability.md) — deps: BETA-004-D — #520
188. [BETA-004-F — No ORM](tasks/08_public_beta/BETA-004-F-no-orm.md) — deps: BETA-004-E — #521
189. [BETA-004-V — Verify Implement optional first-party Postgres capability](tasks/08_public_beta/BETA-004-V-verify-implement-optional-first-party-postgres-capability.md) — deps: BETA-004-A, BETA-004-B, BETA-004-C, BETA-004-D, BETA-004-E, BETA-004-F — #522
190. [BETA-004-Z — Package evidence for Implement optional first-party Postgres capability](tasks/08_public_beta/BETA-004-Z-package-evidence-for-implement-optional-first-party-postgres-capability.md) — deps: BETA-004-V — #523
191. [BETA-005-A — Support one approved JWT algorithm/profile](tasks/08_public_beta/BETA-005-A-support-one-approved-jwt-algorithm-profile.md) — deps: M27-GATE, M25-GATE — #524
192. [BETA-005-B — Key loading/rotation hooks](tasks/08_public_beta/BETA-005-B-key-loading-rotation-hooks.md) — deps: BETA-005-A — #525
193. [BETA-005-C — Expiry/audience/issuer checks](tasks/08_public_beta/BETA-005-C-expiry-audience-issuer-checks.md) — deps: BETA-005-B — #526
194. [BETA-005-D — Typed 401/403 problems](tasks/08_public_beta/BETA-005-D-typed-401-403-problems.md) — deps: BETA-005-C — #527
195. [BETA-005-E — No secret logging](tasks/08_public_beta/BETA-005-E-no-secret-logging.md) — deps: BETA-005-D — #528
196. [BETA-005-V — Verify Implement JWT/auth reference package](tasks/08_public_beta/BETA-005-V-verify-implement-jwt-auth-reference-package.md) — deps: BETA-005-A, BETA-005-B, BETA-005-C, BETA-005-D, BETA-005-E — #529
197. [BETA-005-Z — Package evidence for Implement JWT/auth reference package](tasks/08_public_beta/BETA-005-Z-package-evidence-for-implement-jwt-auth-reference-package.md) — deps: BETA-005-V — #530
198. [BETA-006-A — Request/route/status/duration](tasks/08_public_beta/BETA-006-A-request-route-status-duration.md) — deps: M3-GATE, M28-GATE — #531
199. [BETA-006-B — Worker queues/quarantine/replacements](tasks/08_public_beta/BETA-006-B-worker-queues-quarantine-replacements.md) — deps: BETA-006-A — #532
200. [BETA-006-C — Fetch and DB pools](tasks/08_public_beta/BETA-006-C-fetch-and-db-pools.md) — deps: BETA-006-B — #533
201. [BETA-006-D — Memory/tasks/slots](tasks/08_public_beta/BETA-006-D-memory-tasks-slots.md) — deps: BETA-006-C — #534
202. [BETA-006-E — Optional trace integration or trace IDs](tasks/08_public_beta/BETA-006-E-optional-trace-integration-or-trace-ids.md) — deps: BETA-006-D — #535
203. [BETA-006-F — Redaction](tasks/08_public_beta/BETA-006-F-redaction.md) — deps: BETA-006-E — #536
204. [BETA-006-V — Verify Implement beta observability baseline](tasks/08_public_beta/BETA-006-V-verify-implement-beta-observability-baseline.md) — deps: BETA-006-A, BETA-006-B, BETA-006-C, BETA-006-D, BETA-006-E, BETA-006-F — #537
205. [BETA-006-Z — Package evidence for Implement beta observability baseline](tasks/08_public_beta/BETA-006-Z-package-evidence-for-implement-beta-observability-baseline.md) — deps: BETA-006-V — #538
206. [BETA-007-A — Environment/file configuration](tasks/08_public_beta/BETA-007-A-environment-file-configuration.md) — deps: M27-GATE — #539
207. [BETA-007-B — Validation at startup](tasks/08_public_beta/BETA-007-B-validation-at-startup.md) — deps: BETA-007-A — #540
208. [BETA-007-C — Secret value wrapper/redaction](tasks/08_public_beta/BETA-007-C-secret-value-wrapper-redaction.md) — deps: BETA-007-B — #541
209. [BETA-007-D — Profile-specific settings](tasks/08_public_beta/BETA-007-D-profile-specific-settings.md) — deps: BETA-007-C — #542
210. [BETA-007-E — No dynamic code execution](tasks/08_public_beta/BETA-007-E-no-dynamic-code-execution.md) — deps: BETA-007-D — #543
211. [BETA-007-V — Verify Implement configuration and secret handling](tasks/08_public_beta/BETA-007-V-verify-implement-configuration-and-secret-handling.md) — deps: BETA-007-A, BETA-007-B, BETA-007-C, BETA-007-D, BETA-007-E — #544
212. [BETA-007-Z — Package evidence for Implement configuration and secret handling](tasks/08_public_beta/BETA-007-Z-package-evidence-for-implement-configuration-and-secret-handling.md) — deps: BETA-007-V — #545
213. [BETA-008-A — Trusted proxy configuration](tasks/08_public_beta/BETA-008-A-trusted-proxy-configuration.md) — deps: M3-GATE, BETA-006-Z — #546
214. [BETA-008-B — Forwarded header policy](tasks/08_public_beta/BETA-008-B-forwarded-header-policy.md) — deps: BETA-008-A — #547
215. [BETA-008-C — Liveness/readiness/startup endpoints](tasks/08_public_beta/BETA-008-C-liveness-readiness-startup-endpoints.md) — deps: BETA-008-B — #548
216. [BETA-008-D — Graceful drain and termination](tasks/08_public_beta/BETA-008-D-graceful-drain-and-termination.md) — deps: BETA-008-C — #549
217. [BETA-008-E — Container example](tasks/08_public_beta/BETA-008-E-container-example.md) — deps: BETA-008-D — #550
218. [BETA-008-V — Verify Implement reverse-proxy, drain, and deployment semantics](tasks/08_public_beta/BETA-008-V-verify-implement-reverse-proxy-drain-and-deployment-semantics.md) — deps: BETA-008-A, BETA-008-B, BETA-008-C, BETA-008-D, BETA-008-E — #551
219. [BETA-008-Z — Package evidence for Implement reverse-proxy, drain, and deployment semantics](tasks/08_public_beta/BETA-008-Z-package-evidence-for-implement-reverse-proxy-drain-and-deployment-semantics.md) — deps: BETA-008-V — #552
220. [BETA-009-A — Run fuzz suites for pack/router/schema/bridge/HTTP](tasks/08_public_beta/BETA-009-A-run-fuzz-suites-for-pack-router-schema-bridge-http.md) — deps: M28-GATE, M3-GATE, BETA-004-Z, BETA-005-Z, BETA-007-Z — #553
221. [BETA-009-B — Dependency vulnerability and license scan](tasks/08_public_beta/BETA-009-B-dependency-vulnerability-and-license-scan.md) — deps: BETA-009-A — #554
222. [BETA-009-C — Threat-model review](tasks/08_public_beta/BETA-009-C-threat-model-review.md) — deps: BETA-009-B — #555
223. [BETA-009-D — Chaos tests for upstream/DB/worker poison](tasks/08_public_beta/BETA-009-D-chaos-tests-for-upstream-db-worker-poison.md) — deps: BETA-009-C — #556
224. [BETA-009-E — No known critical/high exploitable issue](tasks/08_public_beta/BETA-009-E-no-known-critical-high-exploitable-issue.md) — deps: BETA-009-D — #557
225. [BETA-009-V — Verify Run beta security and reliability baseline](tasks/08_public_beta/BETA-009-V-verify-run-beta-security-and-reliability-baseline.md) — deps: BETA-009-A, BETA-009-B, BETA-009-C, BETA-009-D, BETA-009-E — #558
226. [BETA-009-Z — Package evidence for Run beta security and reliability baseline](tasks/08_public_beta/BETA-009-Z-package-evidence-for-run-beta-security-and-reliability-baseline.md) — deps: BETA-009-V — #559
227. [BETA-010-A — Linux x86_64 glibc mandatory working assumption](tasks/08_public_beta/BETA-010-A-linux-x86-64-glibc-mandatory-working-assumption.md) — deps: M26-GATE, M4A-002-Z — #560
228. [BETA-010-B — Linux ARM64 glibc when CI is available](tasks/08_public_beta/BETA-010-B-linux-arm64-glibc-when-ci-is-available.md) — deps: BETA-010-A — #561
229. [BETA-010-C — npm packages under beta tag](tasks/08_public_beta/BETA-010-C-npm-packages-under-beta-tag.md) — deps: BETA-010-B — #562
230. [BETA-010-D — Runtime binary/QPack tools](tasks/08_public_beta/BETA-010-D-runtime-binary-qpack-tools.md) — deps: BETA-010-C — #563
231. [BETA-010-E — Clean install tests](tasks/08_public_beta/BETA-010-E-clean-install-tests.md) — deps: BETA-010-D — #564
232. [BETA-010-V — Verify Create supported beta platform and packaging matrix](tasks/08_public_beta/BETA-010-V-verify-create-supported-beta-platform-and-packaging-matrix.md) — deps: BETA-010-A, BETA-010-B, BETA-010-C, BETA-010-D, BETA-010-E — #565
233. [BETA-010-Z — Package evidence for Create supported beta platform and packaging matrix](tasks/08_public_beta/BETA-010-Z-package-evidence-for-create-supported-beta-platform-and-packaging-matrix.md) — deps: BETA-010-V — #566
234. [BETA-011-A — Use SemVer prerelease](tasks/08_public_beta/BETA-011-A-use-semver-prerelease.md) — deps: M4A-GATE, BETA-010-Z — #567
235. [BETA-011-B — Publish `next`/beta tag](tasks/08_public_beta/BETA-011-B-publish-next-beta-tag.md) — deps: BETA-011-A — #568
236. [BETA-011-C — Generate changelog and migration notes](tasks/08_public_beta/BETA-011-C-generate-changelog-and-migration-notes.md) — deps: BETA-011-B — #569
237. [BETA-011-D — Create GitHub-style release packet](tasks/08_public_beta/BETA-011-D-create-github-style-release-packet.md) — deps: BETA-011-C — #570
238. [BETA-011-E — Support yanking/rollback](tasks/08_public_beta/BETA-011-E-support-yanking-rollback.md) — deps: BETA-011-D — #571
239. [BETA-011-V — Verify Automate beta publishing and versioning](tasks/08_public_beta/BETA-011-V-verify-automate-beta-publishing-and-versioning.md) — deps: BETA-011-A, BETA-011-B, BETA-011-C, BETA-011-D, BETA-011-E — #572
240. [BETA-011-Z — Package evidence for Automate beta publishing and versioning](tasks/08_public_beta/BETA-011-Z-package-evidence-for-automate-beta-publishing-and-versioning.md) — deps: BETA-011-V — #573
241. [BETA-012-A — Installation](tasks/08_public_beta/BETA-012-A-installation.md) — deps: M4A-GATE, BETA-004-Z, BETA-005-Z, BETA-008-Z — #574
242. [BETA-012-B — Quickstart](tasks/08_public_beta/BETA-012-B-quickstart.md) — deps: BETA-012-A — #575
243. [BETA-012-C — Architecture](tasks/08_public_beta/BETA-012-C-architecture.md) — deps: BETA-012-B — #576
244. [BETA-012-D — Contracts/Treaty](tasks/08_public_beta/BETA-012-D-contracts-treaty.md) — deps: BETA-012-C — #577
245. [BETA-012-E — Fetch/Postgres/auth](tasks/08_public_beta/BETA-012-E-fetch-postgres-auth.md) — deps: BETA-012-D — #578
246. [BETA-012-F — Deployment](tasks/08_public_beta/BETA-012-F-deployment.md) — deps: BETA-012-E — #579
247. [BETA-012-G — Troubleshooting](tasks/08_public_beta/BETA-012-G-troubleshooting.md) — deps: BETA-012-F — #580
248. [BETA-012-H — Performance methodology](tasks/08_public_beta/BETA-012-H-performance-methodology.md) — deps: BETA-012-G — #581
249. [BETA-012-I — Limitations/non-goals](tasks/08_public_beta/BETA-012-I-limitations-non-goals.md) — deps: BETA-012-H — #582
250. [BETA-012-V — Verify Complete beta documentation and limitations](tasks/08_public_beta/BETA-012-V-verify-complete-beta-documentation-and-limitations.md) — deps: BETA-012-A, BETA-012-B, BETA-012-C, BETA-012-D, BETA-012-E, BETA-012-F, BETA-012-G, BETA-012-H, BETA-012-I — #583
251. [BETA-012-Z — Package evidence for Complete beta documentation and limitations](tasks/08_public_beta/BETA-012-Z-package-evidence-for-complete-beta-documentation-and-limitations.md) — deps: BETA-012-V — #584
252. [BETA-013-A — Run at least two-hour mixed workload and at least one million requests on reference platform](tasks/08_public_beta/BETA-013-A-run-at-least-two-hour-mixed-workload-and-at-least-one-million-requests-on-refere.md) — deps: BETA-004-Z, BETA-005-Z, BETA-006-Z, BETA-008-Z, BETA-009-Z — #585
253. [BETA-013-B — Include fetch, DB, auth, timeouts, cancellation, worker replacement, and reload](tasks/08_public_beta/BETA-013-B-include-fetch-db-auth-timeouts-cancellation-worker-replacement-and-reload.md) — deps: BETA-013-A — #586
254. [BETA-013-C — Track RSS, heap, slots, tasks, queues, pools, and errors](tasks/08_public_beta/BETA-013-C-track-rss-heap-slots-tasks-queues-pools-and-errors.md) — deps: BETA-013-B — #587
255. [BETA-013-D — Analyze retained growth](tasks/08_public_beta/BETA-013-D-analyze-retained-growth.md) — deps: BETA-013-C — #588
256. [BETA-013-V — Verify Run beta soak and leak qualification](tasks/08_public_beta/BETA-013-V-verify-run-beta-soak-and-leak-qualification.md) — deps: BETA-013-A, BETA-013-B, BETA-013-C, BETA-013-D — #589
257. [BETA-013-Z — Package evidence for Run beta soak and leak qualification](tasks/08_public_beta/BETA-013-Z-package-evidence-for-run-beta-soak-and-leak-qualification.md) — deps: BETA-013-V — #590
258. [BETA-014-A — Include cold start categories, warm microbenchmarks, real DB/auth/I/O, CPU/JIT crossover, cost-normalized metrics, and limitations](tasks/08_public_beta/BETA-014-A-include-cold-start-categories-warm-microbenchmarks-real-db-auth-i-o-cpu-jit-cros.md) — deps: BETA-002-Z, BETA-003-Z, BETA-004-Z, BETA-005-Z, BETA-013-Z — #591
259. [BETA-014-B — Pin all candidates/artifacts](tasks/08_public_beta/BETA-014-B-pin-all-candidates-artifacts.md) — deps: BETA-014-A — #592
260. [BETA-014-C — Retain raw data](tasks/08_public_beta/BETA-014-C-retain-raw-data.md) — deps: BETA-014-B — #593
261. [BETA-014-D — Have wording reviewed](tasks/08_public_beta/BETA-014-D-have-wording-reviewed.md) — deps: BETA-014-C — #594
262. [BETA-014-V — Verify Publish canonical beta benchmark report](tasks/08_public_beta/BETA-014-V-verify-publish-canonical-beta-benchmark-report.md) — deps: BETA-014-A, BETA-014-B, BETA-014-C, BETA-014-D — #595
263. [BETA-014-Z — Package evidence for Publish canonical beta benchmark report](tasks/08_public_beta/BETA-014-Z-package-evidence-for-publish-canonical-beta-benchmark-report.md) — deps: BETA-014-V — #596
264. [BETA-015-A — Source ZIP](tasks/08_public_beta/BETA-015-A-source-zip.md) — deps: BETA-009-Z, BETA-010-Z, BETA-011-Z, BETA-013-Z, BETA-014-Z — #597
265. [BETA-015-B — Git bundle](tasks/08_public_beta/BETA-015-B-git-bundle.md) — deps: BETA-015-A — #598
266. [BETA-015-C — Linux binaries](tasks/08_public_beta/BETA-015-C-linux-binaries.md) — deps: BETA-015-B — #599
267. [BETA-015-D — npm package tarballs](tasks/08_public_beta/BETA-015-D-npm-package-tarballs.md) — deps: BETA-015-C — #600
268. [BETA-015-E — QPack tools](tasks/08_public_beta/BETA-015-E-qpack-tools.md) — deps: BETA-015-D — #601
269. [BETA-015-F — SBOM](tasks/08_public_beta/BETA-015-F-sbom.md) — deps: BETA-015-E — #602
270. [BETA-015-G — Checksums](tasks/08_public_beta/BETA-015-G-checksums.md) — deps: BETA-015-F — #603
271. [BETA-015-H — Review/evidence indexes](tasks/08_public_beta/BETA-015-H-review-evidence-indexes.md) — deps: BETA-015-G — #604
272. [BETA-015-I — Known limitations](tasks/08_public_beta/BETA-015-I-known-limitations.md) — deps: BETA-015-H — #605
273. [BETA-015-V — Verify Generate beta release evidence, SBOM, and checksums](tasks/08_public_beta/BETA-015-V-verify-generate-beta-release-evidence-sbom-and-checksums.md) — deps: BETA-015-A, BETA-015-B, BETA-015-C, BETA-015-D, BETA-015-E, BETA-015-F, BETA-015-G, BETA-015-H, BETA-015-I — #606
274. [BETA-015-Z — Package evidence for Generate beta release evidence, SBOM, and checksums](tasks/08_public_beta/BETA-015-Z-package-evidence-for-generate-beta-release-evidence-sbom-and-checksums.md) — deps: BETA-015-V — #607
275. [BETA-016-A — Fresh Linux VM/container](tasks/08_public_beta/BETA-016-A-fresh-linux-vm-container.md) — deps: BETA-011-Z, BETA-012-Z, BETA-015-Z — #608
276. [BETA-016-B — Install CLI/runtime](tasks/08_public_beta/BETA-016-B-install-cli-runtime.md) — deps: BETA-016-A — #609
277. [BETA-016-C — Scaffold app](tasks/08_public_beta/BETA-016-C-scaffold-app.md) — deps: BETA-016-B — #610
278. [BETA-016-D — Run tests/dev/build](tasks/08_public_beta/BETA-016-D-run-tests-dev-build.md) — deps: BETA-016-C — #611
279. [BETA-016-E — Deploy proof service](tasks/08_public_beta/BETA-016-E-deploy-proof-service.md) — deps: BETA-016-D — #612
280. [BETA-016-F — Use Treaty client](tasks/08_public_beta/BETA-016-F-use-treaty-client.md) — deps: BETA-016-E — #613
281. [BETA-016-V — Verify Run external clean-install and tutorial verification](tasks/08_public_beta/BETA-016-V-verify-run-external-clean-install-and-tutorial-verification.md) — deps: BETA-016-A, BETA-016-B, BETA-016-C, BETA-016-D, BETA-016-E, BETA-016-F — #614
282. [BETA-016-Z — Package evidence for Run external clean-install and tutorial verification](tasks/08_public_beta/BETA-016-Z-package-evidence-for-run-external-clean-install-and-tutorial-verification.md) — deps: BETA-016-V — #615
283. [BETA-GATE — Public Beta Readiness and Release exit gate](gates/BETA-GATE.md) — deps: BETA-001-Z, BETA-002-Z, BETA-003-Z, BETA-004-Z, BETA-005-Z, BETA-006-Z, BETA-007-Z, BETA-008-Z, BETA-009-Z, BETA-010-Z, BETA-011-Z, BETA-012-Z, BETA-013-Z, BETA-014-Z, BETA-015-Z, BETA-016-Z, BETA-017-Z — #625
