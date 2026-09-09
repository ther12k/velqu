---
task_id: M3-001-A
parent_task: M3-001
milestone: M3
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-001-A — Accept ADR

## Atomic goal

Accept ADR.

## Parent intent

Define what JavaScript and native state is per worker versus shared.

## Dependencies

- `M28-GATE` — `gates/M28-GATE.md`

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

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Accept ADR.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

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

## Required evidence for this microtask

- ADR.
- Concurrency model tests plan.
- State examples.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m3-001-a: accept adr
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-001-A) — PASS

- Date: 2026-08-30
- Branch/PR: m3-001-a (squash-merged; see git log for final hash)
- Closes: #372

### Changed files
- `docs/okf/decisions/0036-multi-worker-state-ownership.md` (new, ACCEPTED): the multi-worker concurrency model —
  - one QuickJS runtime per owner thread for its whole lifetime (never moved, locked, or polled cross-thread);
  - per-worker exclusive JS state enumerated (runtime/heap, module-level state, timers/promises, per-worker op registry);
  - shared-immutable enumerated (QPack bytes, route plans, schema tables — frozen after startup);
  - shared-mutable restricted to FOUR named disciplines (MPMC dispatch queues, fixed-size metric shards, pool handles, lifecycle atomics) — anything else requires a new ADR;
  - forbidden outright: JSValue/runtime/context pointers crossing workers, shared heaps, locks held across JS execution, ambient pools touching runtimes;
  - deterministic initialization: worker K evaluates identical pack bytes in pack order with the same construction sequence as worker 0;
  - **concurrency model tests plan** table binding each invariant to its proving packet (M3-002/004/005/007);
  - **state examples**: per-worker module counters, per-worker caches, native permit ownership;
  - alternatives considered and rejected (global-lock shared runtime, worker processes, STM).
- `docs/okf/decisions/index.md`: ADR-0036 indexed.

### Concurrency model tests plan
Embedded in the ADR as a table mapping each invariant to its proving packet (M3-002-A/C/D, M3-004-A/B, M3-005-A/B/C, M3-007-A..D) — no new infrastructure required in this packet.

### State examples
Embedded in the ADR: module-level `hits` counter under 4 workers (4 independent variables), per-worker Map caches, native permit ownership.

### Command results
- `./scripts/validate-okf` → exit 0, errors: []
- `cargo test -p q-engine-quickjs` → 18+101 passed; `-p velqu-runtime` → all suites pass
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**; release binary unchanged (docs-only packet)

### Guardrail mapping
- **Each runtime has one owner thread** — ADR §1.
- **Cross-worker mutable state is explicit** — ADR §4 closes the vocabulary; §5 forbids the rest.
- **Initialization is deterministic** — ADR §6 (worker K ≡ worker 0 at ready).
- **Developer docs describe per-worker globals** — ADR §2 consequences + state examples; M3-001-B owns the developer-facing doc.

### Disclosures
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
