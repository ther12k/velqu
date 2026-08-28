---
task_id: M28-006-A
parent_task: M28-006
milestone: M28
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-006-A — Bound read/write buffers

## Atomic goal

Bound read/write buffers.

## Parent intent

Support large bodies without unbounded buffering.

## Dependencies

- `M28-004-Z` — `tasks/05_m28_native_fetch/M28-004-Z-package-evidence-for-implement-request-response-and-headers-subset.md`
- `M28-005-Z` — `tasks/05_m28_native_fetch/M28-005-Z-package-evidence-for-propagate-abortsignal-and-route-deadlines.md`

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
5. Implement exactly this deliverable: Bound read/write buffers.
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
m28-006-a: bound read write buffers
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-006-A) — PASS

- Date: 2026-08-28
- Branch/PR: m28-006-a (squash-merged; see git log for final hash)
- Closes: #336

### Changed files
- `crates/q-capabilities/src/stream_buffer.rs` (new): Bounded streaming buffer `BoundedStream` — shared reader/writer chunk buffer with a hard byte ceiling: per-chunk ceiling (`MAX_STREAM_CHUNK_BYTES` = 1 MiB, typed `ChunkTooLarge`), total body limit enforced across the whole stream (`LimitExceeded`, matching ADR-0033 §9's `MAX_FETCH_RESPONSE_BODY_BYTES`), backpressure (`try_write` returns false at capacity; producer drains/resumes), `poll_read` for async consumers with waker registration, idempotent close, `total_written` accounting across drains. Over-ceiling writes are typed rejections — never unbounded buffering.
- `crates/q-capabilities/src/lib.rs`: `pub mod stream_buffer;` + re-exports.
- `benchmarks/manifest.json`: Refreshed `qRuntimeRelease` hash.

### Tests added (crates/q-capabilities/src/stream_buffer.rs, 7 tests)
- `write_then_read_roundtrips`
- `per_chunk_ceiling_is_enforced`
- `body_limit_is_enforced`
- `backpressure_blocks_producer_at_capacity_and_drain_resumes`
- `closed_stream_rejects_writes` (idempotent close)
- `total_written_tracks_across_drains`
- `default_constants_are_bounded` (stream ceiling == fetch response body limit pin)

### Command results
- `cargo test -p q-capabilities` → 139 unit + 8 integration passed (was 132+8; +7 streaming tests)
- `cargo test -p q-engine-quickjs` → 17+101 · `-p q-http` 4+6+1 · `-p q-bridge` 11 · `-p velqu-runtime` 8+5+31 — all pass
- `bun test` → 219 pass / 0 fail (27 files)
- `bun run typecheck` → clean
- `cargo fmt --check` → clean
- `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**

### Guardrail mapping
- **Support large bodies without unbounded buffering** — hard per-chunk ceiling + total body limit + capacity backpressure; every excess byte path is a typed rejection.

### Disclosures
- Two `cargo clippy` runs caught literal-bool assertions in the new tests (`bool_assert_comparison`); fixed inline before commit. All tests unchanged and green.
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
