---
task_id: M27-GATE
parent_task: M27-GATE
milestone: M27
priority: P0
mode: GATE_REVIEW
status: TODO
context_card: context/milestones/M27.md
commit_required: true
---

# M27-GATE — M2.7 — Capability Linker and Minimal Web Runtime exit gate

## Atomic goal

Review and decide the M27 exit gate from source, tests, and evidence.

## Parent intent

Capability Linker and Minimal Web Runtime

## Dependencies

- `M27-001-Z` — `tasks/04_m27_capability_linker/M27-001-Z-package-evidence-for-define-capability-abi-and-lifecycle-state-machine.md`
- `M27-002-Z` — `tasks/04_m27_capability_linker/M27-002-Z-package-evidence-for-implement-compile-time-capability-dependency-resolver.md`
- `M27-003-Z` — `tasks/04_m27_capability_linker/M27-003-Z-package-evidence-for-introduce-custom-quickjs-context-profiles.md`
- `M27-004-Z` — `tasks/04_m27_capability_linker/M27-004-Z-package-evidence-for-implement-console-and-timer-core-capabilities.md`
- `M27-005-Z` — `tasks/04_m27_capability_linker/M27-005-Z-package-evidence-for-implement-url-and-urlsearchparams.md`
- `M27-006-Z` — `tasks/04_m27_capability_linker/M27-006-Z-package-evidence-for-implement-textencoder-and-textdecoder.md`
- `M27-007-Z` — `tasks/04_m27_capability_linker/M27-007-Z-package-evidence-for-implement-abortcontroller-and-abortsignal.md`
- `M27-008-Z` — `tasks/04_m27_capability_linker/M27-008-Z-package-evidence-for-implement-crypto-random-subset.md`
- `M27-009-Z` — `tasks/04_m27_capability_linker/M27-009-Z-package-evidence-for-publish-capability-sdk-and-inspection-surface.md`
- `M27-010-Z` — `tasks/04_m27_capability_linker/M27-010-Z-package-evidence-for-establish-web-api-conformance-program.md`
- `M27-011-Z` — `tasks/04_m27_capability_linker/M27-011-Z-package-evidence-for-close-capability-cost-budgets.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M27.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-pack/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Freeze the candidate commit and confirm a clean working tree.
2. Review every dependency evidence packet against source and test reality.
3. Run the full verification commands from the exact candidate commit.
4. Check raw-to-report parity, index commit hashes, artifact hashes, and unresolved P0/P1 findings.
5. If any criterion fails, keep the gate IN_PROGRESS and list the exact blocking task; do not patch silently inside the gate review.
6. If all criteria pass, update the gate status and produce the milestone review packet, source archive, Git bundle, and checksum manifest.

## Parent acceptance guardrails

- Every parent task for M27 has a passing verification and evidence packet.
- Full project verification passes from a clean tree.
- Evidence indexes identify the exact commit and artifacts.
- No unresolved P0/P1 criterion is hidden or waived without owner/reviewer approval.

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
m27-gate: m2 7 capability linker and minimal web runtime exit gate
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
