# Next 25 Dependency-Safe Tasks

The next dependency-ready implementation task is M24-001-V; this short queue lists the first 25 unchecked packets with their issue numbers.

1. [M24-001-V — Verify Freeze ingress ownership and backpressure design](tasks/01_m24_zero_copy_ingress/M24-001-V-verify-freeze-ingress-ownership-and-backpressure-design.md) — deps: M24-001-A, M24-001-B, M24-001-C, M24-001-D — #64
2. [M24-001-Z — Package evidence for Freeze ingress ownership and backpressure design](tasks/01_m24_zero_copy_ingress/M24-001-Z-package-evidence-for-freeze-ingress-ownership-and-backpressure-design.md) — deps: M24-001-V — #65
3. [M24-008-Z — Package evidence for Replace per-request JS closures with native-backed prototypes](tasks/01_m24_zero_copy_ingress/M24-008-Z-package-evidence-for-replace-per-request-js-closures-with-native-backed-prototyp.md) — deps: M24-008-V — #107
4. [M24-009-A — Add counters/histograms for route, queue, decode, bridge, JS, encode, and write stages](tasks/01_m24_zero_copy_ingress/M24-009-A-add-counters-histograms-for-route-queue-decode-bridge-js-encode-and-write-stages.md) — deps: M24-002-Z, M24-003-Z — #108
5. [M24-009-B — Use disabled-by-default or sampled recording](tasks/01_m24_zero_copy_ingress/M24-009-B-use-disabled-by-default-or-sampled-recording.md) — deps: M24-009-A — #109
6. [M24-009-C — Expose slab/queue/body gauges](tasks/01_m24_zero_copy_ingress/M24-009-C-expose-slab-queue-body-gauges.md) — deps: M24-009-B — #110
7. [M24-009-D — Measure instrumentation overhead](tasks/01_m24_zero_copy_ingress/M24-009-D-measure-instrumentation-overhead.md) — deps: M24-009-C — #111
8. [M24-009-V — Verify Add ingress and bridge observability](tasks/01_m24_zero_copy_ingress/M24-009-V-verify-add-ingress-and-bridge-observability.md) — deps: M24-009-A, M24-009-B, M24-009-C, M24-009-D — #112
9. [M24-009-Z — Package evidence for Add ingress and bridge observability](tasks/01_m24_zero_copy_ingress/M24-009-Z-package-evidence-for-add-ingress-and-bridge-observability.md) — deps: M24-009-V — #113
10. [M24-010-A — Fuzz paths, queries, headers, cookies, bodies, handles, and cancellation orderings](tasks/01_m24_zero_copy_ingress/M24-010-A-fuzz-paths-queries-headers-cookies-bodies-handles-and-cancellation-orderings.md) — deps: M24-004-Z, M24-005-Z, M24-006-Z, M24-007-Z, M24-008-Z — #114
11. [M24-010-B — Differentially compare legacy/reference decoding where applicable](tasks/01_m24_zero_copy_ingress/M24-010-B-differentially-compare-legacy-reference-decoding-where-applicable.md) — deps: M24-010-A — #115
12. [M24-010-C — Run property tests for slot lifecycle](tasks/01_m24_zero_copy_ingress/M24-010-C-run-property-tests-for-slot-lifecycle.md) — deps: M24-010-B — #116
13. [M24-010-D — Capture and minimize failures](tasks/01_m24_zero_copy_ingress/M24-010-D-capture-and-minimize-failures.md) — deps: M24-010-C — #117
14. [M24-010-V — Verify Complete ingress bridge fuzzing and conformance](tasks/01_m24_zero_copy_ingress/M24-010-V-verify-complete-ingress-bridge-fuzzing-and-conformance.md) — deps: M24-010-A, M24-010-B, M24-010-C, M24-010-D — #118
15. [M24-010-Z — Package evidence for Complete ingress bridge fuzzing and conformance](tasks/01_m24_zero_copy_ingress/M24-010-Z-package-evidence-for-complete-ingress-bridge-fuzzing-and-conformance.md) — deps: M24-010-V — #119
16. [M24-GATE — M2.4 — Zero-Copy Ingress and Worker-Local Request Bridge exit gate](gates/M24-GATE.md) — deps: M24-001-Z, M24-002-Z, M24-003-Z, M24-004-Z, M24-005-Z, M24-006-Z, M24-007-Z, M24-008-Z, M24-009-Z, M24-010-Z — #627
17. [M25-001-A — Specify objects, arrays, unions, literals, enums, formats, defaults, optional/null, transforms, files, and problem schemas](tasks/02_m25_schema_codecs/M25-001-A-specify-objects-arrays-unions-literals-enums-formats-defaults-optional-null-tran.md) — deps: M24-GATE — #120
18. [M25-001-B — Define compatibility and fallback markers](tasks/02_m25_schema_codecs/M25-001-B-define-compatibility-and-fallback-markers.md) — deps: M25-001-A — #121
19. [M25-001-C — Canonicalize ordering and hashing](tasks/02_m25_schema_codecs/M25-001-C-canonicalize-ordering-and-hashing.md) — deps: M25-001-B — #122
20. [M25-001-D — Document unsupported transformations](tasks/02_m25_schema_codecs/M25-001-D-document-unsupported-transformations.md) — deps: M25-001-C — #123
21. [M25-001-V — Verify Define canonical Schema IR v2](tasks/02_m25_schema_codecs/M25-001-V-verify-define-canonical-schema-ir-v2.md) — deps: M25-001-A, M25-001-B, M25-001-C, M25-001-D — #124
22. [M25-001-Z — Package evidence for Define canonical Schema IR v2](tasks/02_m25_schema_codecs/M25-001-Z-package-evidence-for-define-canonical-schema-ir-v2.md) — deps: M25-001-V — #125
23. [M25-002-A — Compare QuickJS parse/stringify, generic Rust conversion, and generated schema-aware codecs](tasks/02_m25_schema_codecs/M25-002-A-compare-quickjs-parse-stringify-generic-rust-conversion-and-generated-schema-awa.md) — deps: M25-001-Z — #126
24. [M25-002-B — Use 256B, 1KB, 16KB, 64KB, nested objects, arrays 100/1,000, optional/null, and problems](tasks/02_m25_schema_codecs/M25-002-B-use-256b-1kb-16kb-64kb-nested-objects-arrays-100-1-000-optional-null-and-problem.md) — deps: M25-002-A — #127
25. [M25-002-C — Capture CPU, allocation, bridge time, and tails](tasks/02_m25_schema_codecs/M25-002-C-capture-cpu-allocation-bridge-time-and-tails.md) — deps: M25-002-B — #128
