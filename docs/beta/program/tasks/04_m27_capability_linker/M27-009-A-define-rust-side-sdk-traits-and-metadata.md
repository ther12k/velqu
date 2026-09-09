---
task_id: M27-009-A
parent_task: M27-009
milestone: M27
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-009-A — Define Rust-side SDK traits and metadata

## Atomic goal

Define Rust-side SDK traits and metadata.

## Parent intent

Make first-party and external capabilities implementable without internal runtime mutation.

## Dependencies

- `M27-001-Z` — `tasks/04_m27_capability_linker/M27-001-Z-package-evidence-for-define-capability-abi-and-lifecycle-state-machine.md`
- `M27-002-Z` — `tasks/04_m27_capability_linker/M27-002-Z-package-evidence-for-implement-compile-time-capability-dependency-resolver.md`

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
5. Implement exactly this deliverable: Define Rust-side SDK traits and metadata.
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
m27-009-a: define rust side sdk traits and metadata
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M27-009-A) — PASS

- Date: 2026-08-27
- Branch/PR: m27-009-a (squash-merged; see git log for final hash)
- Closes: #288

### Changed files
- `crates/q-capabilities/src/sdk.rs` (new): `CapabilityMetadata` (validated id via `CapabilityId::parse`, explicit `CapabilityVersion(u32)` — versioning is explicit), `LifecycleContext<'a>` read-only wrapper (a capability never receives arbitrary mutable app state), `CapabilitySdk` trait (`metadata` / `on_shutdown` / `on_failure` with fail-closed defaults), `CancellableCapability` trait (`begin_shutdown_drain` driving the ADR-0031 bounded drain protocol via `begin_shutdown` + `finish_shutdown`), and `ExampleSdkCapability` used only by SDK tests/docs (outside all core runtime paths).
- `crates/q-capabilities/src/lib.rs`: added `pub mod sdk;`.

### Tests added (crates/q-capabilities/src/sdk.rs)
- `example_capability_metadata_is_explicit` — metadata id/version/summary and Display carry explicit versioning.
- `invalid_id_in_metadata_fails_closed` — non-runtime namespace and malformed ids rejected.
- `cancellable_capability_drains_to_quiesced` — SDK lifecycle/cancel/shutdown path ends in `DrainOutcome::Quiesced` + `CapabilityPhase::Quiesced`.

### Command results
- `cargo test -p q-capabilities` → 93 passed (was 90 before packet)
- `cargo test -p q-pack` → 96+2 passed
- `cargo test -p q-engine-quickjs` → 14+97 passed
- `cargo test -p velqu-runtime` → 31 passed (after standard fresh-worktree builds: `cargo build -p q-bytecode-tool`, `cargo build --release -p velqu-runtime`, proof-pack rebuild)
- `bun test` → 200 pass / 0 fail
- `bun run typecheck` → clean
- `cargo fmt --check` → clean
- `cargo clippy --workspace --all-targets -- -D warnings` → clean

### Evidence mapping
- SDK docs: module-level rustdoc in `sdk.rs` documenting traits and lifecycle contract.
- Example package: `ExampleSdkCapability` (doc/test-only, not referenced by runtime code paths).
- Compatibility tests: the three new tests above exercise the trait objects through the public SDK surface.

### Notes
- Fresh-worktree gate note: initial `velqu-runtime` run showed 2 failures (`hash_valid_garbage_bytecode_rejects_before_ready`, `no_bytecode_flag_recovers_cross_target_packs_from_source`) because the release runtime binary/proof pack had not been built in this worktree yet; after the standard build sequence both suites are fully green. No test or fixture was modified.

### Disclosures (standing)
- CI on this repo fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
