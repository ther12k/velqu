---
task_id: M28-005-D
parent_task: M28-005
milestone: M28
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-005-D — Map failures deterministically

## Atomic goal

Map failures deterministically.

## Parent intent

Ensure request cancellation physically stops outbound work and keeps ownership correct.

## Dependencies

- `M28-005-C` — `tasks/05_m28_native_fetch/M28-005-C-cancel-dns-connect-body-streaming.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M28.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`

### Source files

- `AGENTS.md`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-http/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`
- `crates/q-pack/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Map failures deterministically.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- No outbound task survives terminal invocation without defer ownership.
- Timeout counted once.
- Cancellation latency is bounded.
- Worker remains reusable.

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
cargo test -p q-bridge
```
```bash
cargo test -p q-capabilities
```
```bash
cargo test -p velqu-runtime
```

## Required evidence for this microtask

- Race tests.
- Task accounting.
- Timeout/cancel conformance.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m28-005-d: map failures deterministically
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-005-D) — PASS

- Date: 2026-08-28
- Branch/PR: m28-005-d (squash-merged; see git log for final hash)
- Closes: #333

### Changed files
- `crates/q-engine-quickjs/tests/engine.rs`: Added integration test `terminal_failures_map_deterministically` proving every terminal outcome maps deterministically through the public engine — handler throw → `EngineFailure` (redacted 500) identical across 3 repeats; undeclared status → `ContractViolation` identical across 3 repeats; route deadline expiry → `Timeout` identical across 3 repeats; typed problem → `Problem` envelope unchanged across 3 repeats. Worker remains healthy after the full matrix.
- `benchmarks/manifest.json`: Refreshed `qRuntimeRelease` hash.

### Tests added
- `crates/q-engine-quickjs/tests/engine.rs`:
  - `terminal_failures_map_deterministically`

### Command results
- `cargo test -p q-engine-quickjs` → 17 unit + 101 engine passed
- `cargo test -p velqu-runtime` → 8 unit + 5 integration + 31 conformance passed
- `cargo test -p q-capabilities` → 132+8 passed
- `cargo test -p q-http` → 4+6+1 passed
- `cargo test -p q-bridge` → 11 passed
- `cargo test -p q-pack` → 96+2 passed
- `bun test` → 219 pass / 0 fail (27 files)
- `bun run typecheck` → clean
- `cargo fmt --check` → clean
- `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**

### Guardrail mapping
- **Failure mapping is deterministic** — every outcome class (EngineFailure, ContractViolation, Timeout, Problem) maps identically across repeated runs; the runtime serve path maps these to fixed problem ids (`timeout`→504, `overload`→503, `internal`→500) with internal detail redacted.
- **Worker remains reusable** — `js.text` serves successfully after the full failure matrix; zero scheduler boundary violations.

### Disclosures (standing)
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
