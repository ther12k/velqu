---
task_id: G0-008-V
parent_task: G0-008
milestone: G0
priority: P1
mode: VERIFY
status: TODO
context_card: context/milestones/G0.md
commit_required: true
---

# G0-008-V — Verify Close canonical performance evidence

## Atomic goal

Prove every acceptance criterion for parent task G0-008 without broadening scope.

## Parent intent

Close the frozen G0 benchmark evidence requirements instead of relying on a single clean pass.

## Dependencies

- `G0-008-A` — `tasks/00_g0_gate_close/G0-008-A-run-warm-workloads-at-concurrency-1-10-and-50-for-at-least-five-independent-repe.md`
- `G0-008-B` — `tasks/00_g0_gate_close/G0-008-B-run-fresh-process-cold-start-measurements-for-25-1-000-and-10-000-routes-with-ra.md`
- `G0-008-C` — `tasks/00_g0_gate_close/G0-008-C-capture-cpu-rss-errors-p50-p95-p99-binary-pack-hashes-machine-state-and-load-gen.md`
- `G0-008-D` — `tasks/00_g0_gate_close/G0-008-D-capture-allocation-startup-profiles-including-the-10-000-route-json-pack-parsing.md`
- `G0-008-E` — `tasks/00_g0_gate_close/G0-008-E-generate-markdown-reports-from-raw-data-and-make-verification-fail-when-raw-repo.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/G0.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-router/src/lib.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-http/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `Cargo.toml`
- `benchmarks/harness/`
- `benchmarks/manifest.json`
- `benchmarks/raw/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Markdown reports are generated from current raw data.
- Verifier fails on stale reports.
- No public claim uses a single spot check.
- Any regression is documented rather than hidden.

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

## Required evidence for this microtask

- Raw benchmark directory.
- Generated report.
- Environment and artifact manifest.
- Ablation results for relevant changes.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
g0-008-v: verify close canonical performance evidence
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
