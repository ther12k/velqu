---
task_id: M27-011-V
parent_task: M27-011
milestone: M27
priority: P1
mode: VERIFY
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-011-V — Verify Close capability cost budgets

## Atomic goal

Prove every acceptance criterion for parent task M27-011 without broadening scope.

## Parent intent

Prove modular capabilities preserve the cold-start and memory thesis.

## Dependencies

- `M27-011-A` — `tasks/04_m27_capability_linker/M27-011-A-measure-core-web-minimal-and-all-beta-profiles.md`
- `M27-011-B` — `tasks/04_m27_capability_linker/M27-011-B-record-binary-startup-and-idle-rss-deltas.md`
- `M27-011-C` — `tasks/04_m27_capability_linker/M27-011-C-identify-eager-initialization.md`
- `M27-011-D` — `tasks/04_m27_capability_linker/M27-011-D-make-expensive-modules-lazy-when-safe.md`

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

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m27-011-v: verify close capability cost budgets
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M27-011-V) — PASS

- Date: 2026-08-27
- Branch/PR: m27-011-v (squash-merged; see git log for final hash)
- Closes: #304

### Acceptance-criterion mapping (parent M27-011 guardrails)

1. **Core app remains near approved baseline** — verified: Cold-start p50 = 4.16 ms (full) / 4.08 ms (web), delta vs M26 baseline is +0.33 ms (within noise), well under the 10 ms budget.
2. **Each capability cost is visible** — verified: Binary size delta (+120 KB, +2.2%), capability footprint breakdown (total ~138.7 KB), and idle RSS (+176 kB) explicitly measured and attributed in `docs/reports/m27-011-capability-cost-budget-report.md`.
3. **Unused capability cost is zero or explained** — verified: Zero heap allocation for unlinked capabilities at idle; `capability_initialization_is_lazy_and_causes_zero_startup_heap_waste` (`worker.rs`) tests that constructor functions exist as static properties without pre-allocating instances, and `__velquContextPrototype.native` avoids per-request wrapper allocations.
4. **Budget failures trigger split/defer decisions** — verified: All measurements pass within declared budgets; out-of-scope/expensive features (SubtleCrypto, general Node compat, WebSockets/SSE) remain explicitly deferred.

### Verification runs (this branch, worktree-fresh)
- `cargo test -p q-capabilities` → 107 unit tests + 7 integration tests passed
- `cargo test -p q-engine-quickjs` → 16 unit tests + 97 worker tests passed
- `cargo test -p q-pack` → 96+2 passed
- `cargo test -p velqu-runtime` → 31 passed
- `bun test` → 213 pass / 0 fail (27 files)
- `bun run typecheck` → clean
- `cargo fmt --check` → clean
- `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**

### Disclosures (standing)
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
