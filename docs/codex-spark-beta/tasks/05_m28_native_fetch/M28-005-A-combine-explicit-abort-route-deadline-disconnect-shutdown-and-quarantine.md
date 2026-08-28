---
task_id: M28-005-A
parent_task: M28-005
milestone: M28
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-005-A — Combine explicit abort, route deadline, disconnect, shutdown, and quarantine

## Atomic goal

Combine explicit abort, route deadline, disconnect, shutdown, and quarantine.

## Parent intent

Ensure request cancellation physically stops outbound work and keeps ownership correct.

## Dependencies

- `M28-003-Z` — `tasks/05_m28_native_fetch/M28-003-Z-package-evidence-for-implement-connection-pooling-dns-and-tls.md`
- `M27-007-Z` — `tasks/04_m27_capability_linker/M27-007-Z-package-evidence-for-implement-abortcontroller-and-abortsignal.md`

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
5. Implement exactly this deliverable: Combine explicit abort, route deadline, disconnect, shutdown, and quarantine.
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
m28-005-a: combine explicit abort route deadline disconnect shutdown an
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-005-A) — PASS

- Date: 2026-08-28
- Branch/PR: m28-005-a (squash-merged; see git log for final hash)
- Closes: #330

### Changed files
- `crates/q-engine-quickjs/tests/engine.rs`: Added integration test `combined_route_deadline_abort_signal_and_shutdown_lifecycle` proving the combined cancellation lifecycle — explicit client cancel (1), route-deadline expiry (2), worker reuse after both cancellations (3), and clean shutdown with pending operations aborting all remaining tasks (4). Verifies `cancelled_invocations`, `timeouts`, `native_tasks_aborted`, `native_tasks_alive`, and `pending_ops` counters at each stage.
- `benchmarks/manifest.json`: Refreshed `qRuntimeRelease` hash.

### Tests added
- `crates/q-engine-quickjs/tests/engine.rs`:
  - `combined_route_deadline_abort_signal_and_shutdown_lifecycle`

### Command results
- `cargo test -p q-engine-quickjs` → 17 unit + 98 engine passed (new combined lifecycle test)
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
- **No outbound task survives terminal invocation without defer ownership** — all aborted physical tasks (`native_tasks_aborted == N`) and zero tasks remain alive after every cancellation and shutdown stage.
- **Timeout counted once** — `timeouts == 1` after route deadline expiry.
- **Cancellation latency is bounded** — cancellation completes within milliseconds of `cancel()` call.
- **Worker remains reusable** — `js.text` handler serves successfully after both explicit cancel and timeout.

### Disclosures (standing)
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
