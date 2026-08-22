# Next 25 Dependency-Safe Tasks

The next dependency-ready implementation task is M25-010-B; this short queue lists the first 25 unchecked packets with their issue numbers.

1. [M25-010-B — Measure generated code/pack size](tasks/02_m25_schema_codecs/M25-010-B-measure-generated-code-pack-size.md) — deps: M25-010-A — #175
2. [M25-010-C — Report cold-start delta at 25/1,000 routes](tasks/02_m25_schema_codecs/M25-010-C-report-cold-start-delta-at-25-1-000-routes.md) — deps: M25-010-B — #176
3. [M25-010-D — Record CPU and RSS](tasks/02_m25_schema_codecs/M25-010-D-record-cpu-and-rss.md) — deps: M25-010-C — #177
4. [M25-010-V — Verify Close codec performance and cold-start evidence](tasks/02_m25_schema_codecs/M25-010-V-verify-close-codec-performance-and-cold-start-evidence.md) — deps: M25-010-A, M25-010-B, M25-010-C, M25-010-D — #178
5. [M25-010-Z — Package evidence for Close codec performance and cold-start evidence](tasks/02_m25_schema_codecs/M25-010-Z-package-evidence-for-close-codec-performance-and-cold-start-evidence.md) — deps: M25-010-V — #179
6. [M25-GATE — M2.5 — Schema-Specialized Input and JSON Output Pipeline exit gate](gates/M25-GATE.md) — deps: M25-001-Z, M25-002-Z, M25-003-Z, M25-004-Z, M25-005-Z, M25-006-Z, M25-007-Z, M25-008-Z, M25-009-Z, M25-010-Z — #628
7. [M26-001-A — Define numeric current mode and legacy v1 adapter](tasks/03_m26_qpack_v2/M26-001-A-define-numeric-current-mode-and-legacy-v1-adapter.md) — deps: M25-GATE — #180
8. [M26-001-B — Specify section directory, alignment, bounds, optional sections, and versioning](tasks/03_m26_qpack_v2/M26-001-B-specify-section-directory-alignment-bounds-optional-sections-and-versioning.md) — deps: M26-001-A — #181
9. [M26-001-C — Separate integrity from authenticity](tasks/03_m26_qpack_v2/M26-001-C-separate-integrity-from-authenticity.md) — deps: M26-001-B — #182
10. [M26-001-D — Define debug/source sidecar policy](tasks/03_m26_qpack_v2/M26-001-D-define-debug-source-sidecar-policy.md) — deps: M26-001-C — #183
11. [M26-001-V — Verify Accept QPack v2 format and compatibility ADR](tasks/03_m26_qpack_v2/M26-001-V-verify-accept-qpack-v2-format-and-compatibility-adr.md) — deps: M26-001-A, M26-001-B, M26-001-C, M26-001-D — #184
12. [M26-001-Z — Package evidence for Accept QPack v2 format and compatibility ADR](tasks/03_m26_qpack_v2/M26-001-Z-package-evidence-for-accept-qpack-v2-format-and-compatibility-adr.md) — deps: M26-001-V — #185
13. [M26-002-A — Include runtime ABI, QuickJS-NG version/build hash, rquickjs version, bytecode format, target triple, pointer width, endianness, and capability hash](tasks/03_m26_qpack_v2/M26-002-A-include-runtime-abi-quickjs-ng-version-build-hash-rquickjs-version-bytecode-form.md) — deps: M26-001-Z — #186
14. [M26-002-B — Fail closed on mismatch](tasks/03_m26_qpack_v2/M26-002-B-fail-closed-on-mismatch.md) — deps: M26-002-A — #187
15. [M26-002-C — Provide explicit source rebuild path](tasks/03_m26_qpack_v2/M26-002-C-provide-explicit-source-rebuild-path.md) — deps: M26-002-B — #188
16. [M26-002-D — Never silently fall back](tasks/03_m26_qpack_v2/M26-002-D-never-silently-fall-back.md) — deps: M26-002-C — #189
17. [M26-002-V — Verify Define strict runtime and bytecode fingerprint](tasks/03_m26_qpack_v2/M26-002-V-verify-define-strict-runtime-and-bytecode-fingerprint.md) — deps: M26-002-A, M26-002-B, M26-002-C, M26-002-D — #190
18. [M26-002-Z — Package evidence for Define strict runtime and bytecode fingerprint](tasks/03_m26_qpack_v2/M26-002-Z-package-evidence-for-define-strict-runtime-and-bytecode-fingerprint.md) — deps: M26-002-V — #191
19. [M26-003-A — Define dense section schemas](tasks/03_m26_qpack_v2/M26-003-A-define-dense-section-schemas.md) — deps: M26-001-Z, G0-GATE, M25-GATE — #192
20. [M26-003-B — Store router nodes/edges/terminals, RoutePlans, schema programs, policy plans, function manifest, debug names, and capability inventory](tasks/03_m26_qpack_v2/M26-003-B-store-router-nodes-edges-terminals-routeplans-schema-programs-policy-plans-funct.md) — deps: M26-003-A — #193
21. [M26-003-C — Use offsets and bounds checks](tasks/03_m26_qpack_v2/M26-003-C-use-offsets-and-bounds-checks.md) — deps: M26-003-B — #194
22. [M26-003-D — Bind sections to execution integrity](tasks/03_m26_qpack_v2/M26-003-D-bind-sections-to-execution-integrity.md) — deps: M26-003-C — #195
23. [M26-003-V — Verify Encode compiled router, RoutePlans, schemas, policies, and functions as sections](tasks/03_m26_qpack_v2/M26-003-V-verify-encode-compiled-router-routeplans-schemas-policies-and-functions-as-secti.md) — deps: M26-003-A, M26-003-B, M26-003-C, M26-003-D — #196
24. [M26-003-Z — Package evidence for Encode compiled router, RoutePlans, schemas, policies, and functions as sections](tasks/03_m26_qpack_v2/M26-003-Z-package-evidence-for-encode-compiled-router-routeplans-schemas-policies-and-func.md) — deps: M26-003-V — #197
25. [M26-004-A — Store raw module bytecode section](tasks/03_m26_qpack_v2/M26-004-A-store-raw-module-bytecode-section.md) — deps: M26-002-Z, M26-003-Z — #198
