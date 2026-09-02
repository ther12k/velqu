---
task_id: M4A-007-Z
parent_task: M4A-007
milestone: M4A
priority: P0
mode: EVIDENCE
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-007-Z — Package evidence for Implement bounded `defer` and lifecycle hooks

## Atomic goal

Create source-backed evidence and handoff for parent task M4A-007; update status only if verification passed.

## Parent intent

Provide after-response cleanup/best-effort work without pretending it is durable jobs.

## Dependencies

- `M4A-007-V` — `tasks/07_m4a_developer_preview/M4A-007-V-verify-implement-bounded-defer-and-lifecycle-hooks.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/devex-beta.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `packages/cli/src/index.ts`
- `packages/compiler/src/index.ts`
- `packages/treaty/src/index.ts`
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

- Response is not delayed beyond defined handoff.
- Deferred work is bounded.
- Shutdown handles or aborts it deterministically.
- Docs warn against durable-job use.

## Targeted commands

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

- Lifecycle tests.
- Load/cleanup tests.
- Operational docs.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m4a-007-z: package evidence for implement bounded defer and lifecycle h
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

---

## Result (M4A-007-Z) — PASS (2026-09-01)

- Branch/PR: m4a-007-z (squash-merged; see git log for final hash)
- Closes: #473
- Parent verification: M4A-007-V PASS (PR #1077); this packet packages
  the source-backed evidence and flips the M4A-007 parent ledger.

### Evidence package

- **Implementation commits:** M4A-007-A (#1073), B (#1074), C (#1075),
  D (#1076), and verification closure V (#1077).
- **Source-backed proofs:** worker-owned bounded queue; handoff-first drain
  ordering on Immediate, Failed, and resolved-watch paths; dedicated
  `DeferredDrain` phase separate from settlement `Cleanup`; configured capacity
  admission; closure-private queue preventing direct recursive bypass;
  deadline interrupt; deterministic timeout/cancel/shutdown handling; six
  `EngineStats` lifecycle counters.
- **Operational specification:** `docs/specs/defer-api.md` — explicitly
  warns that defer is in-memory best-effort work, never a durable job queue,
  and documents owner, phase, bound, cancellation, shutdown, and metric
  semantics.

### Lifecycle and cleanup tests

- `defer_runs_after_response_handoff`
- `defer_queue_is_bounded`
- `defer_drain_deadline_bounds_spinning_callback`
- `shutdown_aborts_queued_deferred_work`
- `defer_admission_requires_invocation_owner`
- `deferred_drain_is_op_free_and_cannot_reenqueue`
- `defer_metrics_expose_lifecycle`
- `defer_queue_is_hidden_from_handlers`
- `defer_recursive_spawning_is_bounded`
- `defer_admission_enforces_configured_capacity`
- `failed_response_is_handed_off_before_defer_drain`

### Parent guardrail proofs

1. **Response is not delayed beyond handoff** — the V regression test proves
   Failed responses are sent before a spinning deferred callback is drained;
   the existing handoff test covers successful responses.
2. **Deferred work is bounded** — configured queue cap, deadline interrupt,
   op-free dedicated phase, and structurally private queue are all pinned by
   negative and positive lifecycle tests.
3. **Shutdown is deterministic** — deadline/cancel paths and shutdown drop
   counter are asserted; queued callbacks are discarded rather than retried.
4. **Durable-job warning** — `docs/specs/defer-api.md` is the operational
   warning and exact behavior reference.

### Gate results

- `cargo test -p q-engine-quickjs` → **113 pass / 0 fail**
- `cargo test -p q-http` → PASS
- `cargo test -p q-bridge` → PASS
- `cargo test -p velqu-runtime` → PASS
- `bun test` → **308 pass / 0 fail (48 files)**
- `bun run typecheck` → clean
- `cargo fmt --check` clean; workspace clippy `-D warnings` → clean
- `./scripts/verify` → **ALL PASS**

### Ledger

- `docs/beta/04_TASK_LEDGER.md`: M4A-007 remains governed by its child
  packets; A/B/C/D/V/Z are now PASS in their task records and indexes.
- `docs/codex-spark-beta/STATUS.md` and `indexes/TASK_INDEX.md`: Z flipped
  TODO → PASS. Spark queues regenerate after merge.

### Disclosures

- Evidence-only packet; no runtime behavior changes.
- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
