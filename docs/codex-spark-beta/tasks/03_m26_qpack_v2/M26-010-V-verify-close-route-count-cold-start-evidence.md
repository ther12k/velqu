---
task_id: M26-010-V
parent_task: M26-010
milestone: M26
priority: P1
mode: VERIFY
status: TODO
context_card: context/milestones/M26.md
commit_required: true
---

# M26-010-V — Verify Close route-count cold-start evidence

## Atomic goal

Prove every acceptance criterion for parent task M26-010 without broadening scope.

## Parent intent

Demonstrate flatter startup scaling and preserve small-app behavior.

## Dependencies

- `M26-010-A` — `tasks/03_m26_qpack_v2/M26-010-A-measure-25-100-1-000-5-000-10-000-routes.md`
- `M26-010-B` — `tasks/03_m26_qpack_v2/M26-010-B-at-least-100-fresh-processes-for-release-evidence.md`
- `M26-010-C` — `tasks/03_m26_qpack_v2/M26-010-C-randomize-source-bytecode-competitor-order.md`
- `M26-010-D` — `tasks/03_m26_qpack_v2/M26-010-D-record-p50-p95-p99-rss-stage-timings-and-hashes.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M26.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/schema-codecs.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `crates/q-runtime/src/main.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-router/src/lib.rs`
- `conformance/routing/routing.conformance.test.ts`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `crates/q-engine-quickjs/src/convert.rs`
- `packages/schema/src/index.ts`
- `conformance/schema/schema.conformance.test.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- No runtime router/schema compilation.
- No base64 decoding.
- 25-route budget is not sacrificed silently.
- 10,000-route scaling is documented honestly.

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

- Raw cold data.
- Generated report.
- Startup-stage trace.
- [ ] QPack v2 is deterministic, fail-closed, and version/fingerprint safe.
- [ ] Production startup maps verified runtime IR and raw bytecode without JSON/base64 reconstruction.
- [ ] Legacy compatibility is isolated.
- [ ] Shared and standalone artifacts pass conformance.
- [ ] Cold-start route scaling evidence is canonical.
- 25/100/1,000/5,000/10,000 route cold start.
- Shared vs standalone RSS/startup.
- Pack parse/allocation stages.
- Source vs bytecode selection.
- No full capability ecosystem.
- No Node compatibility.
- No multi-worker yet.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m26-010-v: verify close route count cold start evidence
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
