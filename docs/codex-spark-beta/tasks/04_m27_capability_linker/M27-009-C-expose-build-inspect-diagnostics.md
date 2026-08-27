---
task_id: M27-009-C
parent_task: M27-009
milestone: M27
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-009-C — Expose build/inspect diagnostics

## Atomic goal

Expose build/inspect diagnostics.

## Parent intent

Make first-party and external capabilities implementable without internal runtime mutation.

## Dependencies

- `M27-009-B` — `tasks/04_m27_capability_linker/M27-009-B-provide-test-harness-and-example-capability.md`

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
- `packages/treaty/src/index.ts`
- `packages/contract/src/index.ts`
- `packages/testing/src/index.ts`
- `conformance/treaty/treaty.conformance.test.ts`
- `crates/q-pack/src/lib.rs`
- `packages/cli/src/index.ts`
- `crates/q-runtime/src/source_map.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Expose build/inspect diagnostics.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Capability does not receive arbitrary mutable app state.
- SDK tests lifecycle/cancel/shutdown.
- Versioning is explicit.
- Example capability remains outside core.

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

## Required evidence for this microtask

- SDK docs.
- Example package.
- Compatibility tests.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m27-009-c: expose build inspect diagnostics
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M27-009-C) — PASS

- Date: 2026-08-27
- Branch/PR: m27-009-c (squash-merged; see git log for final hash)
- Closes: #290

### Changed files
- `crates/q-capabilities/src/diagnostics.rs` (new): build/inspect diagnostics surface — read-only `CapabilityDiagnostics::collect(inventory, registry)` joins the resolved pack inventory (`CapabilityInventory`) with registered SDK metadata (`CapabilityMetadata`), failing closed typed on `MissingMetadata` or `VersionMismatch` (explicit versioning everywhere); renders `lines()` (`id@version — summary` rows in inventory order) and `summary()` header. No lifecycle state is mutated by collection.
- `crates/q-capabilities/src/lib.rs`: added `pub mod diagnostics;` + re-exports.
- `crates/q-capabilities/examples/example_capability.rs`: extended with the inspect-surface demo (inventory joined with SDK metadata, rendered rows) — example remains a cargo target outside core.

### Tests added (crates/q-capabilities/src/diagnostics.rs)
- `collect_joins_inventory_with_sdk_metadata`
- `version_mismatch_fails_closed`
- `missing_metadata_fails_closed`
- `empty_inventory_renders_zero_modules`

### Example run output (excerpt)
```
inspect: 1 capabilities linked
inspect: runtime:example@1 — example greeter capability
```

### Command results
- `cargo test -p q-capabilities` → 100 passed (+4 over M27-009-B)
- `cargo test -p q-pack` → 96+2 passed
- `cargo test -p q-engine-quickjs` → 14+97 passed
- `cargo test -p velqu-runtime` → 31 passed (after standard fresh-worktree builds)
- `bun test` → 200 pass / 0 fail
- `bun run typecheck` → clean
- `cargo fmt --check` → clean
- `cargo clippy --workspace --all-targets -- -D warnings` → clean

### Evidence mapping
- SDK docs: rustdoc on all public items in `diagnostics.rs`.
- Example package: inspect demo added to the cargo example.
- Compatibility tests: four new diagnostics tests over existing public APIs.

### Disclosures (standing)
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
