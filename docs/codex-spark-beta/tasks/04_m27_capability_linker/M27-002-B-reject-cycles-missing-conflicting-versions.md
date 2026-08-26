---
task_id: M27-002-B
parent_task: M27-002
milestone: M27
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-002-B — Reject cycles/missing/conflicting versions

## Atomic goal

Reject cycles/missing/conflicting versions.

## Parent intent

Resolve exactly which capabilities enter each application artifact.

## Dependencies

- `M27-002-A` — `tasks/04_m27_capability_linker/M27-002-A-build-dependency-dag.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M27.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-pack/src/lib.rs`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `docs/specs/pack-format-v1.md`
- `scripts/package`
- `scripts/release-packet`
- `packages/cli/package.json`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Reject cycles/missing/conflicting versions.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Unrelated app pays zero linked capability cost.
- Dependency graph is deterministic.
- Missing capability fails at build or startup.
- `velqu inspect --capabilities` is accurate.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-engine-quickjs
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

- Resolver tests.
- Binary-size delta report.
- Cold-start delta report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m27-002-b: reject cycles missing conflicting versions
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M27-002-B (PASS)

Deliverable: cycle rejection in the capability dependency resolver,
alongside the already-typed missing/conflicting-version failures
(M27-002-A).

### Changed files

- `crates/q-capabilities/src/resolver.rs` — `resolve_closure` now
  runs an iterative DFS with an explicit path stack
  (`in_path`/`path_ids`): O(1) back-edge detection and a
  human-readable cycle path in the error. Nodes commit to the
  resolved set only after all children commit, preserving the
  deterministic id-sorted output. The A-era termination-only test
  was flipped to rejection as planned (it was B's designated
  anchor).
- `crates/q-capabilities/src/identity.rs` — `ResolveError::Cycle {
  path }` variant: typed, carries the full id path in traversal
  order, Display renders `a -> b -> a`.
- `docs/codex-spark-beta/STATUS.md`, `indexes/TASK_INDEX.md` — this
  packet PASS.

### Tests

`cargo test -p q-capabilities` — 43 passed (40 prior + 4 new; one
flipped):
`cycle_is_rejected_with_typed_error_naming_path` (flipped from
`walk_terminates_on_cycles`),
`self_cycle_is_rejected`,
`longer_cycle_path_names_all_members` (a→b→c→a),
`cycle_error_message_contains_arrow_path`.
Missing/conflicting-version rejection was already pinned in A
(`missing_root_capability_fails_typed`,
`missing_transitive_dependency_fails_typed`,
`version_conflict_on_any_edge_fails_typed_with_versions`) and still
passes unchanged against the new DFS.

### Commands (fresh worktree on M27-002-A HEAD 0ea56de)

- `cargo test -p q-pack` 96 · `-p q-engine-quickjs` 98 ·
  `-p q-capabilities` 43 — pass.
- `bun test` 125 pass / 0 fail; `bun run typecheck` clean.
- `cargo fmt --check`, `cargo clippy --workspace --all-targets --
  -D warnings` — clean.

### Notes

- Guardrail mapping: dependency graph deterministic → A's
  order-shuffle determinism tests still pass against the DFS;
  missing capability fails at build → typed Missing on root and
  transitive edges (unchanged); cycles now fail at build the same
  way. Zero-cost/inspect guardrails land with C/D.
