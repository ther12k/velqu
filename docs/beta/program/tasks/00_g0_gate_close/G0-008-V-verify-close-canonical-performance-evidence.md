---
task_id: G0-008-V
parent_task: G0-008
milestone: G0
priority: P1
mode: VERIFY
status: PASS
context_card: context/milestones/G0.md
commit_required: true
---

# G0-008-V — Verify Close canonical performance evidence

## Atomic goal

Prove every acceptance criterion for parent task G0-008 without broadening scope.

## Parent intent

Close the frozen G0 benchmark evidence requirements instead of relying on a single clean pass.

## Dependencies

- `G0-008-A` — `tasks/00_g0_gate_close/G0-008-A-run-warm-workloads-at-concurrency-1-10-and-50-for-at-least-five-independent-repe.md`
- `G0-008-B` — `tasks/00_g0_gate_close/G0-008-B-run-fresh-process-cold-start-measurements-for-25-1-000-and-10-000-routes-with-ra.md`
- `G0-008-C` — `tasks/00_g0_gate_close/G0-008-C-capture-cpu-rss-errors-p50-p95-p99-binary-pack-hashes-machine-state-and-load-gen.md`
- `G0-008-D` — `tasks/00_g0_gate_close/G0-008-D-capture-allocation-startup-profiles-including-the-10-000-route-json-pack-parsing.md`
- `G0-008-E` — `tasks/00_g0_gate_close/G0-008-E-generate-markdown-reports-from-raw-data-and-make-verification-fail-when-raw-repo.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/G0.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-router/src/lib.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-http/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `Cargo.toml`
- `benchmarks/harness/`
- `benchmarks/manifest.json`
- `benchmarks/raw/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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

## Required evidence for this microtask

- Raw benchmark directory.
- Generated report.
- Environment and artifact manifest.
- Ablation results for relevant changes.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
g0-008-v: verify close canonical performance evidence
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
