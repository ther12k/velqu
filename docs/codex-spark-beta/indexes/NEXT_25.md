# Next 25 Dependency-Safe Tasks

The next dependency-ready implementation task is M26-003-D; this short queue lists the first 25 unchecked packets with their issue numbers.

1. [M26-003-D — Bind sections to execution integrity](tasks/03_m26_qpack_v2/M26-003-D-bind-sections-to-execution-integrity.md) — deps: M26-003-C — #195
2. [M26-003-V — Verify Encode compiled router, RoutePlans, schemas, policies, and functions as sections](tasks/03_m26_qpack_v2/M26-003-V-verify-encode-compiled-router-routeplans-schemas-policies-and-functions-as-secti.md) — deps: M26-003-A, M26-003-B, M26-003-C, M26-003-D — #196
3. [M26-003-Z — Package evidence for Encode compiled router, RoutePlans, schemas, policies, and functions as sections](tasks/03_m26_qpack_v2/M26-003-Z-package-evidence-for-encode-compiled-router-routeplans-schemas-policies-and-func.md) — deps: M26-003-V — #197
4. [M26-004-A — Store raw module bytecode section](tasks/03_m26_qpack_v2/M26-004-A-store-raw-module-bytecode-section.md) — deps: M26-002-Z, M26-003-Z — #198
5. [M26-004-B — Load exactly once](tasks/03_m26_qpack_v2/M26-004-B-load-exactly-once.md) — deps: M26-004-A — #199
6. [M26-004-C — Make source optional sidecar/development section](tasks/03_m26_qpack_v2/M26-004-C-make-source-optional-sidecar-development-section.md) — deps: M26-004-B — #200
7. [M26-004-D — Include prelude and handler manifest in the compiled module](tasks/03_m26_qpack_v2/M26-004-D-include-prelude-and-handler-manifest-in-the-compiled-module.md) — deps: M26-004-C — #201
8. [M26-004-V — Verify Embed raw QuickJS bytecode without base64](tasks/03_m26_qpack_v2/M26-004-V-verify-embed-raw-quickjs-bytecode-without-base64.md) — deps: M26-004-A, M26-004-B, M26-004-C, M26-004-D — #202
9. [M26-004-Z — Package evidence for Embed raw QuickJS bytecode without base64](tasks/03_m26_qpack_v2/M26-004-Z-package-evidence-for-embed-raw-quickjs-bytecode-without-base64.md) — deps: M26-004-V — #203
10. [M26-005-A — Use mmap/read-only bytes where supported](tasks/03_m26_qpack_v2/M26-005-A-use-mmap-read-only-bytes-where-supported.md) — deps: M26-003-Z — #204
11. [M26-005-B — Validate all section bounds before access](tasks/03_m26_qpack_v2/M26-005-B-validate-all-section-bounds-before-access.md) — deps: M26-005-A — #205
12. [M26-005-C — Avoid unsafe unchecked access unless independently audited](tasks/03_m26_qpack_v2/M26-005-C-avoid-unsafe-unchecked-access-unless-independently-audited.md) — deps: M26-005-B — #206
13. [M26-005-D — Support embedded pack bytes in standalone binary](tasks/03_m26_qpack_v2/M26-005-D-support-embedded-pack-bytes-in-standalone-binary.md) — deps: M26-005-C — #207
14. [M26-005-V — Verify Implement zero-copy or bounded-copy pack reader](tasks/03_m26_qpack_v2/M26-005-V-verify-implement-zero-copy-or-bounded-copy-pack-reader.md) — deps: M26-005-A, M26-005-B, M26-005-C, M26-005-D — #208
15. [M26-005-Z — Package evidence for Implement zero-copy or bounded-copy pack reader](tasks/03_m26_qpack_v2/M26-005-Z-package-evidence-for-implement-zero-copy-or-bounded-copy-pack-reader.md) — deps: M26-005-V — #209
16. [M26-006-A — Hash required execution sections](tasks/03_m26_qpack_v2/M26-006-A-hash-required-execution-sections.md) — deps: M26-003-Z, M26-004-Z — #210
17. [M26-006-B — Provide Ed25519-compatible signature slot/hook](tasks/03_m26_qpack_v2/M26-006-B-provide-ed25519-compatible-signature-slot-hook.md) — deps: M26-006-A — #211
18. [M26-006-C — Define key discovery/configuration](tasks/03_m26_qpack_v2/M26-006-C-define-key-discovery-configuration.md) — deps: M26-006-B — #212
19. [M26-006-D — Keep unsigned local development supported with explicit policy](tasks/03_m26_qpack_v2/M26-006-D-keep-unsigned-local-development-supported-with-explicit-policy.md) — deps: M26-006-C — #213
20. [M26-006-V — Verify Implement execution integrity and authenticity hooks](tasks/03_m26_qpack_v2/M26-006-V-verify-implement-execution-integrity-and-authenticity-hooks.md) — deps: M26-006-A, M26-006-B, M26-006-C, M26-006-D — #214
21. [M26-006-Z — Package evidence for Implement execution integrity and authenticity hooks](tasks/03_m26_qpack_v2/M26-006-Z-package-evidence-for-implement-execution-integrity-and-authenticity-hooks.md) — deps: M26-006-V — #215
22. [M26-007-A — Remove timestamps/non-deterministic map order](tasks/03_m26_qpack_v2/M26-007-A-remove-timestamps-non-deterministic-map-order.md) — deps: M26-003-Z, M26-004-Z — #216
23. [M26-007-B — Pin compiler/runtime versions](tasks/03_m26_qpack_v2/M26-007-B-pin-compiler-runtime-versions.md) — deps: M26-007-A — #217
24. [M26-007-C — Canonicalize section ordering and padding](tasks/03_m26_qpack_v2/M26-007-C-canonicalize-section-ordering-and-padding.md) — deps: M26-007-B — #218
25. [M26-007-D — Compare independent build outputs](tasks/03_m26_qpack_v2/M26-007-D-compare-independent-build-outputs.md) — deps: M26-007-C — #219
