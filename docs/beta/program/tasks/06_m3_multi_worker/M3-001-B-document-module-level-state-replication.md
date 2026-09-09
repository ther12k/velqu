---
task_id: M3-001-B
parent_task: M3-001
milestone: M3
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-001-B — Document module-level state replication

## Atomic goal

Document module-level state replication.

## Parent intent

Define what JavaScript and native state is per worker versus shared.

## Dependencies

- `M3-001-A` — `tasks/06_m3_multi_worker/M3-001-A-accept-adr.md`

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
5. Implement exactly this deliverable: Document module-level state replication.
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
m3-001-b: document module level state replication
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-001-B) — PASS

- Date: 2026-08-30
- Branch/PR: m3-001-b (squash-merged; see git log for final hash)
- Closes: #373

### Changed files
- `docs/beta/CAPABILITY_AUTHORS.md`: new developer-facing section **"Module-level state under multiple workers (M3)"** — the guardrail "developer docs describe per-worker globals" made concrete:
  - the rule stated first and strictly: every worker gets its own copy of module-level state; N workers = N independent instances;
  - an annotated code example (per-worker `hits` counter and `Map` cache);
  - practical consequences: counters under-count (use the host metrics surface for global numbers), caches replicate (size for one worker's share), one-time init is per worker, no cross-worker messaging in beta;
  - what IS shared (pack code, compiled routes, schema tables, capability config — identical and immutable, ADR-0036 section 3/6) and the absolute rule that JS values never cross workers;
  - links to ADR-0036 for the normative model.

### Command results
- `./scripts/validate-okf` → exit 0
- `cargo test -p q-engine-quickjs` → 18+101 passed; `-p velqu-runtime` → all suites pass
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**; release binary unchanged (docs-only packet)

### Guardrail mapping
- **Developer docs describe per-worker globals** — the CAPABILITY_AUTHORS guide now does, with examples and consequences.

### Disclosures
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
