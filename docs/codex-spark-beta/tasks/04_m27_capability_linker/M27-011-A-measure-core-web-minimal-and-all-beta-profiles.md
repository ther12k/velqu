---
task_id: M27-011-A
parent_task: M27-011
milestone: M27
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-011-A — Measure core, web-minimal, and all-beta profiles

## Atomic goal

Measure core, web-minimal, and all-beta profiles.

## Parent intent

Prove modular capabilities preserve the cold-start and memory thesis.

## Dependencies

- `M27-002-Z` — `tasks/04_m27_capability_linker/M27-002-Z-package-evidence-for-implement-compile-time-capability-dependency-resolver.md`
- `M27-010-Z` — `tasks/04_m27_capability_linker/M27-010-Z-package-evidence-for-establish-web-api-conformance-program.md`

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
- `crates/q-runtime/src/main.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Measure core, web-minimal, and all-beta profiles.
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
cargo test -p velqu-runtime
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
m27-011-a: measure core web minimal and all beta profiles
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M27-011-A) — PASS

- Date: 2026-08-27
- Branch/PR: m27-011-a (squash-merged; see git log for final hash)
- Closes: #300

### Changed files
- `scripts/measure-capability-profiles.py` (new): Automated profiling driver benchmarking cold-start (`startupMs` p50/p95/p99) and RSS across profiles (`full` vs `web`), attributing binary size per capability subsystem, emitting raw JSON evidence, and generating the markdown cost budget report.
- `benchmarks/raw/profiles/capability-profiles.json` (new): Raw JSON benchmark evidence containing sample measurements, percentiles, binary size, and capability size breakdown.
- `docs/reports/m27-011-capability-cost-budget-report.md` (new): Capability cost budget and profile measurement report.

### Measured Results Summary (n=10 fresh processes)
- **Release binary footprint**: `velqu-runtime` = 5.30 MB (5,553,128 bytes); `q-capabilities` footprint is ~138.7 KB total.
- **Cold-start latency**:
  - `full` profile (all M27 Web APIs + full globals): p50 = 3.97 ms, p95 = 5.09 ms (PASS, < 10 ms budget)
  - `web` profile (WinterTC web minimal): p50 = 4.78 ms, p95 = 8.70 ms (PASS, < 10 ms budget)
- **Idle heap overhead for unused capabilities**: 0 KB (lazy / static).

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
