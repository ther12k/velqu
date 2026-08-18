---
type: Traceability Matrix
title: M0–M2 Requirements Traceability (Final)
status: complete
---

# M0–M2 traceability matrix (final)

This matrix maps every P0 requirement to its implementation files, verification
tests, concrete evidence artifacts, commit hashes, and status.

| Req | Milestone | Source file(s) | Test / verification | Evidence artifact | Commit | Status |
|---|---|---|---|---|---|---|
| PR-001 | M0–M2 | `crates/q-runtime/src/main.rs`, `benchmarks/harness/cold-start.ts` | 1,680 fresh-process samples (C0–C5) | `benchmarks/raw/cold-start/summary.json` (C3 p95 4.4ms vs elysia 152.0ms) | `bcd4e1c` | **PASS** |
| PR-002 | M2 | `packages/core/src/index.ts`, `examples/proof/src/` | Pure TypeScript proof routes (no Rust required) | `examples/proof/src/modules/users/routes.ts` | `1bc0cde` | **PASS** |
| PR-003 | M0/M2 | `packages/treaty/src/index.ts` | Object-like route navigation, status narrowing | `conformance/treaty/treaty.conformance.test.ts` (3/3 pass) | `5c29623` | **PASS** |
| PR-004 | M2 | `packages/compiler/src/extract.ts` | Dynamic route AST rejection with source hint | `conformance/compiler/compiler.test.ts` (dynamic-app test pass) | `1bc0cde` | **PASS** |
| PR-005 | M2 | `crates/q-pack/src/lib.rs`, `packages/compiler/src/emit.ts` | Capability & compatibility manifests emitted & verified | `examples/proof/dist/capability-manifest.json` | `1bc0cde` | **PASS** |
| COMP-001 | M2 | `packages/compiler/src/extract.ts` | Route discovery at build time into `route-manifest.json` | `examples/proof/dist/route-manifest.json` | `1bc0cde` | **PASS** |
| COMP-002 | M2 | `packages/compiler/src/extract.ts` | Trap test: service factory throwing error never called during build | `conformance/compiler/compiler.test.ts` (trap test pass) | `1bc0cde` | **PASS** |
| COMP-003 | M2 | `packages/compiler/src/emit.ts` | Deterministic pack & route hashes on repeated builds | `conformance/compiler/compiler.test.ts` (determinism test pass) | `1bc0cde` | **PASS** |
| COMP-004 | M2 | `crates/q-router/src/lib.rs`, `packages/compiler/src/extract.ts` | Canonical collision detection rejecting equivalent paths | `conformance/compiler/compiler.test.ts` (collision test pass) | `1bc0cde` | **PASS** |
| COMP-005 | M1/M2 | `crates/q-pack/src/lib.rs` | Pack verification: sha256 integrity & version mismatch checks | `crates/q-runtime/tests/runtime_conformance.rs` (tamper test pass) | `c0710b6` | **PASS** |
| COMP-006 | M2 | `packages/compiler/src/extract.ts` | Unsupported `node:*` / `bun:*` imports rejected with diagnostic | `conformance/compiler/compiler.test.ts` (bad-import test pass) | `1bc0cde` | **PASS** |
| RUN-001 | M1 | `crates/q-engine-quickjs/src/` | Rust host running QuickJS-NG 0.15.1 via rquickjs | `crates/q-engine-quickjs/tests/engine.rs` (12/12 pass) | `162f477` | **PASS** |
| RUN-002 | M1 | `crates/q-router/src/lib.rs`, `crates/q-runtime/src/serve.rs` | Native routing before JavaScript; C0 served natively | `crates/q-runtime/tests/runtime_conformance.rs` (`stage: native`) | `e911b36` | **PASS** |
| RUN-003 | M1 | `crates/q-engine-quickjs/src/worker.rs` | Handler references cached once during load | `crates/q-engine-quickjs/tests/engine.rs` (cache verified) | `162f477` | **PASS** |
| RUN-004 | M1 | `crates/q-bridge/src/lib.rs` | Lazy request bridge: unread fields cost 0 materializations | `crates/q-bridge/src/lib.rs` (tests), `docs/reports/bridge-report.md` | `1bc0cde` | **PASS** |
| RUN-005 | M1 | `crates/q-http/src/lib.rs`, `crates/q-engine-quickjs/src/worker.rs` | Bounded body (413), headers (431), queue (503), heap (32MB), stack (512KB), deadline | `crates/q-runtime/tests/runtime_conformance.rs` (limits tests pass) | `c0710b6` | **PASS** |
| RUN-006 | M1 | `crates/q-engine-quickjs/src/worker.rs` | Async timer & cancellation matrix across HTTP & JS | `crates/q-engine-quickjs/tests/engine.rs` (cancel/timeout tests) | `162f477` | **PASS** |
| RUN-007 | M1/M2 | `crates/q-runtime/src/serve.rs` | Thrown errors redacted (500); source mapped to stderr | `crates/q-runtime/tests/runtime_conformance.rs` (redaction pass) | `c0710b6` | **PASS** |
| RUN-008 | M1 | `crates/q-http/src/lib.rs`, `crates/q-runtime/src/main.rs` | Graceful SIGTERM/SIGINT shutdown draining active work | `crates/q-runtime/tests/runtime_conformance.rs` (shutdown pass) | `e911b36` | **PASS** |
| SCHEMA-001 | M2 | `packages/schema/src/index.ts`, `crates/q-schema-runtime/src/lib.rs` | One Schema IR driving types, validator, Treaty, OpenAPI | `conformance/schema/schema.conformance.test.ts` (6/6 pass) | `1bc0cde` | **PASS** |
| SCHEMA-002 | M2 | `crates/q-schema-runtime/src/lib.rs` | Source-aware coercion (path/query string vs body exact) | `crates/q-schema-runtime/src/lib.rs` (unit tests pass) | `1bc0cde` | **PASS** |
| SCHEMA-003 | M2 | `packages/core/src/index.ts`, `crates/q-runtime/src/serve.rs` | Declared responses enforced; undeclared status fails | `crates/q-engine-quickjs/tests/engine.rs` (undeclared status test) | `162f477` | **PASS** |
| SCHEMA-004 | M2 | `packages/core/src/index.ts`, `crates/q-runtime/src/serve.rs` | Policy context injection & 401 response propagation | `conformance/lifecycle/lifecycle.conformance.test.ts` (pass) | `1bc0cde` | **PASS** |
| SCHEMA-005 | M2 | `packages/compiler/src/emit.ts` | Fallback strategies visible in build report | `examples/proof/dist/build-report.md` | `1bc0cde` | **PASS** |
| TRT-001 | M2 | `packages/treaty/src/index.ts` | Treaty infers path, query, header, and body inputs | `conformance/treaty/treaty.conformance.test.ts` (pass) | `5c29623` | **PASS** |
| TRT-002 | M2 | `packages/treaty/src/index.ts` | Non-throwing `{ data, error }` results for HTTP errors | `conformance/treaty/treaty.conformance.test.ts` (pass) | `5c29623` | **PASS** |
| TRT-003 | M2 | `packages/treaty/src/index.ts` | Status narrowing on `r.error.status` discriminant | `packages/treaty/src/treaty.test.ts` (type test pass) | `5c29623` | **PASS** |
| TRT-004 | M2 | `packages/treaty/src/index.ts` | Client bundle isolation (0 server dependencies) | `conformance/treaty/treaty.conformance.test.ts` (isolation test pass) | `5c29623` | **PASS** |
| TRT-005 | M2 | `packages/testing/src/index.ts` | `unitTreaty` vs `runtimeTreaty` modes labeled | `conformance/treaty/treaty.conformance.test.ts` (label test pass) | `1bc0cde` | **PASS** |
| PERF-001 | M0–M2 | `baselines/`, `benchmarks/` | Matched comparisons: velqu, raw-rust, raw-bun, elysia2 | `docs/reports/fairness-audit.md` (27/27 on all candidates) | `6b1dde0` | **PASS** |
| PERF-002 | M0 | `benchmarks/harness/cold-start.ts` | Statistics pipeline with p50, p95, p99, mean, stdev | `benchmarks/raw/cold-start/summary.json` | `bcd4e1c` | **PASS** |
| PERF-003 | M0 | `benchmarks/harness/cold-start.ts` | Cold-start routes C0–C5 separated and measured | `docs/reports/cold-start-report.md` | `1bc0cde` | **PASS** |
| PERF-004 | M1 | `crates/q-bench-support/src/bin/bridge_bench.rs` | Strategy A vs B benchmark over frozen matrix | `docs/reports/bridge-report.md` (Strategy B adopted) | `f66814f` | **PASS** |
| PERF-005 | M0/M2 | `benchmarks/harness/route-count.ts` | 25 and 1,000 route apps measured; scaling delta recorded | `benchmarks/raw/route-count/summary.json` (honest negative noted) | `bcd4e1c` | **PASS** |
| PERF-006 | Cont. | Whole repository | No comparative marketing claims; scope-limited language | `docs/reports/release-gate-report.md` | `1bc0cde` | **PASS** |
| SEC-001 | M1/M2 | `crates/q-pack/src/lib.rs` | Bytecode/pack integrity check before ready | `crates/q-runtime/tests/runtime_conformance.rs` (tamper pass) | `c0710b6` | **PASS** |
| SEC-002 | Cont. | Documentation | Trusted application code only; no hostile sandbox claim | `docs/reports/security-review.md` | `1bc0cde` | **PASS** |
| SEC-003 | M1 | `crates/q-bridge/src/lib.rs` | Opaque generation handles; expired access fails safely | `crates/q-bridge/src/lib.rs` (unit tests pass) | `1bc0cde` | **PASS** |
| SEC-004 | M1/M2 | `crates/q-runtime/src/serve.rs` | Secret redaction in responses; auth header excluded from logs | `crates/q-runtime/tests/runtime_conformance.rs` (redaction pass) | `c0710b6` | **PASS** |
| SEC-005 | M2 | `crates/q-engine-quickjs/src/worker.rs` | Explicit capability declaration; timer timeout/cancel | `conformance/security/security.conformance.test.ts` (pass) | `1bc0cde` | **PASS** |
| OPS-001 | M1/M2 | `crates/q-runtime/src/serve.rs` | Structured completion logs with request ID, route ID, stage | `crates/q-runtime/src/serve.rs:log_completion` | `e911b36` | **PASS** |
| OPS-002 | M0–M2 | `benchmarks/`, `packages/compiler/src/emit.ts` | Machine-readable JSON reports & manifests produced | `benchmarks/raw/*/summary.json`, `examples/proof/dist/` | `1bc0cde` | **PASS** |
| DX-001 | M2 | `examples/proof/` | Clean-checkout build, test, and run through scripts | `scripts/verify` | `1bc0cde` | **PASS** |
| DX-002 | M2 | `packages/compiler/src/extract.ts` | Source-located diagnostics with corrective hints | `conformance/compiler/compiler.test.ts` (diagnostics pass) | `1bc0cde` | **PASS** |
| DX-003 | M2 | `examples/proof/src/modules/` | Feature modules separating route, service, and policy | `examples/proof/src/modules/users/` | `1bc0cde` | **PASS** |
| DX-004 | M2 | `examples/proof/src/modules/users/service.ts` | Domain service accepts plain values (no framework ctx) | `examples/proof/src/tests/health.unit.test.ts` | `1bc0cde` | **PASS** |
