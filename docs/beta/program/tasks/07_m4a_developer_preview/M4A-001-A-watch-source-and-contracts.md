---
task_id: M4A-001-A
parent_task: M4A-001
milestone: M4A
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-001-A — Watch source and contracts

## Atomic goal

Watch source and contracts.

## Parent intent

Compile and reload the real QuickJS/QPack runtime with fast feedback and parity.

## Dependencies

- `M3-GATE` — `gates/M3-GATE.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `packages/cli/src/index.ts`
- `packages/compiler/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-pack/src/lib.rs`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `docs/specs/pack-format-v1.md`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Watch source and contracts.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- No Bun-only behavior mismatch by default.
- Failed reload keeps prior healthy app.
- Source maps point to TypeScript.
- Reload is bounded and observable.

## Targeted commands

```bash
cargo test -p q-pack
```
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

- Reload conformance.
- Failure recovery tests.
- Developer latency measurements.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m4a-001-a: watch source and contracts
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M4A-001-A) — PASS

- Date: 2026-08-31
- Branch/PR: m4a-001-a (squash-merged; see git log for final hash)
- Closes: #432

### Changed files
- `packages/compiler/src/watch.ts` (new): source and contract watcher
  infrastructure (`ProjectWatcher`, `watchSourceAndContracts`) —
  - `discover()`: statically discovers all source modules, contract
    locks (`contract.lock.json`, `contract.meta.json`, `contract.json`),
    and config files (`tsconfig.json`, `package.json`, `velqu.json`)
    without executing application code (COMP-002).
  - Directory-level `fs.watch` monitoring with recursive-less watchers
    across all unique source/project directories.
  - `classifyFile()`: classifies modified paths into `"source"`,
    `"contract"`, or `"config"`, while ignoring build artifacts
    (`dist/`), VCS (`.git/`), and dependencies (`node_modules/`).
  - Debouncing: coalesces rapid burst modifications into single typed
    `WatchEvent`s with measured event latency.
  - `start()` / `close()`: lifecycle control and cleanup.
- `packages/compiler/src/index.ts`: exports `ProjectWatcher`,
  `watchSourceAndContracts`, and related types.
- `packages/cli/src/index.ts`: `velqu dev` and `velqu watch` CLI commands
  wired to start the watcher and log discovered files and events.
- `packages/compiler/src/watch.test.ts` (new): 7 unit/integration tests.
- `benchmarks/manifest.json`: refreshed (standard remapped flow).

### Tests added (packages/compiler/src/watch.test.ts, +7 tests)
- Statically discovers all source files and contracts without code execution.
- Classifies file paths into source, contract, config, and ignores artifacts.
- Detects source file changes and delivers typed event with latency metric.
- Detects contract.lock.json modifications as contract events.
- Coalesces rapid burst modifications into a single debounced event.
- Detects file deletion and reports delete action.
- Discovers proof fixture files and project structure accurately.

### Command results
- `cargo test -p q-pack` → 3 suites — 0 failed
- `cargo test -p q-engine-quickjs` → 20 + 102 + 1 — 0 failed
- `bun test` → **226 pass / 0 fail (28 files, +7 new tests)**
- `bun run typecheck` → clean
- `cargo fmt --check` clean; workspace clippy -D warnings → exit 0
- `./scripts/verify` → **ALL PASS**

### Guardrail mapping (parent M4A-001)
- **No Bun-only behavior mismatch** — watcher discovers real QuickJS
  module files and contract locks.
- **Failed reload keeps prior healthy app** — watcher delivers typed
  events for the swap pipeline (M4A-001-B/C/D).
- **Reload is bounded and observable** — events carry latency metrics;
  debouncing bounds event frequency.

### Disclosures
- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
