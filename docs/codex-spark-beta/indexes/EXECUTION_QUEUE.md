# Dependency-Safe Execution Queue

All PASS packets are omitted. The first unchecked dependency-ready task is M25-004-A; future milestone tasks remain TODO until implemented and evidenced.

1. [M25-004-A — Implement generated direct decode where supported](tasks/02_m25_schema_codecs/M25-004-A-implement-generated-direct-decode-where-supported.md) — deps: M25-001-Z, M24-007-Z — #138
2. [M25-004-B — Retain QuickJS/generic fallback for unsupported transformations](tasks/02_m25_schema_codecs/M25-004-B-retain-quickjs-generic-fallback-for-unsupported-transformations.md) — deps: M25-004-A — #139
3. [M25-004-C — Enforce depth, size, array, string, and numeric limits](tasks/02_m25_schema_codecs/M25-004-C-enforce-depth-size-array-string-and-numeric-limits.md) — deps: M25-004-B — #140
4. [M25-004-D — Propagate cancellation and request deadlines](tasks/02_m25_schema_codecs/M25-004-D-propagate-cancellation-and-request-deadlines.md) — deps: M25-004-C — #141
5. [M25-004-V — Verify Generate JSON body decoders](tasks/02_m25_schema_codecs/M25-004-V-verify-generate-json-body-decoders.md) — deps: M25-004-A, M25-004-B, M25-004-C, M25-004-D — #142
6. [M25-004-Z — Package evidence for Generate JSON body decoders](tasks/02_m25_schema_codecs/M25-004-Z-package-evidence-for-generate-json-body-decoders.md) — deps: M25-004-V — #143
7. [M25-005-A — Generate per-status encoders](tasks/02_m25_schema_codecs/M25-005-A-generate-per-status-encoders.md) — deps: M25-001-Z, M25-002-Z — #144
8. [M25-005-B — Read declared properties in fixed order](tasks/02_m25_schema_codecs/M25-005-B-read-declared-properties-in-fixed-order.md) — deps: M25-005-A — #145
9. [M25-005-C — Handle optional/null/union fields](tasks/02_m25_schema_codecs/M25-005-C-handle-optional-null-union-fields.md) — deps: M25-005-B — #146
10. [M25-005-D — Keep QuickJS stringify or generic fallback when measured better](tasks/02_m25_schema_codecs/M25-005-D-keep-quickjs-stringify-or-generic-fallback-when-measured-better.md) — deps: M25-005-C — #147
11. [M25-005-V — Verify Generate status-specific response encoders](tasks/02_m25_schema_codecs/M25-005-V-verify-generate-status-specific-response-encoders.md) — deps: M25-005-A, M25-005-B, M25-005-C, M25-005-D — #148
12. [M25-005-Z — Package evidence for Generate status-specific response encoders](tasks/02_m25_schema_codecs/M25-005-Z-package-evidence-for-generate-status-specific-response-encoders.md) — deps: M25-005-V — #149
13. [M25-006-A — Generate problem type/status/title/detail/custom-field encoders](tasks/02_m25_schema_codecs/M25-006-A-generate-problem-type-status-title-detail-custom-field-encoders.md) — deps: M25-001-Z, M25-005-Z — #150
14. [M25-006-B — Redact unexpected failures](tasks/02_m25_schema_codecs/M25-006-B-redact-unexpected-failures.md) — deps: M25-006-A — #151
15. [M25-006-C — Ensure policy-provided errors flow into Treaty unions](tasks/02_m25_schema_codecs/M25-006-C-ensure-policy-provided-errors-flow-into-treaty-unions.md) — deps: M25-006-B — #152
16. [M25-006-D — Include content type and instance behavior](tasks/02_m25_schema_codecs/M25-006-D-include-content-type-and-instance-behavior.md) — deps: M25-006-C — #153
17. [M25-006-V — Verify Generate RFC 9457 problem encoders](tasks/02_m25_schema_codecs/M25-006-V-verify-generate-rfc-9457-problem-encoders.md) — deps: M25-006-A, M25-006-B, M25-006-C, M25-006-D — #154
18. [M25-006-Z — Package evidence for Generate RFC 9457 problem encoders](tasks/02_m25_schema_codecs/M25-006-Z-package-evidence-for-generate-rfc-9457-problem-encoders.md) — deps: M25-006-V — #155
19. [M25-007-A — Tag fallback reason in RoutePlan](tasks/02_m25_schema_codecs/M25-007-A-tag-fallback-reason-in-routeplan.md) — deps: M25-003-Z, M25-004-Z, M25-005-Z — #156
20. [M25-007-B — Support raw Response/full Request escape hatches](tasks/02_m25_schema_codecs/M25-007-B-support-raw-response-full-request-escape-hatches.md) — deps: M25-007-A — #157
21. [M25-007-C — Keep fallback bounded and deadline-aware](tasks/02_m25_schema_codecs/M25-007-C-keep-fallback-bounded-and-deadline-aware.md) — deps: M25-007-B — #158
22. [M25-007-D — Expose bridge crossings and codec choice in `velqu inspect`](tasks/02_m25_schema_codecs/M25-007-D-expose-bridge-crossings-and-codec-choice-in-velqu-inspect.md) — deps: M25-007-C — #159
23. [M25-007-V — Verify Implement explicit generic and Web fallback paths](tasks/02_m25_schema_codecs/M25-007-V-verify-implement-explicit-generic-and-web-fallback-paths.md) — deps: M25-007-A, M25-007-B, M25-007-C, M25-007-D — #160
24. [M25-007-Z — Package evidence for Implement explicit generic and Web fallback paths](tasks/02_m25_schema_codecs/M25-007-Z-package-evidence-for-implement-explicit-generic-and-web-fallback-paths.md) — deps: M25-007-V — #161
25. [M25-008-A — Generate all projections from canonical IR](tasks/02_m25_schema_codecs/M25-008-A-generate-all-projections-from-canonical-ir.md) — deps: M25-001-Z, M25-003-Z, M25-004-Z, M25-005-Z, M25-006-Z — #162
26. [M25-008-B — Add parity checks to verification](tasks/02_m25_schema_codecs/M25-008-B-add-parity-checks-to-verification.md) — deps: M25-008-A — #163
27. [M25-008-C — Publish compact contract metadata](tasks/02_m25_schema_codecs/M25-008-C-publish-compact-contract-metadata.md) — deps: M25-008-B — #164
28. [M25-008-D — Update semantic diff to Schema IR v2](tasks/02_m25_schema_codecs/M25-008-D-update-semantic-diff-to-schema-ir-v2.md) — deps: M25-008-C — #165
29. [M25-008-V — Verify Unify Treaty, OpenAPI, lock, and runtime schema projection](tasks/02_m25_schema_codecs/M25-008-V-verify-unify-treaty-openapi-lock-and-runtime-schema-projection.md) — deps: M25-008-A, M25-008-B, M25-008-C, M25-008-D — #166
30. [M25-008-Z — Package evidence for Unify Treaty, OpenAPI, lock, and runtime schema projection](tasks/02_m25_schema_codecs/M25-008-Z-package-evidence-for-unify-treaty-openapi-lock-and-runtime-schema-projection.md) — deps: M25-008-V — #167
31. [M25-009-A — Fuzz encoded/decoded values](tasks/02_m25_schema_codecs/M25-009-A-fuzz-encoded-decoded-values.md) — deps: M25-003-Z, M25-004-Z, M25-005-Z, M25-006-Z — #168
32. [M25-009-B — Compare generated output with standards/reference JSON behavior](tasks/02_m25_schema_codecs/M25-009-B-compare-generated-output-with-standards-reference-json-behavior.md) — deps: M25-009-A — #169
33. [M25-009-C — Run malformed and boundary values](tasks/02_m25_schema_codecs/M25-009-C-run-malformed-and-boundary-values.md) — deps: M25-009-B — #170
34. [M25-009-D — Minimize failures into permanent fixtures](tasks/02_m25_schema_codecs/M25-009-D-minimize-failures-into-permanent-fixtures.md) — deps: M25-009-C — #171
35. [M25-009-V — Verify Add codec fuzzing and differential tests](tasks/02_m25_schema_codecs/M25-009-V-verify-add-codec-fuzzing-and-differential-tests.md) — deps: M25-009-A, M25-009-B, M25-009-C, M25-009-D — #172
36. [M25-009-Z — Package evidence for Add codec fuzzing and differential tests](tasks/02_m25_schema_codecs/M25-009-Z-package-evidence-for-add-codec-fuzzing-and-differential-tests.md) — deps: M25-009-V — #173
37. [M25-010-A — Run C2 plus medium/large JSON workloads](tasks/02_m25_schema_codecs/M25-010-A-run-c2-plus-medium-large-json-workloads.md) — deps: M25-002-Z, M25-009-Z — #174
38. [M25-010-B — Measure generated code/pack size](tasks/02_m25_schema_codecs/M25-010-B-measure-generated-code-pack-size.md) — deps: M25-010-A — #175
39. [M25-010-C — Report cold-start delta at 25/1,000 routes](tasks/02_m25_schema_codecs/M25-010-C-report-cold-start-delta-at-25-1-000-routes.md) — deps: M25-010-B — #176
40. [M25-010-D — Record CPU and RSS](tasks/02_m25_schema_codecs/M25-010-D-record-cpu-and-rss.md) — deps: M25-010-C — #177
41. [M25-010-V — Verify Close codec performance and cold-start evidence](tasks/02_m25_schema_codecs/M25-010-V-verify-close-codec-performance-and-cold-start-evidence.md) — deps: M25-010-A, M25-010-B, M25-010-C, M25-010-D — #178
42. [M25-010-Z — Package evidence for Close codec performance and cold-start evidence](tasks/02_m25_schema_codecs/M25-010-Z-package-evidence-for-close-codec-performance-and-cold-start-evidence.md) — deps: M25-010-V — #179
43. [M25-GATE — M2.5 — Schema-Specialized Input and JSON Output Pipeline exit gate](gates/M25-GATE.md) — deps: M25-001-Z, M25-002-Z, M25-003-Z, M25-004-Z, M25-005-Z, M25-006-Z, M25-007-Z, M25-008-Z, M25-009-Z, M25-010-Z — #628
44. [M26-001-A — Define numeric current mode and legacy v1 adapter](tasks/03_m26_qpack_v2/M26-001-A-define-numeric-current-mode-and-legacy-v1-adapter.md) — deps: M25-GATE — #180
45. [M26-001-B — Specify section directory, alignment, bounds, optional sections, and versioning](tasks/03_m26_qpack_v2/M26-001-B-specify-section-directory-alignment-bounds-optional-sections-and-versioning.md) — deps: M26-001-A — #181
46. [M26-001-C — Separate integrity from authenticity](tasks/03_m26_qpack_v2/M26-001-C-separate-integrity-from-authenticity.md) — deps: M26-001-B — #182
47. [M26-001-D — Define debug/source sidecar policy](tasks/03_m26_qpack_v2/M26-001-D-define-debug-source-sidecar-policy.md) — deps: M26-001-C — #183
48. [M26-001-V — Verify Accept QPack v2 format and compatibility ADR](tasks/03_m26_qpack_v2/M26-001-V-verify-accept-qpack-v2-format-and-compatibility-adr.md) — deps: M26-001-A, M26-001-B, M26-001-C, M26-001-D — #184
49. [M26-001-Z — Package evidence for Accept QPack v2 format and compatibility ADR](tasks/03_m26_qpack_v2/M26-001-Z-package-evidence-for-accept-qpack-v2-format-and-compatibility-adr.md) — deps: M26-001-V — #185
50. [M26-002-A — Include runtime ABI, QuickJS-NG version/build hash, rquickjs version, bytecode format, target triple, pointer width, endianness, and capability hash](tasks/03_m26_qpack_v2/M26-002-A-include-runtime-abi-quickjs-ng-version-build-hash-rquickjs-version-bytecode-form.md) — deps: M26-001-Z — #186
51. [M26-002-B — Fail closed on mismatch](tasks/03_m26_qpack_v2/M26-002-B-fail-closed-on-mismatch.md) — deps: M26-002-A — #187
52. [M26-002-C — Provide explicit source rebuild path](tasks/03_m26_qpack_v2/M26-002-C-provide-explicit-source-rebuild-path.md) — deps: M26-002-B — #188
53. [M26-002-D — Never silently fall back](tasks/03_m26_qpack_v2/M26-002-D-never-silently-fall-back.md) — deps: M26-002-C — #189
54. [M26-002-V — Verify Define strict runtime and bytecode fingerprint](tasks/03_m26_qpack_v2/M26-002-V-verify-define-strict-runtime-and-bytecode-fingerprint.md) — deps: M26-002-A, M26-002-B, M26-002-C, M26-002-D — #190
55. [M26-002-Z — Package evidence for Define strict runtime and bytecode fingerprint](tasks/03_m26_qpack_v2/M26-002-Z-package-evidence-for-define-strict-runtime-and-bytecode-fingerprint.md) — deps: M26-002-V — #191
56. [M26-003-A — Define dense section schemas](tasks/03_m26_qpack_v2/M26-003-A-define-dense-section-schemas.md) — deps: M26-001-Z, G0-GATE, M25-GATE — #192
57. [M26-003-B — Store router nodes/edges/terminals, RoutePlans, schema programs, policy plans, function manifest, debug names, and capability inventory](tasks/03_m26_qpack_v2/M26-003-B-store-router-nodes-edges-terminals-routeplans-schema-programs-policy-plans-funct.md) — deps: M26-003-A — #193
58. [M26-003-C — Use offsets and bounds checks](tasks/03_m26_qpack_v2/M26-003-C-use-offsets-and-bounds-checks.md) — deps: M26-003-B — #194
59. [M26-003-D — Bind sections to execution integrity](tasks/03_m26_qpack_v2/M26-003-D-bind-sections-to-execution-integrity.md) — deps: M26-003-C — #195
60. [M26-003-V — Verify Encode compiled router, RoutePlans, schemas, policies, and functions as sections](tasks/03_m26_qpack_v2/M26-003-V-verify-encode-compiled-router-routeplans-schemas-policies-and-functions-as-secti.md) — deps: M26-003-A, M26-003-B, M26-003-C, M26-003-D — #196
61. [M26-003-Z — Package evidence for Encode compiled router, RoutePlans, schemas, policies, and functions as sections](tasks/03_m26_qpack_v2/M26-003-Z-package-evidence-for-encode-compiled-router-routeplans-schemas-policies-and-func.md) — deps: M26-003-V — #197
62. [M26-004-A — Store raw module bytecode section](tasks/03_m26_qpack_v2/M26-004-A-store-raw-module-bytecode-section.md) — deps: M26-002-Z, M26-003-Z — #198
63. [M26-004-B — Load exactly once](tasks/03_m26_qpack_v2/M26-004-B-load-exactly-once.md) — deps: M26-004-A — #199
64. [M26-004-C — Make source optional sidecar/development section](tasks/03_m26_qpack_v2/M26-004-C-make-source-optional-sidecar-development-section.md) — deps: M26-004-B — #200
65. [M26-004-D — Include prelude and handler manifest in the compiled module](tasks/03_m26_qpack_v2/M26-004-D-include-prelude-and-handler-manifest-in-the-compiled-module.md) — deps: M26-004-C — #201
66. [M26-004-V — Verify Embed raw QuickJS bytecode without base64](tasks/03_m26_qpack_v2/M26-004-V-verify-embed-raw-quickjs-bytecode-without-base64.md) — deps: M26-004-A, M26-004-B, M26-004-C, M26-004-D — #202
67. [M26-004-Z — Package evidence for Embed raw QuickJS bytecode without base64](tasks/03_m26_qpack_v2/M26-004-Z-package-evidence-for-embed-raw-quickjs-bytecode-without-base64.md) — deps: M26-004-V — #203
68. [M26-005-A — Use mmap/read-only bytes where supported](tasks/03_m26_qpack_v2/M26-005-A-use-mmap-read-only-bytes-where-supported.md) — deps: M26-003-Z — #204
69. [M26-005-B — Validate all section bounds before access](tasks/03_m26_qpack_v2/M26-005-B-validate-all-section-bounds-before-access.md) — deps: M26-005-A — #205
70. [M26-005-C — Avoid unsafe unchecked access unless independently audited](tasks/03_m26_qpack_v2/M26-005-C-avoid-unsafe-unchecked-access-unless-independently-audited.md) — deps: M26-005-B — #206
71. [M26-005-D — Support embedded pack bytes in standalone binary](tasks/03_m26_qpack_v2/M26-005-D-support-embedded-pack-bytes-in-standalone-binary.md) — deps: M26-005-C — #207
72. [M26-005-V — Verify Implement zero-copy or bounded-copy pack reader](tasks/03_m26_qpack_v2/M26-005-V-verify-implement-zero-copy-or-bounded-copy-pack-reader.md) — deps: M26-005-A, M26-005-B, M26-005-C, M26-005-D — #208
73. [M26-005-Z — Package evidence for Implement zero-copy or bounded-copy pack reader](tasks/03_m26_qpack_v2/M26-005-Z-package-evidence-for-implement-zero-copy-or-bounded-copy-pack-reader.md) — deps: M26-005-V — #209
74. [M26-006-A — Hash required execution sections](tasks/03_m26_qpack_v2/M26-006-A-hash-required-execution-sections.md) — deps: M26-003-Z, M26-004-Z — #210
75. [M26-006-B — Provide Ed25519-compatible signature slot/hook](tasks/03_m26_qpack_v2/M26-006-B-provide-ed25519-compatible-signature-slot-hook.md) — deps: M26-006-A — #211
76. [M26-006-C — Define key discovery/configuration](tasks/03_m26_qpack_v2/M26-006-C-define-key-discovery-configuration.md) — deps: M26-006-B — #212
77. [M26-006-D — Keep unsigned local development supported with explicit policy](tasks/03_m26_qpack_v2/M26-006-D-keep-unsigned-local-development-supported-with-explicit-policy.md) — deps: M26-006-C — #213
78. [M26-006-V — Verify Implement execution integrity and authenticity hooks](tasks/03_m26_qpack_v2/M26-006-V-verify-implement-execution-integrity-and-authenticity-hooks.md) — deps: M26-006-A, M26-006-B, M26-006-C, M26-006-D — #214
79. [M26-006-Z — Package evidence for Implement execution integrity and authenticity hooks](tasks/03_m26_qpack_v2/M26-006-Z-package-evidence-for-implement-execution-integrity-and-authenticity-hooks.md) — deps: M26-006-V — #215
80. [M26-007-A — Remove timestamps/non-deterministic map order](tasks/03_m26_qpack_v2/M26-007-A-remove-timestamps-non-deterministic-map-order.md) — deps: M26-003-Z, M26-004-Z — #216
81. [M26-007-B — Pin compiler/runtime versions](tasks/03_m26_qpack_v2/M26-007-B-pin-compiler-runtime-versions.md) — deps: M26-007-A — #217
82. [M26-007-C — Canonicalize section ordering and padding](tasks/03_m26_qpack_v2/M26-007-C-canonicalize-section-ordering-and-padding.md) — deps: M26-007-B — #218
83. [M26-007-D — Compare independent build outputs](tasks/03_m26_qpack_v2/M26-007-D-compare-independent-build-outputs.md) — deps: M26-007-C — #219
84. [M26-007-V — Verify Guarantee reproducible release packs](tasks/03_m26_qpack_v2/M26-007-V-verify-guarantee-reproducible-release-packs.md) — deps: M26-007-A, M26-007-B, M26-007-C, M26-007-D — #220
85. [M26-007-Z — Package evidence for Guarantee reproducible release packs](tasks/03_m26_qpack_v2/M26-007-Z-package-evidence-for-guarantee-reproducible-release-packs.md) — deps: M26-007-V — #221
86. [M26-008-A — Implement separate v1 reader/adapter](tasks/03_m26_qpack_v2/M26-008-A-implement-separate-v1-reader-adapter.md) — deps: M26-001-Z, M26-005-Z — #222
87. [M26-008-B — Provide `velqu pack migrate` or rebuild guidance](tasks/03_m26_qpack_v2/M26-008-B-provide-velqu-pack-migrate-or-rebuild-guidance.md) — deps: M26-008-A — #223
88. [M26-008-C — Deprecate mixed-mode packs](tasks/03_m26_qpack_v2/M26-008-C-deprecate-mixed-mode-packs.md) — deps: M26-008-B — #224
89. [M26-008-D — Test deterministic failures for unsupported legacy features](tasks/03_m26_qpack_v2/M26-008-D-test-deterministic-failures-for-unsupported-legacy-features.md) — deps: M26-008-C — #225
90. [M26-008-V — Verify Provide explicit v1 compatibility and migration tool](tasks/03_m26_qpack_v2/M26-008-V-verify-provide-explicit-v1-compatibility-and-migration-tool.md) — deps: M26-008-A, M26-008-B, M26-008-C, M26-008-D — #226
91. [M26-008-Z — Package evidence for Provide explicit v1 compatibility and migration tool](tasks/03_m26_qpack_v2/M26-008-Z-package-evidence-for-provide-explicit-v1-compatibility-and-migration-tool.md) — deps: M26-008-V — #227
92. [M26-009-A — Shared mode: `velqu-runtime` plus app.qpack](tasks/03_m26_qpack_v2/M26-009-A-shared-mode-velqu-runtime-plus-app-qpack.md) — deps: M26-004-Z, M26-005-Z — #228
93. [M26-009-B — Standalone mode: embedded qpack executable](tasks/03_m26_qpack_v2/M26-009-B-standalone-mode-embedded-qpack-executable.md) — deps: M26-009-A — #229
94. [M26-009-C — Ensure exact runtime fingerprint](tasks/03_m26_qpack_v2/M26-009-C-ensure-exact-runtime-fingerprint.md) — deps: M26-009-B — #230
95. [M26-009-D — Define source-map/debug sidecars](tasks/03_m26_qpack_v2/M26-009-D-define-source-map-debug-sidecars.md) — deps: M26-009-C — #231
96. [M26-009-V — Verify Build shared-runtime and standalone deployment artifacts](tasks/03_m26_qpack_v2/M26-009-V-verify-build-shared-runtime-and-standalone-deployment-artifacts.md) — deps: M26-009-A, M26-009-B, M26-009-C, M26-009-D — #232
97. [M26-009-Z — Package evidence for Build shared-runtime and standalone deployment artifacts](tasks/03_m26_qpack_v2/M26-009-Z-package-evidence-for-build-shared-runtime-and-standalone-deployment-artifacts.md) — deps: M26-009-V — #233
98. [M26-010-A — Measure 25/100/1,000/5,000/10,000 routes](tasks/03_m26_qpack_v2/M26-010-A-measure-25-100-1-000-5-000-10-000-routes.md) — deps: M26-004-Z, M26-005-Z, M26-009-Z — #234
99. [M26-010-B — At least 100 fresh processes for release evidence](tasks/03_m26_qpack_v2/M26-010-B-at-least-100-fresh-processes-for-release-evidence.md) — deps: M26-010-A — #235
100. [M26-010-C — Randomize source/bytecode/competitor order](tasks/03_m26_qpack_v2/M26-010-C-randomize-source-bytecode-competitor-order.md) — deps: M26-010-B — #236
101. [M26-010-D — Record p50/p95/p99, RSS, stage timings, and hashes](tasks/03_m26_qpack_v2/M26-010-D-record-p50-p95-p99-rss-stage-timings-and-hashes.md) — deps: M26-010-C — #237
102. [M26-010-V — Verify Close route-count cold-start evidence](tasks/03_m26_qpack_v2/M26-010-V-verify-close-route-count-cold-start-evidence.md) — deps: M26-010-A, M26-010-B, M26-010-C, M26-010-D — #238
103. [M26-010-Z — Package evidence for Close route-count cold-start evidence](tasks/03_m26_qpack_v2/M26-010-Z-package-evidence-for-close-route-count-cold-start-evidence.md) — deps: M26-010-V — #239
104. [M26-GATE — M2.6 — Binary QPack v2 and Reproducible Artifact ABI exit gate](gates/M26-GATE.md) — deps: M26-001-Z, M26-002-Z, M26-003-Z, M26-004-Z, M26-005-Z, M26-006-Z, M26-007-Z, M26-008-Z, M26-009-Z, M26-010-Z — #629
105. [M27-001-A — Accept ADR](tasks/04_m27_capability_linker/M27-001-A-accept-adr.md) — deps: M26-GATE — #240
106. [M27-001-B — Define CapabilityId/version/dependencies](tasks/04_m27_capability_linker/M27-001-B-define-capabilityid-version-dependencies.md) — deps: M27-001-A — #241
107. [M27-001-C — Define native operation owner/deadline state](tasks/04_m27_capability_linker/M27-001-C-define-native-operation-owner-deadline-state.md) — deps: M27-001-B — #242
108. [M27-001-D — Define lifecycle phases and bounded shutdown](tasks/04_m27_capability_linker/M27-001-D-define-lifecycle-phases-and-bounded-shutdown.md) — deps: M27-001-C — #243
109. [M27-001-V — Verify Define capability ABI and lifecycle state machine](tasks/04_m27_capability_linker/M27-001-V-verify-define-capability-abi-and-lifecycle-state-machine.md) — deps: M27-001-A, M27-001-B, M27-001-C, M27-001-D — #244
110. [M27-001-Z — Package evidence for Define capability ABI and lifecycle state machine](tasks/04_m27_capability_linker/M27-001-Z-package-evidence-for-define-capability-abi-and-lifecycle-state-machine.md) — deps: M27-001-V — #245
111. [M27-002-A — Build dependency DAG](tasks/04_m27_capability_linker/M27-002-A-build-dependency-dag.md) — deps: M27-001-Z — #246
112. [M27-002-B — Reject cycles/missing/conflicting versions](tasks/04_m27_capability_linker/M27-002-B-reject-cycles-missing-conflicting-versions.md) — deps: M27-002-A — #247
113. [M27-002-C — Emit capability inventory/hash into QPack](tasks/04_m27_capability_linker/M27-002-C-emit-capability-inventory-hash-into-qpack.md) — deps: M27-002-B — #248
114. [M27-002-D — Remove unused modules](tasks/04_m27_capability_linker/M27-002-D-remove-unused-modules.md) — deps: M27-002-C — #249
115. [M27-002-V — Verify Implement compile-time capability dependency resolver](tasks/04_m27_capability_linker/M27-002-V-verify-implement-compile-time-capability-dependency-resolver.md) — deps: M27-002-A, M27-002-B, M27-002-C, M27-002-D — #250
116. [M27-002-Z — Package evidence for Implement compile-time capability dependency resolver](tasks/04_m27_capability_linker/M27-002-Z-package-evidence-for-implement-compile-time-capability-dependency-resolver.md) — deps: M27-002-V — #251
117. [M27-003-A — Build configurable intrinsic profiles](tasks/04_m27_capability_linker/M27-003-A-build-configurable-intrinsic-profiles.md) — deps: M27-002-Z — #252
118. [M27-003-B — Compile application requirements](tasks/04_m27_capability_linker/M27-003-B-compile-application-requirements.md) — deps: M27-003-A — #253
119. [M27-003-C — Report missing API/intrinsic diagnostics](tasks/04_m27_capability_linker/M27-003-C-report-missing-api-intrinsic-diagnostics.md) — deps: M27-003-B — #254
120. [M27-003-D — Retain full profile for compatibility testing](tasks/04_m27_capability_linker/M27-003-D-retain-full-profile-for-compatibility-testing.md) — deps: M27-003-C — #255
121. [M27-003-V — Verify Introduce custom QuickJS context profiles](tasks/04_m27_capability_linker/M27-003-V-verify-introduce-custom-quickjs-context-profiles.md) — deps: M27-003-A, M27-003-B, M27-003-C, M27-003-D — #256
122. [M27-003-Z — Package evidence for Introduce custom QuickJS context profiles](tasks/04_m27_capability_linker/M27-003-Z-package-evidence-for-introduce-custom-quickjs-context-profiles.md) — deps: M27-003-V — #257
123. [M27-004-A — Port timer cancellation/accounting](tasks/04_m27_capability_linker/M27-004-A-port-timer-cancellation-accounting.md) — deps: M27-001-Z, M27-002-Z — #258
124. [M27-004-B — Define console levels and redaction](tasks/04_m27_capability_linker/M27-004-B-define-console-levels-and-redaction.md) — deps: M27-004-A — #259
125. [M27-004-C — Keep logs asynchronous/bounded](tasks/04_m27_capability_linker/M27-004-C-keep-logs-asynchronous-bounded.md) — deps: M27-004-B — #260
126. [M27-004-D — Support shutdown and quarantine](tasks/04_m27_capability_linker/M27-004-D-support-shutdown-and-quarantine.md) — deps: M27-004-C — #261
127. [M27-004-V — Verify Implement console and timer core capabilities](tasks/04_m27_capability_linker/M27-004-V-verify-implement-console-and-timer-core-capabilities.md) — deps: M27-004-A, M27-004-B, M27-004-C, M27-004-D — #262
128. [M27-004-Z — Package evidence for Implement console and timer core capabilities](tasks/04_m27_capability_linker/M27-004-Z-package-evidence-for-implement-console-and-timer-core-capabilities.md) — deps: M27-004-V — #263
129. [M27-005-A — Adopt or adapt a proven implementation](tasks/04_m27_capability_linker/M27-005-A-adopt-or-adapt-a-proven-implementation.md) — deps: M27-001-Z, M27-003-Z — #264
130. [M27-005-B — Run selected WPT/WinterTC cases](tasks/04_m27_capability_linker/M27-005-B-run-selected-wpt-wintertc-cases.md) — deps: M27-005-A — #265
131. [M27-005-C — Define host/path encoding behavior](tasks/04_m27_capability_linker/M27-005-C-define-host-path-encoding-behavior.md) — deps: M27-005-B — #266
132. [M27-005-D — Keep parser limits explicit](tasks/04_m27_capability_linker/M27-005-D-keep-parser-limits-explicit.md) — deps: M27-005-C — #267
133. [M27-005-V — Verify Implement URL and URLSearchParams](tasks/04_m27_capability_linker/M27-005-V-verify-implement-url-and-urlsearchparams.md) — deps: M27-005-A, M27-005-B, M27-005-C, M27-005-D — #268
134. [M27-005-Z — Package evidence for Implement URL and URLSearchParams](tasks/04_m27_capability_linker/M27-005-Z-package-evidence-for-implement-url-and-urlsearchparams.md) — deps: M27-005-V — #269
135. [M27-006-A — Support UTF-8 baseline](tasks/04_m27_capability_linker/M27-006-A-support-utf-8-baseline.md) — deps: M27-001-Z, M27-003-Z — #270
136. [M27-006-B — Define invalid sequence/replacement behavior](tasks/04_m27_capability_linker/M27-006-B-define-invalid-sequence-replacement-behavior.md) — deps: M27-006-A — #271
137. [M27-006-C — Integrate TypedArray ownership](tasks/04_m27_capability_linker/M27-006-C-integrate-typedarray-ownership.md) — deps: M27-006-B — #272
138. [M27-006-D — Run WPT subset](tasks/04_m27_capability_linker/M27-006-D-run-wpt-subset.md) — deps: M27-006-C — #273
139. [M27-006-V — Verify Implement TextEncoder and TextDecoder](tasks/04_m27_capability_linker/M27-006-V-verify-implement-textencoder-and-textdecoder.md) — deps: M27-006-A, M27-006-B, M27-006-C, M27-006-D — #274
140. [M27-006-Z — Package evidence for Implement TextEncoder and TextDecoder](tasks/04_m27_capability_linker/M27-006-Z-package-evidence-for-implement-textencoder-and-textdecoder.md) — deps: M27-006-V — #275
141. [M27-007-A — Define signal state/listeners/reason](tasks/04_m27_capability_linker/M27-007-A-define-signal-state-listeners-reason.md) — deps: M27-001-Z, M27-003-Z — #276
142. [M27-007-B — Bridge route deadline and explicit cancellation](tasks/04_m27_capability_linker/M27-007-B-bridge-route-deadline-and-explicit-cancellation.md) — deps: M27-007-A — #277
143. [M27-007-C — Prevent listener leaks](tasks/04_m27_capability_linker/M27-007-C-prevent-listener-leaks.md) — deps: M27-007-B — #278
144. [M27-007-D — Make cancellation idempotent](tasks/04_m27_capability_linker/M27-007-D-make-cancellation-idempotent.md) — deps: M27-007-C — #279
145. [M27-007-V — Verify Implement AbortController and AbortSignal](tasks/04_m27_capability_linker/M27-007-V-verify-implement-abortcontroller-and-abortsignal.md) — deps: M27-007-A, M27-007-B, M27-007-C, M27-007-D — #280
146. [M27-007-Z — Package evidence for Implement AbortController and AbortSignal](tasks/04_m27_capability_linker/M27-007-Z-package-evidence-for-implement-abortcontroller-and-abortsignal.md) — deps: M27-007-V — #281
147. [M27-008-A — Implement `getRandomValues` and `randomUUID` through OS CSPRNG](tasks/04_m27_capability_linker/M27-008-A-implement-getrandomvalues-and-randomuuid-through-os-csprng.md) — deps: M27-001-Z, M27-003-Z — #282
148. [M27-008-B — Enforce typed-array and size constraints](tasks/04_m27_capability_linker/M27-008-B-enforce-typed-array-and-size-constraints.md) — deps: M27-008-A — #283
149. [M27-008-C — Define unavailable-entropy failure](tasks/04_m27_capability_linker/M27-008-C-define-unavailable-entropy-failure.md) — deps: M27-008-B — #284
150. [M27-008-D — Do not implement custom cryptography](tasks/04_m27_capability_linker/M27-008-D-do-not-implement-custom-cryptography.md) — deps: M27-008-C — #285
151. [M27-008-V — Verify Implement crypto random subset](tasks/04_m27_capability_linker/M27-008-V-verify-implement-crypto-random-subset.md) — deps: M27-008-A, M27-008-B, M27-008-C, M27-008-D — #286
152. [M27-008-Z — Package evidence for Implement crypto random subset](tasks/04_m27_capability_linker/M27-008-Z-package-evidence-for-implement-crypto-random-subset.md) — deps: M27-008-V — #287
153. [M27-009-A — Define Rust-side SDK traits and metadata](tasks/04_m27_capability_linker/M27-009-A-define-rust-side-sdk-traits-and-metadata.md) — deps: M27-001-Z, M27-002-Z — #288
154. [M27-009-B — Provide test harness and example capability](tasks/04_m27_capability_linker/M27-009-B-provide-test-harness-and-example-capability.md) — deps: M27-009-A — #289
155. [M27-009-C — Expose build/inspect diagnostics](tasks/04_m27_capability_linker/M27-009-C-expose-build-inspect-diagnostics.md) — deps: M27-009-B — #290
156. [M27-009-D — Define semver/ABI compatibility](tasks/04_m27_capability_linker/M27-009-D-define-semver-abi-compatibility.md) — deps: M27-009-C — #291
157. [M27-009-V — Verify Publish capability SDK and inspection surface](tasks/04_m27_capability_linker/M27-009-V-verify-publish-capability-sdk-and-inspection-surface.md) — deps: M27-009-A, M27-009-B, M27-009-C, M27-009-D — #292
158. [M27-009-Z — Package evidence for Publish capability SDK and inspection surface](tasks/04_m27_capability_linker/M27-009-Z-package-evidence-for-publish-capability-sdk-and-inspection-surface.md) — deps: M27-009-V — #293
159. [M27-010-A — Pin WPT/WinterTC subsets](tasks/04_m27_capability_linker/M27-010-A-pin-wpt-wintertc-subsets.md) — deps: M27-005-Z, M27-006-Z, M27-007-Z, M27-008-Z — #294
160. [M27-010-B — Record skips and reasons](tasks/04_m27_capability_linker/M27-010-B-record-skips-and-reasons.md) — deps: M27-010-A — #295
161. [M27-010-C — Automate regression reports](tasks/04_m27_capability_linker/M27-010-C-automate-regression-reports.md) — deps: M27-010-B — #296
162. [M27-010-D — Keep unsupported APIs explicit](tasks/04_m27_capability_linker/M27-010-D-keep-unsupported-apis-explicit.md) — deps: M27-010-C — #297
163. [M27-010-V — Verify Establish Web API conformance program](tasks/04_m27_capability_linker/M27-010-V-verify-establish-web-api-conformance-program.md) — deps: M27-010-A, M27-010-B, M27-010-C, M27-010-D — #298
164. [M27-010-Z — Package evidence for Establish Web API conformance program](tasks/04_m27_capability_linker/M27-010-Z-package-evidence-for-establish-web-api-conformance-program.md) — deps: M27-010-V — #299
165. [M27-011-A — Measure core, web-minimal, and all-beta profiles](tasks/04_m27_capability_linker/M27-011-A-measure-core-web-minimal-and-all-beta-profiles.md) — deps: M27-002-Z, M27-010-Z — #300
166. [M27-011-B — Record binary, startup, and idle RSS deltas](tasks/04_m27_capability_linker/M27-011-B-record-binary-startup-and-idle-rss-deltas.md) — deps: M27-011-A — #301
167. [M27-011-C — Identify eager initialization](tasks/04_m27_capability_linker/M27-011-C-identify-eager-initialization.md) — deps: M27-011-B — #302
168. [M27-011-D — Make expensive modules lazy when safe](tasks/04_m27_capability_linker/M27-011-D-make-expensive-modules-lazy-when-safe.md) — deps: M27-011-C — #303
169. [M27-011-V — Verify Close capability cost budgets](tasks/04_m27_capability_linker/M27-011-V-verify-close-capability-cost-budgets.md) — deps: M27-011-A, M27-011-B, M27-011-C, M27-011-D — #304
170. [M27-011-Z — Package evidence for Close capability cost budgets](tasks/04_m27_capability_linker/M27-011-Z-package-evidence-for-close-capability-cost-budgets.md) — deps: M27-011-V — #305
171. [M27-GATE — M2.7 — Capability Linker and Minimal Web Runtime exit gate](gates/M27-GATE.md) — deps: M27-001-Z, M27-002-Z, M27-003-Z, M27-004-Z, M27-005-Z, M27-006-Z, M27-007-Z, M27-008-Z, M27-009-Z, M27-010-Z, M27-011-Z — #630
172. [M28-001-A — Define URL schemes, redirect policy, DNS rebinding controls, proxy behavior, TLS roots, timeout layers, compression, and body limits](tasks/05_m28_native_fetch/M28-001-A-define-url-schemes-redirect-policy-dns-rebinding-controls-proxy-behavior-tls-roo.md) — deps: M27-GATE — #306
173. [M28-001-B — Specify reverse-proxy and outbound trust](tasks/05_m28_native_fetch/M28-001-B-specify-reverse-proxy-and-outbound-trust.md) — deps: M28-001-A — #307
174. [M28-001-C — Define unsupported Web features](tasks/05_m28_native_fetch/M28-001-C-define-unsupported-web-features.md) — deps: M28-001-B — #308
175. [M28-001-D — Document same-process trusted-code assumption](tasks/05_m28_native_fetch/M28-001-D-document-same-process-trusted-code-assumption.md) — deps: M28-001-C — #309
176. [M28-001-V — Verify Accept fetch, TLS, redirect, and SSRF security ADR](tasks/05_m28_native_fetch/M28-001-V-verify-accept-fetch-tls-redirect-and-ssrf-security-adr.md) — deps: M28-001-A, M28-001-B, M28-001-C, M28-001-D — #310
177. [M28-001-Z — Package evidence for Accept fetch, TLS, redirect, and SSRF security ADR](tasks/05_m28_native_fetch/M28-001-Z-package-evidence-for-accept-fetch-tls-redirect-and-ssrf-security-adr.md) — deps: M28-001-V — #311
178. [M28-002-A — Compare reqwest and lower-level Hyper/Rustls approach](tasks/05_m28_native_fetch/M28-002-A-compare-reqwest-and-lower-level-hyper-rustls-approach.md) — deps: M28-001-Z — #312
179. [M28-002-B — Measure dependency/binary/startup cost](tasks/05_m28_native_fetch/M28-002-B-measure-dependency-binary-startup-cost.md) — deps: M28-002-A — #313
180. [M28-002-C — Test DNS/TLS/pool behavior](tasks/05_m28_native_fetch/M28-002-C-test-dns-tls-pool-behavior.md) — deps: M28-002-B — #314
181. [M28-002-D — Record maintenance/security considerations](tasks/05_m28_native_fetch/M28-002-D-record-maintenance-security-considerations.md) — deps: M28-002-C — #315
182. [M28-002-V — Verify Select native HTTP client stack from evidence](tasks/05_m28_native_fetch/M28-002-V-verify-select-native-http-client-stack-from-evidence.md) — deps: M28-002-A, M28-002-B, M28-002-C, M28-002-D — #316
183. [M28-002-Z — Package evidence for Select native HTTP client stack from evidence](tasks/05_m28_native_fetch/M28-002-Z-package-evidence-for-select-native-http-client-stack-from-evidence.md) — deps: M28-002-V — #317
184. [M28-003-A — Lazy pool initialization](tasks/05_m28_native_fetch/M28-003-A-lazy-pool-initialization.md) — deps: M28-002-Z — #318
185. [M28-003-B — Bound idle/active connections and DNS cache](tasks/05_m28_native_fetch/M28-003-B-bound-idle-active-connections-and-dns-cache.md) — deps: M28-003-A — #319
186. [M28-003-C — Use verified TLS roots and hostname validation](tasks/05_m28_native_fetch/M28-003-C-use-verified-tls-roots-and-hostname-validation.md) — deps: M28-003-B — #320
187. [M28-003-D — Define keepalive and shutdown](tasks/05_m28_native_fetch/M28-003-D-define-keepalive-and-shutdown.md) — deps: M28-003-C — #321
188. [M28-003-V — Verify Implement connection pooling, DNS, and TLS](tasks/05_m28_native_fetch/M28-003-V-verify-implement-connection-pooling-dns-and-tls.md) — deps: M28-003-A, M28-003-B, M28-003-C, M28-003-D — #322
189. [M28-003-Z — Package evidence for Implement connection pooling, DNS, and TLS](tasks/05_m28_native_fetch/M28-003-Z-package-evidence-for-implement-connection-pooling-dns-and-tls.md) — deps: M28-003-V — #323
190. [M28-004-A — Implement method, URL, selected headers, body types, status, and response methods](tasks/05_m28_native_fetch/M28-004-A-implement-method-url-selected-headers-body-types-status-and-response-methods.md) — deps: M28-003-Z, M27-005-Z, M27-006-Z — #324
191. [M28-004-B — Use lazy native-backed objects](tasks/05_m28_native_fetch/M28-004-B-use-lazy-native-backed-objects.md) — deps: M28-004-A — #325
192. [M28-004-C — Define clone/body-used semantics for beta](tasks/05_m28_native_fetch/M28-004-C-define-clone-body-used-semantics-for-beta.md) — deps: M28-004-B — #326
193. [M28-004-D — Keep unsupported API diagnostics explicit](tasks/05_m28_native_fetch/M28-004-D-keep-unsupported-api-diagnostics-explicit.md) — deps: M28-004-C — #327
194. [M28-004-V — Verify Implement Request, Response, and Headers subset](tasks/05_m28_native_fetch/M28-004-V-verify-implement-request-response-and-headers-subset.md) — deps: M28-004-A, M28-004-B, M28-004-C, M28-004-D — #328
195. [M28-004-Z — Package evidence for Implement Request, Response, and Headers subset](tasks/05_m28_native_fetch/M28-004-Z-package-evidence-for-implement-request-response-and-headers-subset.md) — deps: M28-004-V — #329
196. [M28-005-A — Combine explicit abort, route deadline, disconnect, shutdown, and quarantine](tasks/05_m28_native_fetch/M28-005-A-combine-explicit-abort-route-deadline-disconnect-shutdown-and-quarantine.md) — deps: M28-003-Z, M27-007-Z — #330
197. [M28-005-B — Use one terminal state for each operation](tasks/05_m28_native_fetch/M28-005-B-use-one-terminal-state-for-each-operation.md) — deps: M28-005-A — #331
198. [M28-005-C — Cancel DNS/connect/body streaming](tasks/05_m28_native_fetch/M28-005-C-cancel-dns-connect-body-streaming.md) — deps: M28-005-B — #332
199. [M28-005-D — Map failures deterministically](tasks/05_m28_native_fetch/M28-005-D-map-failures-deterministically.md) — deps: M28-005-C — #333
200. [M28-005-V — Verify Propagate AbortSignal and route deadlines](tasks/05_m28_native_fetch/M28-005-V-verify-propagate-abortsignal-and-route-deadlines.md) — deps: M28-005-A, M28-005-B, M28-005-C, M28-005-D — #334
201. [M28-005-Z — Package evidence for Propagate AbortSignal and route deadlines](tasks/05_m28_native_fetch/M28-005-Z-package-evidence-for-propagate-abortsignal-and-route-deadlines.md) — deps: M28-005-V — #335
202. [M28-006-A — Bound read/write buffers](tasks/05_m28_native_fetch/M28-006-A-bound-read-write-buffers.md) — deps: M28-004-Z, M28-005-Z — #336
203. [M28-006-B — Propagate downstream backpressure](tasks/05_m28_native_fetch/M28-006-B-propagate-downstream-backpressure.md) — deps: M28-006-A — #337
204. [M28-006-C — Cancel on consumer stop/disconnect](tasks/05_m28_native_fetch/M28-006-C-cancel-on-consumer-stop-disconnect.md) — deps: M28-006-B — #338
205. [M28-006-D — Define maximum body helper sizes](tasks/05_m28_native_fetch/M28-006-D-define-maximum-body-helper-sizes.md) — deps: M28-006-C — #339
206. [M28-006-V — Verify Implement streaming and strict backpressure](tasks/05_m28_native_fetch/M28-006-V-verify-implement-streaming-and-strict-backpressure.md) — deps: M28-006-A, M28-006-B, M28-006-C, M28-006-D — #340
207. [M28-006-Z — Package evidence for Implement streaming and strict backpressure](tasks/05_m28_native_fetch/M28-006-Z-package-evidence-for-implement-streaming-and-strict-backpressure.md) — deps: M28-006-V — #341
208. [M28-007-A — Limit redirect count](tasks/05_m28_native_fetch/M28-007-A-limit-redirect-count.md) — deps: M28-003-Z, M28-004-Z — #342
209. [M28-007-B — Reapply SSRF/DNS policy on every hop](tasks/05_m28_native_fetch/M28-007-B-reapply-ssrf-dns-policy-on-every-hop.md) — deps: M28-007-A — #343
210. [M28-007-C — Define credential/header stripping](tasks/05_m28_native_fetch/M28-007-C-define-credential-header-stripping.md) — deps: M28-007-B — #344
211. [M28-007-D — Bound decompression ratio and output](tasks/05_m28_native_fetch/M28-007-D-bound-decompression-ratio-and-output.md) — deps: M28-007-C — #345
212. [M28-007-V — Verify Implement redirect and compression policy](tasks/05_m28_native_fetch/M28-007-V-verify-implement-redirect-and-compression-policy.md) — deps: M28-007-A, M28-007-B, M28-007-C, M28-007-D — #346
213. [M28-007-Z — Package evidence for Implement redirect and compression policy](tasks/05_m28_native_fetch/M28-007-Z-package-evidence-for-implement-redirect-and-compression-policy.md) — deps: M28-007-V — #347
214. [M28-008-A — Resolve and validate addresses before connect](tasks/05_m28_native_fetch/M28-008-A-resolve-and-validate-addresses-before-connect.md) — deps: M28-001-Z, M28-003-Z, M28-007-Z — #348
215. [M28-008-B — Revalidate redirects and connection targets](tasks/05_m28_native_fetch/M28-008-B-revalidate-redirects-and-connection-targets.md) — deps: M28-008-A — #349
216. [M28-008-C — Support allow/deny configuration](tasks/05_m28_native_fetch/M28-008-C-support-allow-deny-configuration.md) — deps: M28-008-B — #350
217. [M28-008-D — Define proxy interaction](tasks/05_m28_native_fetch/M28-008-D-define-proxy-interaction.md) — deps: M28-008-C — #351
218. [M28-008-V — Verify Implement SSRF and network egress controls](tasks/05_m28_native_fetch/M28-008-V-verify-implement-ssrf-and-network-egress-controls.md) — deps: M28-008-A, M28-008-B, M28-008-C, M28-008-D — #352
219. [M28-008-Z — Package evidence for Implement SSRF and network egress controls](tasks/05_m28_native_fetch/M28-008-Z-package-evidence-for-implement-ssrf-and-network-egress-controls.md) — deps: M28-008-V — #353
220. [M28-009-A — Expose pool wait, DNS, connect, TLS, TTFB, body, errors, cancellations](tasks/05_m28_native_fetch/M28-009-A-expose-pool-wait-dns-connect-tls-ttfb-body-errors-cancellations.md) — deps: M28-003-Z, M28-005-Z, M28-006-Z — #354
221. [M28-009-B — Sample/aggregate metrics](tasks/05_m28_native_fetch/M28-009-B-sample-aggregate-metrics.md) — deps: M28-009-A — #355
222. [M28-009-C — Drain pool on shutdown](tasks/05_m28_native_fetch/M28-009-C-drain-pool-on-shutdown.md) — deps: M28-009-B — #356
223. [M28-009-D — Quarantine rejects new work](tasks/05_m28_native_fetch/M28-009-D-quarantine-rejects-new-work.md) — deps: M28-009-C — #357
224. [M28-009-V — Verify Integrate lifecycle, observability, and shutdown](tasks/05_m28_native_fetch/M28-009-V-verify-integrate-lifecycle-observability-and-shutdown.md) — deps: M28-009-A, M28-009-B, M28-009-C, M28-009-D — #358
225. [M28-009-Z — Package evidence for Integrate lifecycle, observability, and shutdown](tasks/05_m28_native_fetch/M28-009-Z-package-evidence-for-integrate-lifecycle-observability-and-shutdown.md) — deps: M28-009-V — #359
226. [M28-010-A — Run selected WPT cases](tasks/05_m28_native_fetch/M28-010-A-run-selected-wpt-cases.md) — deps: M28-004-Z, M28-005-Z, M28-006-Z, M28-007-Z, M28-008-Z — #360
227. [M28-010-B — Create deterministic DNS/TLS/redirect/slow/body fixtures](tasks/05_m28_native_fetch/M28-010-B-create-deterministic-dns-tls-redirect-slow-body-fixtures.md) — deps: M28-010-A — #361
228. [M28-010-C — Fuzz headers and URLs](tasks/05_m28_native_fetch/M28-010-C-fuzz-headers-and-urls.md) — deps: M28-010-B — #362
229. [M28-010-D — Test proxy and cancellation](tasks/05_m28_native_fetch/M28-010-D-test-proxy-and-cancellation.md) — deps: M28-010-C — #363
230. [M28-010-V — Verify Complete fetch conformance and fault testing](tasks/05_m28_native_fetch/M28-010-V-verify-complete-fetch-conformance-and-fault-testing.md) — deps: M28-010-A, M28-010-B, M28-010-C, M28-010-D — #364
231. [M28-010-Z — Package evidence for Complete fetch conformance and fault testing](tasks/05_m28_native_fetch/M28-010-Z-package-evidence-for-complete-fetch-conformance-and-fault-testing.md) — deps: M28-010-V — #365
232. [M28-011-A — Run 1/5/10/25ms upstream latency](tasks/05_m28_native_fetch/M28-011-A-run-1-5-10-25ms-upstream-latency.md) — deps: M28-009-Z, M28-010-Z — #366
233. [M28-011-B — Run one, two, and four parallel calls](tasks/05_m28_native_fetch/M28-011-B-run-one-two-and-four-parallel-calls.md) — deps: M28-011-A — #367
234. [M28-011-C — Mix timeout/success/malformed responses](tasks/05_m28_native_fetch/M28-011-C-mix-timeout-success-malformed-responses.md) — deps: M28-011-B — #368
235. [M28-011-D — Test concurrency 1/10/50/200](tasks/05_m28_native_fetch/M28-011-D-test-concurrency-1-10-50-200.md) — deps: M28-011-C — #369
236. [M28-011-V — Verify Run controlled upstream and fan-out benchmarks](tasks/05_m28_native_fetch/M28-011-V-verify-run-controlled-upstream-and-fan-out-benchmarks.md) — deps: M28-011-A, M28-011-B, M28-011-C, M28-011-D — #370
237. [M28-011-Z — Package evidence for Run controlled upstream and fan-out benchmarks](tasks/05_m28_native_fetch/M28-011-Z-package-evidence-for-run-controlled-upstream-and-fan-out-benchmarks.md) — deps: M28-011-V — #371
238. [M28-GATE — M2.8 — Native Outbound Fetch exit gate](gates/M28-GATE.md) — deps: M28-001-Z, M28-002-Z, M28-003-Z, M28-004-Z, M28-005-Z, M28-006-Z, M28-007-Z, M28-008-Z, M28-009-Z, M28-010-Z, M28-011-Z — #631
239. [M3-001-A — Accept ADR](tasks/06_m3_multi_worker/M3-001-A-accept-adr.md) — deps: M28-GATE — #372
240. [M3-001-B — Document module-level state replication](tasks/06_m3_multi_worker/M3-001-B-document-module-level-state-replication.md) — deps: M3-001-A — #373
241. [M3-001-C — Forbid JSValue sharing](tasks/06_m3_multi_worker/M3-001-C-forbid-jsvalue-sharing.md) — deps: M3-001-B — #374
242. [M3-001-D — Define service/capability shared handles and thread safety](tasks/06_m3_multi_worker/M3-001-D-define-service-capability-shared-handles-and-thread-safety.md) — deps: M3-001-C — #375
243. [M3-001-V — Verify Freeze independent-worker state semantics](tasks/06_m3_multi_worker/M3-001-V-verify-freeze-independent-worker-state-semantics.md) — deps: M3-001-A, M3-001-B, M3-001-C, M3-001-D — #376
244. [M3-001-Z — Package evidence for Freeze independent-worker state semantics](tasks/06_m3_multi_worker/M3-001-Z-package-evidence-for-freeze-independent-worker-state-semantics.md) — deps: M3-001-V — #377
245. [M3-002-A — Use bounded per-worker queues](tasks/06_m3_multi_worker/M3-002-A-use-bounded-per-worker-queues.md) — deps: M3-001-Z — #378
246. [M3-002-B — Select worker using outstanding-load strategy](tasks/06_m3_multi_worker/M3-002-B-select-worker-using-outstanding-load-strategy.md) — deps: M3-002-A — #379
247. [M3-002-C — Define admission and overload response](tasks/06_m3_multi_worker/M3-002-C-define-admission-and-overload-response.md) — deps: M3-002-B — #380
248. [M3-002-D — Preserve RouteId/RoutePlan before dispatch](tasks/06_m3_multi_worker/M3-002-D-preserve-routeid-routeplan-before-dispatch.md) — deps: M3-002-C — #381
249. [M3-002-V — Verify Implement bounded worker dispatcher](tasks/06_m3_multi_worker/M3-002-V-verify-implement-bounded-worker-dispatcher.md) — deps: M3-002-A, M3-002-B, M3-002-C, M3-002-D — #382
250. [M3-002-Z — Package evidence for Implement bounded worker dispatcher](tasks/06_m3_multi_worker/M3-002-Z-package-evidence-for-implement-bounded-worker-dispatcher.md) — deps: M3-002-V — #383
251. [M3-003-A — Serverless starts one worker only](tasks/06_m3_multi_worker/M3-003-A-serverless-starts-one-worker-only.md) — deps: M3-002-Z — #384
252. [M3-003-B — Service marks ready after worker 0 and adds workers adaptively](tasks/06_m3_multi_worker/M3-003-B-service-marks-ready-after-worker-0-and-adds-workers-adaptively.md) — deps: M3-003-A — #385
253. [M3-003-C — Throughput initializes configured workers before ready](tasks/06_m3_multi_worker/M3-003-C-throughput-initializes-configured-workers-before-ready.md) — deps: M3-003-B — #386
254. [M3-003-D — Expose profile in inspect/config](tasks/06_m3_multi_worker/M3-003-D-expose-profile-in-inspect-config.md) — deps: M3-003-C — #387
255. [M3-003-V — Verify Implement serverless, service, and throughput profiles](tasks/06_m3_multi_worker/M3-003-V-verify-implement-serverless-service-and-throughput-profiles.md) — deps: M3-003-A, M3-003-B, M3-003-C, M3-003-D — #388
256. [M3-003-Z — Package evidence for Implement serverless, service, and throughput profiles](tasks/06_m3_multi_worker/M3-003-Z-package-evidence-for-implement-serverless-service-and-throughput-profiles.md) — deps: M3-003-V — #389
257. [M3-004-A — Share immutable mapped QPack bytes](tasks/06_m3_multi_worker/M3-004-A-share-immutable-mapped-qpack-bytes.md) — deps: M3-002-Z, M26-GATE — #390
258. [M3-004-B — Create separate QuickJS runtimes/functions/context state](tasks/06_m3_multi_worker/M3-004-B-create-separate-quickjs-runtimes-functions-context-state.md) — deps: M3-004-A — #391
259. [M3-004-C — Validate capability compatibility per worker](tasks/06_m3_multi_worker/M3-004-C-validate-capability-compatibility-per-worker.md) — deps: M3-004-B — #392
260. [M3-004-D — Bound startup parallelism](tasks/06_m3_multi_worker/M3-004-D-bound-startup-parallelism.md) — deps: M3-004-C — #393
261. [M3-004-V — Verify Implement deterministic worker initialization and artifact sharing](tasks/06_m3_multi_worker/M3-004-V-verify-implement-deterministic-worker-initialization-and-artifact-sharing.md) — deps: M3-004-A, M3-004-B, M3-004-C, M3-004-D — #394
262. [M3-004-Z — Package evidence for Implement deterministic worker initialization and artifact sharing](tasks/06_m3_multi_worker/M3-004-Z-package-evidence-for-implement-deterministic-worker-initialization-and-artifact.md) — deps: M3-004-V — #395
263. [M3-005-A — Remove quarantined worker from dispatch](tasks/06_m3_multi_worker/M3-005-A-remove-quarantined-worker-from-dispatch.md) — deps: M3-002-Z, M3-004-Z — #396
264. [M3-005-B — Fail/settle its pending work](tasks/06_m3_multi_worker/M3-005-B-fail-settle-its-pending-work.md) — deps: M3-005-A — #397
265. [M3-005-C — Initialize replacement under bounded policy](tasks/06_m3_multi_worker/M3-005-C-initialize-replacement-under-bounded-policy.md) — deps: M3-005-B — #398
266. [M3-005-D — Aggregate readiness from usable capacity](tasks/06_m3_multi_worker/M3-005-D-aggregate-readiness-from-usable-capacity.md) — deps: M3-005-C — #399
267. [M3-005-V — Verify Implement quarantine, replacement, and readiness aggregation](tasks/06_m3_multi_worker/M3-005-V-verify-implement-quarantine-replacement-and-readiness-aggregation.md) — deps: M3-005-A, M3-005-B, M3-005-C, M3-005-D — #400
268. [M3-005-Z — Package evidence for Implement quarantine, replacement, and readiness aggregation](tasks/06_m3_multi_worker/M3-005-Z-package-evidence-for-implement-quarantine-replacement-and-readiness-aggregation.md) — deps: M3-005-V — #401
269. [M3-006-A — Define thresholds/hysteresis](tasks/06_m3_multi_worker/M3-006-A-define-thresholds-hysteresis.md) — deps: M3-003-Z, M3-005-Z — #402
270. [M3-006-B — Bound min/max workers](tasks/06_m3_multi_worker/M3-006-B-bound-min-max-workers.md) — deps: M3-006-A — #403
271. [M3-006-C — Drain before scale-down](tasks/06_m3_multi_worker/M3-006-C-drain-before-scale-down.md) — deps: M3-006-B — #404
272. [M3-006-D — Avoid oscillation](tasks/06_m3_multi_worker/M3-006-D-avoid-oscillation.md) — deps: M3-006-C — #405
273. [M3-006-V — Verify Implement adaptive scale-up and scale-down](tasks/06_m3_multi_worker/M3-006-V-verify-implement-adaptive-scale-up-and-scale-down.md) — deps: M3-006-A, M3-006-B, M3-006-C, M3-006-D — #406
274. [M3-006-Z — Package evidence for Implement adaptive scale-up and scale-down](tasks/06_m3_multi_worker/M3-006-Z-package-evidence-for-implement-adaptive-scale-up-and-scale-down.md) — deps: M3-006-V — #407
275. [M3-007-A — Track invocation-to-worker ownership](tasks/06_m3_multi_worker/M3-007-A-track-invocation-to-worker-ownership.md) — deps: M3-002-Z, M3-004-Z — #408
276. [M3-007-B — Stop admission on drain](tasks/06_m3_multi_worker/M3-007-B-stop-admission-on-drain.md) — deps: M3-007-A — #409
277. [M3-007-C — Allow bounded in-flight completion](tasks/06_m3_multi_worker/M3-007-C-allow-bounded-in-flight-completion.md) — deps: M3-007-B — #410
278. [M3-007-D — Abort after shutdown deadline](tasks/06_m3_multi_worker/M3-007-D-abort-after-shutdown-deadline.md) — deps: M3-007-C — #411
279. [M3-007-V — Verify Implement multi-worker cancellation and graceful shutdown](tasks/06_m3_multi_worker/M3-007-V-verify-implement-multi-worker-cancellation-and-graceful-shutdown.md) — deps: M3-007-A, M3-007-B, M3-007-C, M3-007-D — #412
280. [M3-007-Z — Package evidence for Implement multi-worker cancellation and graceful shutdown](tasks/06_m3_multi_worker/M3-007-Z-package-evidence-for-implement-multi-worker-cancellation-and-graceful-shutdown.md) — deps: M3-007-V — #413
281. [M3-008-A — Add route/global queue limits or weighted admission](tasks/06_m3_multi_worker/M3-008-A-add-route-global-queue-limits-or-weighted-admission.md) — deps: M3-002-Z, M3-006-Z — #414
282. [M3-008-B — Define long-running JS policy](tasks/06_m3_multi_worker/M3-008-B-define-long-running-js-policy.md) — deps: M3-008-A — #415
283. [M3-008-C — Expose load-shed reasons](tasks/06_m3_multi_worker/M3-008-C-expose-load-shed-reasons.md) — deps: M3-008-B — #416
284. [M3-008-D — Test mixed workloads](tasks/06_m3_multi_worker/M3-008-D-test-mixed-workloads.md) — deps: M3-008-C — #417
285. [M3-008-V — Verify Add fairness and overload controls](tasks/06_m3_multi_worker/M3-008-V-verify-add-fairness-and-overload-controls.md) — deps: M3-008-A, M3-008-B, M3-008-C, M3-008-D — #418
286. [M3-008-Z — Package evidence for Add fairness and overload controls](tasks/06_m3_multi_worker/M3-008-Z-package-evidence-for-add-fairness-and-overload-controls.md) — deps: M3-008-V — #419
287. [M3-009-A — Measure 1/2/4 workers](tasks/06_m3_multi_worker/M3-009-A-measure-1-2-4-workers.md) — deps: M3-003-Z, M3-006-Z, M3-008-Z — #420
288. [M3-009-B — Report throughput, p50/p95/p99, queue time, CPU, RSS, errors](tasks/06_m3_multi_worker/M3-009-B-report-throughput-p50-p95-p99-queue-time-cpu-rss-errors.md) — deps: M3-009-A — #421
289. [M3-009-C — Run C1/C2/C3 and controlled I/O](tasks/06_m3_multi_worker/M3-009-C-run-c1-c2-c3-and-controlled-i-o.md) — deps: M3-009-B — #422
290. [M3-009-D — Record physical core topology](tasks/06_m3_multi_worker/M3-009-D-record-physical-core-topology.md) — deps: M3-009-C — #423
291. [M3-009-V — Verify Close multi-worker scaling and memory evidence](tasks/06_m3_multi_worker/M3-009-V-verify-close-multi-worker-scaling-and-memory-evidence.md) — deps: M3-009-A, M3-009-B, M3-009-C, M3-009-D — #424
292. [M3-009-Z — Package evidence for Close multi-worker scaling and memory evidence](tasks/06_m3_multi_worker/M3-009-Z-package-evidence-for-close-multi-worker-scaling-and-memory-evidence.md) — deps: M3-009-V — #425
293. [M3-010-A — Run multi-hour mixed load](tasks/06_m3_multi_worker/M3-010-A-run-multi-hour-mixed-load.md) — deps: M3-005-Z, M3-007-Z, M3-009-Z — #426
294. [M3-010-B — Inject worker poison, upstream timeout, disconnect, and shutdown](tasks/06_m3_multi_worker/M3-010-B-inject-worker-poison-upstream-timeout-disconnect-and-shutdown.md) — deps: M3-010-A — #427
295. [M3-010-C — Track retained memory and task/slot counts](tasks/06_m3_multi_worker/M3-010-C-track-retained-memory-and-task-slot-counts.md) — deps: M3-010-B — #428
296. [M3-010-D — Verify recovery](tasks/06_m3_multi_worker/M3-010-D-verify-recovery.md) — deps: M3-010-C — #429
297. [M3-010-V — Verify Run multi-worker soak and recovery](tasks/06_m3_multi_worker/M3-010-V-verify-run-multi-worker-soak-and-recovery.md) — deps: M3-010-A, M3-010-B, M3-010-C, M3-010-D — #430
298. [M3-010-Z — Package evidence for Run multi-worker soak and recovery](tasks/06_m3_multi_worker/M3-010-Z-package-evidence-for-run-multi-worker-soak-and-recovery.md) — deps: M3-010-V — #431
299. [M3-GATE — M3 — Multi-Worker Service Runtime exit gate](gates/M3-GATE.md) — deps: M3-001-Z, M3-002-Z, M3-003-Z, M3-004-Z, M3-005-Z, M3-006-Z, M3-007-Z, M3-008-Z, M3-009-Z, M3-010-Z — #632
300. [M4A-001-A — Watch source and contracts](tasks/07_m4a_developer_preview/M4A-001-A-watch-source-and-contracts.md) — deps: M3-GATE — #432
301. [M4A-001-B — Build incremental temporary QPack](tasks/07_m4a_developer_preview/M4A-001-B-build-incremental-temporary-qpack.md) — deps: M4A-001-A — #433
302. [M4A-001-C — Load new worker before switching traffic](tasks/07_m4a_developer_preview/M4A-001-C-load-new-worker-before-switching-traffic.md) — deps: M4A-001-B — #434
303. [M4A-001-D — Drain old worker and surface compile/runtime errors](tasks/07_m4a_developer_preview/M4A-001-D-drain-old-worker-and-surface-compile-runtime-errors.md) — deps: M4A-001-C — #435
304. [M4A-001-V — Verify Implement actual-runtime `velqu dev` loop](tasks/07_m4a_developer_preview/M4A-001-V-verify-implement-actual-runtime-velqu-dev-loop.md) — deps: M4A-001-A, M4A-001-B, M4A-001-C, M4A-001-D — #436
305. [M4A-001-Z — Package evidence for Implement actual-runtime `velqu dev` loop](tasks/07_m4a_developer_preview/M4A-001-Z-package-evidence-for-implement-actual-runtime-velqu-dev-loop.md) — deps: M4A-001-V — #437
306. [M4A-002-A — Implement and document `velqu dev`, `build`, `inspect`, `contract diff`, `test`, `pack inspect/migrate`, and diagnostics](tasks/07_m4a_developer_preview/M4A-002-A-implement-and-document-velqu-dev-build-inspect-contract-diff-test-pack-inspect-m.md) — deps: M4A-001-Z, M26-GATE — #438
307. [M4A-002-B — Stable exit codes](tasks/07_m4a_developer_preview/M4A-002-B-stable-exit-codes.md) — deps: M4A-002-A — #439
308. [M4A-002-C — Machine-readable output option](tasks/07_m4a_developer_preview/M4A-002-C-machine-readable-output-option.md) — deps: M4A-002-B — #440
309. [M4A-002-D — Helpful actionable errors](tasks/07_m4a_developer_preview/M4A-002-D-helpful-actionable-errors.md) — deps: M4A-002-C — #441
310. [M4A-002-V — Verify Complete CLI command surface](tasks/07_m4a_developer_preview/M4A-002-V-verify-complete-cli-command-surface.md) — deps: M4A-002-A, M4A-002-B, M4A-002-C, M4A-002-D — #442
311. [M4A-002-Z — Package evidence for Complete CLI command surface](tasks/07_m4a_developer_preview/M4A-002-Z-package-evidence-for-complete-cli-command-surface.md) — deps: M4A-002-V — #443
312. [M4A-003-A — Starter API](tasks/07_m4a_developer_preview/M4A-003-A-starter-api.md) — deps: M4A-002-Z — #444
313. [M4A-003-B — Treaty client example](tasks/07_m4a_developer_preview/M4A-003-B-treaty-client-example.md) — deps: M4A-003-A — #445
314. [M4A-003-C — Testing setup](tasks/07_m4a_developer_preview/M4A-003-C-testing-setup.md) — deps: M4A-003-B — #446
315. [M4A-003-D — Optional fetch/profile choices](tasks/07_m4a_developer_preview/M4A-003-D-optional-fetch-profile-choices.md) — deps: M4A-003-C — #447
316. [M4A-003-V — Verify Implement project scaffolding](tasks/07_m4a_developer_preview/M4A-003-V-verify-implement-project-scaffolding.md) — deps: M4A-003-A, M4A-003-B, M4A-003-C, M4A-003-D — #448
317. [M4A-003-Z — Package evidence for Implement project scaffolding](tasks/07_m4a_developer_preview/M4A-003-Z-package-evidence-for-implement-project-scaffolding.md) — deps: M4A-003-V — #449
318. [M4A-004-A — Unit-local direct generated dispatcher](tasks/07_m4a_developer_preview/M4A-004-A-unit-local-direct-generated-dispatcher.md) — deps: M25-GATE, M4A-001-Z — #450
319. [M4A-004-B — Runtime-local actual Rust/QuickJS process](tasks/07_m4a_developer_preview/M4A-004-B-runtime-local-actual-rust-quickjs-process.md) — deps: M4A-004-A — #451
320. [M4A-004-C — Remote fetch client](tasks/07_m4a_developer_preview/M4A-004-C-remote-fetch-client.md) — deps: M4A-004-B — #452
321. [M4A-004-D — Exact method/body/query/status/problem typing](tasks/07_m4a_developer_preview/M4A-004-D-exact-method-body-query-status-problem-typing.md) — deps: M4A-004-C — #453
322. [M4A-004-V — Verify Complete Treaty unit-local, runtime-local, and remote modes](tasks/07_m4a_developer_preview/M4A-004-V-verify-complete-treaty-unit-local-runtime-local-and-remote-modes.md) — deps: M4A-004-A, M4A-004-B, M4A-004-C, M4A-004-D — #454
323. [M4A-004-Z — Package evidence for Complete Treaty unit-local, runtime-local, and remote modes](tasks/07_m4a_developer_preview/M4A-004-Z-package-evidence-for-complete-treaty-unit-local-runtime-local-and-remote-modes.md) — deps: M4A-004-V — #455
324. [M4A-005-A — Generate d.ts/client/OpenAPI/contract lock](tasks/07_m4a_developer_preview/M4A-005-A-generate-d-ts-client-openapi-contract-lock.md) — deps: M4A-004-Z — #456
325. [M4A-005-B — Tree-shakable client](tasks/07_m4a_developer_preview/M4A-005-B-tree-shakable-client.md) — deps: M4A-005-A — #457
326. [M4A-005-C — Version and public contract hash](tasks/07_m4a_developer_preview/M4A-005-C-version-and-public-contract-hash.md) — deps: M4A-005-B — #458
327. [M4A-005-D — Package verification](tasks/07_m4a_developer_preview/M4A-005-D-package-verification.md) — deps: M4A-005-C — #459
328. [M4A-005-V — Verify Publish compact contract and SDK artifacts](tasks/07_m4a_developer_preview/M4A-005-V-verify-publish-compact-contract-and-sdk-artifacts.md) — deps: M4A-005-A, M4A-005-B, M4A-005-C, M4A-005-D — #460
329. [M4A-005-Z — Package evidence for Publish compact contract and SDK artifacts](tasks/07_m4a_developer_preview/M4A-005-Z-package-evidence-for-publish-compact-contract-and-sdk-artifacts.md) — deps: M4A-005-V — #461
330. [M4A-006-A — Structured diagnostic codes](tasks/07_m4a_developer_preview/M4A-006-A-structured-diagnostic-codes.md) — deps: M4A-001-Z, M4A-002-Z — #462
331. [M4A-006-B — Source-map-aware stacks](tasks/07_m4a_developer_preview/M4A-006-B-source-map-aware-stacks.md) — deps: M4A-006-A — #463
332. [M4A-006-C — Redaction policy](tasks/07_m4a_developer_preview/M4A-006-C-redaction-policy.md) — deps: M4A-006-B — #464
333. [M4A-006-D — Inspect route plan, fields, codecs, capabilities, crossings, and debug names](tasks/07_m4a_developer_preview/M4A-006-D-inspect-route-plan-fields-codecs-capabilities-crossings-and-debug-names.md) — deps: M4A-006-C — #465
334. [M4A-006-V — Verify Finalize diagnostics, source maps, and inspect output](tasks/07_m4a_developer_preview/M4A-006-V-verify-finalize-diagnostics-source-maps-and-inspect-output.md) — deps: M4A-006-A, M4A-006-B, M4A-006-C, M4A-006-D — #466
335. [M4A-006-Z — Package evidence for Finalize diagnostics, source maps, and inspect output](tasks/07_m4a_developer_preview/M4A-006-Z-package-evidence-for-finalize-diagnostics-source-maps-and-inspect-output.md) — deps: M4A-006-V — #467
336. [M4A-007-A — Define deferred owner, queue, deadline, cancellation, shutdown](tasks/07_m4a_developer_preview/M4A-007-A-define-deferred-owner-queue-deadline-cancellation-shutdown.md) — deps: M27-GATE, M3-GATE — #468
337. [M4A-007-B — Separate cleanup from best-effort work](tasks/07_m4a_developer_preview/M4A-007-B-separate-cleanup-from-best-effort-work.md) — deps: M4A-007-A — #469
338. [M4A-007-C — Expose metrics](tasks/07_m4a_developer_preview/M4A-007-C-expose-metrics.md) — deps: M4A-007-B — #470
339. [M4A-007-D — Forbid unbounded recursive spawning](tasks/07_m4a_developer_preview/M4A-007-D-forbid-unbounded-recursive-spawning.md) — deps: M4A-007-C — #471
340. [M4A-007-V — Verify Implement bounded `defer` and lifecycle hooks](tasks/07_m4a_developer_preview/M4A-007-V-verify-implement-bounded-defer-and-lifecycle-hooks.md) — deps: M4A-007-A, M4A-007-B, M4A-007-C, M4A-007-D — #472
341. [M4A-007-Z — Package evidence for Implement bounded `defer` and lifecycle hooks](tasks/07_m4a_developer_preview/M4A-007-Z-package-evidence-for-implement-bounded-defer-and-lifecycle-hooks.md) — deps: M4A-007-V — #473
342. [M4A-008-A — Quickstart](tasks/07_m4a_developer_preview/M4A-008-A-quickstart.md) — deps: M4A-002-Z, M4A-004-Z, M4A-006-Z — #474
343. [M4A-008-B — Routes/schemas/policies/services](tasks/07_m4a_developer_preview/M4A-008-B-routes-schemas-policies-services.md) — deps: M4A-008-A — #475
344. [M4A-008-C — Treaty](tasks/07_m4a_developer_preview/M4A-008-C-treaty.md) — deps: M4A-008-B — #476
345. [M4A-008-D — Fetch/capabilities](tasks/07_m4a_developer_preview/M4A-008-D-fetch-capabilities.md) — deps: M4A-008-C — #477
346. [M4A-008-E — Runtime profiles](tasks/07_m4a_developer_preview/M4A-008-E-runtime-profiles.md) — deps: M4A-008-D — #478
347. [M4A-008-F — Deployment behind reverse proxy](tasks/07_m4a_developer_preview/M4A-008-F-deployment-behind-reverse-proxy.md) — deps: M4A-008-E — #479
348. [M4A-008-G — Limits and non-goals](tasks/07_m4a_developer_preview/M4A-008-G-limits-and-non-goals.md) — deps: M4A-008-F — #480
349. [M4A-008-V — Verify Build documentation and examples](tasks/07_m4a_developer_preview/M4A-008-V-verify-build-documentation-and-examples.md) — deps: M4A-008-A, M4A-008-B, M4A-008-C, M4A-008-D, M4A-008-E, M4A-008-F, M4A-008-G — #481
350. [M4A-008-Z — Package evidence for Build documentation and examples](tasks/07_m4a_developer_preview/M4A-008-Z-package-evidence-for-build-documentation-and-examples.md) — deps: M4A-008-V — #482
351. [M4A-009-A — Feature modules](tasks/07_m4a_developer_preview/M4A-009-A-feature-modules.md) — deps: M4A-004-Z, M4A-007-Z, M28-GATE — #483
352. [M4A-009-B — JWT-like policy reference](tasks/07_m4a_developer_preview/M4A-009-B-jwt-like-policy-reference.md) — deps: M4A-009-A — #484
353. [M4A-009-C — Controlled upstream](tasks/07_m4a_developer_preview/M4A-009-C-controlled-upstream.md) — deps: M4A-009-B — #485
354. [M4A-009-D — Metrics/readiness/shutdown](tasks/07_m4a_developer_preview/M4A-009-D-metrics-readiness-shutdown.md) — deps: M4A-009-C — #486
355. [M4A-009-E — Treaty client](tasks/07_m4a_developer_preview/M4A-009-E-treaty-client.md) — deps: M4A-009-D — #487
356. [M4A-009-V — Verify Build realistic private-alpha proof service](tasks/07_m4a_developer_preview/M4A-009-V-verify-build-realistic-private-alpha-proof-service.md) — deps: M4A-009-A, M4A-009-B, M4A-009-C, M4A-009-D, M4A-009-E — #488
357. [M4A-009-Z — Package evidence for Build realistic private-alpha proof service](tasks/07_m4a_developer_preview/M4A-009-Z-package-evidence-for-build-realistic-private-alpha-proof-service.md) — deps: M4A-009-V — #489
358. [M4A-010-A — Provide clean install packet](tasks/07_m4a_developer_preview/M4A-010-A-provide-clean-install-packet.md) — deps: M4A-003-Z, M4A-008-Z, M4A-009-Z — #490
359. [M4A-010-B — Collect task-based feedback](tasks/07_m4a_developer_preview/M4A-010-B-collect-task-based-feedback.md) — deps: M4A-010-A — #491
360. [M4A-010-C — Classify P0/P1/P2](tasks/07_m4a_developer_preview/M4A-010-C-classify-p0-p1-p2.md) — deps: M4A-010-B — #492
361. [M4A-010-D — Fix beta-blocking findings and publish limitations](tasks/07_m4a_developer_preview/M4A-010-D-fix-beta-blocking-findings-and-publish-limitations.md) — deps: M4A-010-C — #493
362. [M4A-010-V — Verify Run invited developer alpha and close P0/P1 feedback](tasks/07_m4a_developer_preview/M4A-010-V-verify-run-invited-developer-alpha-and-close-p0-p1-feedback.md) — deps: M4A-010-A, M4A-010-B, M4A-010-C, M4A-010-D — #494
363. [M4A-010-Z — Package evidence for Run invited developer alpha and close P0/P1 feedback](tasks/07_m4a_developer_preview/M4A-010-Z-package-evidence-for-run-invited-developer-alpha-and-close-p0-p1-feedback.md) — deps: M4A-010-V — #495
364. [M4A-GATE — M4A — Developer Preview and Private Alpha exit gate](gates/M4A-GATE.md) — deps: M4A-001-Z, M4A-002-Z, M4A-003-Z, M4A-004-Z, M4A-005-Z, M4A-006-Z, M4A-007-Z, M4A-008-Z, M4A-009-Z, M4A-010-Z — #633
365. [BETA-001-A — Add Postgres compose, seed/reset, controlled upstream, result schema, load generator, and report generator](tasks/08_public_beta/BETA-001-A-add-postgres-compose-seed-reset-controlled-upstream-result-schema-load-generator.md) — deps: G0-GATE — #496
366. [BETA-001-B — Pin candidate versions](tasks/08_public_beta/BETA-001-B-pin-candidate-versions.md) — deps: BETA-001-A — #497
367. [BETA-001-C — Define fairness checks](tasks/08_public_beta/BETA-001-C-define-fairness-checks.md) — deps: BETA-001-B — #498
368. [BETA-001-D — Keep raw samples](tasks/08_public_beta/BETA-001-D-keep-raw-samples.md) — deps: BETA-001-C — #499
369. [BETA-001-V — Verify Make the real-world benchmark harness executable](tasks/08_public_beta/BETA-001-V-verify-make-the-real-world-benchmark-harness-executable.md) — deps: BETA-001-A, BETA-001-B, BETA-001-C, BETA-001-D — #500
370. [BETA-001-Z — Package evidence for Make the real-world benchmark harness executable](tasks/08_public_beta/BETA-001-Z-package-evidence-for-make-the-real-world-benchmark-harness-executable.md) — deps: BETA-001-V — #503
371. [BETA-002-A — Match SQL, pool, JWT, timeouts, logging, responses, compression, and deployment limits](tasks/08_public_beta/BETA-002-A-match-sql-pool-jwt-timeouts-logging-responses-compression-and-deployment-limits.md) — deps: BETA-001-Z — #504
372. [BETA-002-B — Pin versions](tasks/08_public_beta/BETA-002-B-pin-versions.md) — deps: BETA-002-A — #505
373. [BETA-002-C — Add contract-response verification](tasks/08_public_beta/BETA-002-C-add-contract-response-verification.md) — deps: BETA-002-B — #506
374. [BETA-002-D — Document unavoidable differences](tasks/08_public_beta/BETA-002-D-document-unavoidable-differences.md) — deps: BETA-002-C — #507
375. [BETA-002-V — Verify Implement matched competitor candidates](tasks/08_public_beta/BETA-002-V-verify-implement-matched-competitor-candidates.md) — deps: BETA-002-A, BETA-002-B, BETA-002-C, BETA-002-D — #508
376. [BETA-002-Z — Package evidence for Implement matched competitor candidates](tasks/08_public_beta/BETA-002-Z-package-evidence-for-implement-matched-competitor-candidates.md) — deps: BETA-002-V — #509
377. [BETA-003-A — Run 0/1/5/10/25ms I/O, payload matrices, and CPU operation levels](tasks/08_public_beta/BETA-003-A-run-0-1-5-10-25ms-i-o-payload-matrices-and-cpu-operation-levels.md) — deps: BETA-001-Z, M28-GATE, M3-GATE — #510
378. [BETA-003-B — Measure first request through steady state](tasks/08_public_beta/BETA-003-B-measure-first-request-through-steady-state.md) — deps: BETA-003-A — #511
379. [BETA-003-C — Calculate cumulative crossover request counts](tasks/08_public_beta/BETA-003-C-calculate-cumulative-crossover-request-counts.md) — deps: BETA-003-B — #512
380. [BETA-003-D — Report losses honestly](tasks/08_public_beta/BETA-003-D-report-losses-honestly.md) — deps: BETA-003-C — #513
381. [BETA-003-V — Verify Run controlled I/O and CPU/JIT crossover suites](tasks/08_public_beta/BETA-003-V-verify-run-controlled-i-o-and-cpu-jit-crossover-suites.md) — deps: BETA-003-A, BETA-003-B, BETA-003-C, BETA-003-D — #514
382. [BETA-003-Z — Package evidence for Run controlled I/O and CPU/JIT crossover suites](tasks/08_public_beta/BETA-003-Z-package-evidence-for-run-controlled-i-o-and-cpu-jit-crossover-suites.md) — deps: BETA-003-V — #515
383. [BETA-004-A — Use capability ABI](tasks/08_public_beta/BETA-004-A-use-capability-abi.md) — deps: M27-GATE, BETA-001-Z — #516
384. [BETA-004-B — Lazy pool](tasks/08_public_beta/BETA-004-B-lazy-pool.md) — deps: BETA-004-A — #517
385. [BETA-004-C — Parameterized queries/transactions](tasks/08_public_beta/BETA-004-C-parameterized-queries-transactions.md) — deps: BETA-004-B — #518
386. [BETA-004-D — Deadline/cancellation/shutdown](tasks/08_public_beta/BETA-004-D-deadline-cancellation-shutdown.md) — deps: BETA-004-C — #519
387. [BETA-004-E — Pool limits and observability](tasks/08_public_beta/BETA-004-E-pool-limits-and-observability.md) — deps: BETA-004-D — #520
388. [BETA-004-F — No ORM](tasks/08_public_beta/BETA-004-F-no-orm.md) — deps: BETA-004-E — #521
389. [BETA-004-V — Verify Implement optional first-party Postgres capability](tasks/08_public_beta/BETA-004-V-verify-implement-optional-first-party-postgres-capability.md) — deps: BETA-004-A, BETA-004-B, BETA-004-C, BETA-004-D, BETA-004-E, BETA-004-F — #522
390. [BETA-004-Z — Package evidence for Implement optional first-party Postgres capability](tasks/08_public_beta/BETA-004-Z-package-evidence-for-implement-optional-first-party-postgres-capability.md) — deps: BETA-004-V — #523
391. [BETA-005-A — Support one approved JWT algorithm/profile](tasks/08_public_beta/BETA-005-A-support-one-approved-jwt-algorithm-profile.md) — deps: M27-GATE, M25-GATE — #524
392. [BETA-005-B — Key loading/rotation hooks](tasks/08_public_beta/BETA-005-B-key-loading-rotation-hooks.md) — deps: BETA-005-A — #525
393. [BETA-005-C — Expiry/audience/issuer checks](tasks/08_public_beta/BETA-005-C-expiry-audience-issuer-checks.md) — deps: BETA-005-B — #526
394. [BETA-005-D — Typed 401/403 problems](tasks/08_public_beta/BETA-005-D-typed-401-403-problems.md) — deps: BETA-005-C — #527
395. [BETA-005-E — No secret logging](tasks/08_public_beta/BETA-005-E-no-secret-logging.md) — deps: BETA-005-D — #528
396. [BETA-005-V — Verify Implement JWT/auth reference package](tasks/08_public_beta/BETA-005-V-verify-implement-jwt-auth-reference-package.md) — deps: BETA-005-A, BETA-005-B, BETA-005-C, BETA-005-D, BETA-005-E — #529
397. [BETA-005-Z — Package evidence for Implement JWT/auth reference package](tasks/08_public_beta/BETA-005-Z-package-evidence-for-implement-jwt-auth-reference-package.md) — deps: BETA-005-V — #530
398. [BETA-006-A — Request/route/status/duration](tasks/08_public_beta/BETA-006-A-request-route-status-duration.md) — deps: M3-GATE, M28-GATE — #531
399. [BETA-006-B — Worker queues/quarantine/replacements](tasks/08_public_beta/BETA-006-B-worker-queues-quarantine-replacements.md) — deps: BETA-006-A — #532
400. [BETA-006-C — Fetch and DB pools](tasks/08_public_beta/BETA-006-C-fetch-and-db-pools.md) — deps: BETA-006-B — #533
401. [BETA-006-D — Memory/tasks/slots](tasks/08_public_beta/BETA-006-D-memory-tasks-slots.md) — deps: BETA-006-C — #534
402. [BETA-006-E — Optional trace integration or trace IDs](tasks/08_public_beta/BETA-006-E-optional-trace-integration-or-trace-ids.md) — deps: BETA-006-D — #535
403. [BETA-006-F — Redaction](tasks/08_public_beta/BETA-006-F-redaction.md) — deps: BETA-006-E — #536
404. [BETA-006-V — Verify Implement beta observability baseline](tasks/08_public_beta/BETA-006-V-verify-implement-beta-observability-baseline.md) — deps: BETA-006-A, BETA-006-B, BETA-006-C, BETA-006-D, BETA-006-E, BETA-006-F — #537
405. [BETA-006-Z — Package evidence for Implement beta observability baseline](tasks/08_public_beta/BETA-006-Z-package-evidence-for-implement-beta-observability-baseline.md) — deps: BETA-006-V — #538
406. [BETA-007-A — Environment/file configuration](tasks/08_public_beta/BETA-007-A-environment-file-configuration.md) — deps: M27-GATE — #539
407. [BETA-007-B — Validation at startup](tasks/08_public_beta/BETA-007-B-validation-at-startup.md) — deps: BETA-007-A — #540
408. [BETA-007-C — Secret value wrapper/redaction](tasks/08_public_beta/BETA-007-C-secret-value-wrapper-redaction.md) — deps: BETA-007-B — #541
409. [BETA-007-D — Profile-specific settings](tasks/08_public_beta/BETA-007-D-profile-specific-settings.md) — deps: BETA-007-C — #542
410. [BETA-007-E — No dynamic code execution](tasks/08_public_beta/BETA-007-E-no-dynamic-code-execution.md) — deps: BETA-007-D — #543
411. [BETA-007-V — Verify Implement configuration and secret handling](tasks/08_public_beta/BETA-007-V-verify-implement-configuration-and-secret-handling.md) — deps: BETA-007-A, BETA-007-B, BETA-007-C, BETA-007-D, BETA-007-E — #544
412. [BETA-007-Z — Package evidence for Implement configuration and secret handling](tasks/08_public_beta/BETA-007-Z-package-evidence-for-implement-configuration-and-secret-handling.md) — deps: BETA-007-V — #545
413. [BETA-008-A — Trusted proxy configuration](tasks/08_public_beta/BETA-008-A-trusted-proxy-configuration.md) — deps: M3-GATE, BETA-006-Z — #546
414. [BETA-008-B — Forwarded header policy](tasks/08_public_beta/BETA-008-B-forwarded-header-policy.md) — deps: BETA-008-A — #547
415. [BETA-008-C — Liveness/readiness/startup endpoints](tasks/08_public_beta/BETA-008-C-liveness-readiness-startup-endpoints.md) — deps: BETA-008-B — #548
416. [BETA-008-D — Graceful drain and termination](tasks/08_public_beta/BETA-008-D-graceful-drain-and-termination.md) — deps: BETA-008-C — #549
417. [BETA-008-E — Container example](tasks/08_public_beta/BETA-008-E-container-example.md) — deps: BETA-008-D — #550
418. [BETA-008-V — Verify Implement reverse-proxy, drain, and deployment semantics](tasks/08_public_beta/BETA-008-V-verify-implement-reverse-proxy-drain-and-deployment-semantics.md) — deps: BETA-008-A, BETA-008-B, BETA-008-C, BETA-008-D, BETA-008-E — #551
419. [BETA-008-Z — Package evidence for Implement reverse-proxy, drain, and deployment semantics](tasks/08_public_beta/BETA-008-Z-package-evidence-for-implement-reverse-proxy-drain-and-deployment-semantics.md) — deps: BETA-008-V — #552
420. [BETA-009-A — Run fuzz suites for pack/router/schema/bridge/HTTP](tasks/08_public_beta/BETA-009-A-run-fuzz-suites-for-pack-router-schema-bridge-http.md) — deps: M28-GATE, M3-GATE, BETA-004-Z, BETA-005-Z, BETA-007-Z — #553
421. [BETA-009-B — Dependency vulnerability and license scan](tasks/08_public_beta/BETA-009-B-dependency-vulnerability-and-license-scan.md) — deps: BETA-009-A — #554
422. [BETA-009-C — Threat-model review](tasks/08_public_beta/BETA-009-C-threat-model-review.md) — deps: BETA-009-B — #555
423. [BETA-009-D — Chaos tests for upstream/DB/worker poison](tasks/08_public_beta/BETA-009-D-chaos-tests-for-upstream-db-worker-poison.md) — deps: BETA-009-C — #556
424. [BETA-009-E — No known critical/high exploitable issue](tasks/08_public_beta/BETA-009-E-no-known-critical-high-exploitable-issue.md) — deps: BETA-009-D — #557
425. [BETA-009-V — Verify Run beta security and reliability baseline](tasks/08_public_beta/BETA-009-V-verify-run-beta-security-and-reliability-baseline.md) — deps: BETA-009-A, BETA-009-B, BETA-009-C, BETA-009-D, BETA-009-E — #558
426. [BETA-009-Z — Package evidence for Run beta security and reliability baseline](tasks/08_public_beta/BETA-009-Z-package-evidence-for-run-beta-security-and-reliability-baseline.md) — deps: BETA-009-V — #559
427. [BETA-010-A — Linux x86_64 glibc mandatory working assumption](tasks/08_public_beta/BETA-010-A-linux-x86-64-glibc-mandatory-working-assumption.md) — deps: M26-GATE, M4A-002-Z — #560
428. [BETA-010-B — Linux ARM64 glibc when CI is available](tasks/08_public_beta/BETA-010-B-linux-arm64-glibc-when-ci-is-available.md) — deps: BETA-010-A — #561
429. [BETA-010-C — npm packages under beta tag](tasks/08_public_beta/BETA-010-C-npm-packages-under-beta-tag.md) — deps: BETA-010-B — #562
430. [BETA-010-D — Runtime binary/QPack tools](tasks/08_public_beta/BETA-010-D-runtime-binary-qpack-tools.md) — deps: BETA-010-C — #563
431. [BETA-010-E — Clean install tests](tasks/08_public_beta/BETA-010-E-clean-install-tests.md) — deps: BETA-010-D — #564
432. [BETA-010-V — Verify Create supported beta platform and packaging matrix](tasks/08_public_beta/BETA-010-V-verify-create-supported-beta-platform-and-packaging-matrix.md) — deps: BETA-010-A, BETA-010-B, BETA-010-C, BETA-010-D, BETA-010-E — #565
433. [BETA-010-Z — Package evidence for Create supported beta platform and packaging matrix](tasks/08_public_beta/BETA-010-Z-package-evidence-for-create-supported-beta-platform-and-packaging-matrix.md) — deps: BETA-010-V — #566
434. [BETA-011-A — Use SemVer prerelease](tasks/08_public_beta/BETA-011-A-use-semver-prerelease.md) — deps: M4A-GATE, BETA-010-Z — #567
435. [BETA-011-B — Publish `next`/beta tag](tasks/08_public_beta/BETA-011-B-publish-next-beta-tag.md) — deps: BETA-011-A — #568
436. [BETA-011-C — Generate changelog and migration notes](tasks/08_public_beta/BETA-011-C-generate-changelog-and-migration-notes.md) — deps: BETA-011-B — #569
437. [BETA-011-D — Create GitHub-style release packet](tasks/08_public_beta/BETA-011-D-create-github-style-release-packet.md) — deps: BETA-011-C — #570
438. [BETA-011-E — Support yanking/rollback](tasks/08_public_beta/BETA-011-E-support-yanking-rollback.md) — deps: BETA-011-D — #571
439. [BETA-011-V — Verify Automate beta publishing and versioning](tasks/08_public_beta/BETA-011-V-verify-automate-beta-publishing-and-versioning.md) — deps: BETA-011-A, BETA-011-B, BETA-011-C, BETA-011-D, BETA-011-E — #572
440. [BETA-011-Z — Package evidence for Automate beta publishing and versioning](tasks/08_public_beta/BETA-011-Z-package-evidence-for-automate-beta-publishing-and-versioning.md) — deps: BETA-011-V — #573
441. [BETA-012-A — Installation](tasks/08_public_beta/BETA-012-A-installation.md) — deps: M4A-GATE, BETA-004-Z, BETA-005-Z, BETA-008-Z — #574
442. [BETA-012-B — Quickstart](tasks/08_public_beta/BETA-012-B-quickstart.md) — deps: BETA-012-A — #575
443. [BETA-012-C — Architecture](tasks/08_public_beta/BETA-012-C-architecture.md) — deps: BETA-012-B — #576
444. [BETA-012-D — Contracts/Treaty](tasks/08_public_beta/BETA-012-D-contracts-treaty.md) — deps: BETA-012-C — #577
445. [BETA-012-E — Fetch/Postgres/auth](tasks/08_public_beta/BETA-012-E-fetch-postgres-auth.md) — deps: BETA-012-D — #578
446. [BETA-012-F — Deployment](tasks/08_public_beta/BETA-012-F-deployment.md) — deps: BETA-012-E — #579
447. [BETA-012-G — Troubleshooting](tasks/08_public_beta/BETA-012-G-troubleshooting.md) — deps: BETA-012-F — #580
448. [BETA-012-H — Performance methodology](tasks/08_public_beta/BETA-012-H-performance-methodology.md) — deps: BETA-012-G — #581
449. [BETA-012-I — Limitations/non-goals](tasks/08_public_beta/BETA-012-I-limitations-non-goals.md) — deps: BETA-012-H — #582
450. [BETA-012-V — Verify Complete beta documentation and limitations](tasks/08_public_beta/BETA-012-V-verify-complete-beta-documentation-and-limitations.md) — deps: BETA-012-A, BETA-012-B, BETA-012-C, BETA-012-D, BETA-012-E, BETA-012-F, BETA-012-G, BETA-012-H, BETA-012-I — #583
451. [BETA-012-Z — Package evidence for Complete beta documentation and limitations](tasks/08_public_beta/BETA-012-Z-package-evidence-for-complete-beta-documentation-and-limitations.md) — deps: BETA-012-V — #584
452. [BETA-013-A — Run at least two-hour mixed workload and at least one million requests on reference platform](tasks/08_public_beta/BETA-013-A-run-at-least-two-hour-mixed-workload-and-at-least-one-million-requests-on-refere.md) — deps: BETA-004-Z, BETA-005-Z, BETA-006-Z, BETA-008-Z, BETA-009-Z — #585
453. [BETA-013-B — Include fetch, DB, auth, timeouts, cancellation, worker replacement, and reload](tasks/08_public_beta/BETA-013-B-include-fetch-db-auth-timeouts-cancellation-worker-replacement-and-reload.md) — deps: BETA-013-A — #586
454. [BETA-013-C — Track RSS, heap, slots, tasks, queues, pools, and errors](tasks/08_public_beta/BETA-013-C-track-rss-heap-slots-tasks-queues-pools-and-errors.md) — deps: BETA-013-B — #587
455. [BETA-013-D — Analyze retained growth](tasks/08_public_beta/BETA-013-D-analyze-retained-growth.md) — deps: BETA-013-C — #588
456. [BETA-013-V — Verify Run beta soak and leak qualification](tasks/08_public_beta/BETA-013-V-verify-run-beta-soak-and-leak-qualification.md) — deps: BETA-013-A, BETA-013-B, BETA-013-C, BETA-013-D — #589
457. [BETA-013-Z — Package evidence for Run beta soak and leak qualification](tasks/08_public_beta/BETA-013-Z-package-evidence-for-run-beta-soak-and-leak-qualification.md) — deps: BETA-013-V — #590
458. [BETA-014-A — Include cold start categories, warm microbenchmarks, real DB/auth/I/O, CPU/JIT crossover, cost-normalized metrics, and limitations](tasks/08_public_beta/BETA-014-A-include-cold-start-categories-warm-microbenchmarks-real-db-auth-i-o-cpu-jit-cros.md) — deps: BETA-002-Z, BETA-003-Z, BETA-004-Z, BETA-005-Z, BETA-013-Z — #591
459. [BETA-014-B — Pin all candidates/artifacts](tasks/08_public_beta/BETA-014-B-pin-all-candidates-artifacts.md) — deps: BETA-014-A — #592
460. [BETA-014-C — Retain raw data](tasks/08_public_beta/BETA-014-C-retain-raw-data.md) — deps: BETA-014-B — #593
461. [BETA-014-D — Have wording reviewed](tasks/08_public_beta/BETA-014-D-have-wording-reviewed.md) — deps: BETA-014-C — #594
462. [BETA-014-V — Verify Publish canonical beta benchmark report](tasks/08_public_beta/BETA-014-V-verify-publish-canonical-beta-benchmark-report.md) — deps: BETA-014-A, BETA-014-B, BETA-014-C, BETA-014-D — #595
463. [BETA-014-Z — Package evidence for Publish canonical beta benchmark report](tasks/08_public_beta/BETA-014-Z-package-evidence-for-publish-canonical-beta-benchmark-report.md) — deps: BETA-014-V — #596
464. [BETA-015-A — Source ZIP](tasks/08_public_beta/BETA-015-A-source-zip.md) — deps: BETA-009-Z, BETA-010-Z, BETA-011-Z, BETA-013-Z, BETA-014-Z — #597
465. [BETA-015-B — Git bundle](tasks/08_public_beta/BETA-015-B-git-bundle.md) — deps: BETA-015-A — #598
466. [BETA-015-C — Linux binaries](tasks/08_public_beta/BETA-015-C-linux-binaries.md) — deps: BETA-015-B — #599
467. [BETA-015-D — npm package tarballs](tasks/08_public_beta/BETA-015-D-npm-package-tarballs.md) — deps: BETA-015-C — #600
468. [BETA-015-E — QPack tools](tasks/08_public_beta/BETA-015-E-qpack-tools.md) — deps: BETA-015-D — #601
469. [BETA-015-F — SBOM](tasks/08_public_beta/BETA-015-F-sbom.md) — deps: BETA-015-E — #602
470. [BETA-015-G — Checksums](tasks/08_public_beta/BETA-015-G-checksums.md) — deps: BETA-015-F — #603
471. [BETA-015-H — Review/evidence indexes](tasks/08_public_beta/BETA-015-H-review-evidence-indexes.md) — deps: BETA-015-G — #604
472. [BETA-015-I — Known limitations](tasks/08_public_beta/BETA-015-I-known-limitations.md) — deps: BETA-015-H — #605
473. [BETA-015-V — Verify Generate beta release evidence, SBOM, and checksums](tasks/08_public_beta/BETA-015-V-verify-generate-beta-release-evidence-sbom-and-checksums.md) — deps: BETA-015-A, BETA-015-B, BETA-015-C, BETA-015-D, BETA-015-E, BETA-015-F, BETA-015-G, BETA-015-H, BETA-015-I — #606
474. [BETA-015-Z — Package evidence for Generate beta release evidence, SBOM, and checksums](tasks/08_public_beta/BETA-015-Z-package-evidence-for-generate-beta-release-evidence-sbom-and-checksums.md) — deps: BETA-015-V — #607
475. [BETA-016-A — Fresh Linux VM/container](tasks/08_public_beta/BETA-016-A-fresh-linux-vm-container.md) — deps: BETA-011-Z, BETA-012-Z, BETA-015-Z — #608
476. [BETA-016-B — Install CLI/runtime](tasks/08_public_beta/BETA-016-B-install-cli-runtime.md) — deps: BETA-016-A — #609
477. [BETA-016-C — Scaffold app](tasks/08_public_beta/BETA-016-C-scaffold-app.md) — deps: BETA-016-B — #610
478. [BETA-016-D — Run tests/dev/build](tasks/08_public_beta/BETA-016-D-run-tests-dev-build.md) — deps: BETA-016-C — #611
479. [BETA-016-E — Deploy proof service](tasks/08_public_beta/BETA-016-E-deploy-proof-service.md) — deps: BETA-016-D — #612
480. [BETA-016-F — Use Treaty client](tasks/08_public_beta/BETA-016-F-use-treaty-client.md) — deps: BETA-016-E — #613
481. [BETA-016-V — Verify Run external clean-install and tutorial verification](tasks/08_public_beta/BETA-016-V-verify-run-external-clean-install-and-tutorial-verification.md) — deps: BETA-016-A, BETA-016-B, BETA-016-C, BETA-016-D, BETA-016-E, BETA-016-F — #614
482. [BETA-016-Z — Package evidence for Run external clean-install and tutorial verification](tasks/08_public_beta/BETA-016-Z-package-evidence-for-run-external-clean-install-and-tutorial-verification.md) — deps: BETA-016-V — #615
483. [BETA-GATE — Public Beta Readiness and Release exit gate](gates/BETA-GATE.md) — deps: BETA-001-Z, BETA-002-Z, BETA-003-Z, BETA-004-Z, BETA-005-Z, BETA-006-Z, BETA-007-Z, BETA-008-Z, BETA-009-Z, BETA-010-Z, BETA-011-Z, BETA-012-Z, BETA-013-Z, BETA-014-Z, BETA-015-Z, BETA-016-Z, BETA-017-Z — #625
