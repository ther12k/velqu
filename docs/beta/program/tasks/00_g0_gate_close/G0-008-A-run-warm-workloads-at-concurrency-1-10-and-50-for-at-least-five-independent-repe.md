---
task_id: G0-008-A
parent_task: G0-008
milestone: G0
priority: P1
mode: VERIFY_OR_FIX
status: PASS
context_card: context/milestones/G0.md
commit_required: true
---

# G0-008-A — Run warm workloads at concurrency 1, 10, and 50 for at least five independent repetitions with randomized candidate order

## Atomic goal

Run warm workloads at concurrency 1, 10, and 50 for at least five independent repetitions with randomized candidate order.

## Parent intent

Close the frozen G0 benchmark evidence requirements instead of relying on a single clean pass.

## Dependencies

- `G0-004-Z` — `tasks/00_g0_gate_close/G0-004-Z-package-evidence-for-load-the-serialized-router-directly.md`
- `G0-005-Z` — `tasks/00_g0_gate_close/G0-005-Z-package-evidence-for-complete-operational-routeid-policyid-and-schemaid-usage.md`
- `G0-007-Z` — `tasks/00_g0_gate_close/G0-007-Z-package-evidence-for-remove-duplicate-legacy-state-from-current-packs.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/G0.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-router/src/lib.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `benchmarks/harness/`
- `benchmarks/manifest.json`
- `benchmarks/raw/`
- `docs/reports/`
- `scripts/benchmark`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Run warm workloads at concurrency 1, 10, and 50 for at least five independent repetitions with randomized candidate order.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Markdown reports are generated from current raw data.
- Verifier fails on stale reports.
- No public claim uses a single spot check.
- Any regression is documented rather than hidden.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-router
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

## Required evidence for this microtask

- Raw benchmark directory.
- Generated report.
- Environment and artifact manifest.
- Ablation results for relevant changes.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
g0-008-a: run warm workloads at concurrency 1 10 and 50 for at least f
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record

- Status: **PASS**
- Evidence checkpoint: `03cc48955c2f8b05c29cf6ca196572c67ed5dd2d`; the final release packet binds the exact clean HEAD after documentation updates.
- Source/evidence files:
  - `benchmarks/harness/warm.ts`
  - `benchmarks/harness/cold-start.ts`
  - `benchmarks/harness/route-count.ts`
  - `scripts/capture-startup-profile.py`
  - `scripts/alloc-tracer.c`
  - `scripts/generate-benchmark-reports.py`
- Verification:
  - `python3 scripts/validate-benchmark-evidence.py`
  - `python3 scripts/generate-benchmark-reports.py --check`
  - `5 warm repetitions / 240 cells / zero errors`
  - `5 route-count samples per cell / zero failures`
  - `allocator profile captured`
- Evidence artifacts:
  - `REVIEW_INDEX.json`
  - `EVIDENCE_INDEX.json`
  - `benchmarks/manifest.json`
  - `benchmarks/raw/profiles/startup-10000.json`
  - `benchmarks/raw/profiles/startup-10000.alloc.json`
  - `release/SOURCE-COMMIT.txt`
  - `release/SHA256SUMS.txt`
- Remaining risk: Linux perf hardware counters are unavailable (`perf_event_paranoid=4`); allocator counts are captured by `scripts/alloc-tracer.c` and are explicitly scoped as startup instrumentation.
- Next dependency-ready task: the next packet in `indexes/EXECUTION_QUEUE.md`.
