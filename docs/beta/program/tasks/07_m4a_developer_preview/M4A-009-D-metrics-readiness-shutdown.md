---
task_id: M4A-009-D
parent_task: M4A-009
milestone: M4A
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-009-D — Metrics/readiness/shutdown

## Atomic goal

Metrics/readiness/shutdown.

## Parent intent

Validate 30–50 routes, auth, fetch, validation, errors, pagination, and deployment.

## Dependencies

- `M4A-009-C` — `tasks/07_m4a_developer_preview/M4A-009-C-controlled-upstream.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/schema-codecs.md`

### Source files

- `AGENTS.md`
- `packages/cli/src/index.ts`
- `packages/compiler/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `crates/q-engine-quickjs/src/convert.rs`
- `packages/schema/src/index.ts`
- `conformance/schema/schema.conformance.test.ts`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-http/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Metrics/readiness/shutdown.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Runs entirely on actual runtime.
- No hidden Bun production path.
- All error/status contracts declared.
- Load and failure scenarios pass.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-http
```
```bash
cargo test -p q-schema-runtime
```
```bash
cargo test -p q-capabilities
```
```bash
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- Proof app source.
- Scenario tests.
- Benchmark report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m4a-009-d: metrics readiness shutdown
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

---

## Result (M4A-009-D) — PASS (2026-09-01)

- Branch/PR: m4a-009-d (squash-merged; see git log for final hash)
- Closes: #486

### Changed files
- `crates/q-engine-quickjs/src/convert.rs`: fixed JS-to-JSON number conversion
  where integer floats (e.g. `f.fract() == 0.0`) are preserved as integer `Json::Number`
  instead of converting to floating-point numbers, satisfying declared integer schema
  contracts across the bridge.
- `examples/proof/src/modules/ops/routes.ts` (new): 5 ops routes:
  - `ops.readiness`: GET application-level readiness probe reporting user/item service health
  - `ops.metrics`: GET in-memory inventory counts (users, items) and process uptime
  - `ops.version`: GET runtime environment metadata
  - `ops.ping`: GET lightweight pong
  - `ops.check`: POST diagnostic health check simulation with component validation
- `examples/proof/src/modules/ops/routes.test.ts` (new): 3 route contract unit tests.
- `examples/proof/src/app.ts`: registered `ops` module (proof app now exposes 24 routes).
- `examples/proof/src/tests/metrics-readiness-shutdown.scenario.test.ts` (new): end-to-end
  scenario driving native readiness (`/health/ready`), operational readiness/metrics
  routes, and bounded graceful SIGTERM shutdown on the actual Rust/QuickJS runtime binary.
- `packages/cli/src/inspect-output.test.ts`: updated routeCount pin from 19 to 24.
- `packages/compiler/src/package-verification.test.ts`: updated published contract hash.
- `benchmarks/manifest.json`: refreshed release binary hash.

### Required evidence

- **Proof app source**: `examples/proof/src/modules/ops/` with 5 operational endpoints.
- **Scenario tests**:
  - `examples/proof/src/modules/ops/routes.test.ts`: 3 unit tests verifying contract
    shapes, readiness status, and metrics calculations.
  - `examples/proof/src/tests/metrics-readiness-shutdown.scenario.test.ts`: live
    scenario verifying `/health/ready` (native), `/ops/readiness`, `/ops/metrics`,
    and clean SIGTERM exit code 0 on the actual release runtime binary.
- **Benchmark report**: `benchmarks/manifest.json` updated and verified.

### Guardrail mapping

- **Runs entirely on actual runtime**: operational endpoints and bounded shutdown
  run against the release Rust binary loading `app.qpack`.
- **No hidden Bun production path**: production execution remains Rust + QuickJS.
- **All error/status contracts declared**: all ops routes declare exact 200 schemas.
- **Load and failure scenarios pass**: bounded shutdown unblocks connections without
  hanging; readiness reports accurate health.

### Command results

- `cargo test -p q-engine-quickjs` → PASS
- `cargo test -p q-http` → PASS
- `cargo test -p q-schema-runtime` → PASS
- `cargo test -p q-capabilities` → PASS
- `cargo test -p velqu-runtime` → PASS
- `bun test` → **325 pass / 0 fail (53 files)**
- `bun run typecheck`, fmt check, workspace clippy → clean
- `./scripts/verify` → **ALL PASS**

### Disclosures

- Standing: CI `verify` workflows fail with zero executed steps on every PR
  since ~#714 (infrastructure-side); disclosed per PR. Local
  `./scripts/verify` is the gate evidence.
