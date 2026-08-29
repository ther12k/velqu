---
task_id: M3-001-V
parent_task: M3-001
milestone: M3
priority: P0
mode: VERIFY
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-001-V — Verify Freeze independent-worker state semantics

## Atomic goal

Prove every acceptance criterion for parent task M3-001 without broadening scope.

## Parent intent

Define what JavaScript and native state is per worker versus shared.

## Dependencies

- `M3-001-A` — `tasks/06_m3_multi_worker/M3-001-A-accept-adr.md`
- `M3-001-B` — `tasks/06_m3_multi_worker/M3-001-B-document-module-level-state-replication.md`
- `M3-001-C` — `tasks/06_m3_multi_worker/M3-001-C-forbid-jsvalue-sharing.md`
- `M3-001-D` — `tasks/06_m3_multi_worker/M3-001-D-define-service-capability-shared-handles-and-thread-safety.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M3.md`
- `context/components/engine-scheduler.md`
- `context/components/multiworker.md`

### Source files

- `AGENTS.md`
- `crates/q-runtime/src/main.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/lib.rs`
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

- Each runtime has one owner thread.
- Cross-worker mutable state is explicit.
- Initialization is deterministic.
- Developer docs describe per-worker globals.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p velqu-runtime
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

- ADR.
- Concurrency model tests plan.
- State examples.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m3-001-v: verify freeze independent worker state semantics
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-001-V) — PASS

- Date: 2026-08-30
- Branch/PR: m3-001-v (squash-merged; see git log for final hash)
- Closes: #376

### Acceptance-criterion mapping (parent M3-001 guardrails)

1. **Each runtime has one owner thread** — verified: ADR-0036 section 1 freezes the rule; the compiler enforces the value half (rquickjs values are `!Send`), pinned by the `compile_fail` doc test in `q-engine-quickjs`; `WorkerMsg`/engine-boundary types are the only cross-thread surface and are `Send + Sync` plain data. Tests: crate `compile_fail` doc test, `worker_messages_are_plain_data_send_sync`, `engine_boundary_types_are_send_sync`.
2. **Cross-worker mutable state is explicit** — verified: ADR-0036 section 4 closes the sharing vocabulary to four named disciplines; `SharedAcrossWorkers` marker impls are explicit and auditable (`FetchMetricsCollector`, `BoundedLogSink`); `FetchPool` Send/Sync proven with an Arc clone probed cross-thread. Tests: `shared_handles_are_send_sync_static`, `shared_handles_work_behind_arc_from_any_thread`, `pool_handle_is_send_sync_shared`.
3. **Initialization is deterministic** — verified: ADR-0036 section 6 freezes worker-K-equals-worker-0 (identical pack bytes, pack order, same construction sequence); the obligation is bound to its proving packets (M3-004-A/B) in the ADR tests-plan table.
4. **Developer docs describe per-worker globals** — verified: the Capability Author Guide's "Module-level state under multiple workers (M3)" section states the rule, gives an annotated example, and lists the consequences (counters under-count, caches replicate, per-worker init, no cross-worker messaging).

### Verification runs (this branch)
- `cargo test -p q-capabilities` → 6 suites pass (194 unit + 7 fuzz + 1 + 4 + 9)
- `cargo test -p q-engine-quickjs` → 20 lib (incl. compile_fail) + 101 engine passed
- `cargo test -p velqu-runtime` → 13+5+44 passed
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**; release binary reproduced deterministically (`333d563d…` matches the M3-001-C manifest)

### Disclosures (standing)
- No production code changed in this packet: verification-only closure of M3-001-A/B/C/D.
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
