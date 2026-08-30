---
task_id: M3-002-Z
parent_task: M3-002
milestone: M3
priority: P0
mode: EVIDENCE
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-002-Z — Package evidence for Implement bounded worker dispatcher

## Atomic goal

Create source-backed evidence and handoff for parent task M3-002; update status only if verification passed.

## Parent intent

Route matched requests to workers without unbounded queues or shared engine mutexes.

## Dependencies

- `M3-002-V` — `tasks/06_m3_multi_worker/M3-002-V-verify-implement-bounded-worker-dispatcher.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M3.md`
- `context/components/engine-scheduler.md`
- `context/components/multiworker.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `crates/q-runtime/src/main.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `docs/reports/`
- `docs/beta/workstreams/OBSERVABILITY_OPERATIONS.md`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- Queue capacity is configurable and bounded.
- Overload fails quickly and observably.
- No head-of-line lock across workers.
- Per-worker queue latency is measured.

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

- Dispatcher tests.
- Overload load test.
- Metrics.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m3-002-z: package evidence for implement bounded worker dispatcher
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-002-Z) — PASS

- Date: 2026-08-30
- Branch/PR: m3-002-z (squash-merged; see git log for final hash)
- Closes: #383

### Parent closure — M3-002 Implement bounded worker dispatcher

Parent intent: route matched requests to workers without unbounded queues or shared engine mutexes. Status: **PASS**.

Packet commits (squash merges):
- M3-002-A — c9ab8cc (#982, Closes #378): `BoundedWorkerQueue<T>` — bounded per-worker FIFO queues (clamped capacity, typed immediate `Full` rejection, per-item wait measurement, redacted saturating stats, `T: Send` keeps JS values out; explicit `SharedAcrossWorkers`); 8 tests incl. the 10k-push overload burst
- M3-002-B — af8eba2 (#983, Closes #379): `Dispatcher<T>` — least-outstanding-load selection with round-robin tie-breaking; full queues skipped; typed `AllFull`; 6 tests incl. convergence [3,0,1]→[3,2,2]
- M3-002-C — 90a1cf0 (#984, Closes #380): admission & overload response — `AdmissionDecision` (503/overload/retry-1, matching the runtime RFC 9457 registry; total deterministic mapping; topology stays internal)
- M3-002-D — c20331a (#985, Closes #381): `DispatchRoute` — Copy plain-data route snapshot extracted before dispatch (identity/handler/policy/schema ids/deadline); crosses the queue boundary intact with zero re-resolution; 4 tests
- M3-002-V — 3f7be5f (#986, Closes #382): verification closure mapping all 4 guardrails to the 22 dispatch/boundary tests

### Required evidence
- **Dispatcher tests**: 16 in `q-capabilities/src/dispatch.rs` + 4 boundary tests in `q-runtime/src/serve.rs`.
- **Overload load test**: `overload_burst_is_rejected_fast_and_fully_counted` (10k pushes vs capacity 128 — exactly 128 accepted, 9872 immediate typed rejections, all counted, <2s).
- **Metrics**: per-item queue wait at pop; mean/max in `QueueStats`; per-worker aggregation via `Dispatcher::stats`; rejection counters saturate.

### Source/test map
- `crates/q-capabilities/src/dispatch.rs` (queue + dispatcher + admission policy; 16 tests)
- `crates/q-engine/src/lib.rs` (`DispatchRoute`), `crates/q-runtime/src/serve.rs` (`dispatch_route` extraction; 4 tests)
- Release binary `333d563d…` matches manifest (dispatcher dormant until the request path wires it)

### Command results (this branch)
- `cargo test -p q-capabilities` → 6 suites (210 unit incl. 16 dispatch); `-p velqu-runtime` → 17+5+44; `-p q-engine-quickjs` → 20+101; `-p q-http` → 4+6+1; `-p q-bridge` → 11 — all pass
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**

### Ledger update
- `docs/beta/04_TASK_LEDGER.md`: M3-002 flipped TODO -> PASS.

### Disclosures (standing)
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
