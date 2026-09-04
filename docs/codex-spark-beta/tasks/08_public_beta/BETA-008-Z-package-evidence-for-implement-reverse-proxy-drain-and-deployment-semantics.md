---
task_id: BETA-008-Z
parent_task: BETA-008
milestone: BETA
priority: P0
mode: EVIDENCE
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-008-Z — Package evidence for Implement reverse-proxy, drain, and deployment semantics

## Atomic goal

Create source-backed evidence and handoff for parent task BETA-008; update status only if verification passed.

## Parent intent

Make the beta deployable behind common cloud/reverse-proxy setups.

## Dependencies

- `BETA-008-V` — `tasks/08_public_beta/BETA-008-V-verify-implement-reverse-proxy-drain-and-deployment-semantics.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/devex-beta.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `docs/beta/`
- `scripts/verify`
- `package.json`
- `.github/workflows/verify.yml`
- `crates/q-runtime/src/serve.rs`
- `crates/q-runtime/src/main.rs`
- `examples/proof/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- Spoofed forwarding headers are ignored unless trusted.
- Readiness drops before drain.
- In-flight requests honor deadline.
- Container shutdown exits deterministically.

## Targeted commands

```bash
cargo test -p velqu-runtime
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

- Proxy tests.
- Container smoke test.
- Runbook.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
beta-008-z: package evidence for implement reverse proxy drain and deplo
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
