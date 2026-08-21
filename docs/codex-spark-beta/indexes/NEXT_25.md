# Next 25 Dependency-Safe Tasks

The next dependency-ready implementation task is M25-001-D; this short queue lists the first 25 unchecked packets with their issue numbers.

1. [M25-001-D — Document unsupported transformations](tasks/02_m25_schema_codecs/M25-001-D-document-unsupported-transformations.md) — deps: M25-001-C — #123
2. [M25-001-V — Verify Define canonical Schema IR v2](tasks/02_m25_schema_codecs/M25-001-V-verify-define-canonical-schema-ir-v2.md) — deps: M25-001-A, M25-001-B, M25-001-C, M25-001-D — #124
3. [M25-001-Z — Package evidence for Define canonical Schema IR v2](tasks/02_m25_schema_codecs/M25-001-Z-package-evidence-for-define-canonical-schema-ir-v2.md) — deps: M25-001-V — #125
4. [M25-002-A — Compare QuickJS parse/stringify, generic Rust conversion, and generated schema-aware codecs](tasks/02_m25_schema_codecs/M25-002-A-compare-quickjs-parse-stringify-generic-rust-conversion-and-generated-schema-awa.md) — deps: M25-001-Z — #126
5. [M25-002-B — Use 256B, 1KB, 16KB, 64KB, nested objects, arrays 100/1,000, optional/null, and problems](tasks/02_m25_schema_codecs/M25-002-B-use-256b-1kb-16kb-64kb-nested-objects-arrays-100-1-000-optional-null-and-problem.md) — deps: M25-002-A — #127
6. [M25-002-C — Capture CPU, allocation, bridge time, and tails](tasks/02_m25_schema_codecs/M25-002-C-capture-cpu-allocation-bridge-time-and-tails.md) — deps: M25-002-B — #128
7. [M25-002-D — Select strategies by evidence](tasks/02_m25_schema_codecs/M25-002-D-select-strategies-by-evidence.md) — deps: M25-002-C — #129
8. [M25-002-V — Verify Build reproducible decoder/encoder strategy benchmark](tasks/02_m25_schema_codecs/M25-002-V-verify-build-reproducible-decoder-encoder-strategy-benchmark.md) — deps: M25-002-A, M25-002-B, M25-002-C, M25-002-D — #130
9. [M25-002-Z — Package evidence for Build reproducible decoder/encoder strategy benchmark](tasks/02_m25_schema_codecs/M25-002-Z-package-evidence-for-build-reproducible-decoder-encoder-strategy-benchmark.md) — deps: M25-002-V — #131
10. [M25-003-A — Generate direct decoder programs keyed by SchemaId](tasks/02_m25_schema_codecs/M25-003-A-generate-direct-decoder-programs-keyed-by-schemaid.md) — deps: M25-001-Z, M24-GATE — #132
11. [M25-003-B — Validate byte ranges and header/query values without generic object trees](tasks/02_m25_schema_codecs/M25-003-B-validate-byte-ranges-and-header-query-values-without-generic-object-trees.md) — deps: M25-003-A — #133
12. [M25-003-C — Return typed RFC 9457 problems](tasks/02_m25_schema_codecs/M25-003-C-return-typed-rfc-9457-problems.md) — deps: M25-003-B — #134
13. [M25-003-D — Preserve declared coercion semantics exactly](tasks/02_m25_schema_codecs/M25-003-D-preserve-declared-coercion-semantics-exactly.md) — deps: M25-003-C — #135
14. [M25-003-V — Verify Generate params/query/header decoders](tasks/02_m25_schema_codecs/M25-003-V-verify-generate-params-query-header-decoders.md) — deps: M25-003-A, M25-003-B, M25-003-C, M25-003-D — #136
15. [M25-003-Z — Package evidence for Generate params/query/header decoders](tasks/02_m25_schema_codecs/M25-003-Z-package-evidence-for-generate-params-query-header-decoders.md) — deps: M25-003-V — #137
16. [M25-004-A — Implement generated direct decode where supported](tasks/02_m25_schema_codecs/M25-004-A-implement-generated-direct-decode-where-supported.md) — deps: M25-001-Z, M24-007-Z — #138
17. [M25-004-B — Retain QuickJS/generic fallback for unsupported transformations](tasks/02_m25_schema_codecs/M25-004-B-retain-quickjs-generic-fallback-for-unsupported-transformations.md) — deps: M25-004-A — #139
18. [M25-004-C — Enforce depth, size, array, string, and numeric limits](tasks/02_m25_schema_codecs/M25-004-C-enforce-depth-size-array-string-and-numeric-limits.md) — deps: M25-004-B — #140
19. [M25-004-D — Propagate cancellation and request deadlines](tasks/02_m25_schema_codecs/M25-004-D-propagate-cancellation-and-request-deadlines.md) — deps: M25-004-C — #141
20. [M25-004-V — Verify Generate JSON body decoders](tasks/02_m25_schema_codecs/M25-004-V-verify-generate-json-body-decoders.md) — deps: M25-004-A, M25-004-B, M25-004-C, M25-004-D — #142
21. [M25-004-Z — Package evidence for Generate JSON body decoders](tasks/02_m25_schema_codecs/M25-004-Z-package-evidence-for-generate-json-body-decoders.md) — deps: M25-004-V — #143
22. [M25-005-A — Generate per-status encoders](tasks/02_m25_schema_codecs/M25-005-A-generate-per-status-encoders.md) — deps: M25-001-Z, M25-002-Z — #144
23. [M25-005-B — Read declared properties in fixed order](tasks/02_m25_schema_codecs/M25-005-B-read-declared-properties-in-fixed-order.md) — deps: M25-005-A — #145
24. [M25-005-C — Handle optional/null/union fields](tasks/02_m25_schema_codecs/M25-005-C-handle-optional-null-union-fields.md) — deps: M25-005-B — #146
25. [M25-005-D — Keep QuickJS stringify or generic fallback when measured better](tasks/02_m25_schema_codecs/M25-005-D-keep-quickjs-stringify-or-generic-fallback-when-measured-better.md) — deps: M25-005-C — #147
