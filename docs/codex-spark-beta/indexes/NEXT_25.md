# Next 25 Dependency-Safe Tasks

The next dependency-ready implementation task is M24-001-V; this short queue lists the first 25 unchecked packets with their issue numbers.

1. [M24-001-V — Verify Freeze ingress ownership and backpressure design](tasks/01_m24_zero_copy_ingress/M24-001-V-verify-freeze-ingress-ownership-and-backpressure-design.md) — deps: M24-001-A, M24-001-B, M24-001-C, M24-001-D — #64
2. [M24-001-Z — Package evidence for Freeze ingress ownership and backpressure design](tasks/01_m24_zero_copy_ingress/M24-001-Z-package-evidence-for-freeze-ingress-ownership-and-backpressure-design.md) — deps: M24-001-V — #65
3. [M24-007-C — Enforce content length and streaming limits](tasks/01_m24_zero_copy_ingress/M24-007-C-enforce-content-length-and-streaming-limits.md) — deps: M24-007-B — #98
4. [M24-007-D — Cache one decoded representation and reject incompatible second reads](tasks/01_m24_zero_copy_ingress/M24-007-D-cache-one-decoded-representation-and-reject-incompatible-second-reads.md) — deps: M24-007-C — #99
5. [M24-007-V — Verify Implement bounded read-once body admission](tasks/01_m24_zero_copy_ingress/M24-007-V-verify-implement-bounded-read-once-body-admission.md) — deps: M24-007-A, M24-007-B, M24-007-C, M24-007-D — #100
6. [M24-007-Z — Package evidence for Implement bounded read-once body admission](tasks/01_m24_zero_copy_ingress/M24-007-Z-package-evidence-for-implement-bounded-read-once-body-admission.md) — deps: M24-007-V — #101
7. [M24-008-A — Create shared Context/Request prototypes or native classes](tasks/01_m24_zero_copy_ingress/M24-008-A-create-shared-context-request-prototypes-or-native-classes.md) — deps: M24-003-Z, M24-005-Z, M24-006-Z, M24-007-Z — #102
8. [M24-008-B — Store only opaque handle and route plan references per request](tasks/01_m24_zero_copy_ingress/M24-008-B-store-only-opaque-handle-and-route-plan-references-per-request.md) — deps: M24-008-A — #103
9. [M24-008-C — Cache native capability objects](tasks/01_m24_zero_copy_ingress/M24-008-C-cache-native-capability-objects.md) — deps: M24-008-B — #104
10. [M24-008-D — Keep full Web Request construction as explicit fallback](tasks/01_m24_zero_copy_ingress/M24-008-D-keep-full-web-request-construction-as-explicit-fallback.md) — deps: M24-008-C — #105
11. [M24-008-V — Verify Replace per-request JS closures with native-backed prototypes](tasks/01_m24_zero_copy_ingress/M24-008-V-verify-replace-per-request-js-closures-with-native-backed-prototypes.md) — deps: M24-008-A, M24-008-B, M24-008-C, M24-008-D — #106
12. [M24-008-Z — Package evidence for Replace per-request JS closures with native-backed prototypes](tasks/01_m24_zero_copy_ingress/M24-008-Z-package-evidence-for-replace-per-request-js-closures-with-native-backed-prototyp.md) — deps: M24-008-V — #107
13. [M24-009-A — Add counters/histograms for route, queue, decode, bridge, JS, encode, and write stages](tasks/01_m24_zero_copy_ingress/M24-009-A-add-counters-histograms-for-route-queue-decode-bridge-js-encode-and-write-stages.md) — deps: M24-002-Z, M24-003-Z — #108
14. [M24-009-B — Use disabled-by-default or sampled recording](tasks/01_m24_zero_copy_ingress/M24-009-B-use-disabled-by-default-or-sampled-recording.md) — deps: M24-009-A — #109
15. [M24-009-C — Expose slab/queue/body gauges](tasks/01_m24_zero_copy_ingress/M24-009-C-expose-slab-queue-body-gauges.md) — deps: M24-009-B — #110
16. [M24-009-D — Measure instrumentation overhead](tasks/01_m24_zero_copy_ingress/M24-009-D-measure-instrumentation-overhead.md) — deps: M24-009-C — #111
17. [M24-009-V — Verify Add ingress and bridge observability](tasks/01_m24_zero_copy_ingress/M24-009-V-verify-add-ingress-and-bridge-observability.md) — deps: M24-009-A, M24-009-B, M24-009-C, M24-009-D — #112
18. [M24-009-Z — Package evidence for Add ingress and bridge observability](tasks/01_m24_zero_copy_ingress/M24-009-Z-package-evidence-for-add-ingress-and-bridge-observability.md) — deps: M24-009-V — #113
19. [M24-010-A — Fuzz paths, queries, headers, cookies, bodies, handles, and cancellation orderings](tasks/01_m24_zero_copy_ingress/M24-010-A-fuzz-paths-queries-headers-cookies-bodies-handles-and-cancellation-orderings.md) — deps: M24-004-Z, M24-005-Z, M24-006-Z, M24-007-Z, M24-008-Z — #114
20. [M24-010-B — Differentially compare legacy/reference decoding where applicable](tasks/01_m24_zero_copy_ingress/M24-010-B-differentially-compare-legacy-reference-decoding-where-applicable.md) — deps: M24-010-A — #115
21. [M24-010-C — Run property tests for slot lifecycle](tasks/01_m24_zero_copy_ingress/M24-010-C-run-property-tests-for-slot-lifecycle.md) — deps: M24-010-B — #116
22. [M24-010-D — Capture and minimize failures](tasks/01_m24_zero_copy_ingress/M24-010-D-capture-and-minimize-failures.md) — deps: M24-010-C — #117
23. [M24-010-V — Verify Complete ingress bridge fuzzing and conformance](tasks/01_m24_zero_copy_ingress/M24-010-V-verify-complete-ingress-bridge-fuzzing-and-conformance.md) — deps: M24-010-A, M24-010-B, M24-010-C, M24-010-D — #118
24. [M24-010-Z — Package evidence for Complete ingress bridge fuzzing and conformance](tasks/01_m24_zero_copy_ingress/M24-010-Z-package-evidence-for-complete-ingress-bridge-fuzzing-and-conformance.md) — deps: M24-010-V — #119
25. [M24-GATE — M2.4 — Zero-Copy Ingress and Worker-Local Request Bridge exit gate](gates/M24-GATE.md) — deps: M24-001-Z, M24-002-Z, M24-003-Z, M24-004-Z, M24-005-Z, M24-006-Z, M24-007-Z, M24-008-Z, M24-009-Z, M24-010-Z — #627
