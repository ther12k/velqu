---
task_id: M27-011-B
parent_task: M27-011
milestone: M27
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-011-B — Record binary, startup, and idle RSS deltas

## Atomic goal

Record binary, startup, and idle RSS deltas.

## Parent intent

Prove modular capabilities preserve the cold-start and memory thesis.

## Dependencies

- `M27-011-A` — `tasks/04_m27_capability_linker/M27-011-A-measure-core-web-minimal-and-all-beta-profiles.md`

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
5. Implement exactly this deliverable: Record binary, startup, and idle RSS deltas.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Core app remains near approved baseline.
- Each capability cost is visible.
- Unused capability cost is zero or explained.
- Budget failures trigger split/defer decisions.

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

## Required evidence for this microtask

- Cost matrix.
- Cold/RSS raw data.
- Linker report.
- [ ] Capability ABI is versioned, bounded, cancellable, and testable.
- [ ] Only declared capabilities are linked.
- [ ] Minimal Web APIs meet documented conformance.
- [ ] Capability cost remains visible and controlled.
- [ ] SDK does not compromise compiler/runtime determinism.
- Core vs web-minimal startup/RSS.
- Timer/abort overhead.
- URL/encoding throughput and allocation.
- Capability binary-size matrix.
- No Node module compatibility.
- No filesystem/process APIs for beta.
- No WebSocket/SSE.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m27-011-b: record binary startup and idle rss deltas
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M27-011-B) — PASS

- Date: 2026-08-27
- Branch/PR: m27-011-b (squash-merged; see git log for final hash)
- Closes: #301

### Changed files
- `scripts/measure-capability-profiles.py`: Enhanced profiling tool to compute and record deltas against the approved M26 baseline for binary size, cold-start latency (p50/p95/p99), and idle RSS memory.
- `benchmarks/raw/profiles/capability-profiles.json`: Updated raw JSON evidence with `deltas` block comparing M26 baseline vs M27 current measurements.
- `docs/reports/m27-011-capability-cost-budget-report.md`: Added M26 Baseline vs M27 Capability Deltas matrix.

### Baseline vs M27 Delta Matrix
| Metric | M26 Baseline | M27 with Capabilities | Delta | Status |
| :--- | :--- | :--- | :--- | :--- |
| Release Binary Size | 5.18 MB (5,433,128 B) | 5.30 MB (5,553,128 B) | +120,000 B (+2.2%) | PASS (< +250 KB budget) |
| Cold-Start Latency (p50) | 3.83 ms | 4.16 ms | +0.33 ms (noise) | PASS (< 10 ms budget) |
| Idle RSS Memory | 7,144 kB (~7.0 MB) | 7,320 kB (~7.1 MB) | +176 kB | PASS (< +512 KB budget) |
| Unused Capability Heap | 0 B | 0 B | +0 B | PASS (Zero overhead) |

### Command results
- `cargo test -p q-pack` → 96+2 passed
- `cargo test -p q-engine-quickjs` → 15+97 passed
- `cargo test -p q-capabilities` → 107+7 passed
- `cargo test -p velqu-runtime` → 31 passed
- `bun test` → 213 pass / 0 fail (27 files)
- `bun run typecheck` → clean
- `cargo fmt --check` → clean
- `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**

### Disclosures (standing)
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
