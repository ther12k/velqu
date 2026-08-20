---
task_id: BETA-004-Z
parent_task: BETA-004
milestone: BETA
priority: P0
mode: EVIDENCE
status: TODO
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-004-Z — Package evidence for Implement optional first-party Postgres capability

## Atomic goal

Create source-backed evidence and handoff for parent task BETA-004; update status only if verification passed.

## Parent intent

Provide a real database story without enlarging core.

## Dependencies

- `BETA-004-V` — `tasks/08_public_beta/BETA-004-V-verify-implement-optional-first-party-postgres-capability.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `docs/beta/`
- `scripts/verify`
- `package.json`
- `.github/workflows/verify.yml`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-pack/src/lib.rs`
- `benchmarks/harness/`
- `benchmarks/manifest.json`
- `benchmarks/raw/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- App without Postgres pays zero dependency/init cost.
- Queries are parameterized.
- Timeout cancels/releases connection safely.
- Pool exhaustion is bounded.
- W1/W2/W3 workloads pass.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-capabilities
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
```bash
./scripts/validate-okf
```

## Required evidence for this microtask

- Capability tests.
- Real-world results.
- Cold/RSS cost report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
beta-004-z: package evidence for implement optional first party postgres
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
