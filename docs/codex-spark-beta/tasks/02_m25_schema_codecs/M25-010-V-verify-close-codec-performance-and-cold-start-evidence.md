---
task_id: M25-010-V
parent_task: M25-010
milestone: M25
priority: P1
mode: VERIFY
status: TODO
context_card: context/milestones/M25.md
commit_required: true
---

# M25-010-V — Verify Close codec performance and cold-start evidence

## Atomic goal

Prove every acceptance criterion for parent task M25-010 without broadening scope.

## Parent intent

Prove the selected strategies improve real payloads without inflating startup unacceptably.

## Dependencies

- `M25-010-A` — `tasks/02_m25_schema_codecs/M25-010-A-run-c2-plus-medium-large-json-workloads.md`
- `M25-010-B` — `tasks/02_m25_schema_codecs/M25-010-B-measure-generated-code-pack-size.md`
- `M25-010-C` — `tasks/02_m25_schema_codecs/M25-010-C-report-cold-start-delta-at-25-1-000-routes.md`
- `M25-010-D` — `tasks/02_m25_schema_codecs/M25-010-D-record-cpu-and-rss.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M25.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/schema-codecs.md`

### Source files

- `AGENTS.md`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-engine-quickjs/src/convert.rs`
- `packages/schema/src/index.ts`
- `packages/compiler/src/emit.ts`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `conformance/schema/schema.conformance.test.ts`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-http/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `Cargo.toml`
- `packages/cli/src/index.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- C2 materially improves or limitation is documented.
- No unapproved cold-start regression.
- Reports match raw data.
- Route-specific strategy is inspectable.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-http
```
```bash
cargo test -p q-schema-runtime
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

- Raw performance suite.
- Generated report.
- Decision matrix.
- [ ] Canonical Schema IR drives runtime, Treaty, OpenAPI, lock, and diff.
- [ ] Generated decoders/encoders are semantically equivalent and bounded.
- [ ] Fallbacks are explicit and measured.
- [ ] Response errors/problems are exact and redacted correctly.
- [ ] Performance evidence supports route-level strategy selection.
- C2 small JSON.
- 1KB/16KB/64KB dynamic payloads.
- Arrays 100/1,000.
- Request decode and response encode stage timings.
- No binary QPack encoding yet.
- No capability API expansion.
- No ORM.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m25-010-v: verify close codec performance and cold start evidence
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
