---
task_id: M27-002-A
parent_task: M27-002
milestone: M27
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-002-A — Build dependency DAG

## Atomic goal

Build dependency DAG.

## Parent intent

Resolve exactly which capabilities enter each application artifact.

## Dependencies

- `M27-001-Z` — `tasks/04_m27_capability_linker/M27-001-Z-package-evidence-for-define-capability-abi-and-lifecycle-state-machine.md`

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

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Build dependency DAG.
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
m27-002-a: build dependency dag
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M27-002-A (PASS)

Deliverable: the capability dependency DAG builder.

### Changed files

- `crates/q-capabilities/src/resolver.rs` — new module:
  `resolve_closure(roots, universe) -> DependencyDag`. Transitive
  closure over descriptors; every edge resolved with ADR-0029
  semantics (exact version, typed `Missing`/`VersionConflict`
  naming id + both versions); output sorted by capability id so
  root order and universe order cannot change it; visited-once
  walk guarantees termination on any graph shape (cycle *rejection*
  is M27-002-B — the termination test is B's flip anchor);
  first-descriptor-wins duplicate rule pinned per ADR-0029 §4.
- `crates/q-capabilities/src/lib.rs` — `pub mod resolver` +
  re-exports (`resolve_closure`, `DependencyDag`).
- `docs/codex-spark-beta/STATUS.md`, `indexes/TASK_INDEX.md` — this
  packet PASS.

### Tests (resolver evidence)

`cargo test -p q-capabilities` — 40 passed (30 prior + 10
resolver): `transitive_closure_includes_all_levels` (unreachable
capabilities excluded), `closure_is_deterministic_regardless_of_input_order`
(shuffled roots + reversed universe → byte-identical result),
`diamond_dependency_resolves_to_one_entry`, `duplicate_roots_dedupe`,
`empty_roots_resolve_to_empty_closure` (guardrail: unrelated app
links nothing), `missing_root_capability_fails_typed`,
`missing_transitive_dependency_fails_typed`,
`version_conflict_on_any_edge_fails_typed_with_versions` (root and
transitive edges), `walk_terminates_on_cycles`,
`deterministic_first_descriptor_wins_on_duplicate_ids`.

### Size / cold-start delta reports

None produced in A, by construction: `q-capabilities` is not yet a
dependency of any production binary (verified: no crate in the
workspace depends on it), so the release artifacts are
byte-identical and startup behavior is unchanged. Measured delta
reports belong to the packets that change real artifacts — C
(inventory hash into the pack) and D (unused-module removal). No
zero-filled report files are fabricated for A.

### Commands (fresh worktree on M27-001-Z HEAD ed268f2)

- `cargo test -p q-pack` 96 · `-p q-engine-quickjs` 98 ·
  `-p q-capabilities` 40 — pass.
- `bun test` 125 pass / 0 fail (after worktree setup: release
  velqu-runtime, velqu-bytecode, proof pack, baseline installs);
  `bun run typecheck` clean.
- `cargo fmt --check`, `cargo clippy -p q-capabilities --all-targets
  -- -D warnings` — clean.

### Notes

- Guardrail mapping: deterministic graph →
  `closure_is_deterministic_regardless_of_input_order` +
  id-sorted output; missing capability fails at build →
  `missing_root_capability_fails_typed` /
  `missing_transitive_dependency_fails_typed`; zero-cost for
  unrelated apps → `empty_roots_resolve_to_empty_closure`. The
  `velqu inspect --capabilities` guardrail is delivered with the
  inventory emission (C).
