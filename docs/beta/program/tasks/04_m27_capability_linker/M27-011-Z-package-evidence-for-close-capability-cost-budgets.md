---
task_id: M27-011-Z
parent_task: M27-011
milestone: M27
priority: P1
mode: EVIDENCE
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-011-Z — Package evidence for Close capability cost budgets

## Atomic goal

Create source-backed evidence and handoff for parent task M27-011; update status only if verification passed.

## Parent intent

Prove modular capabilities preserve the cold-start and memory thesis.

## Dependencies

- `M27-011-V` — `tasks/04_m27_capability_linker/M27-011-V-verify-close-capability-cost-budgets.md`

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
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `packages/treaty/src/index.ts`
- `packages/contract/src/index.ts`
- `packages/testing/src/index.ts`
- `conformance/treaty/treaty.conformance.test.ts`
- `crates/q-pack/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

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

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m27-011-z: package evidence for close capability cost budgets
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M27-011-Z) — PASS

- Date: 2026-08-27
- Branch/PR: m27-011-z (squash-merged; see git log for final hash)
- Closes: #305

### Parent closure — M27-011 Close capability cost budgets

Parent intent: prove modular capabilities preserve the cold-start and memory thesis. Status: **PASS**.

Packet commits (squash merges):
- M27-011-A — 17f25df (#902, Closes #300): Measured cold-start, RSS, and capability footprints across runtime profiles (`scripts/measure-capability-profiles.py`, `benchmarks/raw/profiles/capability-profiles.json`, `docs/reports/m27-011-capability-cost-budget-report.md`)
- M27-011-B — 7d87fa1 (#903, Closes #301): Recorded M26 baseline vs M27 capability deltas matrix (binary size +120 KB, cold-start +0.33 ms noise, idle RSS +176 kB, 0 B heap overhead for unlinked capabilities)
- M27-011-C — f495ba9 (#904, Closes #302): Eager vs lazy initialization audit and tests in `crates/q-engine-quickjs/src/worker.rs`
- M27-011-D — 65ab8cf (#905, Closes #303): Made `c.native` inherited from `__velquContextPrototype.native` to eliminate per-request object wrapper allocations
- M27-011-V — 894ea97 (#906, Closes #304): Verification closure mapping all 4 acceptance guardrails

### Evidence ledger (required microtask evidence)
- **Cost matrix & Cold/RSS raw data**: `docs/reports/m27-011-capability-cost-budget-report.md` + `benchmarks/raw/profiles/capability-profiles.json`.
- **Capability ABI**: Versioned, bounded, cancellable, and testable (ADR-0028..0032).
- **Zero cost for unused capabilities**: 0 bytes persistent heap overhead, 0 extra bridge invocations.
- **CI output / Local gates**: All suites re-run green on this branch (q-capabilities 107+7, q-pack 96+2, q-engine-quickjs 16+97, velqu-runtime 31, bun test 213/213, ./scripts/verify ALL PASS).

### Command results (this branch)
- `cargo test -p q-capabilities` → 107 unit tests + 7 integration tests passed
- `cargo test -p q-engine-quickjs` → 16 unit tests + 97 worker tests passed
- `cargo test -p q-pack` → 96+2 passed
- `cargo test -p velqu-runtime` → 31 passed
- `bun test` → 213 pass / 0 fail (27 files)
- `bun run typecheck` → clean
- `cargo fmt --check` → clean
- `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**

### Ledger update
- `docs/beta/04_TASK_LEDGER.md`: M27-011 flipped TODO -> PASS.

### Disclosures (standing)
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
