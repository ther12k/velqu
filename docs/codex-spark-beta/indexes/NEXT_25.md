# Next 25 Dependency-Safe Tasks

The next dependency-ready implementation task is M26-005-A; this short queue lists the first 25 unchecked packets with their issue numbers.

1. [M26-005-A — Use mmap/read-only bytes where supported](tasks/03_m26_qpack_v2/M26-005-A-use-mmap-read-only-bytes-where-supported.md) — deps: M26-003-Z — #204
2. [M26-005-B — Validate all section bounds before access](tasks/03_m26_qpack_v2/M26-005-B-validate-all-section-bounds-before-access.md) — deps: M26-005-A — #205
3. [M26-005-C — Avoid unsafe unchecked access unless independently audited](tasks/03_m26_qpack_v2/M26-005-C-avoid-unsafe-unchecked-access-unless-independently-audited.md) — deps: M26-005-B — #206
4. [M26-005-D — Support embedded pack bytes in standalone binary](tasks/03_m26_qpack_v2/M26-005-D-support-embedded-pack-bytes-in-standalone-binary.md) — deps: M26-005-C — #207
5. [M26-005-V — Verify Implement zero-copy or bounded-copy pack reader](tasks/03_m26_qpack_v2/M26-005-V-verify-implement-zero-copy-or-bounded-copy-pack-reader.md) — deps: M26-005-A, M26-005-B, M26-005-C, M26-005-D — #208
6. [M26-005-Z — Package evidence for Implement zero-copy or bounded-copy pack reader](tasks/03_m26_qpack_v2/M26-005-Z-package-evidence-for-implement-zero-copy-or-bounded-copy-pack-reader.md) — deps: M26-005-V — #209
7. [M26-006-A — Hash required execution sections](tasks/03_m26_qpack_v2/M26-006-A-hash-required-execution-sections.md) — deps: M26-003-Z, M26-004-Z — #210
8. [M26-006-B — Provide Ed25519-compatible signature slot/hook](tasks/03_m26_qpack_v2/M26-006-B-provide-ed25519-compatible-signature-slot-hook.md) — deps: M26-006-A — #211
9. [M26-006-C — Define key discovery/configuration](tasks/03_m26_qpack_v2/M26-006-C-define-key-discovery-configuration.md) — deps: M26-006-B — #212
10. [M26-006-D — Keep unsigned local development supported with explicit policy](tasks/03_m26_qpack_v2/M26-006-D-keep-unsigned-local-development-supported-with-explicit-policy.md) — deps: M26-006-C — #213
11. [M26-006-V — Verify Implement execution integrity and authenticity hooks](tasks/03_m26_qpack_v2/M26-006-V-verify-implement-execution-integrity-and-authenticity-hooks.md) — deps: M26-006-A, M26-006-B, M26-006-C, M26-006-D — #214
12. [M26-006-Z — Package evidence for Implement execution integrity and authenticity hooks](tasks/03_m26_qpack_v2/M26-006-Z-package-evidence-for-implement-execution-integrity-and-authenticity-hooks.md) — deps: M26-006-V — #215
13. [M26-007-A — Remove timestamps/non-deterministic map order](tasks/03_m26_qpack_v2/M26-007-A-remove-timestamps-non-deterministic-map-order.md) — deps: M26-003-Z, M26-004-Z — #216
14. [M26-007-B — Pin compiler/runtime versions](tasks/03_m26_qpack_v2/M26-007-B-pin-compiler-runtime-versions.md) — deps: M26-007-A — #217
15. [M26-007-C — Canonicalize section ordering and padding](tasks/03_m26_qpack_v2/M26-007-C-canonicalize-section-ordering-and-padding.md) — deps: M26-007-B — #218
16. [M26-007-D — Compare independent build outputs](tasks/03_m26_qpack_v2/M26-007-D-compare-independent-build-outputs.md) — deps: M26-007-C — #219
17. [M26-007-V — Verify Guarantee reproducible release packs](tasks/03_m26_qpack_v2/M26-007-V-verify-guarantee-reproducible-release-packs.md) — deps: M26-007-A, M26-007-B, M26-007-C, M26-007-D — #220
18. [M26-007-Z — Package evidence for Guarantee reproducible release packs](tasks/03_m26_qpack_v2/M26-007-Z-package-evidence-for-guarantee-reproducible-release-packs.md) — deps: M26-007-V — #221
19. [M26-008-A — Implement separate v1 reader/adapter](tasks/03_m26_qpack_v2/M26-008-A-implement-separate-v1-reader-adapter.md) — deps: M26-001-Z, M26-005-Z — #222
20. [M26-008-B — Provide `velqu pack migrate` or rebuild guidance](tasks/03_m26_qpack_v2/M26-008-B-provide-velqu-pack-migrate-or-rebuild-guidance.md) — deps: M26-008-A — #223
21. [M26-008-C — Deprecate mixed-mode packs](tasks/03_m26_qpack_v2/M26-008-C-deprecate-mixed-mode-packs.md) — deps: M26-008-B — #224
22. [M26-008-D — Test deterministic failures for unsupported legacy features](tasks/03_m26_qpack_v2/M26-008-D-test-deterministic-failures-for-unsupported-legacy-features.md) — deps: M26-008-C — #225
23. [M26-008-V — Verify Provide explicit v1 compatibility and migration tool](tasks/03_m26_qpack_v2/M26-008-V-verify-provide-explicit-v1-compatibility-and-migration-tool.md) — deps: M26-008-A, M26-008-B, M26-008-C, M26-008-D — #226
24. [M26-008-Z — Package evidence for Provide explicit v1 compatibility and migration tool](tasks/03_m26_qpack_v2/M26-008-Z-package-evidence-for-provide-explicit-v1-compatibility-and-migration-tool.md) — deps: M26-008-V — #227
25. [M26-009-A — Shared mode: `velqu-runtime` plus app.qpack](tasks/03_m26_qpack_v2/M26-009-A-shared-mode-velqu-runtime-plus-app-qpack.md) — deps: M26-004-Z, M26-005-Z — #228
