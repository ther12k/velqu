---
task_id: M4A-010-V
parent_task: M4A-010
milestone: M4A
priority: P1
mode: VERIFY
status: TODO
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-010-V — Verify Run invited developer alpha and close P0/P1 feedback

## Atomic goal

Prove every acceptance criterion for parent task M4A-010 without broadening scope.

## Parent intent

Find product friction before public beta.

## Dependencies

- `M4A-010-A` — `tasks/07_m4a_developer_preview/M4A-010-A-provide-clean-install-packet.md`
- `M4A-010-B` — `tasks/07_m4a_developer_preview/M4A-010-B-collect-task-based-feedback.md`
- `M4A-010-C` — `tasks/07_m4a_developer_preview/M4A-010-C-classify-p0-p1-p2.md`
- `M4A-010-D` — `tasks/07_m4a_developer_preview/M4A-010-D-fix-beta-blocking-findings-and-publish-limitations.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `packages/cli/src/index.ts`
- `packages/compiler/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-runtime/src/source_map.rs`
- `examples/proof/`
- `README.md`
- `docs/beta/`
- `packages/auth-jwt/ (create if absent)`
- `packages/core/src/index.ts`
- `conformance/security/security.conformance.test.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Invited users can install, scaffold, run, test, and build without author intervention.
- No open alpha P0/P1.
- P2 backlog is explicit.
- Docs reflect observed confusion.

## Targeted commands

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

- Feedback summary.
- Issue disposition.
- Re-run install evidence.
- [ ] Actual-runtime developer loop works.
- [ ] CLI, scaffolding, Treaty modes, diagnostics, and docs are usable.
- [ ] Proof service demonstrates real framework composition.
- [ ] Invited alpha users complete core tasks.
- [ ] No public beta claim yet.
- Dev reload latency.
- Typecheck/editor scale.
- Proof-service controlled I/O.
- Install/build artifact sizes.
- No SLA.
- No public production endorsement.
- Breaking API changes still allowed.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m4a-010-v: verify run invited developer alpha and close p0 p1 feedback
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
