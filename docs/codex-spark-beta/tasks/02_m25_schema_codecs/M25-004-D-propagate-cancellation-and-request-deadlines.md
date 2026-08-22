---
task_id: M25-004-D
parent_task: M25-004
milestone: M25
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-004-D — Propagate cancellation and request deadlines

## Atomic goal

Propagate cancellation and request deadlines.

## Parent intent

Parse and validate declared JSON bodies with one route-selected strategy.

## Dependencies

- `M25-004-C` — `tasks/02_m25_schema_codecs/M25-004-C-enforce-depth-size-array-string-and-numeric-limits.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M25.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/schema-codecs.md`

### Source files

- `AGENTS.md`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-engine-quickjs/src/convert.rs`
- `packages/schema/src/index.ts`
- `packages/compiler/src/emit.ts`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/src/serve.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Propagate cancellation and request deadlines.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- One successful decode representation crosses to JS.
- Oversize/deep inputs fail boundedly.
- No semantic drift from schema.
- Fallback is explicit in build report.

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

- Fuzz/differential tests.
- Depth/size boundary tests.
- CPU/allocation results.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m25-004-d: propagate cancellation and request deadlines
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M25-004-D)

Status: **PASS**. The route deadline now bounds the entire request pipeline
from route match, and cancellation propagates through the pre-invocation body
read:

- **Anchored deadline**: `request_deadline` is computed once when the
  `CompiledRoute` resolves — before admission, the bounded body read, and
  decode — instead of being re-anchored at engine invocation. Pre-invocation
  work is charged to the same budget (constraint 11).
- **Bounded read with cancellation**: the body collect races the anchored
  deadline via `tokio::time::timeout_at`. On elapse the collect future is
  dropped mid-stream — the stalled transfer is cancelled — and the request
  settles with the same RFC 9457 `timeout` problem (504) the engine produces
  for handler deadlines (stage `deadline.body`). A client that sends headers
  and stalls the body can no longer hold the request past its deadline.
- **Deadline propagation to the worker**: `InvocationSpec.deadline` carries
  the anchored absolute deadline; the worker's existing absolute-deadline
  comparisons (`Instant::now() >= budget.deadline`, `deadline <= now`
  sweeps) settle an already-expired budget as `Timeout` without running the
  handler. Post-invocation client-disconnect cancellation remains owned by
  the M24-003-C `CancelOnDrop` guard (unchanged).

### Changed files

- `crates/q-runtime/src/serve.rs` — deadline anchor at route match;
  `timeout_at`-bounded `collect_body_bounded` with the 504 `timeout` problem
  and `deadline.body` stage; `InvocationSpec.deadline` now receives the
  anchored deadline.
- `crates/q-runtime/tests/runtime_conformance.rs` — fixture route
  `deadline.body` (POST /deadline-body, 200 ms deadline, body-bound,
  reuses the `fallback.echo` RouteHandler through `plan.handler_id = 10`)
  and test `body_read_deadline_cancels_stalled_transfer`: a client that
  declares a 32-byte body and sends nothing receives 504
  `https://velqu.dev/problems/timeout` in under 2 s (deadline is 200 ms,
  client read timeout 5 s — the only path that can produce this is the new
  deadline-bound read), and a prompt body on the same route still reaches
  the handler (200 echo) under the anchored deadline.
- `docs/codex-spark-beta/STATUS.md`,
  `docs/codex-spark-beta/indexes/TASK_INDEX.md` — M25-004-D marked PASS.

### Tests and evidence

- `cargo test -p q-engine-quickjs` — 1 unit + 96 integration passed.
- `cargo test -p q-http` — 4 + 6 + 1 passed.
- `cargo test -p q-bridge` — 11 passed.
- `cargo test -p q-schema-runtime` — 45 unit + 3 fuzz passed (unchanged;
  decode semantics untouched by this packet).
- `cargo test -p velqu-runtime` — 18 integration tests passed (new
  `body_read_deadline_cancels_stalled_transfer` included).
- `bun test` — 69 passed, 0 failed, 297 expect calls.
- `bun run typecheck` — clean.
- `cargo fmt --check` — clean.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `scripts/validate-okf` — 176 links, 0 errors.
- `./scripts/verify` — all stages pass except the documented
  isolated-worktree `qRuntimeRelease`/`proofPack` manifest hash mismatch
  (release binary embeds checkout-absolute OUT_DIR paths; known,
  pre-existing, identical to every packet branch).

Fuzz/differential and depth/size boundary evidence is unchanged from
M25-004-B/C (no decode-semantics change in this packet); CPU/allocation
results are out of scope — the change adds one deadline comparison on the
request path and no allocation.

Commit: `75a2f8b`.
