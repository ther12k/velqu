---
task_id: M27-002-V
parent_task: M27-002
milestone: M27
priority: P0
mode: VERIFY
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-002-V — Verify Implement compile-time capability dependency resolver

## Atomic goal

Prove every acceptance criterion for parent task M27-002 without broadening scope.

## Parent intent

Resolve exactly which capabilities enter each application artifact.

## Dependencies

- `M27-002-A` — `tasks/04_m27_capability_linker/M27-002-A-build-dependency-dag.md`
- `M27-002-B` — `tasks/04_m27_capability_linker/M27-002-B-reject-cycles-missing-conflicting-versions.md`
- `M27-002-C` — `tasks/04_m27_capability_linker/M27-002-C-emit-capability-inventory-hash-into-qpack.md`
- `M27-002-D` — `tasks/04_m27_capability_linker/M27-002-D-remove-unused-modules.md`

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
- `packages/cli/src/index.ts`
- `crates/q-runtime/src/source_map.rs`
- `examples/proof/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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
cargo test -p velqu-runtime
```
```bash
bun test
```
```bash
bun run typecheck
```
```bash
cargo fmt --check
```
```bash
cargo clippy --workspace --all-targets -- -D warnings
```
```bash
./scripts/verify
```

## Required evidence for this microtask

- Resolver tests.
- Binary-size delta report.
- Cold-start delta report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m27-002-v: verify implement compile time capability dependency resolver
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Verification record — M27-002-V (PASS)

Parent: M27-002 "Implement compile-time capability dependency
resolver". All four implementation packets merged before this
branch: M27-002-A (PR #848, #246), M27-002-B (PR #849, #247),
M27-002-C (PR #850, #248), M27-002-D (PR #851, #249).

### Acceptance criterion mapping (parent guardrails)

1. **Unrelated app pays zero linked capability cost.**
   `resolveLinkedModules([])` → `[]` and `empty_roots_resolve_to_
   empty_closure`; an app with no capability-granting routes emits a
   count-prefix-only canonical inventory (`00000000`, hash-pinned)
   whose section presence is structurally identical to pre-M27
   packs; measured before/after (m27-002-d-prune-deltas.md): the
   zero-link side's bytes unchanged, +56 B only when a module is
   actually linked.

2. **Dependency graph is deterministic.** Rust:
   `closure_is_deterministic_regardless_of_input_order`,
   `diamond_dependency_resolves_to_one_entry`, id-sorted output;
   TS: resolver-mirroring prune is Map+sort based with pinned
   vectors; cross-language hash equality pinned on both sides
   (`canonical_hash_matches_cross_language_vectors` /
   capability-inventory.test.ts) — which caught and fixed a real
   encoding-order bug in C.

3. **Missing capability fails at build or startup.** Build time:
   typed `Missing`/`VersionConflict` on root and transitive edges,
   cycles rejected with full path (B), unknown grants fail the
   build naming the grant (D). Runtime side stays fail-closed via
   q-pack verify() rejecting hash-bound inventory mismatches
   (`capability_inventory_section_is_hash_bound_and_canonical`).

4. **`velqu inspect --capabilities` is accurate.** Reads the pack's
   actual inventory, recomputes the canonical hash, fails loud on
   mismatch/unsorted, reports pre-inventory packs honestly
   (`unknown`); 5 dedicated tests in
   capability-inventory.test.ts.

### Required evidence

- Resolver tests: 51 in q-capabilities (lifecycle 7 + identity 9 +
  operations 7 + shutdown 7 + resolver 13 + inventory 8), plus 16
  TS-side (prune 3, inspect accuracy 5, vectors 2, detection 4, C
  suite 2 existing).
- Binary-size delta report:
  `docs/reports/m27-002-d-prune-deltas.md`.
- Cold-start delta report: same document, raw samples retained.

### Manifest refresh (matched evidence)

First verify run failed validate-benchmark-evidence exactly as it
should: C/D changed the compiled proof pack (timers inventory now
present — compare-builds confirms byte-identical independent
rebuilds at 5329b73…) so the recorded proofPack/qRuntimeRelease
hashes were stale. Refreshed per the established procedure under
verify's exact remap environment; diff is two artifact hashes +
generatedAt/commit metadata, zero raw-data changes.

### Changed files

- This task record; `benchmarks/manifest.json` (matched refresh
  above). No defects found in A–D beyond those fixed within them.

### Commands and results (fresh worktree on parent HEAD 17d4491)

- `cargo test -p q-pack` 98 · `-p q-engine-quickjs` 98 ·
  `-p q-capabilities` 51 · `-p velqu-runtime` 30 — pass.
- `bun test` 139 pass / 0 fail; `bun run typecheck`,
  `cargo fmt --check`, `cargo clippy --workspace --all-targets --
  -D warnings` — clean.
- `./scripts/verify` — ALL PASS (exit 0) after the matched manifest
  refresh.
