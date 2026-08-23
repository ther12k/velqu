# Next 25 Dependency-Safe Tasks

The next dependency-ready implementation task is M26-001-Z; this short queue lists the first 25 unchecked packets with their issue numbers.

1. [M26-001-Z — Package evidence for Accept QPack v2 format and compatibility ADR](tasks/03_m26_qpack_v2/M26-001-Z-package-evidence-for-accept-qpack-v2-format-and-compatibility-adr.md) — deps: M26-001-V — #185
2. [M26-002-A — Include runtime ABI, QuickJS-NG version/build hash, rquickjs version, bytecode format, target triple, pointer width, endianness, and capability hash](tasks/03_m26_qpack_v2/M26-002-A-include-runtime-abi-quickjs-ng-version-build-hash-rquickjs-version-bytecode-form.md) — deps: M26-001-Z — #186
3. [M26-002-B — Fail closed on mismatch](tasks/03_m26_qpack_v2/M26-002-B-fail-closed-on-mismatch.md) — deps: M26-002-A — #187
4. [M26-002-C — Provide explicit source rebuild path](tasks/03_m26_qpack_v2/M26-002-C-provide-explicit-source-rebuild-path.md) — deps: M26-002-B — #188
5. [M26-002-D — Never silently fall back](tasks/03_m26_qpack_v2/M26-002-D-never-silently-fall-back.md) — deps: M26-002-C — #189
6. [M26-002-V — Verify Define strict runtime and bytecode fingerprint](tasks/03_m26_qpack_v2/M26-002-V-verify-define-strict-runtime-and-bytecode-fingerprint.md) — deps: M26-002-A, M26-002-B, M26-002-C, M26-002-D — #190
7. [M26-002-Z — Package evidence for Define strict runtime and bytecode fingerprint](tasks/03_m26_qpack_v2/M26-002-Z-package-evidence-for-define-strict-runtime-and-bytecode-fingerprint.md) — deps: M26-002-V — #191
8. [M26-003-A — Define dense section schemas](tasks/03_m26_qpack_v2/M26-003-A-define-dense-section-schemas.md) — deps: M26-001-Z, G0-GATE, M25-GATE — #192
9. [M26-003-B — Store router nodes/edges/terminals, RoutePlans, schema programs, policy plans, function manifest, debug names, and capability inventory](tasks/03_m26_qpack_v2/M26-003-B-store-router-nodes-edges-terminals-routeplans-schema-programs-policy-plans-funct.md) — deps: M26-003-A — #193
10. [M26-003-C — Use offsets and bounds checks](tasks/03_m26_qpack_v2/M26-003-C-use-offsets-and-bounds-checks.md) — deps: M26-003-B — #194
11. [M26-003-D — Bind sections to execution integrity](tasks/03_m26_qpack_v2/M26-003-D-bind-sections-to-execution-integrity.md) — deps: M26-003-C — #195
12. [M26-003-V — Verify Encode compiled router, RoutePlans, schemas, policies, and functions as sections](tasks/03_m26_qpack_v2/M26-003-V-verify-encode-compiled-router-routeplans-schemas-policies-and-functions-as-secti.md) — deps: M26-003-A, M26-003-B, M26-003-C, M26-003-D — #196
13. [M26-003-Z — Package evidence for Encode compiled router, RoutePlans, schemas, policies, and functions as sections](tasks/03_m26_qpack_v2/M26-003-Z-package-evidence-for-encode-compiled-router-routeplans-schemas-policies-and-func.md) — deps: M26-003-V — #197
14. [M26-004-A — Store raw module bytecode section](tasks/03_m26_qpack_v2/M26-004-A-store-raw-module-bytecode-section.md) — deps: M26-002-Z, M26-003-Z — #198
15. [M26-004-B — Load exactly once](tasks/03_m26_qpack_v2/M26-004-B-load-exactly-once.md) — deps: M26-004-A — #199
16. [M26-004-C — Make source optional sidecar/development section](tasks/03_m26_qpack_v2/M26-004-C-make-source-optional-sidecar-development-section.md) — deps: M26-004-B — #200
17. [M26-004-D — Include prelude and handler manifest in the compiled module](tasks/03_m26_qpack_v2/M26-004-D-include-prelude-and-handler-manifest-in-the-compiled-module.md) — deps: M26-004-C — #201
18. [M26-004-V — Verify Embed raw QuickJS bytecode without base64](tasks/03_m26_qpack_v2/M26-004-V-verify-embed-raw-quickjs-bytecode-without-base64.md) — deps: M26-004-A, M26-004-B, M26-004-C, M26-004-D — #202
19. [M26-004-Z — Package evidence for Embed raw QuickJS bytecode without base64](tasks/03_m26_qpack_v2/M26-004-Z-package-evidence-for-embed-raw-quickjs-bytecode-without-base64.md) — deps: M26-004-V — #203
20. [M26-005-A — Use mmap/read-only bytes where supported](tasks/03_m26_qpack_v2/M26-005-A-use-mmap-read-only-bytes-where-supported.md) — deps: M26-003-Z — #204
21. [M26-005-B — Validate all section bounds before access](tasks/03_m26_qpack_v2/M26-005-B-validate-all-section-bounds-before-access.md) — deps: M26-005-A — #205
22. [M26-005-C — Avoid unsafe unchecked access unless independently audited](tasks/03_m26_qpack_v2/M26-005-C-avoid-unsafe-unchecked-access-unless-independently-audited.md) — deps: M26-005-B — #206
23. [M26-005-D — Support embedded pack bytes in standalone binary](tasks/03_m26_qpack_v2/M26-005-D-support-embedded-pack-bytes-in-standalone-binary.md) — deps: M26-005-C — #207
24. [M26-005-V — Verify Implement zero-copy or bounded-copy pack reader](tasks/03_m26_qpack_v2/M26-005-V-verify-implement-zero-copy-or-bounded-copy-pack-reader.md) — deps: M26-005-A, M26-005-B, M26-005-C, M26-005-D — #208
25. [M26-005-Z — Package evidence for Implement zero-copy or bounded-copy pack reader](tasks/03_m26_qpack_v2/M26-005-Z-package-evidence-for-implement-zero-copy-or-bounded-copy-pack-reader.md) — deps: M26-005-V — #209
