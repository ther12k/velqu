# Next 25 Dependency-Safe Tasks

The next dependency-ready implementation task is M26-001-A; this short queue lists the first 25 unchecked packets with their issue numbers.

1. [M26-001-A — Define numeric current mode and legacy v1 adapter](tasks/03_m26_qpack_v2/M26-001-A-define-numeric-current-mode-and-legacy-v1-adapter.md) — deps: M25-GATE — #180
2. [M26-001-B — Specify section directory, alignment, bounds, optional sections, and versioning](tasks/03_m26_qpack_v2/M26-001-B-specify-section-directory-alignment-bounds-optional-sections-and-versioning.md) — deps: M26-001-A — #181
3. [M26-001-C — Separate integrity from authenticity](tasks/03_m26_qpack_v2/M26-001-C-separate-integrity-from-authenticity.md) — deps: M26-001-B — #182
4. [M26-001-D — Define debug/source sidecar policy](tasks/03_m26_qpack_v2/M26-001-D-define-debug-source-sidecar-policy.md) — deps: M26-001-C — #183
5. [M26-001-V — Verify Accept QPack v2 format and compatibility ADR](tasks/03_m26_qpack_v2/M26-001-V-verify-accept-qpack-v2-format-and-compatibility-adr.md) — deps: M26-001-A, M26-001-B, M26-001-C, M26-001-D — #184
6. [M26-001-Z — Package evidence for Accept QPack v2 format and compatibility ADR](tasks/03_m26_qpack_v2/M26-001-Z-package-evidence-for-accept-qpack-v2-format-and-compatibility-adr.md) — deps: M26-001-V — #185
7. [M26-002-A — Include runtime ABI, QuickJS-NG version/build hash, rquickjs version, bytecode format, target triple, pointer width, endianness, and capability hash](tasks/03_m26_qpack_v2/M26-002-A-include-runtime-abi-quickjs-ng-version-build-hash-rquickjs-version-bytecode-form.md) — deps: M26-001-Z — #186
8. [M26-002-B — Fail closed on mismatch](tasks/03_m26_qpack_v2/M26-002-B-fail-closed-on-mismatch.md) — deps: M26-002-A — #187
9. [M26-002-C — Provide explicit source rebuild path](tasks/03_m26_qpack_v2/M26-002-C-provide-explicit-source-rebuild-path.md) — deps: M26-002-B — #188
10. [M26-002-D — Never silently fall back](tasks/03_m26_qpack_v2/M26-002-D-never-silently-fall-back.md) — deps: M26-002-C — #189
11. [M26-002-V — Verify Define strict runtime and bytecode fingerprint](tasks/03_m26_qpack_v2/M26-002-V-verify-define-strict-runtime-and-bytecode-fingerprint.md) — deps: M26-002-A, M26-002-B, M26-002-C, M26-002-D — #190
12. [M26-002-Z — Package evidence for Define strict runtime and bytecode fingerprint](tasks/03_m26_qpack_v2/M26-002-Z-package-evidence-for-define-strict-runtime-and-bytecode-fingerprint.md) — deps: M26-002-V — #191
13. [M26-003-A — Define dense section schemas](tasks/03_m26_qpack_v2/M26-003-A-define-dense-section-schemas.md) — deps: M26-001-Z, G0-GATE, M25-GATE — #192
14. [M26-003-B — Store router nodes/edges/terminals, RoutePlans, schema programs, policy plans, function manifest, debug names, and capability inventory](tasks/03_m26_qpack_v2/M26-003-B-store-router-nodes-edges-terminals-routeplans-schema-programs-policy-plans-funct.md) — deps: M26-003-A — #193
15. [M26-003-C — Use offsets and bounds checks](tasks/03_m26_qpack_v2/M26-003-C-use-offsets-and-bounds-checks.md) — deps: M26-003-B — #194
16. [M26-003-D — Bind sections to execution integrity](tasks/03_m26_qpack_v2/M26-003-D-bind-sections-to-execution-integrity.md) — deps: M26-003-C — #195
17. [M26-003-V — Verify Encode compiled router, RoutePlans, schemas, policies, and functions as sections](tasks/03_m26_qpack_v2/M26-003-V-verify-encode-compiled-router-routeplans-schemas-policies-and-functions-as-secti.md) — deps: M26-003-A, M26-003-B, M26-003-C, M26-003-D — #196
18. [M26-003-Z — Package evidence for Encode compiled router, RoutePlans, schemas, policies, and functions as sections](tasks/03_m26_qpack_v2/M26-003-Z-package-evidence-for-encode-compiled-router-routeplans-schemas-policies-and-func.md) — deps: M26-003-V — #197
19. [M26-004-A — Store raw module bytecode section](tasks/03_m26_qpack_v2/M26-004-A-store-raw-module-bytecode-section.md) — deps: M26-002-Z, M26-003-Z — #198
20. [M26-004-B — Load exactly once](tasks/03_m26_qpack_v2/M26-004-B-load-exactly-once.md) — deps: M26-004-A — #199
21. [M26-004-C — Make source optional sidecar/development section](tasks/03_m26_qpack_v2/M26-004-C-make-source-optional-sidecar-development-section.md) — deps: M26-004-B — #200
22. [M26-004-D — Include prelude and handler manifest in the compiled module](tasks/03_m26_qpack_v2/M26-004-D-include-prelude-and-handler-manifest-in-the-compiled-module.md) — deps: M26-004-C — #201
23. [M26-004-V — Verify Embed raw QuickJS bytecode without base64](tasks/03_m26_qpack_v2/M26-004-V-verify-embed-raw-quickjs-bytecode-without-base64.md) — deps: M26-004-A, M26-004-B, M26-004-C, M26-004-D — #202
24. [M26-004-Z — Package evidence for Embed raw QuickJS bytecode without base64](tasks/03_m26_qpack_v2/M26-004-Z-package-evidence-for-embed-raw-quickjs-bytecode-without-base64.md) — deps: M26-004-V — #203
25. [M26-005-A — Use mmap/read-only bytes where supported](tasks/03_m26_qpack_v2/M26-005-A-use-mmap-read-only-bytes-where-supported.md) — deps: M26-003-Z — #204
