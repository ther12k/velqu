---
task_id: M27-009-V
parent_task: M27-009
milestone: M27
priority: P1
mode: VERIFY
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-009-V — Verify Publish capability SDK and inspection surface

## Atomic goal

Prove every acceptance criterion for parent task M27-009 without broadening scope.

## Parent intent

Make first-party and external capabilities implementable without internal runtime mutation.

## Dependencies

- `M27-009-A` — `tasks/04_m27_capability_linker/M27-009-A-define-rust-side-sdk-traits-and-metadata.md`
- `M27-009-B` — `tasks/04_m27_capability_linker/M27-009-B-provide-test-harness-and-example-capability.md`
- `M27-009-C` — `tasks/04_m27_capability_linker/M27-009-C-expose-build-inspect-diagnostics.md`
- `M27-009-D` — `tasks/04_m27_capability_linker/M27-009-D-define-semver-abi-compatibility.md`

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

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m27-009-v: verify publish capability sdk and inspection surface
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M27-009-V) — PASS

- Date: 2026-08-27
- Branch/PR: m27-009-v (squash-merged; see git log for final hash)
- Closes: #292

### Acceptance-criterion mapping (parent M27-009 guardrails)

1. **Capability does not receive arbitrary mutable app state** — source: `crates/q-capabilities/src/sdk.rs` (`CapabilitySdk`/`CancellableCapability` take only `&self` + `&mut CapabilityLifecycle`; `LifecycleContext` is a read-only wrapper); negative proof: `assert_ops_gate_fails_closed` / `ops_gate_rejects_start_outside_ready` (typed `NotReady`, never mutated state).
2. **SDK tests lifecycle/cancel/shutdown** — `full_lifecycle_battery_reports_quiesced`, `expired_drain_battery_fails_closed` (`DeadlineExceeded{pending:1}` + `Failed` terminal, never a late Quiesced), `cancellable_capability_drains_to_quiesced` (all in `harness.rs` / `sdk.rs`).
3. **Versioning is explicit** — `example_capability_metadata_is_explicit`, `invalid_id_in_metadata_fails_closed`, diagnostics `version_mismatch_fails_closed` / `missing_metadata_fails_closed`, compat `sdk_abi_revision_is_explicit` + selector policy tests (`crates/q-capabilities/src/{sdk,diagnostics,compat}.rs`), ADR-0032.
4. **Example capability remains outside core** — `crates/q-capabilities/examples/example_capability.rs` is a cargo example target only; no core/runtime path references it; re-ran green in this packet (see output below).

### Verification runs (this branch, worktree-fresh)
- `cargo test -p q-capabilities` → 107 passed
- `cargo test -p q-pack` → 96+2 passed
- `cargo test -p q-engine-quickjs` → 14+97 passed
- `cargo test -p velqu-runtime` → 31 passed
- `bun test` → 200 pass / 0 fail
- `bun run typecheck` → clean
- `cargo fmt --check` → clean
- `./scripts/verify` → **ALL PASS (exit 0)** — includes workspace clippy `-D warnings`, independent-build reproducibility (12 artifacts byte-identical, app.qpack 5329b73d…), benchmark artifact validation with no errors.

### Example run (re-verified)
```
graceful: LifecycleReport { id: "runtime:example", version: 1, ready_phase: Ready, drain_outcome: "quiesced", terminal_phase: Quiesced }
expired:  LifecycleReport { id: "runtime:example", version: 1, ready_phase: Ready, drain_outcome: "deadline-exceeded", terminal_phase: Failed }
ops gate: fails closed outside Ready ✓
inspect: 1 capabilities linked
inspect: runtime:example@1 — example greeter capability
```

### Defect fixed in this packet (disclosed)
- `crates/q-capabilities/src/harness.rs:106`: clippy `redundant_guards` (`if pending == 1` → pattern `{ pending: 1 }`). First `./scripts/verify` run on this branch failed `FAIL: cargo clippy`; after the one-line fix the full verify passes. Root cause of the earlier miss: the per-packet gate command piped clippy through `tail`, so the shell saw `tail`'s exit status and masked clippy failures in B/C/D local runs. This V packet caught it through the full verify gate; going forward clippy exit status is checked explicitly (`set -o pipefail`). No test or fixture was weakened.

### Disclosures (standing)
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
