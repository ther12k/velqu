---
task_id: M24-010-C
parent_task: M24-010
milestone: M24
priority: P0
mode: IMPLEMENT
status: TODO
context_card: context/milestones/M24.md
commit_required: true
---

# M24-010-C — Run property tests for slot lifecycle

## Atomic goal

Run property tests for slot lifecycle.

## Parent intent

Prove lazy materialization is semantically safe under malformed and adversarial inputs.

## Dependencies

- `M24-010-B` — `tasks/01_m24_zero_copy_ingress/M24-010-B-differentially-compare-legacy-reference-decoding-where-applicable.md`

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
- `crates/q-runtime/tests/runtime_conformance.rs`
- `conformance/security/security.conformance.test.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-http/tests/fuzz_parsers.rs`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `.github/workflows/verify.yml`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Run property tests for slot lifecycle.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- No panic, leak, stale-handle access, or unbounded allocation.
- Queue-empty-or-quarantined remains true.
- All failing cases become regression fixtures.
- Fuzz corpus is committed or reproducibly generated.

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
cargo test -p q-schema-runtime
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

- Fuzz summaries.
- Regression corpus.
- Sanitizer-compatible test output.
- [ ] Request routing precedes decode/materialization.
- [ ] Unread fields are not materialized.
- [ ] Worker-local slab has no global mutex and is generation-safe.
- [ ] Bodies, queues, parsers, and disconnect handling are bounded.
- [ ] Measured fixed overhead improves without semantic regression.
- C0/C1 fixed-overhead comparison versus G0 baseline.
- C3 parameter allocation/latency.
- Header/query/body allocation matrix.
- Concurrency 1/10/50 with stage timings.
- No schema-specialized JSON codecs beyond compatibility hooks.
- No QPack binary format change.
- No multi-worker dispatch.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m24-010-c: run property tests for slot lifecycle
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
