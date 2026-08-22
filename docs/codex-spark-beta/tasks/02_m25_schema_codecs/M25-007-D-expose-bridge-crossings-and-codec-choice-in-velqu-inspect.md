---
task_id: M25-007-D
parent_task: M25-007
milestone: M25
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-007-D — Expose bridge crossings and codec choice in `velqu inspect`

## Atomic goal

Expose bridge crossings and codec choice in `velqu inspect`.

## Parent intent

Support advanced cases without hiding performance or semantic costs.

## Dependencies

- `M25-007-C` — `tasks/02_m25_schema_codecs/M25-007-C-keep-fallback-bounded-and-deadline-aware.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M25.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/schema-codecs.md`

### Source files

- `AGENTS.md`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-engine-quickjs/src/convert.rs`
- `packages/schema/src/index.ts`
- `packages/compiler/src/emit.ts`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `conformance/schema/schema.conformance.test.ts`
- `packages/cli/src/index.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Expose bridge crossings and codec choice in `velqu inspect`.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Fallback never activates silently.
- Raw Response bypass behavior is documented.
- No contract claim is generated when adapter lacks required projection.
- Fallback routes pass conformance.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-http
```
```bash
cargo test -p q-bridge
```
```bash
cargo test -p q-schema-runtime
```
```bash
cargo test -p velqu-runtime
```
```bash
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- Inspect snapshots.
- Fallback integration tests.
- Performance delta report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m25-007-d: expose bridge crossings and codec choice in velqu inspect
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M25-007-D)

Status: **PASS**. `velqu inspect` exposes per-route codec choice and the
bridge-crossing model:

- `packages/compiler/src/index.ts`: the route manifest now carries the
  REAL per-route facts (previously `validationStrategy`/`responseStrategy`
  were hardcoded "native" for every route — a latent bug): actual
  strategies from the M25-002-D decisions, the M25-007-A fallback reasons,
  the codec labels (`direct-decoder`/`generic-fallback` for validation,
  `direct-encoder`/`engine-stringify` for response), and the
  bridge-crossing model (`single-prevalidated` — one crossing with
  pre-validated values — vs `lazy-per-field` crossings on fallback routes).
- `packages/cli/src/index.ts`: `velqu inspect routes` renders the new
  `codec=` and `bridge=` columns alongside the strategy reasons (e.g.
  `val=js(unsupported-transform) ... codec=generic-fallback/direct-encoder
  bridge=lazy-per-field`).
- Inspect snapshot evidence: the compiler conformance test builds the
  fallback fixture, asserts the manifest facts per route (fb.body
  validation js + reason + generic codec + lazy bridge; fb.resp response
  js + measured reason + engine-stringify; std.get native with direct
  codecs and the single pre-validated crossing), and executes the actual
  CLI (`velqu inspect routes`) asserting the rendered snapshot contains
  every fact.

### Tests and evidence

- Compiler conformance "route manifest exposes codec choice and bridge
  crossings (M25-007-D)" — manifest assertions + live CLI inspect
  snapshot (inspect-snapshot evidence required by the parent).
- `cargo test -p q-engine-quickjs` — 1 + 96 passed.
- `cargo test -p q-schema-runtime` — 57 unit + 3 fuzz passed.
- `cargo test -p velqu-runtime` — 24 integration passed.
- `cargo test -p q-pack` — 41 + 2 passed.
- `bun test` — 75 passed, 0 failed, 340 expect calls.
- `bun run typecheck` — clean. `cargo fmt --check` — clean.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `scripts/validate-okf` — 176 links, 0 errors.
- `./scripts/verify` — all stages pass except the documented
  isolated-worktree `qRuntimeRelease`/`proofPack` manifest hash mismatch
  (known, pre-existing on every packet branch).

Commit: `bf6714b`.
