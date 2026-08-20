---
task_id: M28-001-A
parent_task: M28-001
milestone: M28
priority: P0
mode: IMPLEMENT
status: TODO
context_card: context/milestones/M28.md
commit_required: true
---

# M28-001-A — Define URL schemes, redirect policy, DNS rebinding controls, proxy behavior, TLS roots, timeout layers, compression, and body limits

## Atomic goal

Define URL schemes, redirect policy, DNS rebinding controls, proxy behavior, TLS roots, timeout layers, compression, and body limits.

## Parent intent

Freeze the public subset, trust boundaries, defaults, and non-goals.

## Dependencies

- `M27-GATE` — `gates/M27-GATE.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M28.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`

### Source files

- `AGENTS.md`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-pack/src/lib.rs`
- `Cargo.toml`
- `crates/q-runtime/src/main.rs`
- `docs/beta/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Define URL schemes, redirect policy, DNS rebinding controls, proxy behavior, TLS roots, timeout layers, compression, and body limits.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Security defaults fail closed.
- Private/link-local/metadata behavior is explicit.
- Redirect revalidation is required.
- Direct TLS policy is documented.

## Targeted commands

```bash
cargo test -p q-pack
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
cargo test -p q-capabilities
```
```bash
cargo test -p velqu-runtime
```

## Required evidence for this microtask

- ADR.
- Threat model.
- Security test matrix.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m28-001-a: define url schemes redirect policy dns rebinding controls pr
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
