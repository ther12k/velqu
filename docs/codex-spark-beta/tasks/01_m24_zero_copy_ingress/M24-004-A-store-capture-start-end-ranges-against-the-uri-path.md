---
task_id: M24-004-A
parent_task: M24-004
milestone: M24
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M24.md
commit_required: true
---

# M24-004-A — Store capture start/end ranges against the URI path

## Atomic goal

Store capture start/end ranges against the URI path.

## Parent intent

Avoid allocating parameter strings until validation or JavaScript access requires them.

## Dependencies

- `M24-002-Z` — `tasks/01_m24_zero_copy_ingress/M24-002-Z-package-evidence-for-route-before-request-materialization.md`
- `M24-003-Z` — `tasks/01_m24_zero_copy_ingress/M24-003-Z-package-evidence-for-implement-worker-local-generation-checked-request-slab.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M24.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/schema-codecs.md`

### Source files

- `AGENTS.md`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `crates/q-engine-quickjs/src/convert.rs`
- `packages/schema/src/index.ts`
- `conformance/schema/schema.conformance.test.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Store capture start/end ranges against the URI path.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Parameterized routes preserve exact names and values.
- No owned parameter string on an unread path.
- Percent-decoding policy is explicit and tested.
- Invalid encodings fail consistently.

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
cargo test -p q-schema-runtime
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

## Required evidence for this microtask

- Allocation test.
- Reference router parity.
- Encoding edge-case corpus.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m24-004-a: store capture start end ranges against the uri path
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record

- Status: **PASS**
- Deliverable: capture start/end byte ranges against the URI path. `q_router::MatchResult::Found` now carries `param_ranges: Vec<(u32, u32)>` — Copy byte ranges into the resolved path — instead of `Vec<(String, String)>` owned pairs; `search_route` records `seg_ranges[seg_idx]` instead of allocating `curr_seg.to_string()` per capture, and `resolve` computes segment offsets while splitting (no join, no per-segment allocation). `Router::materialize_params(route_index, path, ranges)` is the single allocation path, zipping the route's declared names with the path bytes between the recorded boundaries. `q-runtime/serve.rs` materializes parameter strings ONLY when the route declares param needs, a params schema, or a policy — an unread path allocates zero parameter strings. Percent-decoding policy is explicit and tested: captured values are the raw path bytes; percent sequences are not decoded at capture or materialization (decoding/byte-level validation is M24-004-C/D scope), so behavior is identical for any input. `match_path` (pre-split segments) is retained as a test-convenience wrapper that rejoins and delegates to `resolve`.
- Changed files:
  - `crates/q-router/src/lib.rs` (range-based MatchResult, offset-tracking resolve, range-collecting search_route, materialize_params, laziness/parity/encoding tests, migrated unit + property parity assertions)
  - `crates/q-runtime/src/serve.rs` (lazy materialization gated on params schema / FieldNeeds.params / policy presence)
  - `docs/codex-spark-beta/tasks/01_m24_zero_copy_ingress/M24-004-A-store-capture-start-end-ranges-against-the-uri-path.md`, `docs/codex-spark-beta/STATUS.md`, `docs/codex-spark-beta/indexes/TASK_INDEX.md`
- Tests: new `capture_ranges_defer_string_allocation_and_match_reference_values` (ranges point into the original path; materialized values equal the previous allocate-at-match behavior; offsets are path-relative facts) and `capture_ranges_encoding_corpus_is_raw_and_panic_free` (percent sequences `%20`/`%2F`/`%ZZ`/truncated `%2`, multibyte UTF-8, emoji, trailing/leading slashes — raw-bytes policy, zero panics). Reference parity preserved: the generated property suite now compares reference-vs-serialized routers through materialized values over every probe path; all prior param assertions migrated with identical expected values. End-to-end: runtime conformance 13/13 and Bun HTTP conformance 35/35 pass against the range-based router.
- Verification: `cargo test -p q-router` PASS (14, incl. property parity); `cargo test -p q-engine-quickjs` PASS (1 + 90); `cargo test -p q-http` PASS (2 + 3); `cargo test -p q-bridge` PASS (9); `cargo test -p q-schema-runtime` PASS (2 + fuzz); `cargo test -p velqu-runtime` PASS (13); `bun run typecheck` PASS; `bun test` PASS (35/0 after proof-pack build — the only prior failures were missing-artifact timeouts in this fresh worktree); `cargo fmt --check` PASS; `cargo clippy --workspace --all-targets -- -D warnings` PASS. Raw logs: `/tmp/m24-004-a-type.log`, `/tmp/m24-004-a-bun.log`, `/tmp/m24-004-a-proof.log`.
- Acceptance criteria proven: parameterized routes preserve exact names and values (parity + materialize tests); no owned parameter string on an unread path (MatchResult carries Copy ranges; serve materializes only under declared needs); percent-decoding policy explicit and tested (raw bytes, corpus); invalid encodings fail consistently (no decode step exists to fail — corpus documents consistent raw behavior; schema-layer handling is M24-004-C/D).
- Remaining risk / deferred by design: numeric/UUID byte-level validation (M24-004-C), lazy JS string materialization through the bridge (M24-004-D), route-specific name binding after RouteId selection (M24-004-B — names currently bind at materialize time from the matched route's declaration).
- Next dependency-ready task: M24-004-B (bind route-specific parameter names after RouteId selection).

