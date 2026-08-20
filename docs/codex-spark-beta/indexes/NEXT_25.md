# Next 25 Dependency-Safe Tasks

The next dependency-ready implementation task is M24-001-V; this short queue lists the first 25 unchecked packets with their issue numbers.

1. [M24-001-V — Verify Freeze ingress ownership and backpressure design](tasks/01_m24_zero_copy_ingress/M24-001-V-verify-freeze-ingress-ownership-and-backpressure-design.md) — deps: M24-001-A, M24-001-B, M24-001-C, M24-001-D — #64
2. [M24-001-Z — Package evidence for Freeze ingress ownership and backpressure design](tasks/01_m24_zero_copy_ingress/M24-001-Z-package-evidence-for-freeze-ingress-ownership-and-backpressure-design.md) — deps: M24-001-V — #65
3. [M24-010-C — Run property tests for slot lifecycle](tasks/01_m24_zero_copy_ingress/M24-010-C-run-property-tests-for-slot-lifecycle.md) — deps: M24-010-B — #116
4. [M24-010-D — Capture and minimize failures](tasks/01_m24_zero_copy_ingress/M24-010-D-capture-and-minimize-failures.md) — deps: M24-010-C — #117
5. [M24-010-V — Verify Complete ingress bridge fuzzing and conformance](tasks/01_m24_zero_copy_ingress/M24-010-V-verify-complete-ingress-bridge-fuzzing-and-conformance.md) — deps: M24-010-A, M24-010-B, M24-010-C, M24-010-D — #118
6. [M24-010-Z — Package evidence for Complete ingress bridge fuzzing and conformance](tasks/01_m24_zero_copy_ingress/M24-010-Z-package-evidence-for-complete-ingress-bridge-fuzzing-and-conformance.md) — deps: M24-010-V — #119
7. [M24-GATE — M2.4 — Zero-Copy Ingress and Worker-Local Request Bridge exit gate](gates/M24-GATE.md) — deps: M24-001-Z, M24-002-Z, M24-003-Z, M24-004-Z, M24-005-Z, M24-006-Z, M24-007-Z, M24-008-Z, M24-009-Z, M24-010-Z — #627
8. [M25-001-A — Specify objects, arrays, unions, literals, enums, formats, defaults, optional/null, transforms, files, and problem schemas](tasks/02_m25_schema_codecs/M25-001-A-specify-objects-arrays-unions-literals-enums-formats-defaults-optional-null-tran.md) — deps: M24-GATE — #120
9. [M25-001-B — Define compatibility and fallback markers](tasks/02_m25_schema_codecs/M25-001-B-define-compatibility-and-fallback-markers.md) — deps: M25-001-A — #121
10. [M25-001-C — Canonicalize ordering and hashing](tasks/02_m25_schema_codecs/M25-001-C-canonicalize-ordering-and-hashing.md) — deps: M25-001-B — #122
11. [M25-001-D — Document unsupported transformations](tasks/02_m25_schema_codecs/M25-001-D-document-unsupported-transformations.md) — deps: M25-001-C — #123
12. [M25-001-V — Verify Define canonical Schema IR v2](tasks/02_m25_schema_codecs/M25-001-V-verify-define-canonical-schema-ir-v2.md) — deps: M25-001-A, M25-001-B, M25-001-C, M25-001-D — #124
13. [M25-001-Z — Package evidence for Define canonical Schema IR v2](tasks/02_m25_schema_codecs/M25-001-Z-package-evidence-for-define-canonical-schema-ir-v2.md) — deps: M25-001-V — #125
14. [M25-002-A — Compare QuickJS parse/stringify, generic Rust conversion, and generated schema-aware codecs](tasks/02_m25_schema_codecs/M25-002-A-compare-quickjs-parse-stringify-generic-rust-conversion-and-generated-schema-awa.md) — deps: M25-001-Z — #126
15. [M25-002-B — Use 256B, 1KB, 16KB, 64KB, nested objects, arrays 100/1,000, optional/null, and problems](tasks/02_m25_schema_codecs/M25-002-B-use-256b-1kb-16kb-64kb-nested-objects-arrays-100-1-000-optional-null-and-problem.md) — deps: M25-002-A — #127
16. [M25-002-C — Capture CPU, allocation, bridge time, and tails](tasks/02_m25_schema_codecs/M25-002-C-capture-cpu-allocation-bridge-time-and-tails.md) — deps: M25-002-B — #128
17. [M25-002-D — Select strategies by evidence](tasks/02_m25_schema_codecs/M25-002-D-select-strategies-by-evidence.md) — deps: M25-002-C — #129
18. [M25-002-V — Verify Build reproducible decoder/encoder strategy benchmark](tasks/02_m25_schema_codecs/M25-002-V-verify-build-reproducible-decoder-encoder-strategy-benchmark.md) — deps: M25-002-A, M25-002-B, M25-002-C, M25-002-D — #130
19. [M25-002-Z — Package evidence for Build reproducible decoder/encoder strategy benchmark](tasks/02_m25_schema_codecs/M25-002-Z-package-evidence-for-build-reproducible-decoder-encoder-strategy-benchmark.md) — deps: M25-002-V — #131
20. [M25-003-A — Generate direct decoder programs keyed by SchemaId](tasks/02_m25_schema_codecs/M25-003-A-generate-direct-decoder-programs-keyed-by-schemaid.md) — deps: M25-001-Z, M24-GATE — #132
21. [M25-003-B — Validate byte ranges and header/query values without generic object trees](tasks/02_m25_schema_codecs/M25-003-B-validate-byte-ranges-and-header-query-values-without-generic-object-trees.md) — deps: M25-003-A — #133
22. [M25-003-C — Return typed RFC 9457 problems](tasks/02_m25_schema_codecs/M25-003-C-return-typed-rfc-9457-problems.md) — deps: M25-003-B — #134
23. [M25-003-D — Preserve declared coercion semantics exactly](tasks/02_m25_schema_codecs/M25-003-D-preserve-declared-coercion-semantics-exactly.md) — deps: M25-003-C — #135
24. [M25-003-V — Verify Generate params/query/header decoders](tasks/02_m25_schema_codecs/M25-003-V-verify-generate-params-query-header-decoders.md) — deps: M25-003-A, M25-003-B, M25-003-C, M25-003-D — #136
25. [M25-003-Z — Package evidence for Generate params/query/header decoders](tasks/02_m25_schema_codecs/M25-003-Z-package-evidence-for-generate-params-query-header-decoders.md) — deps: M25-003-V — #137
