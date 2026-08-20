---
task_id: M27-010-V
parent_task: M27-010
milestone: M27
priority: P1
mode: VERIFY
status: TODO
context_card: context/milestones/M27.md
commit_required: true
---

# M27-010-V — Verify Establish Web API conformance program

## Atomic goal

Prove every acceptance criterion for parent task M27-010 without broadening scope.

## Parent intent

Separate standards compatibility from internal framework tests.

## Dependencies

- `M27-010-A` — `tasks/04_m27_capability_linker/M27-010-A-pin-wpt-wintertc-subsets.md`
- `M27-010-B` — `tasks/04_m27_capability_linker/M27-010-B-record-skips-and-reasons.md`
- `M27-010-C` — `tasks/04_m27_capability_linker/M27-010-C-automate-regression-reports.md`
- `M27-010-D` — `tasks/04_m27_capability_linker/M27-010-D-keep-unsupported-apis-explicit.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M27.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-http/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `Cargo.toml`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- No unsupported API is advertised.
- Pass/fail/skip counts are reproducible.
- Behavioral regressions block relevant gate.
- Reports link to exact runtime build.

## Targeted commands

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
```bash
./scripts/verify
```

## Required evidence for this microtask

- Conformance report.
- Pinned test manifest.
- CI output.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m27-010-v: verify establish web api conformance program
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
