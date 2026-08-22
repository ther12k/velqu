# Next 25 Dependency-Safe Tasks

The next dependency-ready implementation task is M25-007-C; this short queue lists the first 25 unchecked packets with their issue numbers.

1. [M25-007-C — Keep fallback bounded and deadline-aware](tasks/02_m25_schema_codecs/M25-007-C-keep-fallback-bounded-and-deadline-aware.md) — deps: M25-007-B — #158
2. [M25-007-D — Expose bridge crossings and codec choice in `velqu inspect`](tasks/02_m25_schema_codecs/M25-007-D-expose-bridge-crossings-and-codec-choice-in-velqu-inspect.md) — deps: M25-007-C — #159
3. [M25-007-V — Verify Implement explicit generic and Web fallback paths](tasks/02_m25_schema_codecs/M25-007-V-verify-implement-explicit-generic-and-web-fallback-paths.md) — deps: M25-007-A, M25-007-B, M25-007-C, M25-007-D — #160
4. [M25-007-Z — Package evidence for Implement explicit generic and Web fallback paths](tasks/02_m25_schema_codecs/M25-007-Z-package-evidence-for-implement-explicit-generic-and-web-fallback-paths.md) — deps: M25-007-V — #161
5. [M25-008-A — Generate all projections from canonical IR](tasks/02_m25_schema_codecs/M25-008-A-generate-all-projections-from-canonical-ir.md) — deps: M25-001-Z, M25-003-Z, M25-004-Z, M25-005-Z, M25-006-Z — #162
6. [M25-008-B — Add parity checks to verification](tasks/02_m25_schema_codecs/M25-008-B-add-parity-checks-to-verification.md) — deps: M25-008-A — #163
7. [M25-008-C — Publish compact contract metadata](tasks/02_m25_schema_codecs/M25-008-C-publish-compact-contract-metadata.md) — deps: M25-008-B — #164
8. [M25-008-D — Update semantic diff to Schema IR v2](tasks/02_m25_schema_codecs/M25-008-D-update-semantic-diff-to-schema-ir-v2.md) — deps: M25-008-C — #165
9. [M25-008-V — Verify Unify Treaty, OpenAPI, lock, and runtime schema projection](tasks/02_m25_schema_codecs/M25-008-V-verify-unify-treaty-openapi-lock-and-runtime-schema-projection.md) — deps: M25-008-A, M25-008-B, M25-008-C, M25-008-D — #166
10. [M25-008-Z — Package evidence for Unify Treaty, OpenAPI, lock, and runtime schema projection](tasks/02_m25_schema_codecs/M25-008-Z-package-evidence-for-unify-treaty-openapi-lock-and-runtime-schema-projection.md) — deps: M25-008-V — #167
11. [M25-009-A — Fuzz encoded/decoded values](tasks/02_m25_schema_codecs/M25-009-A-fuzz-encoded-decoded-values.md) — deps: M25-003-Z, M25-004-Z, M25-005-Z, M25-006-Z — #168
12. [M25-009-B — Compare generated output with standards/reference JSON behavior](tasks/02_m25_schema_codecs/M25-009-B-compare-generated-output-with-standards-reference-json-behavior.md) — deps: M25-009-A — #169
13. [M25-009-C — Run malformed and boundary values](tasks/02_m25_schema_codecs/M25-009-C-run-malformed-and-boundary-values.md) — deps: M25-009-B — #170
14. [M25-009-D — Minimize failures into permanent fixtures](tasks/02_m25_schema_codecs/M25-009-D-minimize-failures-into-permanent-fixtures.md) — deps: M25-009-C — #171
15. [M25-009-V — Verify Add codec fuzzing and differential tests](tasks/02_m25_schema_codecs/M25-009-V-verify-add-codec-fuzzing-and-differential-tests.md) — deps: M25-009-A, M25-009-B, M25-009-C, M25-009-D — #172
16. [M25-009-Z — Package evidence for Add codec fuzzing and differential tests](tasks/02_m25_schema_codecs/M25-009-Z-package-evidence-for-add-codec-fuzzing-and-differential-tests.md) — deps: M25-009-V — #173
17. [M25-010-A — Run C2 plus medium/large JSON workloads](tasks/02_m25_schema_codecs/M25-010-A-run-c2-plus-medium-large-json-workloads.md) — deps: M25-002-Z, M25-009-Z — #174
18. [M25-010-B — Measure generated code/pack size](tasks/02_m25_schema_codecs/M25-010-B-measure-generated-code-pack-size.md) — deps: M25-010-A — #175
19. [M25-010-C — Report cold-start delta at 25/1,000 routes](tasks/02_m25_schema_codecs/M25-010-C-report-cold-start-delta-at-25-1-000-routes.md) — deps: M25-010-B — #176
20. [M25-010-D — Record CPU and RSS](tasks/02_m25_schema_codecs/M25-010-D-record-cpu-and-rss.md) — deps: M25-010-C — #177
21. [M25-010-V — Verify Close codec performance and cold-start evidence](tasks/02_m25_schema_codecs/M25-010-V-verify-close-codec-performance-and-cold-start-evidence.md) — deps: M25-010-A, M25-010-B, M25-010-C, M25-010-D — #178
22. [M25-010-Z — Package evidence for Close codec performance and cold-start evidence](tasks/02_m25_schema_codecs/M25-010-Z-package-evidence-for-close-codec-performance-and-cold-start-evidence.md) — deps: M25-010-V — #179
23. [M25-GATE — M2.5 — Schema-Specialized Input and JSON Output Pipeline exit gate](gates/M25-GATE.md) — deps: M25-001-Z, M25-002-Z, M25-003-Z, M25-004-Z, M25-005-Z, M25-006-Z, M25-007-Z, M25-008-Z, M25-009-Z, M25-010-Z — #628
24. [M26-001-A — Define numeric current mode and legacy v1 adapter](tasks/03_m26_qpack_v2/M26-001-A-define-numeric-current-mode-and-legacy-v1-adapter.md) — deps: M25-GATE — #180
25. [M26-001-B — Specify section directory, alignment, bounds, optional sections, and versioning](tasks/03_m26_qpack_v2/M26-001-B-specify-section-directory-alignment-bounds-optional-sections-and-versioning.md) — deps: M26-001-A — #181
