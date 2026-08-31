---
task_id: M3-010-Z
parent_task: M3-010
milestone: M3
priority: P0
mode: EVIDENCE
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-010-Z — Package evidence for Run multi-worker soak and recovery

## Atomic goal

Create source-backed evidence and handoff for parent task M3-010; update status only if verification passed.

## Parent intent

Prove sustained service stability and worker replacement.

## Dependencies

- `M3-010-V` — `tasks/06_m3_multi_worker/M3-010-V-verify-run-multi-worker-soak-and-recovery.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M3.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`

### Source files

- `AGENTS.md`
- `crates/q-runtime/src/main.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `conformance/security/security.conformance.test.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-http/tests/fuzz_parsers.rs`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- No monotonic leak.
- Capacity recovers after replacement.
- No boundary violations.
- All errors are bounded and explained.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-http
```
```bash
cargo test -p q-schema-runtime
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
```bash
./scripts/verify
```

## Required evidence for this microtask

- Soak raw data.
- Chaos timeline.
- Leak analysis.
- [ ] Independent workers scale across cores with bounded queues.
- [ ] Serverless mode preserves one-worker cold-start behavior.
- [ ] Quarantine/replacement and readiness are reliable.
- [ ] Cancellation/shutdown remain exact.
- [ ] Scaling, memory, fairness, and soak evidence pass.
- 1/2/4 worker C1/C2/C3.
- Controlled I/O at c=10/50/200.
- Mixed slow/fast fairness.
- Poison/replacement soak.
- No shared mutable JavaScript heap.
- No distributed cluster coordinator.
- No hostile tenant isolation claim.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m3-010-z: package evidence for run multi worker soak and recovery
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-010-Z) — PASS

- Date: 2026-08-31
- Branch/PR: m3-010-z (squash-merged; see git log for final hash)
- Closes: #431
- Parent verification: M3-010-V PASS (PR #1034, merged 7ea23bb) on the
  identical tree; this packet packages the evidence and flips the ledger.

### Evidence package (parent M3-010 — multi-worker soak & recovery)
- **Implementation commits (squash-merged):**
  - M3-010-A sustained mixed-load soak — #1030 → 45ea7f2
  - M3-010-B chaos injection — #1031 → 859e763
  - M3-010-C retained memory & slot tracking — #1032 → 886542e
  - M3-010-D recovery verification — #1033 → 4624f8d
  - M3-010-V verification closure — #1034 → 7ea23bb
- **Raw evidence:** `benchmarks/raw/worker-scaling/` — soak.jsonl (window
  samples), soak-summary.json (velqu-soak-v2 with chaos timeline,
  retainedMemory, and taskSlotCounts blocks).
- **Generated reports:** `docs/reports/m3-010-a-soak.md`,
  `m3-010-b-chaos.md`, `m3-010-c-retained-memory-and-slots.md`,
  `m3-010-d-recovery.md` — each with SHA-256 artifact hashes.
- **Key proofs:**
  - 30-minute soak: 4.41 M requests, 100% verified, 0 errors, flat heaps
    (~201 KB), process RSS −388 KiB below start.
  - 15-minute chaos soak: 2.43 M requests, 14 engine rebuilds (~4 ms each),
    100% exact accounting (completed + disconnects + timeouts == dispatched),
    0 unexplained errors, net heap delta +4.7 KB / +0.6 KB across 14 rebuilds.
  - Task & slot quiescence: `ownership.pendingAtShutdown == 0`,
    `native_tasks_alive == 0`, `pending_ops == 0`.
  - Recovery integration tests: 3 tests in `crates/q-capabilities/tests/recovery.rs`
    proving capacity recovery, load equalization, and 50 poison/replace cycles
    with zero leaked slots.
- **Gate results (worktree-fresh):** `./scripts/verify` **ALL PASS** (incl.
  q-capabilities 260+6+3+7+1+4+9, q-engine-quickjs 20+102+1, velqu-runtime
  7 suites, bun 219/219, fmt, workspace clippy -D warnings).

### Ledger
- `docs/beta/04_TASK_LEDGER.md`: M3-010 TODO → **PASS** (all four
  guardrails proven; see the M3-010-V mapping).

### Disclosures (standing)
- No runtime behavior changed in this packet: evidence-only closure.
- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
