---
task_id: M4A-004-B
parent_task: M4A-004
milestone: M4A
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-004-B — Runtime-local actual Rust/QuickJS process

## Atomic goal

Runtime-local actual Rust/QuickJS process.

## Parent intent

Deliver Eden-quality type-safe clients and distinct test fidelity levels.

## Dependencies

- `M4A-004-A` — `tasks/07_m4a_developer_preview/M4A-004-A-unit-local-direct-generated-dispatcher.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/engine-scheduler.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `packages/cli/src/index.ts`
- `packages/compiler/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `packages/contract/src/index.ts`
- `conformance/treaty/treaty.conformance.test.ts`
- `packages/compiler/src/emit.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Runtime-local actual Rust/QuickJS process.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- No public `any`.
- 2xx data and non-2xx errors narrow correctly.
- Undeclared status is a contract error.
- All modes share the same contract.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
```
```bash
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- Negative type tests.
- Mode parity tests.
- Typecheck scale benchmark.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m4a-004-b: runtime local actual rust quickjs process
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M4A-004-B) — PASS

- Date: 2026-09-01
- Branch/PR: m4a-004-b (squash-merged; see git log for final hash)
- Closes: #451

### Changed files
- `packages/testing/src/index.ts`: strengthened `runtimeTreaty` into the
  runtime-local fidelity adapter — loads the published `contract.json` route
  table via `contractFromBuild` (optional explicit table remains for
  compatibility), searches for the runtime binary from pack checkout/cwd/env,
  passes an explicit `serviceProfile`, captures the Rust ready identity from
  both stdout/stderr without pipe deadlock, and closes with bounded
  SIGTERM → drain timeout → SIGKILL returning the exit code.
- `conformance/treaty/treaty.conformance.test.ts`: uses
  `contractFromBuild("examples/proof/dist")` instead of a duplicate
  hand-written route table, proving source-of-truth contract parity.
- `packages/testing/src/runtime-local.test.ts` (new): 3 tests — published
  contract loading, actual Rust + QuickJS serving/typed route behavior with
  bounded drain and exit 0, and explicit `service:2` profile readiness.
- `benchmarks/manifest.json`: refreshed.

### Required evidence

- **Negative type tests**: inherited from M4A-004-A's typecheck-only
  `packages/treaty/src/types-negative.test-d.ts`; `bun run typecheck` remains
  clean.
- **Mode parity tests**: runtime-local conformance plus the existing direct
  vs loopback parity suite; all modes use the same Treaty status-splitting
  machinery and published contract route table.
- **Typecheck scale benchmark**: inherited from M4A-004-A's raw 25/100/200
  route measurements; this packet does not change type-level surface.

### Guardrail mapping (parent M4A-004)

- **No public `any`** — runtime adapter adds concrete contract/ready/handle
  types only.
- **2xx data and non-2xx errors narrow correctly** — real QuickJS runtime
  conformance proves health/hello/users/timer success plus 422/401 error
  unions through Treaty.
- **Undeclared status is a contract error** — direct dispatcher guard remains
  enforced by `UndeclaredStatusError`; runtime-local adapter never converts
  process/contract failures to network results.
- **All modes share the same contract** — runtime-local reads the emitted
  `contract.json`, and the Treaty client is identical to direct/loopback modes.

### Command results

- `cargo test -p q-engine-quickjs` → all suites — 0 failed
- `cargo test -p velqu-runtime` → 7 suites — 0 failed
- `bun test` → **286 pass / 0 fail (40 files)**
- `bun run typecheck` → clean
- `cargo fmt --check` clean; workspace clippy -D warnings → exit 0
- `./scripts/verify` → **ALL PASS**

### Disclosures

- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
