---
task_id: M3-002-C
parent_task: M3-002
milestone: M3
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-002-C — Define admission and overload response

## Atomic goal

Define admission and overload response.

## Parent intent

Route matched requests to workers without unbounded queues or shared engine mutexes.

## Dependencies

- `M3-002-B` — `tasks/06_m3_multi_worker/M3-002-B-select-worker-using-outstanding-load-strategy.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M3.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
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
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Define admission and overload response.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Queue capacity is configurable and bounded.
- Overload fails quickly and observably.
- No head-of-line lock across workers.
- Per-worker queue latency is measured.

## Targeted commands

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
cargo test -p velqu-runtime
```

## Required evidence for this microtask

- Dispatcher tests.
- Overload load test.
- Metrics.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m3-002-c: define admission and overload response
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-002-C) — PASS

- Date: 2026-08-30
- Branch/PR: m3-002-c (squash-merged; see git log for final hash)
- Closes: #380

### Changed files
- `crates/q-capabilities/src/dispatch.rs`: the admission and overload response policy —
  - `AdmissionDecision { status, problem, retry_after_secs, detail }`: the typed verdict the HTTP layer renders; redacted by construction (congestion reason + bounds only, never job/client data, never worker topology).
  - `admission_response(&QueueError) -> AdmissionDecision`: total and deterministic — every QueueError variant maps to exactly one verdict. Both `Full` (per-worker) and `AllFull` (global) render the SAME client-facing verdict (503/overload/retry-1): which worker was full is scheduler topology, not the client's business.
  - `RETRY_AFTER_OVERLOAD_SECS = 1`: matches the runtime's existing quarantine retry-after posture so clients see one consistent backoff hint; `problem: "overload"` matches the existing RFC 9457 registry kind (`.../problems/overload`, 503) in `q-runtime/src/problems.rs`.
- `crates/q-capabilities/src/lib.rs`: re-exports (`admission_response`, `AdmissionDecision`, `RETRY_AFTER_OVERLOAD_SECS`).

### Tests added (+2 → 210 q-capabilities lib tests)
- `admission_response_is_total_deterministic_and_redacted` (503 + overload kind + retry 1 for both classes; determinism; topology stays internal)
- `admission_verdict_composes_with_dispatcher_overload` (saturated dispatcher -> AllFull -> the exact 503/overload/1 verdict)

### Command results
- `cargo test -p q-capabilities` → **210 unit (was 208) + 7 + 1 + 4 + 9** — 0 failed
- `cargo test -p q-engine-quickjs` → 20+101 · `-p q-http` 4+6+1 · `-p q-bridge` 11 · `-p velqu-runtime` 13+5+44 — all pass
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**; release binary unchanged (`333d563d…`)

### Guardrail mapping
- **Overload fails quickly and observably** — the verdict is typed, immediate, retryable (503 + Retry-After), and the per-queue rejected counters (M3-002-A) keep counting underneath.

### Disclosures
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
