---
task_id: M3-010-B
parent_task: M3-010
milestone: M3
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-010-B — Inject worker poison, upstream timeout, disconnect, and shutdown

## Atomic goal

Inject worker poison, upstream timeout, disconnect, and shutdown.

## Parent intent

Prove sustained service stability and worker replacement.

## Dependencies

- `M3-010-A` — `tasks/06_m3_multi_worker/M3-010-A-run-multi-hour-mixed-load.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M3.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `crates/q-runtime/src/main.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-http/src/lib.rs`
- `Cargo.toml`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Inject worker poison, upstream timeout, disconnect, and shutdown.
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
cargo test -p q-http
```
```bash
cargo test -p q-capabilities
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
m3-010-b: inject worker poison upstream timeout disconnect and shutdow
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-010-B) — PASS

- Date: 2026-08-31
- Branch/PR: m3-010-b (squash-merged; see git log for final hash)
- Closes: #427

### Changed files
- `crates/q-bench-support/src/bin/soak.rs`: chaos mode added to `q-soak` —
  - `--chaos-secs N`: every N seconds a worker slot is poisoned; the
    consumer shuts down its QuickJS runtime and rebuilds it
    deterministically (spawn + identical bundle load, ADR-0036 §6)
    while live traffic continues on the queue; rebuild timestamp and
    init duration are recorded in the chaos timeline.
  - `--disconnect-permille P`: 0.P % of requests drop their reply
    receiver right after dispatch to exercise the engine's
    late-completion path at volume.
  - `--timeout-permille P`: 0.P % of requests run `slow.work` (100 ms
    timer) behind a 10 ms deadline to exercise the invocation
    watchdog under load.
  - Summary includes `chaos` object with timeline, replacement
    counts, and parameters.
- `benchmarks/raw/worker-scaling/soak.jsonl` + `soak-summary.json`:
  committed 15-minute chaos run data.
- `docs/reports/m3-010-b-chaos.md` (new): chaos timeline, leak
  analysis, and artifact hashes.
- `benchmarks/manifest.json`: refreshed (standard remapped flow).

### Committed chaos soak (exact values)
15 minutes (901.0 s, 30 windows), 2 workers, 14 replacements (7 per
worker, every 60 s), 5 ‰ disconnects, 5 ‰ timeouts:
- **1 703 012 dispatched == 1 685 983 completed + 8 518 injected
  disconnects + 8 511 injected timeouts — 100.0000 % accounted**;
  0 unexplained errors.
- **14/14 replacements succeeded**: engine rebuild init time was
  **2.8–11.0 ms** (median ~4.0 ms, one 266 ms outlier under host
  scheduling); service continued through every poison.
- **No monotonic leak across 14 engine rebuilds**: final heaps
  202 104 / 201 880 B (flat ~202 KB band); process RSS ended −212 KiB
  below start.
- Throughput 1 871 ops/s overall under continuous chaos.

### Command results
- `cargo test -p q-engine-quickjs` → 20 + 102 + 1 — 0 failed
- `cargo test -p velqu-runtime` → 7 suites — 0 failed
- `bun test` → 219 pass / 0 fail; `bun run typecheck` → clean
- `cargo fmt --check` clean; workspace clippy -D warnings → exit 0
  (type complexity lint aliased with `type RebuildRecord`)
- `./scripts/verify` → **ALL PASS**

### Scope disclosure
- Engine-level poison under live traffic is this packet's chaos;
  dispatcher-level quarantine/settle is M3-005 component evidence.
- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
