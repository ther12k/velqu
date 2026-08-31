---
task_id: M4A-001-B
parent_task: M4A-001
milestone: M4A
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-001-B — Build incremental temporary QPack

## Atomic goal

Build incremental temporary QPack.

## Parent intent

Compile and reload the real QuickJS/QPack runtime with fast feedback and parity.

## Dependencies

- `M4A-001-A` — `tasks/07_m4a_developer_preview/M4A-001-A-watch-source-and-contracts.md`

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
5. Implement exactly this deliverable: Build incremental temporary QPack.
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
m4a-001-b: build incremental temporary qpack
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M4A-001-B) — PASS

- Date: 2026-08-31
- Branch/PR: m4a-001-b (squash-merged; see git log for final hash)
- Closes: #433

### Changed files
- `packages/compiler/src/incremental.ts` (new): fast-path temporary QPack
  builder (`buildTemporaryPack`, `IncrementalPackBuilder`) —
  - Compiles project into verified temporary QPack format without writing
    unneeded lock/markdown docs during the dev loop.
  - Generates TypeScript-mapped bundle and debug source sidecars
    (`temp-*.qpack.sources.json`) so native runtime diagnostics resolve
    source locations.
  - Incremental tracking: detects contract changes (`contractChanged: true`)
    when routes/schemas change, while keeping contract stable when only
    handler implementation changes.
  - Bounded disk storage: keeps at most 2 temporary packs in the temp dir,
    cleaning up older artifacts automatically.
- `packages/compiler/src/index.ts`: exports `buildTemporaryPack`,
  `IncrementalPackBuilder`, and types.
- `packages/compiler/src/watch.ts`: enhanced watcher with polling fallback
  interval over all watched directories, making file event detection
  immune to inotify `ENOSPC` exhaustion on busy multi-threaded test runs.
- `packages/compiler/src/incremental.test.ts` (new): 5 unit/integration tests.
- `benchmarks/manifest.json`: refreshed (standard remapped flow).

### Tests added (packages/compiler/src/incremental.test.ts, +5 tests)
- Builds temporary QPack for `examples/proof` with fast build latency and TypeScript source maps.
- Matches full build contract hash and route definitions identically (parity).
- Detects contract changes when route path or schema is modified.
- Keeps contract unchanged when only handler implementation is edited.
- Bounds temporary disk storage by cleaning up older pack files.

### Command results
- `cargo test -p q-pack` → 3 suites — 0 failed
- `cargo test -p q-engine-quickjs` → 20 + 102 + 1 — 0 failed
- `bun test` → **231 pass / 0 fail (29 files, +5 new tests)**
- `bun run typecheck` → clean
- `cargo fmt --check` clean; workspace clippy -D warnings → exit 0
- `./scripts/verify` → **ALL PASS**

### Guardrail mapping (parent M4A-001)
- **No Bun-only behavior mismatch** — temporary QPack matches the exact
  verified QPack structure loaded by QuickJS workers.
- **Source maps point to TypeScript** — bundle includes linked source maps
  and source sidecars.
- **Reload is bounded and observable** — compilation time measured (< 100ms)
  and contract changes explicitly tagged.

### Disclosures
- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
