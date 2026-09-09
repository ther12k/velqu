---
task_id: M3-009-Z
parent_task: M3-009
milestone: M3
priority: P1
mode: EVIDENCE
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-009-Z — Package evidence for Close multi-worker scaling and memory evidence

## Atomic goal

Create source-backed evidence and handoff for parent task M3-009; update status only if verification passed.

## Parent intent

Demonstrate real scaling without hiding queue latency or per-worker RSS.

## Dependencies

- `M3-009-V` — `tasks/06_m3_multi_worker/M3-009-V-verify-close-multi-worker-scaling-and-memory-evidence.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M3.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/multiworker.md`

### Source files

- `AGENTS.md`
- `crates/q-runtime/src/main.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/lib.rs`
- `crates/q-pack/src/lib.rs`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `docs/specs/pack-format-v1.md`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- 2 workers achieve approved scaling target or limitation is documented.
- 4-worker memory is budgeted.
- Serverless profile remains unchanged.
- No p99 collapse under saturation.

## Targeted commands

```bash
cargo test -p q-pack
```
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

- Raw scaling data.
- Generated report.
- Artifact/environment hashes.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m3-009-z: package evidence for close multi worker scaling and memory e
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-009-Z) — PASS

- Date: 2026-08-31
- Branch/PR: m3-009-z (squash-merged; see git log for final hash)
- Closes: #425
- Parent verification: M3-009-V PASS (PR #1028, merged ba77906) on the
  identical tree; this packet packages the evidence and flips the ledger.

### Evidence package (parent M3-009 — multi-worker scaling & memory evidence)
- **Implementation commits (squash-merged):**
  - M3-009-A measure 1/2/4 workers — #1024 → ede475d
  - M3-009-B metrics report — #1025 → 476b1fb
  - M3-009-C C1/C2/C3 + controlled I/O — #1026 → cff83c0
  - M3-009-D physical core topology — #1027 → bf8c8b0
  - M3-009-V verification closure — #1028 → ba77906
- **Raw evidence:** `benchmarks/raw/worker-scaling/` — v4
  worker-scaling.jsonl (71 100 samples incl. per-sample queue wait),
  worker-scaling-summary.json (velqu-worker-scaling-v4 with the
  physicalTopology block), host-topology.json (cpuinfo-bound).
- **Generated reports:** `docs/reports/m3-009-a-worker-scaling.md`,
  `m3-009-b-multiworker-metrics.md`, `m3-009-c-controlled-workloads.md`,
  `m3-009-d-host-topology.md` — each with SHA-256 artifact hashes.
- **Headline measurements:** 1→2→4 workers scale 1.97–2.39× / 3.53–4.03×
  (medians, per workload); service p99 flat across W on every run (no
  p99 collapse); per-worker heap identical (201 339 B / 204 182 B C3)
  and stable; 0 classified errors across 45 000 verified requests.
- **Open item (owner decision):** the numeric 2-worker scaling target
  is UNAPPROVED — the guardrail is satisfied via the documented-
  limitation branch; tracked with REVIEW_INDEX open items.
- **Gate results (this branch, worktree-fresh):** `./scripts/verify`
  **ALL PASS** (incl. q-engine-quickjs, velqu-runtime, bun scoped tests,
  fmt, workspace clippy -D warnings).

### Ledger
- `docs/beta/04_TASK_LEDGER.md`: M3-009 TODO → **PASS** (all four
  guardrails proven; see the M3-009-V mapping).

### Disclosures (standing)
- No runtime behavior changed in this packet: evidence-only closure.
- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
