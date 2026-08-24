# Next 25 Dependency-Safe Tasks

The next dependency-ready implementation task is M26-007-B; this short queue lists the first 25 unchecked packets with their issue numbers.

1. [M26-007-B — Pin compiler/runtime versions](tasks/03_m26_qpack_v2/M26-007-B-pin-compiler-runtime-versions.md) — deps: M26-007-A — #217
2. [M26-007-C — Canonicalize section ordering and padding](tasks/03_m26_qpack_v2/M26-007-C-canonicalize-section-ordering-and-padding.md) — deps: M26-007-B — #218
3. [M26-007-D — Compare independent build outputs](tasks/03_m26_qpack_v2/M26-007-D-compare-independent-build-outputs.md) — deps: M26-007-C — #219
4. [M26-007-V — Verify Guarantee reproducible release packs](tasks/03_m26_qpack_v2/M26-007-V-verify-guarantee-reproducible-release-packs.md) — deps: M26-007-A, M26-007-B, M26-007-C, M26-007-D — #220
5. [M26-007-Z — Package evidence for Guarantee reproducible release packs](tasks/03_m26_qpack_v2/M26-007-Z-package-evidence-for-guarantee-reproducible-release-packs.md) — deps: M26-007-V — #221
6. [M26-008-A — Implement separate v1 reader/adapter](tasks/03_m26_qpack_v2/M26-008-A-implement-separate-v1-reader-adapter.md) — deps: M26-001-Z, M26-005-Z — #222
7. [M26-008-B — Provide `velqu pack migrate` or rebuild guidance](tasks/03_m26_qpack_v2/M26-008-B-provide-velqu-pack-migrate-or-rebuild-guidance.md) — deps: M26-008-A — #223
8. [M26-008-C — Deprecate mixed-mode packs](tasks/03_m26_qpack_v2/M26-008-C-deprecate-mixed-mode-packs.md) — deps: M26-008-B — #224
9. [M26-008-D — Test deterministic failures for unsupported legacy features](tasks/03_m26_qpack_v2/M26-008-D-test-deterministic-failures-for-unsupported-legacy-features.md) — deps: M26-008-C — #225
10. [M26-008-V — Verify Provide explicit v1 compatibility and migration tool](tasks/03_m26_qpack_v2/M26-008-V-verify-provide-explicit-v1-compatibility-and-migration-tool.md) — deps: M26-008-A, M26-008-B, M26-008-C, M26-008-D — #226
11. [M26-008-Z — Package evidence for Provide explicit v1 compatibility and migration tool](tasks/03_m26_qpack_v2/M26-008-Z-package-evidence-for-provide-explicit-v1-compatibility-and-migration-tool.md) — deps: M26-008-V — #227
12. [M26-009-A — Shared mode: `velqu-runtime` plus app.qpack](tasks/03_m26_qpack_v2/M26-009-A-shared-mode-velqu-runtime-plus-app-qpack.md) — deps: M26-004-Z, M26-005-Z — #228
13. [M26-009-B — Standalone mode: embedded qpack executable](tasks/03_m26_qpack_v2/M26-009-B-standalone-mode-embedded-qpack-executable.md) — deps: M26-009-A — #229
14. [M26-009-C — Ensure exact runtime fingerprint](tasks/03_m26_qpack_v2/M26-009-C-ensure-exact-runtime-fingerprint.md) — deps: M26-009-B — #230
15. [M26-009-D — Define source-map/debug sidecars](tasks/03_m26_qpack_v2/M26-009-D-define-source-map-debug-sidecars.md) — deps: M26-009-C — #231
16. [M26-009-V — Verify Build shared-runtime and standalone deployment artifacts](tasks/03_m26_qpack_v2/M26-009-V-verify-build-shared-runtime-and-standalone-deployment-artifacts.md) — deps: M26-009-A, M26-009-B, M26-009-C, M26-009-D — #232
17. [M26-009-Z — Package evidence for Build shared-runtime and standalone deployment artifacts](tasks/03_m26_qpack_v2/M26-009-Z-package-evidence-for-build-shared-runtime-and-standalone-deployment-artifacts.md) — deps: M26-009-V — #233
18. [M26-010-A — Measure 25/100/1,000/5,000/10,000 routes](tasks/03_m26_qpack_v2/M26-010-A-measure-25-100-1-000-5-000-10-000-routes.md) — deps: M26-004-Z, M26-005-Z, M26-009-Z — #234
19. [M26-010-B — At least 100 fresh processes for release evidence](tasks/03_m26_qpack_v2/M26-010-B-at-least-100-fresh-processes-for-release-evidence.md) — deps: M26-010-A — #235
20. [M26-010-C — Randomize source/bytecode/competitor order](tasks/03_m26_qpack_v2/M26-010-C-randomize-source-bytecode-competitor-order.md) — deps: M26-010-B — #236
21. [M26-010-D — Record p50/p95/p99, RSS, stage timings, and hashes](tasks/03_m26_qpack_v2/M26-010-D-record-p50-p95-p99-rss-stage-timings-and-hashes.md) — deps: M26-010-C — #237
22. [M26-010-V — Verify Close route-count cold-start evidence](tasks/03_m26_qpack_v2/M26-010-V-verify-close-route-count-cold-start-evidence.md) — deps: M26-010-A, M26-010-B, M26-010-C, M26-010-D — #238
23. [M26-010-Z — Package evidence for Close route-count cold-start evidence](tasks/03_m26_qpack_v2/M26-010-Z-package-evidence-for-close-route-count-cold-start-evidence.md) — deps: M26-010-V — #239
24. [M26-GATE — M2.6 — Binary QPack v2 and Reproducible Artifact ABI exit gate](gates/M26-GATE.md) — deps: M26-001-Z, M26-002-Z, M26-003-Z, M26-004-Z, M26-005-Z, M26-006-Z, M26-007-Z, M26-008-Z, M26-009-Z, M26-010-Z — #629
25. [M27-001-A — Accept ADR](tasks/04_m27_capability_linker/M27-001-A-accept-adr.md) — deps: M26-GATE — #240
