---
task_id: M28-002-V
parent_task: M28-002
milestone: M28
priority: P1
mode: VERIFY
status: TODO
context_card: context/milestones/M28.md
commit_required: true
---

# M28-002-V — Verify Select native HTTP client stack from evidence

## Atomic goal

Prove every acceptance criterion for parent task M28-002 without broadening scope.

## Parent intent

Choose a maintainable implementation based on cold start, size, correctness, streaming, and pooling.

## Dependencies

- `M28-002-A` — `tasks/05_m28_native_fetch/M28-002-A-compare-reqwest-and-lower-level-hyper-rustls-approach.md`
- `M28-002-B` — `tasks/05_m28_native_fetch/M28-002-B-measure-dependency-binary-startup-cost.md`
- `M28-002-C` — `tasks/05_m28_native_fetch/M28-002-C-test-dns-tls-pool-behavior.md`
- `M28-002-D` — `tasks/05_m28_native_fetch/M28-002-D-record-maintenance-security-considerations.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M28.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`
- `Cargo.toml`
- `packages/cli/src/index.ts`
- `crates/q-runtime/src/source_map.rs`
- `examples/proof/`
- `README.md`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Decision is evidence-backed.
- No framework benchmark alone determines choice.
- Selected stack supports cancellation/backpressure.
- Fallback strategy documented.

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

- Spike report.
- Raw measurements.
- Decision record.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m28-002-v: verify select native http client stack from evidence
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
