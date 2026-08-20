---
task_id: M24-001-A
parent_task: M24-001
milestone: M24
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M24.md
commit_required: true
---

# M24-001-A — Accept an ADR with ownership diagrams and terminal invariants

## Atomic goal

Accept an ADR with ownership diagrams and terminal invariants.

## Parent intent

Define ownership from Hyper ingress through routing, worker queue, slab lifetime, cancellation, and response completion.

## Dependencies

- `G0-GATE` — `gates/G0-GATE.md`

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

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Accept an ADR with ownership diagrams and terminal invariants.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

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

## Required evidence for this microtask

- ADR.
- State-machine tests plan.
- Threat/ownership review.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m24-001-a: accept an adr with ownership diagrams and terminal invariant
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record

- Status: **PASS**
- Deliverable: ADR-0021 accepted with the ownership spine (ingress → routing → queue → worker-local slab → settlement), four terminal invariants (INV-1 no borrow beyond owner lifetime, INV-2 explicit queue/body limits, INV-3 single cancellation owner, INV-4 one-and-many-worker support), the D3 slot state machine, the D4 twelve-case state-machine tests plan, and the D5 threat/ownership review.
- Changed files:
  - `docs/okf/decisions/0021-m24-zero-copy-ingress-ownership.md` (new)
  - `docs/okf/decisions/index.md` (append-only registration)
  - `docs/okf/log.md` (append-only log entry)
  - `docs/codex-spark-beta/STATUS.md`, `docs/codex-spark-beta/indexes/EXECUTION_QUEUE.md`, `docs/codex-spark-beta/indexes/NEXT_25.md` (queue advance)
- Verification: `./scripts/validate-okf` PASS (174 links); `cargo test -p q-pack -p q-router -p q-engine-quickjs -p q-http -p q-bridge -p velqu-runtime` all pass (154 tests); `bun test` 35 pass; `bun run typecheck` clean. Docs-only change; no code modified.
- Remaining risk: none for this packet. The ADR freezes design only; implementation risk is carried by M24-001-B/C/D and M24-002…M24-010.
- Next dependency-ready task: M24-001-B (specify body ownership, queue admission, disconnect cancellation, and request-slot lifecycle).
