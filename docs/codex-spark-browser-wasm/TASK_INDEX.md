# Browser-WASM Task Index

Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)

| Issue | Title | Phase | Mode | Priority | Optional | Dependencies |
|---|---|---|---|---|---|---|
| [`BWASM-EPIC`](tasks/00_program/BWASM-EPIC-velqu-browser-wasm-runtime-program.md) | Velqu Browser-WASM runtime program | Program | GATE | P0 | No | — |
| [`BWASM-D-001`](tasks/01_design/BWASM-D-001-freeze-the-browser-wasm-product-and-runtime-contract.md) | Freeze the Browser-WASM product and runtime contract | Architecture and decisions | IMPLEMENT | P0 | No | — |
| [`BWASM-D-002`](tasks/01_design/BWASM-D-002-produce-the-wasm32-portability-baseline-and-dependency-split-map.md) | Produce the wasm32 portability baseline and dependency split map | Architecture and decisions | VERIFY_OR_FIX | P0 | No | `BWASM-D-001` |
| [`BWASM-D-003`](tasks/01_design/BWASM-D-003-define-the-browser-execution-threat-model-and-isolation-contract.md) | Define the browser execution threat model and isolation contract | Architecture and decisions | IMPLEMENT | P0 | No | `BWASM-D-001` |
| [`BWASM-D-004`](tasks/01_design/BWASM-D-004-ratify-support-matrix-compatibility-claims-and-release-budgets.md) | Ratify support matrix, compatibility claims, and release budgets | Architecture and decisions | IMPLEMENT | P0 | No | `BWASM-D-001`, `BWASM-D-002`, `BWASM-D-003` |
| [`BWASM-K-001`](tasks/02_kernel/BWASM-K-001-extract-a-portable-runtime-model-crate.md) | Extract a portable runtime model crate | Portable Rust/WASM kernel | IMPLEMENT | P0 | No | `BWASM-D-001`, `BWASM-D-002` |
| [`BWASM-K-002`](tasks/02_kernel/BWASM-K-002-split-byte-based-qpack-core-from-native-loading-and-tooling.md) | Split byte-based QPack core from native loading and tooling | Portable Rust/WASM kernel | IMPLEMENT | P0 | No | `BWASM-K-001` |
| [`BWASM-K-003`](tasks/02_kernel/BWASM-K-003-extract-a-host-independent-router-core.md) | Extract a host-independent router core | Portable Rust/WASM kernel | IMPLEMENT | P0 | No | `BWASM-K-001` |
| [`BWASM-K-004`](tasks/02_kernel/BWASM-K-004-qualify-the-schema-runtime-for-wasm32-and-expose-bounded-validation.md) | Qualify the schema runtime for wasm32 and expose bounded validation | Portable Rust/WASM kernel | IMPLEMENT | P0 | No | `BWASM-D-002` |
| [`BWASM-K-005`](tasks/02_kernel/BWASM-K-005-implement-the-rust-browser-kernel-and-wasm-bindgen-abi.md) | Implement the Rust Browser Kernel and wasm-bindgen ABI | Portable Rust/WASM kernel | IMPLEMENT | P0 | No | `BWASM-K-002`, `BWASM-K-003`, `BWASM-K-004`, `BWASM-D-003` |
| [`BWASM-K-006`](tasks/02_kernel/BWASM-K-006-verify-and-package-portable-kernel-evidence.md) | Verify and package portable-kernel evidence | Portable Rust/WASM kernel | EVIDENCE | P0 | No | `BWASM-K-001`, `BWASM-K-002`, `BWASM-K-003`, `BWASM-K-004`, `BWASM-K-005`, `BWASM-D-004` |
| [`BWASM-R-001`](tasks/03_runtime/BWASM-R-001-create-velqu-browser-runtime-package-and-public-runtime-contract.md) | Create @velqu/browser-runtime package and public runtime contract | Browser runtime and Worker execution | IMPLEMENT | P0 | No | `BWASM-K-005`, `BWASM-D-001` |
| [`BWASM-R-002`](tasks/03_runtime/BWASM-R-002-implement-fetch-compatible-browser-dispatcher.md) | Implement Fetch-compatible browser dispatcher | Browser runtime and Worker execution | IMPLEMENT | P0 | No | `BWASM-R-001`, `BWASM-K-005` |
| [`BWASM-R-003`](tasks/03_runtime/BWASM-R-003-define-and-emit-the-browser-handler-bundle-contract.md) | Define and emit the browser handler-bundle contract | Browser runtime and Worker execution | IMPLEMENT | P0 | No | `BWASM-R-001`, `BWASM-D-001` |
| [`BWASM-R-004`](tasks/03_runtime/BWASM-R-004-execute-handlers-in-isolated-workers-with-cancellation-and-hard-recovery.md) | Execute handlers in isolated Workers with cancellation and hard recovery | Browser runtime and Worker execution | IMPLEMENT | P0 | No | `BWASM-R-003`, `BWASM-D-003` |
| [`BWASM-R-005`](tasks/03_runtime/BWASM-R-005-integrate-capability-registry-and-treaty-with-the-browser-runtime.md) | Integrate capability registry and Treaty with the browser runtime | Browser runtime and Worker execution | IMPLEMENT | P0 | No | `BWASM-R-002`, `BWASM-R-004`, `BWASM-K-005` |
| [`BWASM-R-006`](tasks/03_runtime/BWASM-R-006-verify-and-package-browser-runtime-evidence.md) | Verify and package browser-runtime evidence | Browser runtime and Worker execution | EVIDENCE | P0 | No | `BWASM-R-001`, `BWASM-R-002`, `BWASM-R-003`, `BWASM-R-004`, `BWASM-R-005`, `BWASM-K-006` |
| [`BWASM-B-001`](tasks/04_build_deploy/BWASM-B-001-add-compiler-target-browser-wasm.md) | Add compiler target browser-wasm | Compiler, artifacts, and static deployment | IMPLEMENT | P0 | No | `BWASM-K-005`, `BWASM-R-003`, `BWASM-D-001` |
| [`BWASM-B-002`](tasks/04_build_deploy/BWASM-B-002-define-content-addressed-browser-artifact-manifest-and-loader.md) | Define content-addressed browser artifact manifest and loader | Compiler, artifacts, and static deployment | IMPLEMENT | P0 | No | `BWASM-B-001` |
| [`BWASM-B-003`](tasks/04_build_deploy/BWASM-B-003-enforce-browser-import-policy-with-source-located-diagnostics.md) | Enforce browser import policy with source-located diagnostics | Compiler, artifacts, and static deployment | IMPLEMENT | P0 | No | `BWASM-B-001`, `BWASM-D-003` |
| [`BWASM-B-004`](tasks/04_build_deploy/BWASM-B-004-add-service-worker-adapter-and-static-host-bootstrap.md) | Add Service Worker adapter and static-host bootstrap | Compiler, artifacts, and static deployment | IMPLEMENT | P0 | No | `BWASM-R-002`, `BWASM-B-002`, `BWASM-D-003` |
| [`BWASM-B-005`](tasks/04_build_deploy/BWASM-B-005-add-cli-build-preview-inspect-and-export-workflows.md) | Add CLI build, preview, inspect, and export workflows | Compiler, artifacts, and static deployment | IMPLEMENT | P0 | No | `BWASM-B-001`, `BWASM-B-002`, `BWASM-B-004` |
| [`BWASM-B-006`](tasks/04_build_deploy/BWASM-B-006-verify-cache-activation-upgrades-rollback-and-static-deployment.md) | Verify cache activation, upgrades, rollback, and static deployment | Compiler, artifacts, and static deployment | VERIFY_OR_FIX | P0 | No | `BWASM-B-004`, `BWASM-B-005` |
| [`BWASM-C-001`](tasks/05_capabilities/BWASM-C-001-implement-browser-safe-timer-crypto-logging-and-restricted-fetch-capabilities.md) | Implement browser-safe timer, crypto, logging, and restricted fetch capabilities | Browser capabilities and persistence | IMPLEMENT | P0 | No | `BWASM-R-005`, `BWASM-D-003` |
| [`BWASM-C-002`](tasks/05_capabilities/BWASM-C-002-make-the-postgres-capability-contract-asynchronous-before-browser-freeze.md) | Make the Postgres capability contract asynchronous before browser freeze | Browser capabilities and persistence | IMPLEMENT | P0 | No | `BWASM-D-001` |
| [`BWASM-C-003`](tasks/05_capabilities/BWASM-C-003-add-optional-pglite-backed-local-sql-capability.md) | Add optional PGlite-backed local SQL capability | Browser capabilities and persistence | IMPLEMENT | P1 | Yes | `BWASM-C-002`, `BWASM-R-005`, `BWASM-B-001` |
| [`BWASM-C-004`](tasks/05_capabilities/BWASM-C-004-add-namespaced-indexeddb-kv-persistence-capability.md) | Add namespaced IndexedDB KV persistence capability | Browser capabilities and persistence | IMPLEMENT | P0 | No | `BWASM-R-005` |
| [`BWASM-C-005`](tasks/05_capabilities/BWASM-C-005-fail-closed-for-deployment-required-and-unavailable-capabilities.md) | Fail closed for deployment-required and unavailable capabilities | Browser capabilities and persistence | IMPLEMENT | P0 | No | `BWASM-R-005`, `BWASM-B-003`, `BWASM-C-001`, `BWASM-C-002`, `BWASM-C-004` |
| [`BWASM-Q-001`](tasks/06_quality_release/BWASM-Q-001-build-shared-native-versus-browser-conformance-and-differential-suites.md) | Build shared native-versus-browser conformance and differential suites | Conformance, security, DevEx, and release qualification | VERIFY_OR_FIX | P0 | No | `BWASM-K-006`, `BWASM-R-006`, `BWASM-B-006`, `BWASM-C-005` |
| [`BWASM-Q-002`](tasks/06_quality_release/BWASM-Q-002-add-real-browser-ci-lanes-and-supported-browser-evidence.md) | Add real-browser CI lanes and supported-browser evidence | Conformance, security, DevEx, and release qualification | IMPLEMENT | P0 | No | `BWASM-Q-001`, `BWASM-B-006`, `BWASM-D-004` |
| [`BWASM-Q-003`](tasks/06_quality_release/BWASM-Q-003-verify-isolated-preview-origin-and-untrusted-code-security-boundaries.md) | Verify isolated preview-origin and untrusted-code security boundaries | Conformance, security, DevEx, and release qualification | VERIFY_OR_FIX | P0 | No | `BWASM-R-004`, `BWASM-B-004`, `BWASM-C-001`, `BWASM-D-003` |
| [`BWASM-Q-004`](tasks/06_quality_release/BWASM-Q-004-add-browser-wasm-observability-and-developer-diagnostics.md) | Add Browser-WASM observability and developer diagnostics | Conformance, security, DevEx, and release qualification | IMPLEMENT | P1 | No | `BWASM-R-006`, `BWASM-B-005` |
| [`BWASM-Q-005`](tasks/06_quality_release/BWASM-Q-005-set-and-enforce-wasm-size-startup-latency-and-leak-budgets.md) | Set and enforce WASM size, startup, latency, and leak budgets | Conformance, security, DevEx, and release qualification | VERIFY_OR_FIX | P0 | No | `BWASM-K-006`, `BWASM-R-006`, `BWASM-B-006`, `BWASM-D-004` |
| [`BWASM-Q-006`](tasks/06_quality_release/BWASM-Q-006-publish-browser-wasm-documentation-limitations-and-migration-guide.md) | Publish Browser-WASM documentation, limitations, and migration guide | Conformance, security, DevEx, and release qualification | IMPLEMENT | P0 | No | `BWASM-B-005`, `BWASM-C-005`, `BWASM-Q-004` |
| [`BWASM-Q-007`](tasks/06_quality_release/BWASM-Q-007-run-an-external-cleanroom-static-deployment-and-offline-exercise.md) | Run an external cleanroom static deployment and offline exercise | Conformance, security, DevEx, and release qualification | EVIDENCE | P0 | No | `BWASM-Q-002`, `BWASM-Q-003`, `BWASM-Q-005`, `BWASM-Q-006` |
| [`BWASM-Q-008`](tasks/06_quality_release/BWASM-Q-008-assemble-release-evidence-sbom-checksums-provenance-and-candidate-packet.md) | Assemble release evidence, SBOM, checksums, provenance, and candidate packet | Conformance, security, DevEx, and release qualification | EVIDENCE | P0 | No | `BWASM-Q-001`, `BWASM-Q-007`, `BWASM-K-006`, `BWASM-R-006`, `BWASM-B-006` |
| [`BWASM-X-001`](tasks/07_optional_parity/BWASM-X-001-spike-quickjs-ng-in-wasm-engine-parity-and-record-go-or-no-go.md) | Spike QuickJS-NG-in-WASM engine parity and record GO or NO-GO | Optional QuickJS-NG WASM parity | VERIFY_OR_FIX | P1 | Yes | `BWASM-K-005`, `BWASM-R-004`, `BWASM-D-004` |
| [`BWASM-GATE`](gates/BWASM-GATE-browser-wasm-beta-readiness-go-or-no-go.md) | Browser-WASM beta readiness GO or NO-GO | Release gate | GATE_REVIEW | P0 | No | `BWASM-D-004`, `BWASM-K-006`, `BWASM-R-006`, `BWASM-B-006`, `BWASM-C-005`, `BWASM-Q-001`, `BWASM-Q-002`, `BWASM-Q-003`, `BWASM-Q-005`, `BWASM-Q-006`, `BWASM-Q-007`, `BWASM-Q-008` |

## Totals

- Total issues: **38**
- Mandatory: **36**
- Optional: **2**
- P0: **35**
- P1: **3**
- Modes: `EVIDENCE` 4, `GATE` 1, `GATE_REVIEW` 1, `IMPLEMENT` 26, `VERIFY_OR_FIX` 6

## Registration rule

Register `00_program` and `01_design` first. Do not assign implementation work whose design dependency is unresolved. Optional issues remain outside the MVP gate unless promoted by a recorded owner decision before candidate freeze.
