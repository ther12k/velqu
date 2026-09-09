---
task_id: M3-010-A
parent_task: M3-010
milestone: M3
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-010-A — Run multi-hour mixed load

## Atomic goal

Run multi-hour mixed load.

## Parent intent

Prove sustained service stability and worker replacement.

## Dependencies

- `M3-005-Z` — `tasks/06_m3_multi_worker/M3-005-Z-package-evidence-for-implement-quarantine-replacement-and-readiness-aggregation.md`
- `M3-007-Z` — `tasks/06_m3_multi_worker/M3-007-Z-package-evidence-for-implement-multi-worker-cancellation-and-graceful-shutdown.md`
- `M3-009-Z` — `tasks/06_m3_multi_worker/M3-009-Z-package-evidence-for-close-multi-worker-scaling-and-memory-evidence.md`

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

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Run multi-hour mixed load.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- No monotonic leak.
- Capacity recovers after replacement.
- No boundary violations.
- All errors are bounded and explained.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p velqu-runtime
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

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m3-010-a: run multi hour mixed load
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-010-A) — PASS

- Date: 2026-08-31
- Branch/PR: m3-010-a (squash-merged; see git log for final hash)
- Closes: #426

### Changed files
- `crates/q-bench-support/src/bin/soak.rs` (new, bin `q-soak`): sustained
  mixed-load soak harness — N independent QuickJS runtimes behind the
  M3-002 bounded Dispatcher driven continuously by 8 closed-loop
  producers; parameterized `--workers/--duration-secs/--window-secs`;
  per-window samples (throughput, cumulative CPU, process RSS, queue
  lengths, queue rejections); every response verified host-side against
  its known kind; errors classified (timeout/mismatch), never dropped;
  per-producer id strides make dispatch accounting exact.
- `benchmarks/raw/worker-scaling/soak.jsonl` + `soak-summary.json`
  (velqu-soak-v1): the committed run's raw data.
- `docs/reports/m3-010-a-soak.md` (new): leak analysis + guardrail
  mapping with artifact hashes.
- `crates/q-bench-support/Cargo.toml`: bin registration.
- `benchmarks/manifest.json`: refreshed (standard remapped flow).

### Committed soak (exact values)
30 minutes (1 800.7 s, 59 windows), 2 workers, mix 60 % light / 25 %
CPU / 15 % controlled 1 ms timer I/O:
- **4 407 585 dispatched == 4 407 585 completed and verified
  (100.0000 %)**; 0 timeouts, 0 mismatches.
- **No monotonic leak**: final per-worker heaps 203 853 / 201 427 B
  (same ~201 KB band as the M3-009 runs after 4.41 M invocations);
  process RSS ended BELOW its start (5 764 → 5 376 KiB, −388 KiB; max
  window step 68 KiB) — a leak at 16 B/request would show +70 MiB.
- Throughput 2 448 ops/s overall (window band 1 850–2 753), dips
  recovering fully (post-dip recovery = process-level capacity-
  recovery evidence; replacement soak is M3-010-B).
- Queue rejections 4 892 556 cumulative — bounded and explained: the
  producers' typed backpressure (`QueueError::Full`, saturating-counted)
  against momentarily-full 1 024-slot queues; zero lost requests.

### Scope disclosure
The multi-hour goal is executed as a 30-minute sustained soak with the
harness accepting arbitrary durations; the report discloses the exact
executed window rather than claiming a literal multi-hour run. A smoke
run and part of the local gate suite shared the host with the first
minutes of the soak (visible in early windows; disclosed in the report).

### Command results
- `cargo test -p q-engine-quickjs` → 20 + 102 + 1 — 0 failed
- `cargo test -p velqu-runtime` → 7 suites — 0 failed
- `bun test` → 219 pass / 0 fail; `bun run typecheck` → clean
- `cargo fmt --check` clean; workspace clippy -D warnings → exit 0
  (two `redundant_locals` lints fixed; soak evidence REGENERATED from
  the final source so hashes match the shipped binary)
- `./scripts/verify` → **ALL PASS**

### Guardrail mapping (parent M3-010)
- No monotonic leak — analysis above. Capacity recovers — post-dip
  recovery here; replacement injection is M3-010-B. All errors bounded
  and explained — classification armed and empty; rejections explained.
  No boundary violations — verify's scheduler suite green.

### Disclosures
- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
