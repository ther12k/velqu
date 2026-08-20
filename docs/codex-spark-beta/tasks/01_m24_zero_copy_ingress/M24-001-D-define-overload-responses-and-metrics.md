---
task_id: M24-001-D
parent_task: M24-001
milestone: M24
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M24.md
commit_required: true
---

# M24-001-D — Define overload responses and metrics

## Atomic goal

Define overload responses and metrics.

## Parent intent

Define ownership from Hyper ingress through routing, worker queue, slab lifetime, cancellation, and response completion.

## Dependencies

- `M24-001-C` — `tasks/01_m24_zero_copy_ingress/M24-001-C-define-no-copy-and-bounded-copy-boundaries.md`

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
5. Implement exactly this deliverable: Define overload responses and metrics.
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
m24-001-d: define overload responses and metrics
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record

- Status: **PASS**
- Deliverable: normative overload-response and metrics specification in `docs/specs/m24-ingress-ownership-and-admission.md` §9.
- Evidence: fixed 414/431/404/405/503/413/400 response matrix; queue-full 503 with `Retry-After: 1` before body poll; bounded metric vocabulary with no request/path/exception labels; permit/slot/settlement accounting; finite backpressure and fairness rules; D-T1–D-T8 overload/metrics test plan.
- Changed files:
  - `docs/specs/m24-ingress-ownership-and-admission.md`
  - `docs/codex-spark-beta/tasks/01_m24_zero_copy_ingress/M24-001-D-define-overload-responses-and-metrics.md`
  - `docs/codex-spark-beta/STATUS.md`
  - `docs/codex-spark-beta/indexes/TASK_INDEX.md`
  - `docs/codex-spark-beta/indexes/EXECUTION_QUEUE.md`
  - `docs/codex-spark-beta/indexes/NEXT_25.md`
- Verification: `./scripts/validate-okf` PASS; targeted Rust crates all pass; `bun test` 35 pass; `bun run typecheck` clean.
- Remaining risk: metric counters/exporter and backpressure assertions are implementation work for M24-007/M24-010; this packet changes specification only.
- Next dependency-ready task: M24-001-V (verify freeze ingress ownership and backpressure design).
