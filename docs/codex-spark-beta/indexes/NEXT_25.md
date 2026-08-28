# Next 25 Dependency-Safe Tasks

The next dependency-ready implementation task is M28-004-C; this short queue lists the first 25 unchecked packets with their issue numbers.

1. [M28-004-C — Define clone/body-used semantics for beta](tasks/05_m28_native_fetch/M28-004-C-define-clone-body-used-semantics-for-beta.md) — deps: M28-004-B — #326
2. [M28-004-D — Keep unsupported API diagnostics explicit](tasks/05_m28_native_fetch/M28-004-D-keep-unsupported-api-diagnostics-explicit.md) — deps: M28-004-C — #327
3. [M28-004-V — Verify Implement Request, Response, and Headers subset](tasks/05_m28_native_fetch/M28-004-V-verify-implement-request-response-and-headers-subset.md) — deps: M28-004-A, M28-004-B, M28-004-C, M28-004-D — #328
4. [M28-004-Z — Package evidence for Implement Request, Response, and Headers subset](tasks/05_m28_native_fetch/M28-004-Z-package-evidence-for-implement-request-response-and-headers-subset.md) — deps: M28-004-V — #329
5. [M28-005-A — Combine explicit abort, route deadline, disconnect, shutdown, and quarantine](tasks/05_m28_native_fetch/M28-005-A-combine-explicit-abort-route-deadline-disconnect-shutdown-and-quarantine.md) — deps: M28-003-Z, M27-007-Z — #330
6. [M28-005-B — Use one terminal state for each operation](tasks/05_m28_native_fetch/M28-005-B-use-one-terminal-state-for-each-operation.md) — deps: M28-005-A — #331
7. [M28-005-C — Cancel DNS/connect/body streaming](tasks/05_m28_native_fetch/M28-005-C-cancel-dns-connect-body-streaming.md) — deps: M28-005-B — #332
8. [M28-005-D — Map failures deterministically](tasks/05_m28_native_fetch/M28-005-D-map-failures-deterministically.md) — deps: M28-005-C — #333
9. [M28-005-V — Verify Propagate AbortSignal and route deadlines](tasks/05_m28_native_fetch/M28-005-V-verify-propagate-abortsignal-and-route-deadlines.md) — deps: M28-005-A, M28-005-B, M28-005-C, M28-005-D — #334
10. [M28-005-Z — Package evidence for Propagate AbortSignal and route deadlines](tasks/05_m28_native_fetch/M28-005-Z-package-evidence-for-propagate-abortsignal-and-route-deadlines.md) — deps: M28-005-V — #335
11. [M28-006-A — Bound read/write buffers](tasks/05_m28_native_fetch/M28-006-A-bound-read-write-buffers.md) — deps: M28-004-Z, M28-005-Z — #336
12. [M28-006-B — Propagate downstream backpressure](tasks/05_m28_native_fetch/M28-006-B-propagate-downstream-backpressure.md) — deps: M28-006-A — #337
13. [M28-006-C — Cancel on consumer stop/disconnect](tasks/05_m28_native_fetch/M28-006-C-cancel-on-consumer-stop-disconnect.md) — deps: M28-006-B — #338
14. [M28-006-D — Define maximum body helper sizes](tasks/05_m28_native_fetch/M28-006-D-define-maximum-body-helper-sizes.md) — deps: M28-006-C — #339
15. [M28-006-V — Verify Implement streaming and strict backpressure](tasks/05_m28_native_fetch/M28-006-V-verify-implement-streaming-and-strict-backpressure.md) — deps: M28-006-A, M28-006-B, M28-006-C, M28-006-D — #340
16. [M28-006-Z — Package evidence for Implement streaming and strict backpressure](tasks/05_m28_native_fetch/M28-006-Z-package-evidence-for-implement-streaming-and-strict-backpressure.md) — deps: M28-006-V — #341
17. [M28-007-A — Limit redirect count](tasks/05_m28_native_fetch/M28-007-A-limit-redirect-count.md) — deps: M28-003-Z, M28-004-Z — #342
18. [M28-007-B — Reapply SSRF/DNS policy on every hop](tasks/05_m28_native_fetch/M28-007-B-reapply-ssrf-dns-policy-on-every-hop.md) — deps: M28-007-A — #343
19. [M28-007-C — Define credential/header stripping](tasks/05_m28_native_fetch/M28-007-C-define-credential-header-stripping.md) — deps: M28-007-B — #344
20. [M28-007-D — Bound decompression ratio and output](tasks/05_m28_native_fetch/M28-007-D-bound-decompression-ratio-and-output.md) — deps: M28-007-C — #345
21. [M28-007-V — Verify Implement redirect and compression policy](tasks/05_m28_native_fetch/M28-007-V-verify-implement-redirect-and-compression-policy.md) — deps: M28-007-A, M28-007-B, M28-007-C, M28-007-D — #346
22. [M28-007-Z — Package evidence for Implement redirect and compression policy](tasks/05_m28_native_fetch/M28-007-Z-package-evidence-for-implement-redirect-and-compression-policy.md) — deps: M28-007-V — #347
23. [M28-008-A — Resolve and validate addresses before connect](tasks/05_m28_native_fetch/M28-008-A-resolve-and-validate-addresses-before-connect.md) — deps: M28-001-Z, M28-003-Z, M28-007-Z — #348
24. [M28-008-B — Revalidate redirects and connection targets](tasks/05_m28_native_fetch/M28-008-B-revalidate-redirects-and-connection-targets.md) — deps: M28-008-A — #349
25. [M28-008-C — Support allow/deny configuration](tasks/05_m28_native_fetch/M28-008-C-support-allow-deny-configuration.md) — deps: M28-008-B — #350
