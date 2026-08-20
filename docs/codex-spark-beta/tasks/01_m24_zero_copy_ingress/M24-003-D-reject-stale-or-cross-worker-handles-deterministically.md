---
task_id: M24-003-D
parent_task: M24-003
milestone: M24
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M24.md
commit_required: true
---

# M24-003-D — Reject stale or cross-worker handles deterministically

## Atomic goal

Reject stale or cross-worker handles deterministically.

## Parent intent

Eliminate the global request-store mutex and keep lazy request access worker-owned.

## Dependencies

- `M24-003-C` — `tasks/01_m24_zero_copy_ingress/M24-003-C-invalidate-at-settlement-timeout-cancellation-quarantine-and-shutdown.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M24.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`

### Source files

- `AGENTS.md`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Reject stale or cross-worker handles deterministically.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- No process-wide request-store mutex on normal access.
- Stale handles always fail.
- No request slot leaks after terminal paths.
- No JS value or request slot crosses worker ownership.

## Targeted commands

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

## Required evidence for this microtask

- Race tests.
- Slot accounting metrics.
- Fuzzed stale-handle operations.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m24-003-d: reject stale or cross worker handles deterministically
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record

- Status: **PASS**
- Deliverable: deterministic stale and cross-worker handle rejection. `BridgeError::ForeignWorker` is the dedicated denial for a capability whose worker identity does not match the slab, decided before any slot is inspected — even when the foreign slab has a live slot at the same index with colliding numbers. The M24-003-B typed handle keeps worker identity out of JavaScript (the prelude pair stays numeric; `local_handle` stamps the owning worker), so a foreign handle can only arrive via Rust; access fails closed with `ForeignWorker` and settle from a foreign worker remains a no-op that touches neither slab. A bounded deterministic-LCG corpus (2048 arbitrary `(worker, slot, generation)` triples across two slabs) proves no forged operation reads request bytes, frees a live slot, or perturbs the valid handle's accounting; the exact live handle keeps working throughout.
- Changed files:
  - `crates/q-bridge/src/lib.rs` (ForeignWorker variant; access pre-check; decoy-slot foreign test; fuzz corpus test)
  - `crates/q-engine-quickjs/tests/engine.rs` (ADR-0021 T11 cross-worker inertness proof across two live engines)
  - `docs/codex-spark-beta/tasks/01_m24_zero_copy_ingress/M24-003-D-reject-stale-or-cross-worker-handles-deterministically.md`, `docs/codex-spark-beta/STATUS.md`, `docs/codex-spark-beta/indexes/TASK_INDEX.md`
- Tests: new `fuzzed_handle_triples_fail_closed_without_side_effects` (q-bridge), strengthened `typed_handle_from_foreign_worker_is_denied_before_slot_lookup` (decoy live slot at the same index; denial materializes zero bytes and settles nothing), new `cross_worker_handle_is_inert_on_foreign_worker` (engine, T11: worker B's settle of worker A's handle is a no-op at both slabs; the owner settles exactly once). Existing stale/reuse proofs remain green: `settlement_expires_handle_and_reuse_is_isolated`, `stale_handle_corpus_never_reads_or_leaks`, `expired_handle_access_fails_deterministically`.
- Verification: `cargo test -p q-engine-quickjs` PASS (1 unit + 90 engine); `cargo test -p q-http` PASS (2 + 3); `cargo test -p q-bridge` PASS (9); `cargo test -p velqu-runtime` PASS (13 conformance); `cargo fmt --check` PASS; `cargo clippy --workspace --all-targets -- -D warnings` PASS. Raw logs: `/tmp/m24-003-d-engine.log`, `/tmp/m24-003-d-http.log`, `/tmp/m24-003-d-bridge.log`, `/tmp/m24-003-d-runtime.log`, `/tmp/m24-003-d-clippy.log`.
- Acceptance criteria proven: no process-wide request-store mutex on any access path; stale handles always fail (corpus + reuse isolation + engine expired-handle proof); no request slot leaks after terminal paths (M24-003-C sweeps, still asserted green); no JS value or request slot crosses worker ownership (typed worker-stamped handles, cross-worker inertness proven at both slabs).
- Remaining risk / deferred by design: M3 multi-worker instantiation exercises the same rules across N real workers; the numeric JS ABI deliberately carries no worker dimension until then.
- Next dependency-ready task: M24-003-V (verify Implement worker-local generation-checked request slab).

