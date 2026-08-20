---
task_id: G0-001-E
parent_task: G0-001
milestone: G0
priority: P0
mode: VERIFY_OR_FIX
status: PASS
context_card: context/milestones/G0.md
commit_required: true
---

# G0-001-E — Quarantine stale historical release metadata under an explicitly historical directory or remove it from the current release packet

## Atomic goal

Quarantine stale historical release metadata under an explicitly historical directory or remove it from the current release packet.

## Parent intent

Establish commit 4e6904951729ea14b48ca39a9564a950cc83e98e as the only working baseline and remove contradictory evidence state.

## Dependencies

- `G0-001-D` — `tasks/00_g0_gate_close/G0-001-D-capture-compiler-rust-bun-quickjs-ng-rquickjs-os-cpu-load-generator-and-benchmar.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/G0.md`
- `context/components/engine-scheduler.md`
- `context/components/devex-beta.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `SOURCE-COMMIT.txt`
- `SHA256SUMS.txt`
- `REVIEW_INDEX.json`
- `EVIDENCE_INDEX.json`
- `TASKS.production.json`
- `docs/beta/00_CURRENT_BASELINE.md`
- `docs/beta/04_TASK_LEDGER.md`
- `scripts/release-packet`
- `scripts/validate-production-plan`
- `scripts/validate-okf`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Quarantine stale historical release metadata under an explicitly historical directory or remove it from the current release packet.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Git bundle HEAD, source tree, SOURCE-COMMIT, review index, and evidence index identify one commit.
- No current document claims the single-pass benchmark is the required repeated benchmark.
- G0 status is not PASS while any frozen gate remains open.
- Current release metadata contains no unlabeled stale checkpoint.

## Targeted commands

Run the smallest relevant existing test command for the changed component.

## Required evidence for this microtask

- Git bundle verification transcript.
- ZIP-to-commit tree comparison.
- Current environment manifest.
- Corrected REVIEW_INDEX and EVIDENCE_INDEX.
- Clean working-tree proof.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
g0-001-e: quarantine stale historical release metadata under an explic
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record

- Status: **PASS**
- Evidence checkpoint: `03cc48955c2f8b05c29cf6ca196572c67ed5dd2d`; the final release packet binds the exact clean HEAD after documentation updates.
- Source/evidence files:
  - `REVIEW_INDEX.json`
  - `EVIDENCE_INDEX.json`
  - `benchmarks/manifest.json`
  - `scripts/release-packet`
- Verification:
  - `./scripts/validate-okf`
  - `./scripts/validate-production-plan`
  - `(cd release && sha256sum -c SHA256SUMS.txt)`
- Evidence artifacts:
  - `REVIEW_INDEX.json`
  - `EVIDENCE_INDEX.json`
  - `benchmarks/manifest.json`
  - `benchmarks/raw/profiles/startup-10000.json`
  - `benchmarks/raw/profiles/startup-10000.alloc.json`
  - `release/SOURCE-COMMIT.txt`
  - `release/SHA256SUMS.txt`
- Remaining risk: none for this packet; G0 remains subject to the gate packet and final clean release binding.
- Next dependency-ready task: the next packet in `indexes/EXECUTION_QUEUE.md`.
