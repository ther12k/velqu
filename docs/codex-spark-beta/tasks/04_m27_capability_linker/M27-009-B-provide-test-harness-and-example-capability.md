---
task_id: M27-009-B
parent_task: M27-009
milestone: M27
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-009-B — Provide test harness and example capability

## Atomic goal

Provide test harness and example capability.

## Parent intent

Make first-party and external capabilities implementable without internal runtime mutation.

## Dependencies

- `M27-009-A` — `tasks/04_m27_capability_linker/M27-009-A-define-rust-side-sdk-traits-and-metadata.md`

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
5. Implement exactly this deliverable: Provide test harness and example capability.
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
m27-009-b: provide test harness and example capability
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M27-009-B) — PASS

- Date: 2026-08-27
- Branch/PR: m27-009-b (squash-merged; see git log for final hash)
- Closes: #289

### Changed files
- `crates/q-capabilities/src/harness.rs` (new): SDK test harness — `run_full_lifecycle` (install/activate → Ready → graceful `begin_shutdown_drain`, requires Quiesced), `run_expired_drain` (Ready → one pending NonCancellable op stands in for in-flight work → `finish_shutdown(deadline_fired=true)` requires `DeadlineExceeded{pending:1}` + Failed terminal phase — fail closed, never a late Quiesced), `assert_ops_gate_fails_closed` (NativeOp start outside Ready must return typed `NotReady`), and machine-checkable `LifecycleReport { id, version, ready_phase, drain_outcome, terminal_phase }`.
- `crates/q-capabilities/src/lib.rs`: added `pub mod harness;` + `pub use harness::{run_expired_drain, run_full_lifecycle, LifecycleReport};`.
- `crates/q-capabilities/examples/example_capability.rs` (new): example capability package as a cargo example target (outside core library/production runtime paths); authors an external-style capability via public API only (`CapabilityMetadata::new("runtime:example", 1, …)`), implements both SDK traits with a fail-closed `on_shutdown`, and runs the full harness battery + ops-gate check.

### Tests added (crates/q-capabilities/src/harness.rs)
- `full_lifecycle_battery_reports_quiesced`
- `expired_drain_battery_fails_closed`
- `ops_gate_rejects_start_outside_ready`

### Example run output (cargo run -p q-capabilities --example example_capability)
```
metadata: runtime:example@1 — example greeter capability
graceful: LifecycleReport { id: "runtime:example", version: 1, ready_phase: Ready, drain_outcome: "quiesced", terminal_phase: Quiesced }
expired:  LifecycleReport { id: "runtime:example", version: 1, ready_phase: Ready, drain_outcome: "deadline-exceeded", terminal_phase: Failed }
ops gate: fails closed outside Ready ✓
```

### Command results
- `cargo test -p q-capabilities` → 96 passed (+3 over M27-009-A)
- `cargo test -p q-pack` → 96+2 passed
- `cargo test -p q-engine-quickjs` → 14+97 passed
- `cargo test -p velqu-runtime` → 31 passed (after standard fresh-worktree builds)
- `bun test` → 200 pass / 0 fail
- `bun run typecheck` → clean
- `cargo fmt --check` → clean
- `cargo clippy --workspace --all-targets -- -D warnings` → clean

### Guardrail mapping
- Capability does not receive arbitrary mutable app state: harness passes only `&C` and `&mut CapabilityLifecycle`; no app-state surface exists in the SDK traits.
- SDK tests lifecycle/cancel/shutdown: harness batteries cover Ready gating, graceful drain to Quiesced, expired-drain DeadlineExceeded→Failed, and typed ops-gate rejection.
- Versioning is explicit: report carries `version` from explicit metadata; Display shows `runtime:example@1`.
- Example capability remains outside core: cargo example target only; core lib unchanged except module/re-export wiring.

### Evidence mapping
- SDK docs: rustdoc on every public item in `harness.rs`.
- Example package: `crates/q-capabilities/examples/example_capability.rs` (runs green end-to-end).
- Compatibility tests: three new harness tests + existing A-packet SDK tests.

### Disclosures (standing)
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
