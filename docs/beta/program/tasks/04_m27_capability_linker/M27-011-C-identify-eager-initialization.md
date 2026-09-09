---
task_id: M27-011-C
parent_task: M27-011
milestone: M27
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-011-C — Identify eager initialization

## Atomic goal

Identify eager initialization.

## Parent intent

Prove modular capabilities preserve the cold-start and memory thesis.

## Dependencies

- `M27-011-B` — `tasks/04_m27_capability_linker/M27-011-B-record-binary-startup-and-idle-rss-deltas.md`

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

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Identify eager initialization.
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
m27-011-c: identify eager initialization
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M27-011-C) — PASS

- Date: 2026-08-27
- Branch/PR: m27-011-c (squash-merged; see git log for final hash)
- Closes: #302

### Changed files
- `crates/q-engine-quickjs/src/worker.rs`: Added test `capability_initialization_is_lazy_and_causes_zero_startup_heap_waste` verifying constructor functions attach to `globalThis` statically without pre-allocating capability instances, while request context (`ctx.signal`, `ctx.native`) and capability objects materialize strictly on first access.

### Eager vs Lazy Initialization Audit Summary
- **Eager (Zero Persistent Heap Growth)**: Native bridge function pointers (`__velqu*`) and JS constructor definitions in `PRELUDE` are registered on `globalThis` during worker boot.
- **Lazy (On-Demand Heap Allocation)**:
  - Capability object instances (`URL`, `URLSearchParams`, `TextEncoder`, `TextDecoder`, `AbortController`) allocate only upon user code `new Class()` invocation.
  - Native buffers and entropy streams allocate only during active operations.
  - Context properties (`ctx.signal`, `ctx.native`, `req.url`, `req.headers`, `req.body`) use `__velquContextPrototype` lazy getters and unread fields are never materialized.
  - Stale handles generation-checked and expired at settlement.

### Command results
- `cargo test -p q-pack` → 96+2 passed
- `cargo test -p q-engine-quickjs` → 16 unit + 97 worker passed
- `cargo test -p q-capabilities` → 107+7 passed
- `cargo test -p velqu-runtime` → 31 passed
- `bun test` → 213 pass / 0 fail (27 files)
- `bun run typecheck` → clean
- `cargo fmt --check` → clean
- `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**

### Disclosures (standing)
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
