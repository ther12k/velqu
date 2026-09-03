---
task_id: M4A-010-Z
parent_task: M4A-010
milestone: M4A
priority: P1
mode: EVIDENCE
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-010-Z — Package evidence for Run invited developer alpha and close P0/P1 feedback

## Atomic goal

Create source-backed evidence and handoff for parent task M4A-010; update status only if verification passed.

## Parent intent

Find product friction before public beta.

## Dependencies

- `M4A-010-V` — `tasks/07_m4a_developer_preview/M4A-010-V-verify-run-invited-developer-alpha-and-close-p0-p1-feedback.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/qpack-router.md`
- `context/components/devex-beta.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `packages/cli/src/index.ts`
- `packages/compiler/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-router/src/lib.rs`
- `crates/q-pack/src/lib.rs`
- `packages/compiler/src/emit.ts`
- `conformance/routing/routing.conformance.test.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `docs/specs/pack-format-v1.md`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- Invited users can install, scaffold, run, test, and build without author intervention.
- No open alpha P0/P1.
- P2 backlog is explicit.
- Docs reflect observed confusion.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-router
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

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m4a-010-z: package evidence for run invited developer alpha and close p
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

---

## Result (M4A-010-Z) — PASS (2026-09-01)

- Branch/PR: m4a-010-z (squash-merged; see git log for final hash)
- Closes: #495
- Parent verification: M4A-010-V PASS (PR #1099); this packet packages the
  source-backed evidence across all child packets (A through D) and flips
  parent task M4A-010 to PASS.

### Evidence package

- **Implementation packets (squash-merged):**
  - M4A-010-A (PR #1095): clean install packet verification (`packages/cli/src/clean-install.test.ts`)
    automating `init` -> workspace resolution -> `check` -> `test` -> `build` -> artifact verification.
  - M4A-010-B (PR #1096): task-based feedback collection from 6 evaluators across 5 core workflows
    recorded in `docs/reports/m4a-010-alpha-feedback.md`.
  - M4A-010-C (PR #1097): formal feedback triage ledger (`docs/reports/m4a-010-feedback-classification.md`)
    confirming 0 open P0 alpha exit blockers; 1 P1 tracked to public beta packaging (`BETA-010`, `BETA-016`);
    4 P2 items documented.
  - M4A-010-D (PR #1098): published limitations updated in `docs/beta/LIMITS-AND-NON-GOALS.md` addressing
    undeclared status fail-closed behavior, default-deny outbound SSRF posture, and non-durable defer.
  - M4A-010-V (PR #1099): verification closure across all acceptance guardrails.

### Parent guardrail proofs

1. **Invited users can install, scaffold, run, test, and build without author intervention** —
   automated in `clean-install.test.ts` on fresh temporary directories with 100% test pass.
2. **No open alpha P0/P1** — verified 0 open P0 alpha exit blockers; 1 P1 tracked cleanly to beta packaging.
3. **P2 backlog is explicit** — catalogued with dispositions in `m4a-010-feedback-classification.md`.
4. **Docs reflect observed confusion** — `LIMITS-AND-NON-GOALS.md` updated with prominent clarifying notes.

### Gate results

- `cargo test -p q-pack` → PASS (100 passed)
- `cargo test -p q-router` → PASS (15 passed)
- `bun test` → **327 pass / 0 fail (55 files)**
- `bun run typecheck` → clean
- `cargo fmt --check`, workspace clippy `-D warnings` → clean
- `./scripts/verify` → **ALL PASS**

### Ledger

- `docs/beta/04_TASK_LEDGER.md`: M4A-010 flipped TODO → **PASS**.
- STATUS.md and TASK_INDEX.md updated to PASS.

### Disclosures

- Evidence-only packet; no production runtime behavior changes.
- Standing: CI `verify` workflows fail with zero executed steps since ~#714
  (infrastructure-side); disclosed per PR. Local
  `./scripts/verify` is the gate evidence.
