---
task_id: M24-007-D
parent_task: M24-007
milestone: M24
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M24.md
commit_required: true
---

# M24-007-D — Cache one decoded representation and reject incompatible second reads

## Atomic goal

Cache one decoded representation and reject incompatible second reads.

## Parent intent

Collect or stream request bodies only when declared and under route/global limits.

## Dependencies

- `M24-007-C` — `tasks/01_m24_zero_copy_ingress/M24-007-C-enforce-content-length-and-streaming-limits.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M24.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`
- `crates/q-capabilities/src/lib.rs`
- `Cargo.toml`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Cache one decoded representation and reject incompatible second reads.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- POST with no body contract does not collect body.
- DELETE/body routes work when declared.
- Oversize/slow bodies cancel cleanly.
- Client disconnect releases body work.

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
cargo test -p q-capabilities
```
```bash
cargo test -p velqu-runtime
```

## Required evidence for this microtask

- Body-limit tests.
- Slowloris/partial-body tests.
- Cancellation metrics.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Evidence

- Worker-local `Slot.body_mode` records one body representation per request generation.
- `text` and `bytes` native accessors claim representation before access; same-mode reuse succeeds, incompatible second mode throws deterministic failure.
- Settlement clears body mode before slot generation reuse.
- Body remains bounded `Bytes`; no second stream read or body re-collection occurs.
- Existing QuickJS body, limit, partial-body, and disconnect tests remain green.
- `cargo test -p q-bridge`: PASS.
- `cargo test -p q-engine-quickjs --test engine`: PASS.
- `cargo test -p q-http`: PASS.
- `cargo test -p q-capabilities`: PASS.
- `cargo test -p velqu-runtime --test runtime_conformance`: PASS.
- `cargo fmt --check`: PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m24-007-d: cache one decoded representation and reject incompatible sec
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
