---
task_id: M28-006-V
parent_task: M28-006
milestone: M28
priority: P0
mode: VERIFY
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-006-V — Verify Implement streaming and strict backpressure

## Atomic goal

Prove every acceptance criterion for parent task M28-006 without broadening scope.

## Parent intent

Support large bodies without unbounded buffering.

## Dependencies

- `M28-006-A` — `tasks/05_m28_native_fetch/M28-006-A-bound-read-write-buffers.md`
- `M28-006-B` — `tasks/05_m28_native_fetch/M28-006-B-propagate-downstream-backpressure.md`
- `M28-006-C` — `tasks/05_m28_native_fetch/M28-006-C-cancel-on-consumer-stop-disconnect.md`
- `M28-006-D` — `tasks/05_m28_native_fetch/M28-006-D-define-maximum-body-helper-sizes.md`

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
- `scripts/package`
- `scripts/release-packet`
- `packages/cli/package.json`
- `package.json`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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

- Streaming load tests.
- Slow consumer tests.
- Memory profile.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m28-006-v: verify implement streaming and strict backpressure
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-006-V) — PASS

- Date: 2026-08-29
- Branch/PR: m28-006-v (squash-merged; see git log for final hash)
- Closes: #340

### Acceptance-criterion mapping (parent M28-006 guardrails)

1. **Large response does not allocate full body unless requested** — verified: `BoundedStream` buffers at a hard capacity (64 KiB default; peak tracked by `max_buffered()`), and the materializing helpers are capped at `MAX_BODY_HELPER_BYTES` (16 MiB, fail closed before copying). Tests: `streaming_load_profile_peak_stays_at_capacity_bound` (8 MiB stream, peak 32768 B ≤ capacity), `default_constants_are_bounded`, `body_helper_cap_composes_with_network_and_text_limits`, worker `body_helper_sizes_fail_closed_above_native_cap`.
2. **Slow upstream/downstream remains bounded** — verified: with the consumer fully stopped, the producer stalls at exactly the buffer capacity and cannot finish (structural, timing-independent). Test: `slow_consumer_suspends_producer_and_stays_bounded` (stall ≤ 65536 B; slow drain resumes the pump with checksum-verified byte order; peak buffered 65536 B == capacity); `poll_write_pends_when_full_and_wakes_after_drain` (waker semantics).
3. **Cancellation releases buffers/connections** — verified: consumer-side `cancel()` zeroes the buffer at cancel time, wakes the parked producer into a typed error, and terminates a mid-stream pump. Tests: `consumer_disconnect_cancels_midstream_and_releases_buffers` (producer cancelled at 196608 B of 8388608 B, `buffered()==0`, peak == capacity), `cancel_releases_buffers_and_fails_writes_typed`, `cancel_after_close_still_releases_residual_buffer`.
4. **Streaming errors are typed** — verified: closed error sets on both layers — `StreamError::{ChunkTooLarge, LimitExceeded, StreamClosed, Cancelled}` and `FetchPolicyError::BodyTooLarge`; JS surface gets named `TypeError`s. Tests: `poll_write_typed_errors_fail_closed`, `typed_errors_propagate_through_async_write_path`, `cancel_wakes_parked_producer_into_typed_error`, `body_helper_sizes_are_defined_and_fail_closed`.

### Required evidence (streaming load / slow consumer / memory profile)
- Streaming load + slow consumer: `crates/q-capabilities/tests/stream_backpressure.rs` (4 tests, deterministic chunk checksums, tokio multi-thread).
- Memory profile lines (`--nocapture`, this run):
  - `m28-006-b profile: body=8388608B chunk=32768B capacity=65536B peak_buffered=65536B`
  - `m28-006-b fast-consumer profile: body=8388608B peak_buffered=32768B capacity=65536B`
  - `m28-006-c disconnect profile: cancelled_at=196608B of 8388608B, peak_buffered=65536B, capacity=65536B`

### Verification runs (this branch, worktree-fresh)
- `cargo test -p q-capabilities` → 149 unit + 4 backpressure + 8 WPT passed
- `cargo test -p q-engine-quickjs` → 18 unit + 101 engine passed
- `cargo test -p q-http` → 4+6+1 passed; `-p q-bridge` → 11 passed
- `cargo test -p velqu-runtime` → 8+5+31 passed
- `bun test` → 219 pass / 0 fail (via ./scripts/verify); `bun run typecheck` → clean
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**; release binary reproduced deterministically (`ef142331…` matches the manifest refreshed in M28-006-D)

### Disclosures (standing)
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
- No production code changed in this packet: verification-only closure of M28-006-A/B/C/D.
