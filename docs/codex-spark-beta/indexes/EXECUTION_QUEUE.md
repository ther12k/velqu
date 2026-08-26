# Dependency-Safe Execution Queue

All PASS packets are omitted. The first unchecked dependency-ready task is M27-005-D; future milestone tasks remain TODO until implemented and evidenced.

1. [M27-005-D — Keep parser limits explicit](tasks/04_m27_capability_linker/M27-005-D-keep-parser-limits-explicit.md) — deps: M27-005-C — #267
2. [M27-005-V — Verify Implement URL and URLSearchParams](tasks/04_m27_capability_linker/M27-005-V-verify-implement-url-and-urlsearchparams.md) — deps: M27-005-A, M27-005-B, M27-005-C, M27-005-D — #268
3. [M27-005-Z — Package evidence for Implement URL and URLSearchParams](tasks/04_m27_capability_linker/M27-005-Z-package-evidence-for-implement-url-and-urlsearchparams.md) — deps: M27-005-V — #269
4. [M27-006-A — Support UTF-8 baseline](tasks/04_m27_capability_linker/M27-006-A-support-utf-8-baseline.md) — deps: M27-001-Z, M27-003-Z — #270
5. [M27-006-B — Define invalid sequence/replacement behavior](tasks/04_m27_capability_linker/M27-006-B-define-invalid-sequence-replacement-behavior.md) — deps: M27-006-A — #271
6. [M27-006-C — Integrate TypedArray ownership](tasks/04_m27_capability_linker/M27-006-C-integrate-typedarray-ownership.md) — deps: M27-006-B — #272
7. [M27-006-D — Run WPT subset](tasks/04_m27_capability_linker/M27-006-D-run-wpt-subset.md) — deps: M27-006-C — #273
8. [M27-006-V — Verify Implement TextEncoder and TextDecoder](tasks/04_m27_capability_linker/M27-006-V-verify-implement-textencoder-and-textdecoder.md) — deps: M27-006-A, M27-006-B, M27-006-C, M27-006-D — #274
9. [M27-006-Z — Package evidence for Implement TextEncoder and TextDecoder](tasks/04_m27_capability_linker/M27-006-Z-package-evidence-for-implement-textencoder-and-textdecoder.md) — deps: M27-006-V — #275
10. [M27-007-A — Define signal state/listeners/reason](tasks/04_m27_capability_linker/M27-007-A-define-signal-state-listeners-reason.md) — deps: M27-001-Z, M27-003-Z — #276
11. [M27-007-B — Bridge route deadline and explicit cancellation](tasks/04_m27_capability_linker/M27-007-B-bridge-route-deadline-and-explicit-cancellation.md) — deps: M27-007-A — #277
12. [M27-007-C — Prevent listener leaks](tasks/04_m27_capability_linker/M27-007-C-prevent-listener-leaks.md) — deps: M27-007-B — #278
13. [M27-007-D — Make cancellation idempotent](tasks/04_m27_capability_linker/M27-007-D-make-cancellation-idempotent.md) — deps: M27-007-C — #279
14. [M27-007-V — Verify Implement AbortController and AbortSignal](tasks/04_m27_capability_linker/M27-007-V-verify-implement-abortcontroller-and-abortsignal.md) — deps: M27-007-A, M27-007-B, M27-007-C, M27-007-D — #280
15. [M27-007-Z — Package evidence for Implement AbortController and AbortSignal](tasks/04_m27_capability_linker/M27-007-Z-package-evidence-for-implement-abortcontroller-and-abortsignal.md) — deps: M27-007-V — #281
16. [M27-008-A — Implement `getRandomValues` and `randomUUID` through OS CSPRNG](tasks/04_m27_capability_linker/M27-008-A-implement-getrandomvalues-and-randomuuid-through-os-csprng.md) — deps: M27-001-Z, M27-003-Z — #282
17. [M27-008-B — Enforce typed-array and size constraints](tasks/04_m27_capability_linker/M27-008-B-enforce-typed-array-and-size-constraints.md) — deps: M27-008-A — #283
18. [M27-008-C — Define unavailable-entropy failure](tasks/04_m27_capability_linker/M27-008-C-define-unavailable-entropy-failure.md) — deps: M27-008-B — #284
19. [M27-008-D — Do not implement custom cryptography](tasks/04_m27_capability_linker/M27-008-D-do-not-implement-custom-cryptography.md) — deps: M27-008-C — #285
20. [M27-008-V — Verify Implement crypto random subset](tasks/04_m27_capability_linker/M27-008-V-verify-implement-crypto-random-subset.md) — deps: M27-008-A, M27-008-B, M27-008-C, M27-008-D — #286
21. [M27-008-Z — Package evidence for Implement crypto random subset](tasks/04_m27_capability_linker/M27-008-Z-package-evidence-for-implement-crypto-random-subset.md) — deps: M27-008-V — #287
22. [M27-009-A — Define Rust-side SDK traits and metadata](tasks/04_m27_capability_linker/M27-009-A-define-rust-side-sdk-traits-and-metadata.md) — deps: M27-001-Z, M27-002-Z — #288
23. [M27-009-B — Provide test harness and example capability](tasks/04_m27_capability_linker/M27-009-B-provide-test-harness-and-example-capability.md) — deps: M27-009-A — #289
24. [M27-009-C — Expose build/inspect diagnostics](tasks/04_m27_capability_linker/M27-009-C-expose-build-inspect-diagnostics.md) — deps: M27-009-B — #290
25. [M27-009-D — Define semver/ABI compatibility](tasks/04_m27_capability_linker/M27-009-D-define-semver-abi-compatibility.md) — deps: M27-009-C — #291
26. [M27-009-V — Verify Publish capability SDK and inspection surface](tasks/04_m27_capability_linker/M27-009-V-verify-publish-capability-sdk-and-inspection-surface.md) — deps: M27-009-A, M27-009-B, M27-009-C, M27-009-D — #292
27. [M27-009-Z — Package evidence for Publish capability SDK and inspection surface](tasks/04_m27_capability_linker/M27-009-Z-package-evidence-for-publish-capability-sdk-and-inspection-surface.md) — deps: M27-009-V — #293
28. [M27-010-A — Pin WPT/WinterTC subsets](tasks/04_m27_capability_linker/M27-010-A-pin-wpt-wintertc-subsets.md) — deps: M27-005-Z, M27-006-Z, M27-007-Z, M27-008-Z — #294
29. [M27-010-B — Record skips and reasons](tasks/04_m27_capability_linker/M27-010-B-record-skips-and-reasons.md) — deps: M27-010-A — #295
30. [M27-010-C — Automate regression reports](tasks/04_m27_capability_linker/M27-010-C-automate-regression-reports.md) — deps: M27-010-B — #296
31. [M27-010-D — Keep unsupported APIs explicit](tasks/04_m27_capability_linker/M27-010-D-keep-unsupported-apis-explicit.md) — deps: M27-010-C — #297
32. [M27-010-V — Verify Establish Web API conformance program](tasks/04_m27_capability_linker/M27-010-V-verify-establish-web-api-conformance-program.md) — deps: M27-010-A, M27-010-B, M27-010-C, M27-010-D — #298
33. [M27-010-Z — Package evidence for Establish Web API conformance program](tasks/04_m27_capability_linker/M27-010-Z-package-evidence-for-establish-web-api-conformance-program.md) — deps: M27-010-V — #299
34. [M27-011-A — Measure core, web-minimal, and all-beta profiles](tasks/04_m27_capability_linker/M27-011-A-measure-core-web-minimal-and-all-beta-profiles.md) — deps: M27-002-Z, M27-010-Z — #300
35. [M27-011-B — Record binary, startup, and idle RSS deltas](tasks/04_m27_capability_linker/M27-011-B-record-binary-startup-and-idle-rss-deltas.md) — deps: M27-011-A — #301
36. [M27-011-C — Identify eager initialization](tasks/04_m27_capability_linker/M27-011-C-identify-eager-initialization.md) — deps: M27-011-B — #302
37. [M27-011-D — Make expensive modules lazy when safe](tasks/04_m27_capability_linker/M27-011-D-make-expensive-modules-lazy-when-safe.md) — deps: M27-011-C — #303
38. [M27-011-V — Verify Close capability cost budgets](tasks/04_m27_capability_linker/M27-011-V-verify-close-capability-cost-budgets.md) — deps: M27-011-A, M27-011-B, M27-011-C, M27-011-D — #304
39. [M27-011-Z — Package evidence for Close capability cost budgets](tasks/04_m27_capability_linker/M27-011-Z-package-evidence-for-close-capability-cost-budgets.md) — deps: M27-011-V — #305
40. [M27-GATE — M2.7 — Capability Linker and Minimal Web Runtime exit gate](gates/M27-GATE.md) — deps: M27-001-Z, M27-002-Z, M27-003-Z, M27-004-Z, M27-005-Z, M27-006-Z, M27-007-Z, M27-008-Z, M27-009-Z, M27-010-Z, M27-011-Z — #630
41. [M28-001-A — Define URL schemes, redirect policy, DNS rebinding controls, proxy behavior, TLS roots, timeout layers, compression, and body limits](tasks/05_m28_native_fetch/M28-001-A-define-url-schemes-redirect-policy-dns-rebinding-controls-proxy-behavior-tls-roo.md) — deps: M27-GATE — #306
42. [M28-001-B — Specify reverse-proxy and outbound trust](tasks/05_m28_native_fetch/M28-001-B-specify-reverse-proxy-and-outbound-trust.md) — deps: M28-001-A — #307
43. [M28-001-C — Define unsupported Web features](tasks/05_m28_native_fetch/M28-001-C-define-unsupported-web-features.md) — deps: M28-001-B — #308
44. [M28-001-D — Document same-process trusted-code assumption](tasks/05_m28_native_fetch/M28-001-D-document-same-process-trusted-code-assumption.md) — deps: M28-001-C — #309
45. [M28-001-V — Verify Accept fetch, TLS, redirect, and SSRF security ADR](tasks/05_m28_native_fetch/M28-001-V-verify-accept-fetch-tls-redirect-and-ssrf-security-adr.md) — deps: M28-001-A, M28-001-B, M28-001-C, M28-001-D — #310
46. [M28-001-Z — Package evidence for Accept fetch, TLS, redirect, and SSRF security ADR](tasks/05_m28_native_fetch/M28-001-Z-package-evidence-for-accept-fetch-tls-redirect-and-ssrf-security-adr.md) — deps: M28-001-V — #311
47. [M28-002-A — Compare reqwest and lower-level Hyper/Rustls approach](tasks/05_m28_native_fetch/M28-002-A-compare-reqwest-and-lower-level-hyper-rustls-approach.md) — deps: M28-001-Z — #312
48. [M28-002-B — Measure dependency/binary/startup cost](tasks/05_m28_native_fetch/M28-002-B-measure-dependency-binary-startup-cost.md) — deps: M28-002-A — #313
49. [M28-002-C — Test DNS/TLS/pool behavior](tasks/05_m28_native_fetch/M28-002-C-test-dns-tls-pool-behavior.md) — deps: M28-002-B — #314
50. [M28-002-D — Record maintenance/security considerations](tasks/05_m28_native_fetch/M28-002-D-record-maintenance-security-considerations.md) — deps: M28-002-C — #315
51. [M28-002-V — Verify Select native HTTP client stack from evidence](tasks/05_m28_native_fetch/M28-002-V-verify-select-native-http-client-stack-from-evidence.md) — deps: M28-002-A, M28-002-B, M28-002-C, M28-002-D — #316
52. [M28-002-Z — Package evidence for Select native HTTP client stack from evidence](tasks/05_m28_native_fetch/M28-002-Z-package-evidence-for-select-native-http-client-stack-from-evidence.md) — deps: M28-002-V — #317
53. [M28-003-A — Lazy pool initialization](tasks/05_m28_native_fetch/M28-003-A-lazy-pool-initialization.md) — deps: M28-002-Z — #318
54. [M28-003-B — Bound idle/active connections and DNS cache](tasks/05_m28_native_fetch/M28-003-B-bound-idle-active-connections-and-dns-cache.md) — deps: M28-003-A — #319
55. [M28-003-C — Use verified TLS roots and hostname validation](tasks/05_m28_native_fetch/M28-003-C-use-verified-tls-roots-and-hostname-validation.md) — deps: M28-003-B — #320
56. [M28-003-D — Define keepalive and shutdown](tasks/05_m28_native_fetch/M28-003-D-define-keepalive-and-shutdown.md) — deps: M28-003-C — #321
57. [M28-003-V — Verify Implement connection pooling, DNS, and TLS](tasks/05_m28_native_fetch/M28-003-V-verify-implement-connection-pooling-dns-and-tls.md) — deps: M28-003-A, M28-003-B, M28-003-C, M28-003-D — #322
58. [M28-003-Z — Package evidence for Implement connection pooling, DNS, and TLS](tasks/05_m28_native_fetch/M28-003-Z-package-evidence-for-implement-connection-pooling-dns-and-tls.md) — deps: M28-003-V — #323
59. [M28-004-A — Implement method, URL, selected headers, body types, status, and response methods](tasks/05_m28_native_fetch/M28-004-A-implement-method-url-selected-headers-body-types-status-and-response-methods.md) — deps: M28-003-Z, M27-005-Z, M27-006-Z — #324
60. [M28-004-B — Use lazy native-backed objects](tasks/05_m28_native_fetch/M28-004-B-use-lazy-native-backed-objects.md) — deps: M28-004-A — #325
61. [M28-004-C — Define clone/body-used semantics for beta](tasks/05_m28_native_fetch/M28-004-C-define-clone-body-used-semantics-for-beta.md) — deps: M28-004-B — #326
62. [M28-004-D — Keep unsupported API diagnostics explicit](tasks/05_m28_native_fetch/M28-004-D-keep-unsupported-api-diagnostics-explicit.md) — deps: M28-004-C — #327
63. [M28-004-V — Verify Implement Request, Response, and Headers subset](tasks/05_m28_native_fetch/M28-004-V-verify-implement-request-response-and-headers-subset.md) — deps: M28-004-A, M28-004-B, M28-004-C, M28-004-D — #328
64. [M28-004-Z — Package evidence for Implement Request, Response, and Headers subset](tasks/05_m28_native_fetch/M28-004-Z-package-evidence-for-implement-request-response-and-headers-subset.md) — deps: M28-004-V — #329
65. [M28-005-A — Combine explicit abort, route deadline, disconnect, shutdown, and quarantine](tasks/05_m28_native_fetch/M28-005-A-combine-explicit-abort-route-deadline-disconnect-shutdown-and-quarantine.md) — deps: M28-003-Z, M27-007-Z — #330
66. [M28-005-B — Use one terminal state for each operation](tasks/05_m28_native_fetch/M28-005-B-use-one-terminal-state-for-each-operation.md) — deps: M28-005-A — #331
67. [M28-005-C — Cancel DNS/connect/body streaming](tasks/05_m28_native_fetch/M28-005-C-cancel-dns-connect-body-streaming.md) — deps: M28-005-B — #332
68. [M28-005-D — Map failures deterministically](tasks/05_m28_native_fetch/M28-005-D-map-failures-deterministically.md) — deps: M28-005-C — #333
69. [M28-005-V — Verify Propagate AbortSignal and route deadlines](tasks/05_m28_native_fetch/M28-005-V-verify-propagate-abortsignal-and-route-deadlines.md) — deps: M28-005-A, M28-005-B, M28-005-C, M28-005-D — #334
70. [M28-005-Z — Package evidence for Propagate AbortSignal and route deadlines](tasks/05_m28_native_fetch/M28-005-Z-package-evidence-for-propagate-abortsignal-and-route-deadlines.md) — deps: M28-005-V — #335
71. [M28-006-A — Bound read/write buffers](tasks/05_m28_native_fetch/M28-006-A-bound-read-write-buffers.md) — deps: M28-004-Z, M28-005-Z — #336
72. [M28-006-B — Propagate downstream backpressure](tasks/05_m28_native_fetch/M28-006-B-propagate-downstream-backpressure.md) — deps: M28-006-A — #337
73. [M28-006-C — Cancel on consumer stop/disconnect](tasks/05_m28_native_fetch/M28-006-C-cancel-on-consumer-stop-disconnect.md) — deps: M28-006-B — #338
74. [M28-006-D — Define maximum body helper sizes](tasks/05_m28_native_fetch/M28-006-D-define-maximum-body-helper-sizes.md) — deps: M28-006-C — #339
75. [M28-006-V — Verify Implement streaming and strict backpressure](tasks/05_m28_native_fetch/M28-006-V-verify-implement-streaming-and-strict-backpressure.md) — deps: M28-006-A, M28-006-B, M28-006-C, M28-006-D — #340
76. [M28-006-Z — Package evidence for Implement streaming and strict backpressure](tasks/05_m28_native_fetch/M28-006-Z-package-evidence-for-implement-streaming-and-strict-backpressure.md) — deps: M28-006-V — #341
77. [M28-007-A — Limit redirect count](tasks/05_m28_native_fetch/M28-007-A-limit-redirect-count.md) — deps: M28-003-Z, M28-004-Z — #342
78. [M28-007-B — Reapply SSRF/DNS policy on every hop](tasks/05_m28_native_fetch/M28-007-B-reapply-ssrf-dns-policy-on-every-hop.md) — deps: M28-007-A — #343
79. [M28-007-C — Define credential/header stripping](tasks/05_m28_native_fetch/M28-007-C-define-credential-header-stripping.md) — deps: M28-007-B — #344
80. [M28-007-D — Bound decompression ratio and output](tasks/05_m28_native_fetch/M28-007-D-bound-decompression-ratio-and-output.md) — deps: M28-007-C — #345
81. [M28-007-V — Verify Implement redirect and compression policy](tasks/05_m28_native_fetch/M28-007-V-verify-implement-redirect-and-compression-policy.md) — deps: M28-007-A, M28-007-B, M28-007-C, M28-007-D — #346
82. [M28-007-Z — Package evidence for Implement redirect and compression policy](tasks/05_m28_native_fetch/M28-007-Z-package-evidence-for-implement-redirect-and-compression-policy.md) — deps: M28-007-V — #347
83. [M28-008-A — Resolve and validate addresses before connect](tasks/05_m28_native_fetch/M28-008-A-resolve-and-validate-addresses-before-connect.md) — deps: M28-001-Z, M28-003-Z, M28-007-Z — #348
84. [M28-008-B — Revalidate redirects and connection targets](tasks/05_m28_native_fetch/M28-008-B-revalidate-redirects-and-connection-targets.md) — deps: M28-008-A — #349
85. [M28-008-C — Support allow/deny configuration](tasks/05_m28_native_fetch/M28-008-C-support-allow-deny-configuration.md) — deps: M28-008-B — #350
86. [M28-008-D — Define proxy interaction](tasks/05_m28_native_fetch/M28-008-D-define-proxy-interaction.md) — deps: M28-008-C — #351
87. [M28-008-V — Verify Implement SSRF and network egress controls](tasks/05_m28_native_fetch/M28-008-V-verify-implement-ssrf-and-network-egress-controls.md) — deps: M28-008-A, M28-008-B, M28-008-C, M28-008-D — #352
88. [M28-008-Z — Package evidence for Implement SSRF and network egress controls](tasks/05_m28_native_fetch/M28-008-Z-package-evidence-for-implement-ssrf-and-network-egress-controls.md) — deps: M28-008-V — #353
89. [M28-009-A — Expose pool wait, DNS, connect, TLS, TTFB, body, errors, cancellations](tasks/05_m28_native_fetch/M28-009-A-expose-pool-wait-dns-connect-tls-ttfb-body-errors-cancellations.md) — deps: M28-003-Z, M28-005-Z, M28-006-Z — #354
90. [M28-009-B — Sample/aggregate metrics](tasks/05_m28_native_fetch/M28-009-B-sample-aggregate-metrics.md) — deps: M28-009-A — #355
91. [M28-009-C — Drain pool on shutdown](tasks/05_m28_native_fetch/M28-009-C-drain-pool-on-shutdown.md) — deps: M28-009-B — #356
92. [M28-009-D — Quarantine rejects new work](tasks/05_m28_native_fetch/M28-009-D-quarantine-rejects-new-work.md) — deps: M28-009-C — #357
93. [M28-009-V — Verify Integrate lifecycle, observability, and shutdown](tasks/05_m28_native_fetch/M28-009-V-verify-integrate-lifecycle-observability-and-shutdown.md) — deps: M28-009-A, M28-009-B, M28-009-C, M28-009-D — #358
94. [M28-009-Z — Package evidence for Integrate lifecycle, observability, and shutdown](tasks/05_m28_native_fetch/M28-009-Z-package-evidence-for-integrate-lifecycle-observability-and-shutdown.md) — deps: M28-009-V — #359
95. [M28-010-A — Run selected WPT cases](tasks/05_m28_native_fetch/M28-010-A-run-selected-wpt-cases.md) — deps: M28-004-Z, M28-005-Z, M28-006-Z, M28-007-Z, M28-008-Z — #360
96. [M28-010-B — Create deterministic DNS/TLS/redirect/slow/body fixtures](tasks/05_m28_native_fetch/M28-010-B-create-deterministic-dns-tls-redirect-slow-body-fixtures.md) — deps: M28-010-A — #361
97. [M28-010-C — Fuzz headers and URLs](tasks/05_m28_native_fetch/M28-010-C-fuzz-headers-and-urls.md) — deps: M28-010-B — #362
98. [M28-010-D — Test proxy and cancellation](tasks/05_m28_native_fetch/M28-010-D-test-proxy-and-cancellation.md) — deps: M28-010-C — #363
99. [M28-010-V — Verify Complete fetch conformance and fault testing](tasks/05_m28_native_fetch/M28-010-V-verify-complete-fetch-conformance-and-fault-testing.md) — deps: M28-010-A, M28-010-B, M28-010-C, M28-010-D — #364
100. [M28-010-Z — Package evidence for Complete fetch conformance and fault testing](tasks/05_m28_native_fetch/M28-010-Z-package-evidence-for-complete-fetch-conformance-and-fault-testing.md) — deps: M28-010-V — #365
101. [M28-011-A — Run 1/5/10/25ms upstream latency](tasks/05_m28_native_fetch/M28-011-A-run-1-5-10-25ms-upstream-latency.md) — deps: M28-009-Z, M28-010-Z — #366
102. [M28-011-B — Run one, two, and four parallel calls](tasks/05_m28_native_fetch/M28-011-B-run-one-two-and-four-parallel-calls.md) — deps: M28-011-A — #367
103. [M28-011-C — Mix timeout/success/malformed responses](tasks/05_m28_native_fetch/M28-011-C-mix-timeout-success-malformed-responses.md) — deps: M28-011-B — #368
104. [M28-011-D — Test concurrency 1/10/50/200](tasks/05_m28_native_fetch/M28-011-D-test-concurrency-1-10-50-200.md) — deps: M28-011-C — #369
105. [M28-011-V — Verify Run controlled upstream and fan-out benchmarks](tasks/05_m28_native_fetch/M28-011-V-verify-run-controlled-upstream-and-fan-out-benchmarks.md) — deps: M28-011-A, M28-011-B, M28-011-C, M28-011-D — #370
106. [M28-011-Z — Package evidence for Run controlled upstream and fan-out benchmarks](tasks/05_m28_native_fetch/M28-011-Z-package-evidence-for-run-controlled-upstream-and-fan-out-benchmarks.md) — deps: M28-011-V — #371
107. [M28-GATE — M2.8 — Native Outbound Fetch exit gate](gates/M28-GATE.md) — deps: M28-001-Z, M28-002-Z, M28-003-Z, M28-004-Z, M28-005-Z, M28-006-Z, M28-007-Z, M28-008-Z, M28-009-Z, M28-010-Z, M28-011-Z — #631
108. [M3-001-A — Accept ADR](tasks/06_m3_multi_worker/M3-001-A-accept-adr.md) — deps: M28-GATE — #372
109. [M3-001-B — Document module-level state replication](tasks/06_m3_multi_worker/M3-001-B-document-module-level-state-replication.md) — deps: M3-001-A — #373
110. [M3-001-C — Forbid JSValue sharing](tasks/06_m3_multi_worker/M3-001-C-forbid-jsvalue-sharing.md) — deps: M3-001-B — #374
111. [M3-001-D — Define service/capability shared handles and thread safety](tasks/06_m3_multi_worker/M3-001-D-define-service-capability-shared-handles-and-thread-safety.md) — deps: M3-001-C — #375
112. [M3-001-V — Verify Freeze independent-worker state semantics](tasks/06_m3_multi_worker/M3-001-V-verify-freeze-independent-worker-state-semantics.md) — deps: M3-001-A, M3-001-B, M3-001-C, M3-001-D — #376
113. [M3-001-Z — Package evidence for Freeze independent-worker state semantics](tasks/06_m3_multi_worker/M3-001-Z-package-evidence-for-freeze-independent-worker-state-semantics.md) — deps: M3-001-V — #377
114. [M3-002-A — Use bounded per-worker queues](tasks/06_m3_multi_worker/M3-002-A-use-bounded-per-worker-queues.md) — deps: M3-001-Z — #378
115. [M3-002-B — Select worker using outstanding-load strategy](tasks/06_m3_multi_worker/M3-002-B-select-worker-using-outstanding-load-strategy.md) — deps: M3-002-A — #379
116. [M3-002-C — Define admission and overload response](tasks/06_m3_multi_worker/M3-002-C-define-admission-and-overload-response.md) — deps: M3-002-B — #380
117. [M3-002-D — Preserve RouteId/RoutePlan before dispatch](tasks/06_m3_multi_worker/M3-002-D-preserve-routeid-routeplan-before-dispatch.md) — deps: M3-002-C — #381
118. [M3-002-V — Verify Implement bounded worker dispatcher](tasks/06_m3_multi_worker/M3-002-V-verify-implement-bounded-worker-dispatcher.md) — deps: M3-002-A, M3-002-B, M3-002-C, M3-002-D — #382
119. [M3-002-Z — Package evidence for Implement bounded worker dispatcher](tasks/06_m3_multi_worker/M3-002-Z-package-evidence-for-implement-bounded-worker-dispatcher.md) — deps: M3-002-V — #383
120. [M3-003-A — Serverless starts one worker only](tasks/06_m3_multi_worker/M3-003-A-serverless-starts-one-worker-only.md) — deps: M3-002-Z — #384
121. [M3-003-B — Service marks ready after worker 0 and adds workers adaptively](tasks/06_m3_multi_worker/M3-003-B-service-marks-ready-after-worker-0-and-adds-workers-adaptively.md) — deps: M3-003-A — #385
122. [M3-003-C — Throughput initializes configured workers before ready](tasks/06_m3_multi_worker/M3-003-C-throughput-initializes-configured-workers-before-ready.md) — deps: M3-003-B — #386
123. [M3-003-D — Expose profile in inspect/config](tasks/06_m3_multi_worker/M3-003-D-expose-profile-in-inspect-config.md) — deps: M3-003-C — #387
124. [M3-003-V — Verify Implement serverless, service, and throughput profiles](tasks/06_m3_multi_worker/M3-003-V-verify-implement-serverless-service-and-throughput-profiles.md) — deps: M3-003-A, M3-003-B, M3-003-C, M3-003-D — #388
125. [M3-003-Z — Package evidence for Implement serverless, service, and throughput profiles](tasks/06_m3_multi_worker/M3-003-Z-package-evidence-for-implement-serverless-service-and-throughput-profiles.md) — deps: M3-003-V — #389
126. [M3-004-A — Share immutable mapped QPack bytes](tasks/06_m3_multi_worker/M3-004-A-share-immutable-mapped-qpack-bytes.md) — deps: M3-002-Z, M26-GATE — #390
127. [M3-004-B — Create separate QuickJS runtimes/functions/context state](tasks/06_m3_multi_worker/M3-004-B-create-separate-quickjs-runtimes-functions-context-state.md) — deps: M3-004-A — #391
128. [M3-004-C — Validate capability compatibility per worker](tasks/06_m3_multi_worker/M3-004-C-validate-capability-compatibility-per-worker.md) — deps: M3-004-B — #392
129. [M3-004-D — Bound startup parallelism](tasks/06_m3_multi_worker/M3-004-D-bound-startup-parallelism.md) — deps: M3-004-C — #393
130. [M3-004-V — Verify Implement deterministic worker initialization and artifact sharing](tasks/06_m3_multi_worker/M3-004-V-verify-implement-deterministic-worker-initialization-and-artifact-sharing.md) — deps: M3-004-A, M3-004-B, M3-004-C, M3-004-D — #394
131. [M3-004-Z — Package evidence for Implement deterministic worker initialization and artifact sharing](tasks/06_m3_multi_worker/M3-004-Z-package-evidence-for-implement-deterministic-worker-initialization-and-artifact.md) — deps: M3-004-V — #395
132. [M3-005-A — Remove quarantined worker from dispatch](tasks/06_m3_multi_worker/M3-005-A-remove-quarantined-worker-from-dispatch.md) — deps: M3-002-Z, M3-004-Z — #396
133. [M3-005-B — Fail/settle its pending work](tasks/06_m3_multi_worker/M3-005-B-fail-settle-its-pending-work.md) — deps: M3-005-A — #397
134. [M3-005-C — Initialize replacement under bounded policy](tasks/06_m3_multi_worker/M3-005-C-initialize-replacement-under-bounded-policy.md) — deps: M3-005-B — #398
135. [M3-005-D — Aggregate readiness from usable capacity](tasks/06_m3_multi_worker/M3-005-D-aggregate-readiness-from-usable-capacity.md) — deps: M3-005-C — #399
136. [M3-005-V — Verify Implement quarantine, replacement, and readiness aggregation](tasks/06_m3_multi_worker/M3-005-V-verify-implement-quarantine-replacement-and-readiness-aggregation.md) — deps: M3-005-A, M3-005-B, M3-005-C, M3-005-D — #400
137. [M3-005-Z — Package evidence for Implement quarantine, replacement, and readiness aggregation](tasks/06_m3_multi_worker/M3-005-Z-package-evidence-for-implement-quarantine-replacement-and-readiness-aggregation.md) — deps: M3-005-V — #401
138. [M3-006-A — Define thresholds/hysteresis](tasks/06_m3_multi_worker/M3-006-A-define-thresholds-hysteresis.md) — deps: M3-003-Z, M3-005-Z — #402
139. [M3-006-B — Bound min/max workers](tasks/06_m3_multi_worker/M3-006-B-bound-min-max-workers.md) — deps: M3-006-A — #403
140. [M3-006-C — Drain before scale-down](tasks/06_m3_multi_worker/M3-006-C-drain-before-scale-down.md) — deps: M3-006-B — #404
141. [M3-006-D — Avoid oscillation](tasks/06_m3_multi_worker/M3-006-D-avoid-oscillation.md) — deps: M3-006-C — #405
142. [M3-006-V — Verify Implement adaptive scale-up and scale-down](tasks/06_m3_multi_worker/M3-006-V-verify-implement-adaptive-scale-up-and-scale-down.md) — deps: M3-006-A, M3-006-B, M3-006-C, M3-006-D — #406
143. [M3-006-Z — Package evidence for Implement adaptive scale-up and scale-down](tasks/06_m3_multi_worker/M3-006-Z-package-evidence-for-implement-adaptive-scale-up-and-scale-down.md) — deps: M3-006-V — #407
144. [M3-007-A — Track invocation-to-worker ownership](tasks/06_m3_multi_worker/M3-007-A-track-invocation-to-worker-ownership.md) — deps: M3-002-Z, M3-004-Z — #408
145. [M3-007-B — Stop admission on drain](tasks/06_m3_multi_worker/M3-007-B-stop-admission-on-drain.md) — deps: M3-007-A — #409
146. [M3-007-C — Allow bounded in-flight completion](tasks/06_m3_multi_worker/M3-007-C-allow-bounded-in-flight-completion.md) — deps: M3-007-B — #410
147. [M3-007-D — Abort after shutdown deadline](tasks/06_m3_multi_worker/M3-007-D-abort-after-shutdown-deadline.md) — deps: M3-007-C — #411
148. [M3-007-V — Verify Implement multi-worker cancellation and graceful shutdown](tasks/06_m3_multi_worker/M3-007-V-verify-implement-multi-worker-cancellation-and-graceful-shutdown.md) — deps: M3-007-A, M3-007-B, M3-007-C, M3-007-D — #412
149. [M3-007-Z — Package evidence for Implement multi-worker cancellation and graceful shutdown](tasks/06_m3_multi_worker/M3-007-Z-package-evidence-for-implement-multi-worker-cancellation-and-graceful-shutdown.md) — deps: M3-007-V — #413
150. [M3-008-A — Add route/global queue limits or weighted admission](tasks/06_m3_multi_worker/M3-008-A-add-route-global-queue-limits-or-weighted-admission.md) — deps: M3-002-Z, M3-006-Z — #414
151. [M3-008-B — Define long-running JS policy](tasks/06_m3_multi_worker/M3-008-B-define-long-running-js-policy.md) — deps: M3-008-A — #415
152. [M3-008-C — Expose load-shed reasons](tasks/06_m3_multi_worker/M3-008-C-expose-load-shed-reasons.md) — deps: M3-008-B — #416
153. [M3-008-D — Test mixed workloads](tasks/06_m3_multi_worker/M3-008-D-test-mixed-workloads.md) — deps: M3-008-C — #417
154. [M3-008-V — Verify Add fairness and overload controls](tasks/06_m3_multi_worker/M3-008-V-verify-add-fairness-and-overload-controls.md) — deps: M3-008-A, M3-008-B, M3-008-C, M3-008-D — #418
155. [M3-008-Z — Package evidence for Add fairness and overload controls](tasks/06_m3_multi_worker/M3-008-Z-package-evidence-for-add-fairness-and-overload-controls.md) — deps: M3-008-V — #419
156. [M3-009-A — Measure 1/2/4 workers](tasks/06_m3_multi_worker/M3-009-A-measure-1-2-4-workers.md) — deps: M3-003-Z, M3-006-Z, M3-008-Z — #420
157. [M3-009-B — Report throughput, p50/p95/p99, queue time, CPU, RSS, errors](tasks/06_m3_multi_worker/M3-009-B-report-throughput-p50-p95-p99-queue-time-cpu-rss-errors.md) — deps: M3-009-A — #421
158. [M3-009-C — Run C1/C2/C3 and controlled I/O](tasks/06_m3_multi_worker/M3-009-C-run-c1-c2-c3-and-controlled-i-o.md) — deps: M3-009-B — #422
159. [M3-009-D — Record physical core topology](tasks/06_m3_multi_worker/M3-009-D-record-physical-core-topology.md) — deps: M3-009-C — #423
160. [M3-009-V — Verify Close multi-worker scaling and memory evidence](tasks/06_m3_multi_worker/M3-009-V-verify-close-multi-worker-scaling-and-memory-evidence.md) — deps: M3-009-A, M3-009-B, M3-009-C, M3-009-D — #424
161. [M3-009-Z — Package evidence for Close multi-worker scaling and memory evidence](tasks/06_m3_multi_worker/M3-009-Z-package-evidence-for-close-multi-worker-scaling-and-memory-evidence.md) — deps: M3-009-V — #425
162. [M3-010-A — Run multi-hour mixed load](tasks/06_m3_multi_worker/M3-010-A-run-multi-hour-mixed-load.md) — deps: M3-005-Z, M3-007-Z, M3-009-Z — #426
163. [M3-010-B — Inject worker poison, upstream timeout, disconnect, and shutdown](tasks/06_m3_multi_worker/M3-010-B-inject-worker-poison-upstream-timeout-disconnect-and-shutdown.md) — deps: M3-010-A — #427
164. [M3-010-C — Track retained memory and task/slot counts](tasks/06_m3_multi_worker/M3-010-C-track-retained-memory-and-task-slot-counts.md) — deps: M3-010-B — #428
165. [M3-010-D — Verify recovery](tasks/06_m3_multi_worker/M3-010-D-verify-recovery.md) — deps: M3-010-C — #429
166. [M3-010-V — Verify Run multi-worker soak and recovery](tasks/06_m3_multi_worker/M3-010-V-verify-run-multi-worker-soak-and-recovery.md) — deps: M3-010-A, M3-010-B, M3-010-C, M3-010-D — #430
167. [M3-010-Z — Package evidence for Run multi-worker soak and recovery](tasks/06_m3_multi_worker/M3-010-Z-package-evidence-for-run-multi-worker-soak-and-recovery.md) — deps: M3-010-V — #431
168. [M3-GATE — M3 — Multi-Worker Service Runtime exit gate](gates/M3-GATE.md) — deps: M3-001-Z, M3-002-Z, M3-003-Z, M3-004-Z, M3-005-Z, M3-006-Z, M3-007-Z, M3-008-Z, M3-009-Z, M3-010-Z — #632
169. [M4A-001-A — Watch source and contracts](tasks/07_m4a_developer_preview/M4A-001-A-watch-source-and-contracts.md) — deps: M3-GATE — #432
170. [M4A-001-B — Build incremental temporary QPack](tasks/07_m4a_developer_preview/M4A-001-B-build-incremental-temporary-qpack.md) — deps: M4A-001-A — #433
171. [M4A-001-C — Load new worker before switching traffic](tasks/07_m4a_developer_preview/M4A-001-C-load-new-worker-before-switching-traffic.md) — deps: M4A-001-B — #434
172. [M4A-001-D — Drain old worker and surface compile/runtime errors](tasks/07_m4a_developer_preview/M4A-001-D-drain-old-worker-and-surface-compile-runtime-errors.md) — deps: M4A-001-C — #435
173. [M4A-001-V — Verify Implement actual-runtime `velqu dev` loop](tasks/07_m4a_developer_preview/M4A-001-V-verify-implement-actual-runtime-velqu-dev-loop.md) — deps: M4A-001-A, M4A-001-B, M4A-001-C, M4A-001-D — #436
174. [M4A-001-Z — Package evidence for Implement actual-runtime `velqu dev` loop](tasks/07_m4a_developer_preview/M4A-001-Z-package-evidence-for-implement-actual-runtime-velqu-dev-loop.md) — deps: M4A-001-V — #437
175. [M4A-002-A — Implement and document `velqu dev`, `build`, `inspect`, `contract diff`, `test`, `pack inspect/migrate`, and diagnostics](tasks/07_m4a_developer_preview/M4A-002-A-implement-and-document-velqu-dev-build-inspect-contract-diff-test-pack-inspect-m.md) — deps: M4A-001-Z, M26-GATE — #438
176. [M4A-002-B — Stable exit codes](tasks/07_m4a_developer_preview/M4A-002-B-stable-exit-codes.md) — deps: M4A-002-A — #439
177. [M4A-002-C — Machine-readable output option](tasks/07_m4a_developer_preview/M4A-002-C-machine-readable-output-option.md) — deps: M4A-002-B — #440
178. [M4A-002-D — Helpful actionable errors](tasks/07_m4a_developer_preview/M4A-002-D-helpful-actionable-errors.md) — deps: M4A-002-C — #441
179. [M4A-002-V — Verify Complete CLI command surface](tasks/07_m4a_developer_preview/M4A-002-V-verify-complete-cli-command-surface.md) — deps: M4A-002-A, M4A-002-B, M4A-002-C, M4A-002-D — #442
180. [M4A-002-Z — Package evidence for Complete CLI command surface](tasks/07_m4a_developer_preview/M4A-002-Z-package-evidence-for-complete-cli-command-surface.md) — deps: M4A-002-V — #443
181. [M4A-003-A — Starter API](tasks/07_m4a_developer_preview/M4A-003-A-starter-api.md) — deps: M4A-002-Z — #444
182. [M4A-003-B — Treaty client example](tasks/07_m4a_developer_preview/M4A-003-B-treaty-client-example.md) — deps: M4A-003-A — #445
183. [M4A-003-C — Testing setup](tasks/07_m4a_developer_preview/M4A-003-C-testing-setup.md) — deps: M4A-003-B — #446
184. [M4A-003-D — Optional fetch/profile choices](tasks/07_m4a_developer_preview/M4A-003-D-optional-fetch-profile-choices.md) — deps: M4A-003-C — #447
185. [M4A-003-V — Verify Implement project scaffolding](tasks/07_m4a_developer_preview/M4A-003-V-verify-implement-project-scaffolding.md) — deps: M4A-003-A, M4A-003-B, M4A-003-C, M4A-003-D — #448
186. [M4A-003-Z — Package evidence for Implement project scaffolding](tasks/07_m4a_developer_preview/M4A-003-Z-package-evidence-for-implement-project-scaffolding.md) — deps: M4A-003-V — #449
187. [M4A-004-A — Unit-local direct generated dispatcher](tasks/07_m4a_developer_preview/M4A-004-A-unit-local-direct-generated-dispatcher.md) — deps: M25-GATE, M4A-001-Z — #450
188. [M4A-004-B — Runtime-local actual Rust/QuickJS process](tasks/07_m4a_developer_preview/M4A-004-B-runtime-local-actual-rust-quickjs-process.md) — deps: M4A-004-A — #451
189. [M4A-004-C — Remote fetch client](tasks/07_m4a_developer_preview/M4A-004-C-remote-fetch-client.md) — deps: M4A-004-B — #452
190. [M4A-004-D — Exact method/body/query/status/problem typing](tasks/07_m4a_developer_preview/M4A-004-D-exact-method-body-query-status-problem-typing.md) — deps: M4A-004-C — #453
191. [M4A-004-V — Verify Complete Treaty unit-local, runtime-local, and remote modes](tasks/07_m4a_developer_preview/M4A-004-V-verify-complete-treaty-unit-local-runtime-local-and-remote-modes.md) — deps: M4A-004-A, M4A-004-B, M4A-004-C, M4A-004-D — #454
192. [M4A-004-Z — Package evidence for Complete Treaty unit-local, runtime-local, and remote modes](tasks/07_m4a_developer_preview/M4A-004-Z-package-evidence-for-complete-treaty-unit-local-runtime-local-and-remote-modes.md) — deps: M4A-004-V — #455
193. [M4A-005-A — Generate d.ts/client/OpenAPI/contract lock](tasks/07_m4a_developer_preview/M4A-005-A-generate-d-ts-client-openapi-contract-lock.md) — deps: M4A-004-Z — #456
194. [M4A-005-B — Tree-shakable client](tasks/07_m4a_developer_preview/M4A-005-B-tree-shakable-client.md) — deps: M4A-005-A — #457
195. [M4A-005-C — Version and public contract hash](tasks/07_m4a_developer_preview/M4A-005-C-version-and-public-contract-hash.md) — deps: M4A-005-B — #458
196. [M4A-005-D — Package verification](tasks/07_m4a_developer_preview/M4A-005-D-package-verification.md) — deps: M4A-005-C — #459
197. [M4A-005-V — Verify Publish compact contract and SDK artifacts](tasks/07_m4a_developer_preview/M4A-005-V-verify-publish-compact-contract-and-sdk-artifacts.md) — deps: M4A-005-A, M4A-005-B, M4A-005-C, M4A-005-D — #460
198. [M4A-005-Z — Package evidence for Publish compact contract and SDK artifacts](tasks/07_m4a_developer_preview/M4A-005-Z-package-evidence-for-publish-compact-contract-and-sdk-artifacts.md) — deps: M4A-005-V — #461
199. [M4A-006-A — Structured diagnostic codes](tasks/07_m4a_developer_preview/M4A-006-A-structured-diagnostic-codes.md) — deps: M4A-001-Z, M4A-002-Z — #462
200. [M4A-006-B — Source-map-aware stacks](tasks/07_m4a_developer_preview/M4A-006-B-source-map-aware-stacks.md) — deps: M4A-006-A — #463
201. [M4A-006-C — Redaction policy](tasks/07_m4a_developer_preview/M4A-006-C-redaction-policy.md) — deps: M4A-006-B — #464
202. [M4A-006-D — Inspect route plan, fields, codecs, capabilities, crossings, and debug names](tasks/07_m4a_developer_preview/M4A-006-D-inspect-route-plan-fields-codecs-capabilities-crossings-and-debug-names.md) — deps: M4A-006-C — #465
203. [M4A-006-V — Verify Finalize diagnostics, source maps, and inspect output](tasks/07_m4a_developer_preview/M4A-006-V-verify-finalize-diagnostics-source-maps-and-inspect-output.md) — deps: M4A-006-A, M4A-006-B, M4A-006-C, M4A-006-D — #466
204. [M4A-006-Z — Package evidence for Finalize diagnostics, source maps, and inspect output](tasks/07_m4a_developer_preview/M4A-006-Z-package-evidence-for-finalize-diagnostics-source-maps-and-inspect-output.md) — deps: M4A-006-V — #467
205. [M4A-007-A — Define deferred owner, queue, deadline, cancellation, shutdown](tasks/07_m4a_developer_preview/M4A-007-A-define-deferred-owner-queue-deadline-cancellation-shutdown.md) — deps: M27-GATE, M3-GATE — #468
206. [M4A-007-B — Separate cleanup from best-effort work](tasks/07_m4a_developer_preview/M4A-007-B-separate-cleanup-from-best-effort-work.md) — deps: M4A-007-A — #469
207. [M4A-007-C — Expose metrics](tasks/07_m4a_developer_preview/M4A-007-C-expose-metrics.md) — deps: M4A-007-B — #470
208. [M4A-007-D — Forbid unbounded recursive spawning](tasks/07_m4a_developer_preview/M4A-007-D-forbid-unbounded-recursive-spawning.md) — deps: M4A-007-C — #471
209. [M4A-007-V — Verify Implement bounded `defer` and lifecycle hooks](tasks/07_m4a_developer_preview/M4A-007-V-verify-implement-bounded-defer-and-lifecycle-hooks.md) — deps: M4A-007-A, M4A-007-B, M4A-007-C, M4A-007-D — #472
210. [M4A-007-Z — Package evidence for Implement bounded `defer` and lifecycle hooks](tasks/07_m4a_developer_preview/M4A-007-Z-package-evidence-for-implement-bounded-defer-and-lifecycle-hooks.md) — deps: M4A-007-V — #473
211. [M4A-008-A — Quickstart](tasks/07_m4a_developer_preview/M4A-008-A-quickstart.md) — deps: M4A-002-Z, M4A-004-Z, M4A-006-Z — #474
212. [M4A-008-B — Routes/schemas/policies/services](tasks/07_m4a_developer_preview/M4A-008-B-routes-schemas-policies-services.md) — deps: M4A-008-A — #475
213. [M4A-008-C — Treaty](tasks/07_m4a_developer_preview/M4A-008-C-treaty.md) — deps: M4A-008-B — #476
214. [M4A-008-D — Fetch/capabilities](tasks/07_m4a_developer_preview/M4A-008-D-fetch-capabilities.md) — deps: M4A-008-C — #477
215. [M4A-008-E — Runtime profiles](tasks/07_m4a_developer_preview/M4A-008-E-runtime-profiles.md) — deps: M4A-008-D — #478
216. [M4A-008-F — Deployment behind reverse proxy](tasks/07_m4a_developer_preview/M4A-008-F-deployment-behind-reverse-proxy.md) — deps: M4A-008-E — #479
217. [M4A-008-G — Limits and non-goals](tasks/07_m4a_developer_preview/M4A-008-G-limits-and-non-goals.md) — deps: M4A-008-F — #480
218. [M4A-008-V — Verify Build documentation and examples](tasks/07_m4a_developer_preview/M4A-008-V-verify-build-documentation-and-examples.md) — deps: M4A-008-A, M4A-008-B, M4A-008-C, M4A-008-D, M4A-008-E, M4A-008-F, M4A-008-G — #481
219. [M4A-008-Z — Package evidence for Build documentation and examples](tasks/07_m4a_developer_preview/M4A-008-Z-package-evidence-for-build-documentation-and-examples.md) — deps: M4A-008-V — #482
220. [M4A-009-A — Feature modules](tasks/07_m4a_developer_preview/M4A-009-A-feature-modules.md) — deps: M4A-004-Z, M4A-007-Z, M28-GATE — #483
221. [M4A-009-B — JWT-like policy reference](tasks/07_m4a_developer_preview/M4A-009-B-jwt-like-policy-reference.md) — deps: M4A-009-A — #484
222. [M4A-009-C — Controlled upstream](tasks/07_m4a_developer_preview/M4A-009-C-controlled-upstream.md) — deps: M4A-009-B — #485
223. [M4A-009-D — Metrics/readiness/shutdown](tasks/07_m4a_developer_preview/M4A-009-D-metrics-readiness-shutdown.md) — deps: M4A-009-C — #486
224. [M4A-009-E — Treaty client](tasks/07_m4a_developer_preview/M4A-009-E-treaty-client.md) — deps: M4A-009-D — #487
225. [M4A-009-V — Verify Build realistic private-alpha proof service](tasks/07_m4a_developer_preview/M4A-009-V-verify-build-realistic-private-alpha-proof-service.md) — deps: M4A-009-A, M4A-009-B, M4A-009-C, M4A-009-D, M4A-009-E — #488
226. [M4A-009-Z — Package evidence for Build realistic private-alpha proof service](tasks/07_m4a_developer_preview/M4A-009-Z-package-evidence-for-build-realistic-private-alpha-proof-service.md) — deps: M4A-009-V — #489
227. [M4A-010-A — Provide clean install packet](tasks/07_m4a_developer_preview/M4A-010-A-provide-clean-install-packet.md) — deps: M4A-003-Z, M4A-008-Z, M4A-009-Z — #490
228. [M4A-010-B — Collect task-based feedback](tasks/07_m4a_developer_preview/M4A-010-B-collect-task-based-feedback.md) — deps: M4A-010-A — #491
229. [M4A-010-C — Classify P0/P1/P2](tasks/07_m4a_developer_preview/M4A-010-C-classify-p0-p1-p2.md) — deps: M4A-010-B — #492
230. [M4A-010-D — Fix beta-blocking findings and publish limitations](tasks/07_m4a_developer_preview/M4A-010-D-fix-beta-blocking-findings-and-publish-limitations.md) — deps: M4A-010-C — #493
231. [M4A-010-V — Verify Run invited developer alpha and close P0/P1 feedback](tasks/07_m4a_developer_preview/M4A-010-V-verify-run-invited-developer-alpha-and-close-p0-p1-feedback.md) — deps: M4A-010-A, M4A-010-B, M4A-010-C, M4A-010-D — #494
232. [M4A-010-Z — Package evidence for Run invited developer alpha and close P0/P1 feedback](tasks/07_m4a_developer_preview/M4A-010-Z-package-evidence-for-run-invited-developer-alpha-and-close-p0-p1-feedback.md) — deps: M4A-010-V — #495
233. [M4A-GATE — M4A — Developer Preview and Private Alpha exit gate](gates/M4A-GATE.md) — deps: M4A-001-Z, M4A-002-Z, M4A-003-Z, M4A-004-Z, M4A-005-Z, M4A-006-Z, M4A-007-Z, M4A-008-Z, M4A-009-Z, M4A-010-Z — #633
234. [BETA-001-V — Verify Make the real-world benchmark harness executable](tasks/08_public_beta/BETA-001-V-verify-make-the-real-world-benchmark-harness-executable.md) — deps: BETA-001-A, BETA-001-B, BETA-001-C, BETA-001-D — #500
235. [BETA-001-Z — Package evidence for Make the real-world benchmark harness executable](tasks/08_public_beta/BETA-001-Z-package-evidence-for-make-the-real-world-benchmark-harness-executable.md) — deps: BETA-001-V — #503
236. [BETA-002-A — Match SQL, pool, JWT, timeouts, logging, responses, compression, and deployment limits](tasks/08_public_beta/BETA-002-A-match-sql-pool-jwt-timeouts-logging-responses-compression-and-deployment-limits.md) — deps: BETA-001-Z — #504
237. [BETA-002-B — Pin versions](tasks/08_public_beta/BETA-002-B-pin-versions.md) — deps: BETA-002-A — #505
238. [BETA-002-C — Add contract-response verification](tasks/08_public_beta/BETA-002-C-add-contract-response-verification.md) — deps: BETA-002-B — #506
239. [BETA-002-D — Document unavoidable differences](tasks/08_public_beta/BETA-002-D-document-unavoidable-differences.md) — deps: BETA-002-C — #507
240. [BETA-002-V — Verify Implement matched competitor candidates](tasks/08_public_beta/BETA-002-V-verify-implement-matched-competitor-candidates.md) — deps: BETA-002-A, BETA-002-B, BETA-002-C, BETA-002-D — #508
241. [BETA-002-Z — Package evidence for Implement matched competitor candidates](tasks/08_public_beta/BETA-002-Z-package-evidence-for-implement-matched-competitor-candidates.md) — deps: BETA-002-V — #509
242. [BETA-003-A — Run 0/1/5/10/25ms I/O, payload matrices, and CPU operation levels](tasks/08_public_beta/BETA-003-A-run-0-1-5-10-25ms-i-o-payload-matrices-and-cpu-operation-levels.md) — deps: BETA-001-Z, M28-GATE, M3-GATE — #510
243. [BETA-003-B — Measure first request through steady state](tasks/08_public_beta/BETA-003-B-measure-first-request-through-steady-state.md) — deps: BETA-003-A — #511
244. [BETA-003-C — Calculate cumulative crossover request counts](tasks/08_public_beta/BETA-003-C-calculate-cumulative-crossover-request-counts.md) — deps: BETA-003-B — #512
245. [BETA-003-D — Report losses honestly](tasks/08_public_beta/BETA-003-D-report-losses-honestly.md) — deps: BETA-003-C — #513
246. [BETA-003-V — Verify Run controlled I/O and CPU/JIT crossover suites](tasks/08_public_beta/BETA-003-V-verify-run-controlled-i-o-and-cpu-jit-crossover-suites.md) — deps: BETA-003-A, BETA-003-B, BETA-003-C, BETA-003-D — #514
247. [BETA-003-Z — Package evidence for Run controlled I/O and CPU/JIT crossover suites](tasks/08_public_beta/BETA-003-Z-package-evidence-for-run-controlled-i-o-and-cpu-jit-crossover-suites.md) — deps: BETA-003-V — #515
248. [BETA-004-A — Use capability ABI](tasks/08_public_beta/BETA-004-A-use-capability-abi.md) — deps: M27-GATE, BETA-001-Z — #516
249. [BETA-004-B — Lazy pool](tasks/08_public_beta/BETA-004-B-lazy-pool.md) — deps: BETA-004-A — #517
250. [BETA-004-C — Parameterized queries/transactions](tasks/08_public_beta/BETA-004-C-parameterized-queries-transactions.md) — deps: BETA-004-B — #518
251. [BETA-004-D — Deadline/cancellation/shutdown](tasks/08_public_beta/BETA-004-D-deadline-cancellation-shutdown.md) — deps: BETA-004-C — #519
252. [BETA-004-E — Pool limits and observability](tasks/08_public_beta/BETA-004-E-pool-limits-and-observability.md) — deps: BETA-004-D — #520
253. [BETA-004-F — No ORM](tasks/08_public_beta/BETA-004-F-no-orm.md) — deps: BETA-004-E — #521
254. [BETA-004-V — Verify Implement optional first-party Postgres capability](tasks/08_public_beta/BETA-004-V-verify-implement-optional-first-party-postgres-capability.md) — deps: BETA-004-A, BETA-004-B, BETA-004-C, BETA-004-D, BETA-004-E, BETA-004-F — #522
255. [BETA-004-Z — Package evidence for Implement optional first-party Postgres capability](tasks/08_public_beta/BETA-004-Z-package-evidence-for-implement-optional-first-party-postgres-capability.md) — deps: BETA-004-V — #523
256. [BETA-005-A — Support one approved JWT algorithm/profile](tasks/08_public_beta/BETA-005-A-support-one-approved-jwt-algorithm-profile.md) — deps: M27-GATE, M25-GATE — #524
257. [BETA-005-B — Key loading/rotation hooks](tasks/08_public_beta/BETA-005-B-key-loading-rotation-hooks.md) — deps: BETA-005-A — #525
258. [BETA-005-C — Expiry/audience/issuer checks](tasks/08_public_beta/BETA-005-C-expiry-audience-issuer-checks.md) — deps: BETA-005-B — #526
259. [BETA-005-D — Typed 401/403 problems](tasks/08_public_beta/BETA-005-D-typed-401-403-problems.md) — deps: BETA-005-C — #527
260. [BETA-005-E — No secret logging](tasks/08_public_beta/BETA-005-E-no-secret-logging.md) — deps: BETA-005-D — #528
261. [BETA-005-V — Verify Implement JWT/auth reference package](tasks/08_public_beta/BETA-005-V-verify-implement-jwt-auth-reference-package.md) — deps: BETA-005-A, BETA-005-B, BETA-005-C, BETA-005-D, BETA-005-E — #529
262. [BETA-005-Z — Package evidence for Implement JWT/auth reference package](tasks/08_public_beta/BETA-005-Z-package-evidence-for-implement-jwt-auth-reference-package.md) — deps: BETA-005-V — #530
263. [BETA-006-A — Request/route/status/duration](tasks/08_public_beta/BETA-006-A-request-route-status-duration.md) — deps: M3-GATE, M28-GATE — #531
264. [BETA-006-B — Worker queues/quarantine/replacements](tasks/08_public_beta/BETA-006-B-worker-queues-quarantine-replacements.md) — deps: BETA-006-A — #532
265. [BETA-006-C — Fetch and DB pools](tasks/08_public_beta/BETA-006-C-fetch-and-db-pools.md) — deps: BETA-006-B — #533
266. [BETA-006-D — Memory/tasks/slots](tasks/08_public_beta/BETA-006-D-memory-tasks-slots.md) — deps: BETA-006-C — #534
267. [BETA-006-E — Optional trace integration or trace IDs](tasks/08_public_beta/BETA-006-E-optional-trace-integration-or-trace-ids.md) — deps: BETA-006-D — #535
268. [BETA-006-F — Redaction](tasks/08_public_beta/BETA-006-F-redaction.md) — deps: BETA-006-E — #536
269. [BETA-006-V — Verify Implement beta observability baseline](tasks/08_public_beta/BETA-006-V-verify-implement-beta-observability-baseline.md) — deps: BETA-006-A, BETA-006-B, BETA-006-C, BETA-006-D, BETA-006-E, BETA-006-F — #537
270. [BETA-006-Z — Package evidence for Implement beta observability baseline](tasks/08_public_beta/BETA-006-Z-package-evidence-for-implement-beta-observability-baseline.md) — deps: BETA-006-V — #538
271. [BETA-007-A — Environment/file configuration](tasks/08_public_beta/BETA-007-A-environment-file-configuration.md) — deps: M27-GATE — #539
272. [BETA-007-B — Validation at startup](tasks/08_public_beta/BETA-007-B-validation-at-startup.md) — deps: BETA-007-A — #540
273. [BETA-007-C — Secret value wrapper/redaction](tasks/08_public_beta/BETA-007-C-secret-value-wrapper-redaction.md) — deps: BETA-007-B — #541
274. [BETA-007-D — Profile-specific settings](tasks/08_public_beta/BETA-007-D-profile-specific-settings.md) — deps: BETA-007-C — #542
275. [BETA-007-E — No dynamic code execution](tasks/08_public_beta/BETA-007-E-no-dynamic-code-execution.md) — deps: BETA-007-D — #543
276. [BETA-007-V — Verify Implement configuration and secret handling](tasks/08_public_beta/BETA-007-V-verify-implement-configuration-and-secret-handling.md) — deps: BETA-007-A, BETA-007-B, BETA-007-C, BETA-007-D, BETA-007-E — #544
277. [BETA-007-Z — Package evidence for Implement configuration and secret handling](tasks/08_public_beta/BETA-007-Z-package-evidence-for-implement-configuration-and-secret-handling.md) — deps: BETA-007-V — #545
278. [BETA-008-A — Trusted proxy configuration](tasks/08_public_beta/BETA-008-A-trusted-proxy-configuration.md) — deps: M3-GATE, BETA-006-Z — #546
279. [BETA-008-B — Forwarded header policy](tasks/08_public_beta/BETA-008-B-forwarded-header-policy.md) — deps: BETA-008-A — #547
280. [BETA-008-C — Liveness/readiness/startup endpoints](tasks/08_public_beta/BETA-008-C-liveness-readiness-startup-endpoints.md) — deps: BETA-008-B — #548
281. [BETA-008-D — Graceful drain and termination](tasks/08_public_beta/BETA-008-D-graceful-drain-and-termination.md) — deps: BETA-008-C — #549
282. [BETA-008-E — Container example](tasks/08_public_beta/BETA-008-E-container-example.md) — deps: BETA-008-D — #550
283. [BETA-008-V — Verify Implement reverse-proxy, drain, and deployment semantics](tasks/08_public_beta/BETA-008-V-verify-implement-reverse-proxy-drain-and-deployment-semantics.md) — deps: BETA-008-A, BETA-008-B, BETA-008-C, BETA-008-D, BETA-008-E — #551
284. [BETA-008-Z — Package evidence for Implement reverse-proxy, drain, and deployment semantics](tasks/08_public_beta/BETA-008-Z-package-evidence-for-implement-reverse-proxy-drain-and-deployment-semantics.md) — deps: BETA-008-V — #552
285. [BETA-009-A — Run fuzz suites for pack/router/schema/bridge/HTTP](tasks/08_public_beta/BETA-009-A-run-fuzz-suites-for-pack-router-schema-bridge-http.md) — deps: M28-GATE, M3-GATE, BETA-004-Z, BETA-005-Z, BETA-007-Z — #553
286. [BETA-009-B — Dependency vulnerability and license scan](tasks/08_public_beta/BETA-009-B-dependency-vulnerability-and-license-scan.md) — deps: BETA-009-A — #554
287. [BETA-009-C — Threat-model review](tasks/08_public_beta/BETA-009-C-threat-model-review.md) — deps: BETA-009-B — #555
288. [BETA-009-D — Chaos tests for upstream/DB/worker poison](tasks/08_public_beta/BETA-009-D-chaos-tests-for-upstream-db-worker-poison.md) — deps: BETA-009-C — #556
289. [BETA-009-E — No known critical/high exploitable issue](tasks/08_public_beta/BETA-009-E-no-known-critical-high-exploitable-issue.md) — deps: BETA-009-D — #557
290. [BETA-009-V — Verify Run beta security and reliability baseline](tasks/08_public_beta/BETA-009-V-verify-run-beta-security-and-reliability-baseline.md) — deps: BETA-009-A, BETA-009-B, BETA-009-C, BETA-009-D, BETA-009-E — #558
291. [BETA-009-Z — Package evidence for Run beta security and reliability baseline](tasks/08_public_beta/BETA-009-Z-package-evidence-for-run-beta-security-and-reliability-baseline.md) — deps: BETA-009-V — #559
292. [BETA-010-A — Linux x86_64 glibc mandatory working assumption](tasks/08_public_beta/BETA-010-A-linux-x86-64-glibc-mandatory-working-assumption.md) — deps: M26-GATE, M4A-002-Z — #560
293. [BETA-010-B — Linux ARM64 glibc when CI is available](tasks/08_public_beta/BETA-010-B-linux-arm64-glibc-when-ci-is-available.md) — deps: BETA-010-A — #561
294. [BETA-010-C — npm packages under beta tag](tasks/08_public_beta/BETA-010-C-npm-packages-under-beta-tag.md) — deps: BETA-010-B — #562
295. [BETA-010-D — Runtime binary/QPack tools](tasks/08_public_beta/BETA-010-D-runtime-binary-qpack-tools.md) — deps: BETA-010-C — #563
296. [BETA-010-E — Clean install tests](tasks/08_public_beta/BETA-010-E-clean-install-tests.md) — deps: BETA-010-D — #564
297. [BETA-010-V — Verify Create supported beta platform and packaging matrix](tasks/08_public_beta/BETA-010-V-verify-create-supported-beta-platform-and-packaging-matrix.md) — deps: BETA-010-A, BETA-010-B, BETA-010-C, BETA-010-D, BETA-010-E — #565
298. [BETA-010-Z — Package evidence for Create supported beta platform and packaging matrix](tasks/08_public_beta/BETA-010-Z-package-evidence-for-create-supported-beta-platform-and-packaging-matrix.md) — deps: BETA-010-V — #566
299. [BETA-011-A — Use SemVer prerelease](tasks/08_public_beta/BETA-011-A-use-semver-prerelease.md) — deps: M4A-GATE, BETA-010-Z — #567
300. [BETA-011-B — Publish `next`/beta tag](tasks/08_public_beta/BETA-011-B-publish-next-beta-tag.md) — deps: BETA-011-A — #568
301. [BETA-011-C — Generate changelog and migration notes](tasks/08_public_beta/BETA-011-C-generate-changelog-and-migration-notes.md) — deps: BETA-011-B — #569
302. [BETA-011-D — Create GitHub-style release packet](tasks/08_public_beta/BETA-011-D-create-github-style-release-packet.md) — deps: BETA-011-C — #570
303. [BETA-011-E — Support yanking/rollback](tasks/08_public_beta/BETA-011-E-support-yanking-rollback.md) — deps: BETA-011-D — #571
304. [BETA-011-V — Verify Automate beta publishing and versioning](tasks/08_public_beta/BETA-011-V-verify-automate-beta-publishing-and-versioning.md) — deps: BETA-011-A, BETA-011-B, BETA-011-C, BETA-011-D, BETA-011-E — #572
305. [BETA-011-Z — Package evidence for Automate beta publishing and versioning](tasks/08_public_beta/BETA-011-Z-package-evidence-for-automate-beta-publishing-and-versioning.md) — deps: BETA-011-V — #573
306. [BETA-012-A — Installation](tasks/08_public_beta/BETA-012-A-installation.md) — deps: M4A-GATE, BETA-004-Z, BETA-005-Z, BETA-008-Z — #574
307. [BETA-012-B — Quickstart](tasks/08_public_beta/BETA-012-B-quickstart.md) — deps: BETA-012-A — #575
308. [BETA-012-C — Architecture](tasks/08_public_beta/BETA-012-C-architecture.md) — deps: BETA-012-B — #576
309. [BETA-012-D — Contracts/Treaty](tasks/08_public_beta/BETA-012-D-contracts-treaty.md) — deps: BETA-012-C — #577
310. [BETA-012-E — Fetch/Postgres/auth](tasks/08_public_beta/BETA-012-E-fetch-postgres-auth.md) — deps: BETA-012-D — #578
311. [BETA-012-F — Deployment](tasks/08_public_beta/BETA-012-F-deployment.md) — deps: BETA-012-E — #579
312. [BETA-012-G — Troubleshooting](tasks/08_public_beta/BETA-012-G-troubleshooting.md) — deps: BETA-012-F — #580
313. [BETA-012-H — Performance methodology](tasks/08_public_beta/BETA-012-H-performance-methodology.md) — deps: BETA-012-G — #581
314. [BETA-012-I — Limitations/non-goals](tasks/08_public_beta/BETA-012-I-limitations-non-goals.md) — deps: BETA-012-H — #582
315. [BETA-012-V — Verify Complete beta documentation and limitations](tasks/08_public_beta/BETA-012-V-verify-complete-beta-documentation-and-limitations.md) — deps: BETA-012-A, BETA-012-B, BETA-012-C, BETA-012-D, BETA-012-E, BETA-012-F, BETA-012-G, BETA-012-H, BETA-012-I — #583
316. [BETA-012-Z — Package evidence for Complete beta documentation and limitations](tasks/08_public_beta/BETA-012-Z-package-evidence-for-complete-beta-documentation-and-limitations.md) — deps: BETA-012-V — #584
317. [BETA-013-A — Run at least two-hour mixed workload and at least one million requests on reference platform](tasks/08_public_beta/BETA-013-A-run-at-least-two-hour-mixed-workload-and-at-least-one-million-requests-on-refere.md) — deps: BETA-004-Z, BETA-005-Z, BETA-006-Z, BETA-008-Z, BETA-009-Z — #585
318. [BETA-013-B — Include fetch, DB, auth, timeouts, cancellation, worker replacement, and reload](tasks/08_public_beta/BETA-013-B-include-fetch-db-auth-timeouts-cancellation-worker-replacement-and-reload.md) — deps: BETA-013-A — #586
319. [BETA-013-C — Track RSS, heap, slots, tasks, queues, pools, and errors](tasks/08_public_beta/BETA-013-C-track-rss-heap-slots-tasks-queues-pools-and-errors.md) — deps: BETA-013-B — #587
320. [BETA-013-D — Analyze retained growth](tasks/08_public_beta/BETA-013-D-analyze-retained-growth.md) — deps: BETA-013-C — #588
321. [BETA-013-V — Verify Run beta soak and leak qualification](tasks/08_public_beta/BETA-013-V-verify-run-beta-soak-and-leak-qualification.md) — deps: BETA-013-A, BETA-013-B, BETA-013-C, BETA-013-D — #589
322. [BETA-013-Z — Package evidence for Run beta soak and leak qualification](tasks/08_public_beta/BETA-013-Z-package-evidence-for-run-beta-soak-and-leak-qualification.md) — deps: BETA-013-V — #590
323. [BETA-014-A — Include cold start categories, warm microbenchmarks, real DB/auth/I/O, CPU/JIT crossover, cost-normalized metrics, and limitations](tasks/08_public_beta/BETA-014-A-include-cold-start-categories-warm-microbenchmarks-real-db-auth-i-o-cpu-jit-cros.md) — deps: BETA-002-Z, BETA-003-Z, BETA-004-Z, BETA-005-Z, BETA-013-Z — #591
324. [BETA-014-B — Pin all candidates/artifacts](tasks/08_public_beta/BETA-014-B-pin-all-candidates-artifacts.md) — deps: BETA-014-A — #592
325. [BETA-014-C — Retain raw data](tasks/08_public_beta/BETA-014-C-retain-raw-data.md) — deps: BETA-014-B — #593
326. [BETA-014-D — Have wording reviewed](tasks/08_public_beta/BETA-014-D-have-wording-reviewed.md) — deps: BETA-014-C — #594
327. [BETA-014-V — Verify Publish canonical beta benchmark report](tasks/08_public_beta/BETA-014-V-verify-publish-canonical-beta-benchmark-report.md) — deps: BETA-014-A, BETA-014-B, BETA-014-C, BETA-014-D — #595
328. [BETA-014-Z — Package evidence for Publish canonical beta benchmark report](tasks/08_public_beta/BETA-014-Z-package-evidence-for-publish-canonical-beta-benchmark-report.md) — deps: BETA-014-V — #596
329. [BETA-015-A — Source ZIP](tasks/08_public_beta/BETA-015-A-source-zip.md) — deps: BETA-009-Z, BETA-010-Z, BETA-011-Z, BETA-013-Z, BETA-014-Z — #597
330. [BETA-015-B — Git bundle](tasks/08_public_beta/BETA-015-B-git-bundle.md) — deps: BETA-015-A — #598
331. [BETA-015-C — Linux binaries](tasks/08_public_beta/BETA-015-C-linux-binaries.md) — deps: BETA-015-B — #599
332. [BETA-015-D — npm package tarballs](tasks/08_public_beta/BETA-015-D-npm-package-tarballs.md) — deps: BETA-015-C — #600
333. [BETA-015-E — QPack tools](tasks/08_public_beta/BETA-015-E-qpack-tools.md) — deps: BETA-015-D — #601
334. [BETA-015-F — SBOM](tasks/08_public_beta/BETA-015-F-sbom.md) — deps: BETA-015-E — #602
335. [BETA-015-G — Checksums](tasks/08_public_beta/BETA-015-G-checksums.md) — deps: BETA-015-F — #603
336. [BETA-015-H — Review/evidence indexes](tasks/08_public_beta/BETA-015-H-review-evidence-indexes.md) — deps: BETA-015-G — #604
337. [BETA-015-I — Known limitations](tasks/08_public_beta/BETA-015-I-known-limitations.md) — deps: BETA-015-H — #605
338. [BETA-015-V — Verify Generate beta release evidence, SBOM, and checksums](tasks/08_public_beta/BETA-015-V-verify-generate-beta-release-evidence-sbom-and-checksums.md) — deps: BETA-015-A, BETA-015-B, BETA-015-C, BETA-015-D, BETA-015-E, BETA-015-F, BETA-015-G, BETA-015-H, BETA-015-I — #606
339. [BETA-015-Z — Package evidence for Generate beta release evidence, SBOM, and checksums](tasks/08_public_beta/BETA-015-Z-package-evidence-for-generate-beta-release-evidence-sbom-and-checksums.md) — deps: BETA-015-V — #607
340. [BETA-016-A — Fresh Linux VM/container](tasks/08_public_beta/BETA-016-A-fresh-linux-vm-container.md) — deps: BETA-011-Z, BETA-012-Z, BETA-015-Z — #608
341. [BETA-016-B — Install CLI/runtime](tasks/08_public_beta/BETA-016-B-install-cli-runtime.md) — deps: BETA-016-A — #609
342. [BETA-016-C — Scaffold app](tasks/08_public_beta/BETA-016-C-scaffold-app.md) — deps: BETA-016-B — #610
343. [BETA-016-D — Run tests/dev/build](tasks/08_public_beta/BETA-016-D-run-tests-dev-build.md) — deps: BETA-016-C — #611
344. [BETA-016-E — Deploy proof service](tasks/08_public_beta/BETA-016-E-deploy-proof-service.md) — deps: BETA-016-D — #612
345. [BETA-016-F — Use Treaty client](tasks/08_public_beta/BETA-016-F-use-treaty-client.md) — deps: BETA-016-E — #613
346. [BETA-016-V — Verify Run external clean-install and tutorial verification](tasks/08_public_beta/BETA-016-V-verify-run-external-clean-install-and-tutorial-verification.md) — deps: BETA-016-A, BETA-016-B, BETA-016-C, BETA-016-D, BETA-016-E, BETA-016-F — #614
347. [BETA-016-Z — Package evidence for Run external clean-install and tutorial verification](tasks/08_public_beta/BETA-016-Z-package-evidence-for-run-external-clean-install-and-tutorial-verification.md) — deps: BETA-016-V — #615
348. [BETA-GATE — Public Beta Readiness and Release exit gate](gates/BETA-GATE.md) — deps: BETA-001-Z, BETA-002-Z, BETA-003-Z, BETA-004-Z, BETA-005-Z, BETA-006-Z, BETA-007-Z, BETA-008-Z, BETA-009-Z, BETA-010-Z, BETA-011-Z, BETA-012-Z, BETA-013-Z, BETA-014-Z, BETA-015-Z, BETA-016-Z, BETA-017-Z — #625
