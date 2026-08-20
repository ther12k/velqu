---
task_id: M4A-GATE
parent_task: M4A-GATE
milestone: M4A
priority: P0
mode: GATE_REVIEW
status: TODO
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-GATE — M4A — Developer Preview and Private Alpha exit gate

## Atomic goal

Review and decide the M4A exit gate from source, tests, and evidence.

## Parent intent

Actual-Runtime Developer Preview and Private Alpha

## Dependencies

- `M4A-001-Z` — `tasks/07_m4a_developer_preview/M4A-001-Z-package-evidence-for-implement-actual-runtime-velqu-dev-loop.md`
- `M4A-002-Z` — `tasks/07_m4a_developer_preview/M4A-002-Z-package-evidence-for-complete-cli-command-surface.md`
- `M4A-003-Z` — `tasks/07_m4a_developer_preview/M4A-003-Z-package-evidence-for-implement-project-scaffolding.md`
- `M4A-004-Z` — `tasks/07_m4a_developer_preview/M4A-004-Z-package-evidence-for-complete-treaty-unit-local-runtime-local-and-remote-modes.md`
- `M4A-005-Z` — `tasks/07_m4a_developer_preview/M4A-005-Z-package-evidence-for-publish-compact-contract-and-sdk-artifacts.md`
- `M4A-006-Z` — `tasks/07_m4a_developer_preview/M4A-006-Z-package-evidence-for-finalize-diagnostics-source-maps-and-inspect-output.md`
- `M4A-007-Z` — `tasks/07_m4a_developer_preview/M4A-007-Z-package-evidence-for-implement-bounded-defer-and-lifecycle-hooks.md`
- `M4A-008-Z` — `tasks/07_m4a_developer_preview/M4A-008-Z-package-evidence-for-build-documentation-and-examples.md`
- `M4A-009-Z` — `tasks/07_m4a_developer_preview/M4A-009-Z-package-evidence-for-build-realistic-private-alpha-proof-service.md`
- `M4A-010-Z` — `tasks/07_m4a_developer_preview/M4A-010-Z-package-evidence-for-run-invited-developer-alpha-and-close-p0-p1-feedback.md`

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

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Freeze the candidate commit and confirm a clean working tree.
2. Review every dependency evidence packet against source and test reality.
3. Run the full verification commands from the exact candidate commit.
4. Check raw-to-report parity, index commit hashes, artifact hashes, and unresolved P0/P1 findings.
5. If any criterion fails, keep the gate IN_PROGRESS and list the exact blocking task; do not patch silently inside the gate review.
6. If all criteria pass, update the gate status and produce the milestone review packet, source archive, Git bundle, and checksum manifest.

## Parent acceptance guardrails

- Every parent task for M4A has a passing verification and evidence packet.
- Full project verification passes from a clean tree.
- Evidence indexes identify the exact commit and artifacts.
- No unresolved P0/P1 criterion is hidden or waived without owner/reviewer approval.

## Targeted commands

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

- Milestone report.
- Review index.
- Evidence index.
- Commit-named source archive and Git bundle.
- SHA-256 manifest.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Implementing missing milestone work inside the gate task.
- Waiving P0/P1 without explicit owner/reviewer approval.
- Calling a single benchmark run canonical when repeated evidence is required.

## Commit guidance

Suggested subject:

```text
m4a-gate: m4a developer preview and private alpha exit gate
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
