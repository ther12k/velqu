---
task_id: M24-005-D
parent_task: M24-005
milestone: M24
priority: P0
mode: IMPLEMENT
status: TODO
context_card: context/milestones/M24.md
commit_required: true
---

# M24-005-D — Keep full Headers escape hatch explicit and costed

## Atomic goal

Keep full Headers escape hatch explicit and costed.

## Parent intent

Expose only headers declared by route or policy without cloning the entire HeaderMap.

## Dependencies

- `M24-005-C` — `tasks/01_m24_zero_copy_ingress/M24-005-C-define-duplicate-header-behavior-and-byte-string-conversion.md`

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
- `crates/q-runtime/tests/runtime_conformance.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Keep full Headers escape hatch explicit and costed.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Route declaring no headers copies none.
- Auth route reads only required headers.
- Duplicate/non-UTF8 behavior matches contract.
- Secret headers are redacted in diagnostics.

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

- Header access tests.
- Allocation profile.
- Security redaction tests.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m24-005-d: keep full headers escape hatch explicit and costed
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
