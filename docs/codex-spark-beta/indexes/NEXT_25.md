# Next 25 Dependency-Safe Tasks

The next dependency-ready implementation task is M25-009-B; this short queue lists the first 25 unchecked packets with their issue numbers.

1. [M25-009-B — Compare generated output with standards/reference JSON behavior](tasks/02_m25_schema_codecs/M25-009-B-compare-generated-output-with-standards-reference-json-behavior.md) — deps: M25-009-A — #169
2. [M25-009-C — Run malformed and boundary values](tasks/02_m25_schema_codecs/M25-009-C-run-malformed-and-boundary-values.md) — deps: M25-009-B — #170
3. [M25-009-D — Minimize failures into permanent fixtures](tasks/02_m25_schema_codecs/M25-009-D-minimize-failures-into-permanent-fixtures.md) — deps: M25-009-C — #171
4. [M25-009-V — Verify Add codec fuzzing and differential tests](tasks/02_m25_schema_codecs/M25-009-V-verify-add-codec-fuzzing-and-differential-tests.md) — deps: M25-009-A, M25-009-B, M25-009-C, M25-009-D — #172
5. [M25-009-Z — Package evidence for Add codec fuzzing and differential tests](tasks/02_m25_schema_codecs/M25-009-Z-package-evidence-for-add-codec-fuzzing-and-differential-tests.md) — deps: M25-009-V — #173
6. [M25-010-A — Run C2 plus medium/large JSON workloads](tasks/02_m25_schema_codecs/M25-010-A-run-c2-plus-medium-large-json-workloads.md) — deps: M25-002-Z, M25-009-Z — #174
7. [M25-010-B — Measure generated code/pack size](tasks/02_m25_schema_codecs/M25-010-B-measure-generated-code-pack-size.md) — deps: M25-010-A — #175
8. [M25-010-C — Report cold-start delta at 25/1,000 routes](tasks/02_m25_schema_codecs/M25-010-C-report-cold-start-delta-at-25-1-000-routes.md) — deps: M25-010-B — #176
9. [M25-010-D — Record CPU and RSS](tasks/02_m25_schema_codecs/M25-010-D-record-cpu-and-rss.md) — deps: M25-010-C — #177
10. [M25-010-V — Verify Close codec performance and cold-start evidence](tasks/02_m25_schema_codecs/M25-010-V-verify-close-codec-performance-and-cold-start-evidence.md) — deps: M25-010-A, M25-010-B, M25-010-C, M25-010-D — #178
11. [M25-010-Z — Package evidence for Close codec performance and cold-start evidence](tasks/02_m25_schema_codecs/M25-010-Z-package-evidence-for-close-codec-performance-and-cold-start-evidence.md) — deps: M25-010-V — #179
12. [M25-GATE — M2.5 — Schema-Specialized Input and JSON Output Pipeline exit gate](gates/M25-GATE.md) — deps: M25-001-Z, M25-002-Z, M25-003-Z, M25-004-Z, M25-005-Z, M25-006-Z, M25-007-Z, M25-008-Z, M25-009-Z, M25-010-Z — #628
13. [M26-001-A — Define numeric current mode and legacy v1 adapter](tasks/03_m26_qpack_v2/M26-001-A-define-numeric-current-mode-and-legacy-v1-adapter.md) — deps: M25-GATE — #180
14. [M26-001-B — Specify section directory, alignment, bounds, optional sections, and versioning](tasks/03_m26_qpack_v2/M26-001-B-specify-section-directory-alignment-bounds-optional-sections-and-versioning.md) — deps: M26-001-A — #181
15. [M26-001-C — Separate integrity from authenticity](tasks/03_m26_qpack_v2/M26-001-C-separate-integrity-from-authenticity.md) — deps: M26-001-B — #182
16. [M26-001-D — Define debug/source sidecar policy](tasks/03_m26_qpack_v2/M26-001-D-define-debug-source-sidecar-policy.md) — deps: M26-001-C — #183
17. [M26-001-V — Verify Accept QPack v2 format and compatibility ADR](tasks/03_m26_qpack_v2/M26-001-V-verify-accept-qpack-v2-format-and-compatibility-adr.md) — deps: M26-001-A, M26-001-B, M26-001-C, M26-001-D — #184
18. [M26-001-Z — Package evidence for Accept QPack v2 format and compatibility ADR](tasks/03_m26_qpack_v2/M26-001-Z-package-evidence-for-accept-qpack-v2-format-and-compatibility-adr.md) — deps: M26-001-V — #185
19. [M26-002-A — Include runtime ABI, QuickJS-NG version/build hash, rquickjs version, bytecode format, target triple, pointer width, endianness, and capability hash](tasks/03_m26_qpack_v2/M26-002-A-include-runtime-abi-quickjs-ng-version-build-hash-rquickjs-version-bytecode-form.md) — deps: M26-001-Z — #186
20. [M26-002-B — Fail closed on mismatch](tasks/03_m26_qpack_v2/M26-002-B-fail-closed-on-mismatch.md) — deps: M26-002-A — #187
21. [M26-002-C — Provide explicit source rebuild path](tasks/03_m26_qpack_v2/M26-002-C-provide-explicit-source-rebuild-path.md) — deps: M26-002-B — #188
22. [M26-002-D — Never silently fall back](tasks/03_m26_qpack_v2/M26-002-D-never-silently-fall-back.md) — deps: M26-002-C — #189
23. [M26-002-V — Verify Define strict runtime and bytecode fingerprint](tasks/03_m26_qpack_v2/M26-002-V-verify-define-strict-runtime-and-bytecode-fingerprint.md) — deps: M26-002-A, M26-002-B, M26-002-C, M26-002-D — #190
24. [M26-002-Z — Package evidence for Define strict runtime and bytecode fingerprint](tasks/03_m26_qpack_v2/M26-002-Z-package-evidence-for-define-strict-runtime-and-bytecode-fingerprint.md) — deps: M26-002-V — #191
25. [M26-003-A — Define dense section schemas](tasks/03_m26_qpack_v2/M26-003-A-define-dense-section-schemas.md) — deps: M26-001-Z, G0-GATE, M25-GATE — #192
