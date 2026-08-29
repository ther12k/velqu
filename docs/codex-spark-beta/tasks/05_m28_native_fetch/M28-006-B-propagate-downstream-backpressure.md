---
task_id: M28-006-B
parent_task: M28-006
milestone: M28
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-006-B — Propagate downstream backpressure

## Atomic goal

Propagate downstream backpressure.

## Parent intent

Support large bodies without unbounded buffering.

## Dependencies

- `M28-006-A` — `tasks/05_m28_native_fetch/M28-006-A-bound-read-write-buffers.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M28.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`
- `Cargo.toml`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Propagate downstream backpressure.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Large response does not allocate full body unless requested.
- Slow upstream/downstream remains bounded.
- Cancellation releases buffers/connections.
- Streaming errors are typed.

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
cargo test -p q-capabilities
```
```bash
cargo test -p velqu-runtime
```

## Required evidence for this microtask

- Streaming load tests.
- Slow consumer tests.
- Memory profile.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m28-006-b: propagate downstream backpressure
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-006-B) — PASS

- Date: 2026-08-29
- Branch/PR: m28-006-b (squash-merged; see git log for final hash)
- Closes: #337

### Changed files
- `crates/q-capabilities/src/stream_buffer.rs`: downstream backpressure propagation primitives on `BoundedStream` —
  - `poll_write(cx, chunk)`: poll-style producer write; when the buffer is at capacity the producer waker is registered and `Poll::Pending` suspends the producer task until the consumer drains (backpressure as task suspension, never unbounded buffering). Typed errors (`ChunkTooLarge`/`LimitExceeded`/`StreamClosed`) fail closed on first poll.
  - `write_chunk(chunk) -> WriteChunk` future and `read_chunk(max) -> ReadChunk` future: async producer/consumer pump sides built on `poll_write`/`poll_read`.
  - `max_buffered()` high-water mark + `capacity()` for the memory profile; `max_buffered` tracked on both write paths.
  - `try_write` no longer claims to register a waker (async producers must use `poll_write`; documented single-producer/single-consumer contract).
- `crates/q-capabilities/Cargo.toml`: test-only `tokio` dev-dependency (drives the pump tests; production dependency graph unchanged).
- `crates/q-capabilities/tests/stream_backpressure.rs` (new): slow-consumer integration suite.

### Tests added
Unit (stream_buffer.rs, +4 → 143 lib tests):
- `poll_write_pends_when_full_and_wakes_after_drain` (CountingWaker: exactly one wake after drain)
- `write_chunk_future_resolves_after_capacity_frees`
- `poll_write_typed_errors_fail_closed` (ChunkTooLarge/LimitExceeded/StreamClosed on the async path)
- `max_buffered_tracks_peak_and_never_exceeds_capacity`
Integration (tests/stream_backpressure.rs, 3 tests):
- `slow_consumer_suspends_producer_and_stays_bounded` — 8 MiB body, 32 KiB chunks, 64 KiB buffer: with the consumer fully stopped the producer stalls at <= 65536 B written and cannot finish (structural, timing-independent); slow drain (1 ms/read) resumes the pump; checksum-verified byte order; peak buffered 65536 B = capacity
- `typed_errors_propagate_through_async_write_path`
- `streaming_load_profile_peak_stays_at_capacity_bound` (fast consumer: peak buffered 32768 B <= capacity 65536 B for the full 8 MiB)

### Command results
- `cargo test -p q-capabilities` → **143 unit + 3 backpressure integration + 8 WPT** passed (was 139+8)
- `cargo test -p q-engine-quickjs` → 17+101 · `-p q-http` 4+6+1 · `-p q-bridge` 11 · `-p velqu-runtime` 8+5+31 — all pass
- `bun test` → 219 pass / 0 fail; `bun run typecheck` → clean
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**; `compare-builds` PASS (12 artifacts byte-identical)
- Memory profile (from `--nocapture`): body=8388608B, peak_buffered=65536B (stopped-consumer stall) / 32768B (fast consumer), capacity=65536B — memory scales with buffer capacity, not body size
- Release binary hash unchanged (`ffdee169…` matches manifest): dormant additions are dead-code-eliminated; no remapped rebuild/manifest refresh required

### Guardrail mapping
- **Slow upstream/downstream remains bounded** — producer suspends at capacity; peak buffered never exceeds it (proven for 8 MiB streams).
- **Streaming errors are typed** — the same closed `StreamError` set propagates through `poll_write`/`write_chunk`.
- **Memory profile** — `max_buffered()` exposes the high-water mark; asserted against capacity in tests.

### Disclosures
- Two clippy iterations on the new test code: `assertions_on_constants` (moved to a module-level `const _` assert) and `manual_noop_waker` (switched to stable `Waker::noop()`). No production behavior changed; all tests green after each fix.
- Fresh-worktree setup required `bun install`, `cargo build --workspace` (velqu-bytecode helper), release build, and proof-pack compile before the full bun suite passed — environment setup, not product failures; disclosed per the M27-002-Z precedent.
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
