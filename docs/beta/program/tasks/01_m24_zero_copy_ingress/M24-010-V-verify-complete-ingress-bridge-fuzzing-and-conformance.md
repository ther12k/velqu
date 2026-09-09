---
task_id: M24-010-V
parent_task: M24-010
milestone: M24
priority: P0
mode: VERIFY
status: PASS
context_card: context/milestones/M24.md
commit_required: true
---

# M24-010-V — Verify Complete ingress bridge fuzzing and conformance

## Atomic goal

Prove every acceptance criterion for parent task M24-010 without broadening scope.

## Parent intent

Prove lazy materialization is semantically safe under malformed and adversarial inputs.

## Dependencies

- `M24-010-A` — `tasks/01_m24_zero_copy_ingress/M24-010-A-fuzz-paths-queries-headers-cookies-bodies-handles-and-cancellation-orderings.md`
- `M24-010-B` — `tasks/01_m24_zero_copy_ingress/M24-010-B-differentially-compare-legacy-reference-decoding-where-applicable.md`
- `M24-010-C` — `tasks/01_m24_zero_copy_ingress/M24-010-C-run-property-tests-for-slot-lifecycle.md`
- `M24-010-D` — `tasks/01_m24_zero_copy_ingress/M24-010-D-capture-and-minimize-failures.md`

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
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`
- `crates/q-capabilities/src/lib.rs`
- `Cargo.toml`
- `benchmarks/harness/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- No panic, leak, stale-handle access, or unbounded allocation.
- Queue-empty-or-quarantined remains true.
- All failing cases become regression fixtures.
- Fuzz corpus is committed or reproducibly generated.

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

## Evidence

Acceptance matrix:

- Routing precedes materialization: runtime `routing_precedes_body_materialization` and native route tests PASS.
- Unread fields stay cold: engine `lazy_ctx_touches_nothing` PASS.
- Worker-local generation-safe slab: q-bridge forged triples, stale corpus, and 4,096-round lifecycle property PASS.
- Bounded bodies/queues/disconnects: runtime body limit, queue saturation, client abort, timeout, quarantine, and shutdown tests PASS.
- Differential decoding: q-http reference corpus PASS.
- Minimized regression corpus: `crates/q-http/tests/regression_corpus.rs` PASS.
- Bun conformance: 36 pass, 0 fail; `bun run typecheck` PASS.
- Rust package suites, format, clippy, and `./scripts/verify`: PASS.
- Sanitizer-compatible output not claimed; no sanitizer toolchain run in environment.
- No QPack format, multi-worker dispatch, schema codec, or benchmark manifest changes.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m24-010-v: verify complete ingress bridge fuzzing and conformance
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
