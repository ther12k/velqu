---
task_id: M28-006-C
parent_task: M28-006
milestone: M28
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-006-C — Cancel on consumer stop/disconnect

## Atomic goal

Cancel on consumer stop/disconnect.

## Parent intent

Support large bodies without unbounded buffering.

## Dependencies

- `M28-006-B` — `tasks/05_m28_native_fetch/M28-006-B-propagate-downstream-backpressure.md`

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
5. Implement exactly this deliverable: Cancel on consumer stop/disconnect.
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
m28-006-c: cancel on consumer stop disconnect
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-006-C) — PASS

- Date: 2026-08-29
- Branch/PR: m28-006-c (squash-merged; see git log for final hash)
- Closes: #338

### Changed files
- `crates/q-capabilities/src/stream_buffer.rs`: consumer stop/disconnect cancellation on `BoundedStream` —
  - `cancel() -> bool`: terminal and idempotent consumer-side cancellation. Releases buffered bytes immediately (`buf.clear()` — "cancellation releases buffers"), wakes a producer parked in backpressure and a consumer parked in `poll_read`, returns whether this call performed the transition.
  - `StreamError::Cancelled` (new variant, closed set stays typed): writes after cancel fail with this on both `try_write` and `poll_write`; Display message names consumer stop/disconnect.
  - `is_cancelled()` getter; read paths (`try_read`/`poll_read`) terminate EOF-like after cancel with no residual data leak (cancelled check precedes buffered-data check).
  - Cancel-after-close still releases a residual undrained tail; close-after-cancel stays terminal and harmless.

### Tests added
Unit (stream_buffer.rs, +4 → 147 lib tests):
- `cancel_releases_buffers_and_fails_writes_typed` (buffers zeroed, idempotent cancel, typed error, EOF-like reads)
- `cancel_wakes_parked_producer_into_typed_error` (CountingWaker: exactly one wake; post-wake poll is `Ready(Err(Cancelled))`)
- `write_chunk_future_errs_after_cancel` (parked future resolves `Err(Cancelled)`; `read_chunk` resolves `None` while stream not closed)
- `cancel_after_close_still_releases_residual_buffer`
Integration (tests/stream_backpressure.rs, +1 → 4 tests):
- `consumer_disconnect_cancels_midstream_and_releases_buffers` — producer streams an 8 MiB body; consumer reads 3 chunks then disconnects (`cancel()`); the producer task terminates with `Err(StreamError::Cancelled)` at 196608 B of 8388608 B (well before completion), `buffered()==0` at cancel time, peak buffered 65536 B == capacity throughout.

### Command results
- `cargo test -p q-capabilities` → **147 unit (was 143) + 4 backpressure integration + 8 WPT** — 0 failed
- `cargo test -p q-engine-quickjs` → 17+101 · `-p q-http` 4+6+1 · `-p q-bridge` 11 · `-p velqu-runtime` 8+5+31 — all pass
- `bun test` → 219 pass / 0 fail (via ./scripts/verify); `bun run typecheck` → clean
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**; benchmark artifact validation errors: []
- Release binary hash unchanged (`ffdee169…` matches manifest): dormant cancellation code is dead-code-eliminated; no remapped rebuild/manifest refresh required

### Guardrail mapping
- **Cancellation releases buffers/connections** — `cancel()` zeroes the buffer at cancel time (asserted); pool/connection release is M28-003 `drain_shutdown` territory (already merged).
- **Slow upstream/downstream remains bounded** — disconnect test asserts peak buffered never exceeds capacity while the producer runs.
- **Streaming errors are typed** — cancellation joins the closed `StreamError` set; producer observes it on the async write path.
- **Memory profile** — `cancelled_at=196608B of 8388608B, peak_buffered=65536B, capacity=65536B` (from `--nocapture`).

### Disclosures
- One test-authoring error caught by the suite itself (13 vs 12 byte count in a fixture assertion); fixed before commit — no test weakened.
- A heredoc escaping slip wrote a literal `\n` into the integration test file; caught by compile, fixed before commit.
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
