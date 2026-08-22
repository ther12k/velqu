# Next 25 Dependency-Safe Tasks

The next dependency-ready implementation task is M25-005-C; this short queue lists the first 25 unchecked packets with their issue numbers.

1. [M25-005-C — Handle optional/null/union fields](tasks/02_m25_schema_codecs/M25-005-C-handle-optional-null-union-fields.md) — deps: M25-005-B — #146
2. [M25-005-D — Keep QuickJS stringify or generic fallback when measured better](tasks/02_m25_schema_codecs/M25-005-D-keep-quickjs-stringify-or-generic-fallback-when-measured-better.md) — deps: M25-005-C — #147
3. [M25-005-V — Verify Generate status-specific response encoders](tasks/02_m25_schema_codecs/M25-005-V-verify-generate-status-specific-response-encoders.md) — deps: M25-005-A, M25-005-B, M25-005-C, M25-005-D — #148
4. [M25-005-Z — Package evidence for Generate status-specific response encoders](tasks/02_m25_schema_codecs/M25-005-Z-package-evidence-for-generate-status-specific-response-encoders.md) — deps: M25-005-V — #149
5. [M25-006-A — Generate problem type/status/title/detail/custom-field encoders](tasks/02_m25_schema_codecs/M25-006-A-generate-problem-type-status-title-detail-custom-field-encoders.md) — deps: M25-001-Z, M25-005-Z — #150
6. [M25-006-B — Redact unexpected failures](tasks/02_m25_schema_codecs/M25-006-B-redact-unexpected-failures.md) — deps: M25-006-A — #151
7. [M25-006-C — Ensure policy-provided errors flow into Treaty unions](tasks/02_m25_schema_codecs/M25-006-C-ensure-policy-provided-errors-flow-into-treaty-unions.md) — deps: M25-006-B — #152
8. [M25-006-D — Include content type and instance behavior](tasks/02_m25_schema_codecs/M25-006-D-include-content-type-and-instance-behavior.md) — deps: M25-006-C — #153
9. [M25-006-V — Verify Generate RFC 9457 problem encoders](tasks/02_m25_schema_codecs/M25-006-V-verify-generate-rfc-9457-problem-encoders.md) — deps: M25-006-A, M25-006-B, M25-006-C, M25-006-D — #154
10. [M25-006-Z — Package evidence for Generate RFC 9457 problem encoders](tasks/02_m25_schema_codecs/M25-006-Z-package-evidence-for-generate-rfc-9457-problem-encoders.md) — deps: M25-006-V — #155
11. [M25-007-A — Tag fallback reason in RoutePlan](tasks/02_m25_schema_codecs/M25-007-A-tag-fallback-reason-in-routeplan.md) — deps: M25-003-Z, M25-004-Z, M25-005-Z — #156
12. [M25-007-B — Support raw Response/full Request escape hatches](tasks/02_m25_schema_codecs/M25-007-B-support-raw-response-full-request-escape-hatches.md) — deps: M25-007-A — #157
13. [M25-007-C — Keep fallback bounded and deadline-aware](tasks/02_m25_schema_codecs/M25-007-C-keep-fallback-bounded-and-deadline-aware.md) — deps: M25-007-B — #158
14. [M25-007-D — Expose bridge crossings and codec choice in `velqu inspect`](tasks/02_m25_schema_codecs/M25-007-D-expose-bridge-crossings-and-codec-choice-in-velqu-inspect.md) — deps: M25-007-C — #159
15. [M25-007-V — Verify Implement explicit generic and Web fallback paths](tasks/02_m25_schema_codecs/M25-007-V-verify-implement-explicit-generic-and-web-fallback-paths.md) — deps: M25-007-A, M25-007-B, M25-007-C, M25-007-D — #160
16. [M25-007-Z — Package evidence for Implement explicit generic and Web fallback paths](tasks/02_m25_schema_codecs/M25-007-Z-package-evidence-for-implement-explicit-generic-and-web-fallback-paths.md) — deps: M25-007-V — #161
17. [M25-008-A — Generate all projections from canonical IR](tasks/02_m25_schema_codecs/M25-008-A-generate-all-projections-from-canonical-ir.md) — deps: M25-001-Z, M25-003-Z, M25-004-Z, M25-005-Z, M25-006-Z — #162
18. [M25-008-B — Add parity checks to verification](tasks/02_m25_schema_codecs/M25-008-B-add-parity-checks-to-verification.md) — deps: M25-008-A — #163
19. [M25-008-C — Publish compact contract metadata](tasks/02_m25_schema_codecs/M25-008-C-publish-compact-contract-metadata.md) — deps: M25-008-B — #164
20. [M25-008-D — Update semantic diff to Schema IR v2](tasks/02_m25_schema_codecs/M25-008-D-update-semantic-diff-to-schema-ir-v2.md) — deps: M25-008-C — #165
21. [M25-008-V — Verify Unify Treaty, OpenAPI, lock, and runtime schema projection](tasks/02_m25_schema_codecs/M25-008-V-verify-unify-treaty-openapi-lock-and-runtime-schema-projection.md) — deps: M25-008-A, M25-008-B, M25-008-C, M25-008-D — #166
22. [M25-008-Z — Package evidence for Unify Treaty, OpenAPI, lock, and runtime schema projection](tasks/02_m25_schema_codecs/M25-008-Z-package-evidence-for-unify-treaty-openapi-lock-and-runtime-schema-projection.md) — deps: M25-008-V — #167
23. [M25-009-A — Fuzz encoded/decoded values](tasks/02_m25_schema_codecs/M25-009-A-fuzz-encoded-decoded-values.md) — deps: M25-003-Z, M25-004-Z, M25-005-Z, M25-006-Z — #168
24. [M25-009-B — Compare generated output with standards/reference JSON behavior](tasks/02_m25_schema_codecs/M25-009-B-compare-generated-output-with-standards-reference-json-behavior.md) — deps: M25-009-A — #169
25. [M25-009-C — Run malformed and boundary values](tasks/02_m25_schema_codecs/M25-009-C-run-malformed-and-boundary-values.md) — deps: M25-009-B — #170
