---
task_id: M24-001-V
parent_task: M24-001
milestone: M24
priority: P0
mode: VERIFY
status: TODO
context_card: context/milestones/M24.md
commit_required: true
---

# M24-001-V — Verify Freeze ingress ownership and backpressure design

## Atomic goal

Prove every acceptance criterion for parent task M24-001 without broadening scope.

## Parent intent

Define ownership from Hyper ingress through routing, worker queue, slab lifetime, cancellation, and response completion.

## Dependencies

- `M24-001-A` — `tasks/01_m24_zero_copy_ingress/M24-001-A-accept-an-adr-with-ownership-diagrams-and-terminal-invariants.md`
- `M24-001-B` — `tasks/01_m24_zero_copy_ingress/M24-001-B-specify-body-ownership-queue-admission-disconnect-cancellation-and-request-slot.md`
- `M24-001-C` — `tasks/01_m24_zero_copy_ingress/M24-001-C-define-no-copy-and-bounded-copy-boundaries.md`
- `M24-001-D` — `tasks/01_m24_zero_copy_ingress/M24-001-D-define-overload-responses-and-metrics.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M24.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`

### Source files

- `AGENTS.md`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-router/src/lib.rs`
- `crates/q-pack/src/lib.rs`
- `packages/compiler/src/emit.ts`
- `conformance/routing/routing.conformance.test.ts`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- No request data is borrowed beyond its owner lifetime.
- Queue/body limits are explicit.
- Cancellation has one owner.
- Design supports one and multiple workers.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-router
```
```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-http
```
```bash
cargo test -p q-bridge
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

## Required evidence for this microtask

- ADR.
- State-machine tests plan.
- Threat/ownership review.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m24-001-v: verify freeze ingress ownership and backpressure design
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Verification blocker record

- Task ID: `M24-001-V`
- Status: `BLOCKED` (packet status remains `TODO`)
- Blocking fact: M24-002-A through M24-002-Z are now implemented and evidenced, so the earlier q-http eager-materialization blocker is resolved. Parent M24-001 still cannot pass because the frozen ownership spine requires worker-local request storage, complete single-owner settlement across disconnect/timeout/quarantine/shutdown, slab/permit lifecycle accounting, and the remaining M24-003 through M24-010 ownership proofs. The current bridge remains a process-wide `Mutex<Vec<Slot>>`, and the documented T6/T9/T10/T12 lifecycle extensions are not yet complete.
- Exact source locations: `crates/q-bridge/src/lib.rs:76-147` (process-wide request store); `docs/okf/decisions/0021-m24-zero-copy-ingress-ownership.md:154-172` (T1–T12 proof plan); `docs/specs/m24-ingress-ownership-and-admission.md:418-434` (D-T1–D-T8 ownership/metrics test plan).
- Source-backed evidence already available: M24-002-V/Z merged via PRs #644/#645; route-first, bounded URI/header/body admission, queue-full response, requestless bridge counters, routing/policy conformance, and OKF validation are recorded there. This does not prove worker-local slab ownership or the full M24-001 lifecycle gate.
- Exact prior verification results: M24-002 acceptance matrix passed targeted Rust suites, 35/35 Bun tests, typecheck, format, Clippy, and OKF validation (174 links, 0 errors). The repository-wide verifier retained the known temporary-worktree/canonical `qRuntimeRelease` benchmark hash limitation; no benchmark manifest was changed.
- Dependency or owner required: Complete and evidence M24-003 through M24-010 lifecycle, slab, decoding, body, observability, and fuzz/conformance work, then rerun M24-001-V. Keep `M24-001-V`, `M24-001-Z`, and `M24-GATE` TODO until those proofs exist.
- Safe work completed before stopping: audited the stale blocker against current master, preserved all status/index truth, and mapped the remaining gaps to ADR-0021 T6/T9/T10/T12 and the M24 specification’s D-T1–D-T8 plan.
- Files changed but not committed: this packet record only.
- Suggested next action: continue with the explicitly authorized M24-003 implementation only after the M24-001-Z dependency is owner-approved; do not mark M24-001-V/Z or M24-GATE PASS from this correction.
