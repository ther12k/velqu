---
task_id: M3-GATE
parent_task: M3-GATE
milestone: M3
priority: P0
mode: GATE_REVIEW
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-GATE — M3 — Multi-Worker Service Runtime exit gate

## Atomic goal

Review and decide the M3 exit gate from source, tests, and evidence.

## Parent intent

Multi-Worker Service Runtime

## Dependencies

- `M3-001-Z` — `tasks/06_m3_multi_worker/M3-001-Z-package-evidence-for-freeze-independent-worker-state-semantics.md`
- `M3-002-Z` — `tasks/06_m3_multi_worker/M3-002-Z-package-evidence-for-implement-bounded-worker-dispatcher.md`
- `M3-003-Z` — `tasks/06_m3_multi_worker/M3-003-Z-package-evidence-for-implement-serverless-service-and-throughput-profiles.md`
- `M3-004-Z` — `tasks/06_m3_multi_worker/M3-004-Z-package-evidence-for-implement-deterministic-worker-initialization-and-artifact.md`
- `M3-005-Z` — `tasks/06_m3_multi_worker/M3-005-Z-package-evidence-for-implement-quarantine-replacement-and-readiness-aggregation.md`
- `M3-006-Z` — `tasks/06_m3_multi_worker/M3-006-Z-package-evidence-for-implement-adaptive-scale-up-and-scale-down.md`
- `M3-007-Z` — `tasks/06_m3_multi_worker/M3-007-Z-package-evidence-for-implement-multi-worker-cancellation-and-graceful-shutdown.md`
- `M3-008-Z` — `tasks/06_m3_multi_worker/M3-008-Z-package-evidence-for-add-fairness-and-overload-controls.md`
- `M3-009-Z` — `tasks/06_m3_multi_worker/M3-009-Z-package-evidence-for-close-multi-worker-scaling-and-memory-evidence.md`
- `M3-010-Z` — `tasks/06_m3_multi_worker/M3-010-Z-package-evidence-for-run-multi-worker-soak-and-recovery.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M3.md`
- `context/components/engine-scheduler.md`
- `context/components/multiworker.md`

### Source files

- `AGENTS.md`
- `crates/q-runtime/src/main.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Freeze the candidate commit and confirm a clean working tree.
2. Review every dependency evidence packet against source and test reality.
3. Run the full verification commands from the exact candidate commit.
4. Check raw-to-report parity, index commit hashes, artifact hashes, and unresolved P0/P1 findings.
5. If any criterion fails, keep the gate IN_PROGRESS and list the exact blocking task; do not patch silently inside the gate review.
6. If all criteria pass, update the gate status and produce the milestone review packet, source archive, Git bundle, and checksum manifest.

## Parent acceptance guardrails

- Every parent task for M3 has a passing verification and evidence packet.
- Full project verification passes from a clean tree.
- Evidence indexes identify the exact commit and artifacts.
- No unresolved P0/P1 criterion is hidden or waived without owner/reviewer approval.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
```
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
m3-gate: m3 multi worker service runtime exit gate
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-GATE) — PASS

- Date: 2026-08-31
- Branch/PR: m3-gate (squash-merged; see git log for final hash)
- Candidate commit: 8b36acc (clean tree at gate time)

### Gate decision: PASS
- All 10 parents (M3-001..M3-010) have V+Z packets PASS; ledger row updated.
- Full verification from the clean candidate commit: ./scripts/verify exit 0 (Rust: q-capabilities 260+6+3+7+1+4+9, q-engine-quickjs 20+102+1, velqu-runtime 55+6+5+2+35, q-http 4+6+1, q-bridge 11; TypeScript 219; fmt/clippy -D warnings clean; benchmark evidence current; release binary matches manifest).
- Review packet: docs/reports/m3-gate-review.md; indexes EVIDENCE_INDEX.json / REVIEW_INDEX.json updated to the M3 checkpoint (commit rewritten by scripts/release-packet at release time).
- Milestone report, source archive, Git bundle, and SHA-256 manifest produced via scripts/release-packet (release/ artifacts are untracked by design).
- No unresolved P0/P1 findings hidden or waived: the PACK_FORMAT_CURRENT owner decision remains tracked in REVIEW_INDEX openItems (carried from M26); the numeric 2-worker scaling target is tracked in REVIEW_INDEX openItems.
