---
task_id: M28-006-Z
parent_task: M28-006
milestone: M28
priority: P0
mode: EVIDENCE
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-006-Z — Package evidence for Implement streaming and strict backpressure

## Atomic goal

Create source-backed evidence and handoff for parent task M28-006; update status only if verification passed.

## Parent intent

Support large bodies without unbounded buffering.

## Dependencies

- `M28-006-V` — `tasks/05_m28_native_fetch/M28-006-V-verify-implement-streaming-and-strict-backpressure.md`

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
- `crates/q-runtime/src/main.rs`
- `crates/q-engine-quickjs/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

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

- Streaming load tests.
- Slow consumer tests.
- Memory profile.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m28-006-z: package evidence for implement streaming and strict backpres
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-006-Z) — PASS

- Date: 2026-08-29
- Branch/PR: m28-006-z (squash-merged; see git log for final hash)
- Closes: #341

### Parent closure — M28-006 Implement streaming and strict backpressure

Parent intent: support large bodies without unbounded buffering. Status: **PASS**.

Packet commits (squash merges):
- M28-006-A — 85c8186 (#939, Closes #336): `BoundedStream` bounded chunk buffer — 1 MiB per-chunk ceiling (`ChunkTooLarge`), total body limit == ADR-0033 §9 response cap (`LimitExceeded`), capacity backpressure, `poll_read`, 64 KiB default buffer
- M28-006-B — 6be2cf3 (#940, Closes #337): backpressure propagation — `poll_write` parks the producer at capacity (`Poll::Pending` + waker), `write_chunk`/`read_chunk` pump futures, `max_buffered()` memory profile; slow consumer provably stalls the producer at 65536 B on an 8 MiB stream
- M28-006-C — 9756e3c (#941, Closes #338): consumer stop/disconnect — terminal idempotent `cancel()` releases buffers, wakes the parked producer into typed `StreamError::Cancelled`, EOF-like reads; mid-stream disconnect stops the producer at 196608 B of 8388608 B
- M28-006-D — 7316581 (#942, Closes #339): maximum body helper sizes — `MAX_BODY_HELPER_BYTES` = 16 MiB pinned to the network cap (≤ `MAX_TEXT_BUFFER_LEN`), `BodyHelper` enum + `check_body_helper_size()` + `FetchPolicyError::BodyTooLarge`, `__velquBodyHelperLimit()` native binding, fail-closed `TypeError` in `text()/json()/arrayBuffer()/bytes()` before any copy
- M28-006-V — edacae5 (#943, Closes #340): verification closure mapping all 4 acceptance guardrails to tests and profiles

### Evidence ledger (required microtask evidence)
- **Streaming load tests**: `crates/q-capabilities/tests/stream_backpressure.rs` — 8 MiB body in 32 KiB chunks against a 64 KiB buffer; FNV checksum verifies byte order end-to-end; `streaming_load_profile_peak_stays_at_capacity_bound`.
- **Slow consumer tests**: `slow_consumer_suspends_producer_and_stays_bounded` (consumer fully stopped → producer stalls at exactly capacity, timing-independent); `poll_write_pends_when_full_and_wakes_after_drain` (CountingWaker: exactly one wake on drain).
- **Memory profile** (`--nocapture`, reproduced this run): `body=8388608B chunk=32768B capacity=65536B peak_buffered=65536B`; `fast-consumer peak_buffered=32768B`; `disconnect cancelled_at=196608B of 8388608B peak_buffered=65536B` — memory scales with buffer capacity, never with body size.

### Source/test map
- `crates/q-capabilities/src/stream_buffer.rs` (BoundedStream + cancellation; 11 unit tests)
- `crates/q-capabilities/src/fetch_policy.rs` (helper caps; 2 new policy tests)
- `crates/q-capabilities/tests/stream_backpressure.rs` (4 integration tests: load, slow consumer, typed errors, disconnect)
- `crates/q-engine-quickjs/src/worker.rs` (`__velquBodyHelperLimit` binding + `body_helper_sizes_fail_closed_above_native_cap`)
- `crates/q-engine-quickjs/src/prelude.rs` (fail-closed helper checks in text/json/arrayBuffer/bytes)

### Command results (this branch)
- `cargo test -p q-capabilities` → 149 unit + 4 backpressure + 8 WPT passed
- `cargo test -p q-engine-quickjs` → 18 unit + 101 engine passed
- `cargo test -p q-http` → 4+6+1 passed; `-p q-bridge` → 11 passed
- `cargo test -p velqu-runtime` → 8+5+31 passed
- `bun test` → 219 pass / 0 fail (via ./scripts/verify); `bun run typecheck` → clean
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**

### Ledger update
- `docs/beta/04_TASK_LEDGER.md`: M28-006 flipped TODO -> PASS.

### Disclosures (standing)
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
