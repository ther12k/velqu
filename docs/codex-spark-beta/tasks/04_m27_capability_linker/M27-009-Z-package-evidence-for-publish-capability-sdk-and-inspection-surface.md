---
task_id: M27-009-Z
parent_task: M27-009
milestone: M27
priority: P1
mode: EVIDENCE
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-009-Z — Package evidence for Publish capability SDK and inspection surface

## Atomic goal

Create source-backed evidence and handoff for parent task M27-009; update status only if verification passed.

## Parent intent

Make first-party and external capabilities implementable without internal runtime mutation.

## Dependencies

- `M27-009-V` — `tasks/04_m27_capability_linker/M27-009-V-verify-publish-capability-sdk-and-inspection-surface.md`

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
- `packages/treaty/src/index.ts`
- `packages/contract/src/index.ts`
- `packages/testing/src/index.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

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

- SDK docs.
- Example package.
- Compatibility tests.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m27-009-z: package evidence for publish capability sdk and inspection s
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M27-009-Z) — PASS

- Date: 2026-08-27
- Branch/PR: m27-009-z (squash-merged; see git log for final hash)
- Closes: #293

### Parent closure — M27-009 Publish capability SDK and inspection surface

Parent intent: make first-party and external capabilities implementable without internal runtime mutation. Status: **PASS**.

Packet commits (squash merges):
- M27-009-A — ff06be8 (#890, Closes #288): SDK traits + explicit metadata (`crates/q-capabilities/src/sdk.rs`)
- M27-009-B — 2caab89 (#891, Closes #289): test harness + example capability (`src/harness.rs`, `examples/example_capability.rs`)
- M27-009-C — e54c325 (#892, Closes #290): build/inspect diagnostics (`src/diagnostics.rs`)
- M27-009-D — 9fd9e05 (#893, Closes #291): semver/ABI policy + ADR-0032 (`src/compat.rs`)
- M27-009-V — 46a9a9c (#894, Closes #292): verification closure incl. clippy `redundant_guards` fix in `harness.rs`

### Evidence ledger (required microtask evidence)
- **SDK docs**: rustdoc on all public items of `sdk.rs`, `harness.rs`, `diagnostics.rs`, `compat.rs`; policy statement in module docs; ADR-0032 under `docs/okf/decisions/`.
- **Example package**: `crates/q-capabilities/examples/example_capability.rs` — cargo example target outside core; output verified green in M27-009-V (graceful Quiesced, expired fail-closed, ops gate typed rejection, inspect rows).
- **Compatibility tests** (names live in the listed modules): sdk: `example_capability_metadata_is_explicit`, `invalid_id_in_metadata_fails_closed`, `cancellable_capability_drains_to_quiesced`; harness: `full_lifecycle_battery_reports_quiesced`, `expired_drain_battery_fails_closed`, `ops_gate_rejects_start_outside_ready`; diagnostics: `collect_joins_inventory_with_sdk_metadata`, `version_mismatch_fails_closed`, `missing_metadata_fails_closed`, `empty_inventory_renders_zero_modules`; compat: `parts_pack_and_roundtrip`, `out_of_range_components_fail_closed`, `unpack_rejects_versions_above_component_ceiling`, `major_changes_break_abi_within_major_is_compatible`, `exact_selector_matches_only_identical_version`, `compatible_selector_follows_semver_policy`, `sdk_abi_revision_is_explicit`.

### Command results (this branch)
- `cargo test -p q-capabilities` → 107 passed
- `cargo test -p q-pack` → 96+2 passed
- `cargo test -p q-engine-quickjs` → 14+97 passed
- `cargo test -p velqu-runtime` → 31 passed
- `bun test` → 200 pass / 0 fail
- `bun run typecheck` → clean
- `cargo fmt --check` → clean
- `cargo clippy --workspace --all-targets -- -D warnings` → exit 0 (checked explicitly with `set -o pipefail`)
- `./scripts/verify` → **ALL PASS (exit 0)**

### Notes
- Bookkeeping-only packet: no runtime code changed after M27-009-V; all suites re-run here on this branch as recorded above.
- Standing disclosure: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side).
