---
task_id: BETA-004-D
parent_task: BETA-004
milestone: BETA
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-004-D — Deadline/cancellation/shutdown

## Atomic goal

Deadline/cancellation/shutdown.

## Parent intent

Provide a real database story without enlarging core.

## Dependencies

- `BETA-004-C` — `tasks/08_public_beta/BETA-004-C-parameterized-queries-transactions.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `docs/beta/`
- `scripts/verify`
- `package.json`
- `.github/workflows/verify.yml`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-pack/src/lib.rs`
- `benchmarks/real-world/postgres/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Deadline/cancellation/shutdown.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- App without Postgres pays zero dependency/init cost.
- Queries are parameterized.
- Timeout cancels/releases connection safely.
- Pool exhaustion is bounded.
- W1/W2/W3 workloads pass.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-capabilities
```
```bash
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- Capability tests.
- Real-world results.
- Cold/RSS cost report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
beta-004-d: deadline cancellation shutdown
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-004-D) — PASS (2026-09-04)

- Branch/PR: beta-004-d (squash-merged; see git log for final hash)
- Closes: #519

### Changed files
- `crates/q-capability-postgres/src/dialer.rs` (new): engine dialer
  contract — `PostgresQueryDialer` (deadline bounds acquire AND
  execution; rows as JSON), JSON param parsing (closed scalars; nested
  shapes rejected typed), **discard-on-error lease semantics** (a lease
  that failed or timed out mid-flight closes — never parked/reused),
  `PostgresQueryHandle` + `pool_from_url`.
- `crates/q-capability-postgres/src/query.rs` / `lib.rs`: `SqlParam::Float`,
  `SqlRow::column_names`.
- `crates/q-engine-quickjs/src/worker.rs`: `__velquPostgresQuery` native
  (installed only when the host wired a pool), phase guards
  (Invocation-only), owner-tagged NativeOp, tokio task + abort_handle,
  `WorkerMsg::PostgresComplete` arm with the full resolution tail
  (finish_resolved + settle_background), `OpCompletion::Postgres`,
  `OpKind::Postgres`, postgres op counters.
- `crates/q-engine-quickjs/src/prelude.rs`: `native.postgres.sql` —
  op-table Promise pattern; typed fail-closed error without the
  binding.
- `crates/q-engine-quickjs/src/lib.rs`: `QuickJsConfig.postgres_handle`.
- `crates/q-runtime/src/lib.rs`: startup linking — pack inventory
  requires `runtime:postgres` + env set -> pool (lazy); required + env
  missing -> typed startup rejection; not required -> never constructed.
- `crates/q-pack/src/lib.rs`: `postgres` route grant accepted.
- `crates/q-engine-quickjs/tests/engine.rs`: 4 deterministic tests.
- `docs/reports/beta-004-d-deadline-cancellation-shutdown.md` (new).

### Required evidence

- **Capability tests**: 4 engine tests (dialer resolution with bound
  param + deadline echo; fail-closed unlinked; deadline-0 pre-I/O
  rejection; phase-guard refusals) + 22 pool unit tests + 6 tx-flow
  tests unchanged/green.
- **Real-world results**: live end-to-end on the benchmark stack —
  `GET /db/users/usr_1` -> `{"row":{"id":"usr_1","qty":42}}` through
  HTTP -> QuickJS -> native binding -> pool -> real Postgres; missing
  row -> `{"row":null}`; startup without `VELQU_DATABASE_URL` ->
  typed `ready ok:false` rejection before serving.
- **Cold/RSS cost report**: pool construction remains lazy (no I/O
  until first query); apps without the grant never construct a pool;
  startup rejection costs nothing (pre-serve).

### Commands

- `cargo test -p q-capability-postgres` -> 22 unit + 2 live pass
- `cargo test -p q-engine-quickjs` -> 137 pass / 0 failed
- `cargo test -p q-capabilities` / `-p q-pack` -> all suites ok
- `bun test` -> 383 pass / 0 fail (62 files)
- fmt / clippy (`-D warnings`) / typecheck -> clean
- `./scripts/verify` -> ALL PASS (M0-M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)
  (isolated netns; standing port-3000 environment note, BETA-002-C record)

### Guardrail mapping

- **Timeout cancels/releases safely**: deadline bounds acquire+query;
  timeout/cancel discards the lease (close) — a mid-flight connection
  is never reused; engine cancel aborts the task via abort_handle with
  owner-checked settlement.
- **App without Postgres pays zero dependency/init cost**: no grant ->
  no pool, no binding (prelude sql fails closed), unchanged pack cost.
- **Queries are parameterized**: unchanged wire path from C.
- **Pool exhaustion is bounded**: unchanged from B.
- **W1/W2/W3 workloads pass**: parent exit; not claimed here.

### Standing CI disclosure

CI `verify` workflows stall/fail with zero executed steps on PR creation
across all branches (infrastructure-side, tracked since ~#714); the local
`./scripts/verify` run above is the real gate evidence for this packet.
