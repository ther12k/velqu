# BETA-004-C — Parameterized Queries and Transactions (runtime:postgres)

Status: **MEASURED** (live-Postgres evidence). Layer over the BETA-004-B
lazy pool; engine-level deadline/cancel wiring is BETA-004-D.

## What was built (`crates/q-capability-postgres`)

- **`query.rs`** — the bounded query contract: closed scalar parameter
  set (`Null | Bool | Int(i64) | Text`; no nested structures — the ORM
  door stays shut), fail-closed statement-text (100 KB) / param-count
  (100) / deadline (1..=30 s) ceilings, a deterministic `$N` placeholder
  scan that rejects unbound placeholders *before anything is sent*, and
  typed row conversion with exact NULL-vs-conversion-failure semantics
  (`Ok(None)` = NULL, `Err` = typed `ColumnConversion`).
- **`executor.rs`** — production `QueryExecutor` over a pooled
  tokio-postgres client: every statement runs the extended protocol
  (Parse/Bind/Execute — parameters bound server-side; there is no
  string-interpolation path in the crate), under the caller's deadline;
  backend errors carry SQLSTATE + message (credentials never appear;
  URL fragments stripped defensively). Integer binding and row reads
  are width-matched per declared column type (INT2/4/8, FLOAT4/8).
- **`transaction.rs`** — transaction flow with a hard safety rule:
  `BEGIN` → work → `COMMIT` on `Outcome::Commit`, `ROLLBACK` on
  `Outcome::Rollback` **or on any error (including early `?` return)**.
  An open transaction is never leaked. 6 deterministic flow tests run
  against a recording executor (statement order asserted: BEGIN/COMMIT,
  BEGIN/ROLLBACK, work-error→ROLLBACK, BEGIN-failure short-circuit,
  COMMIT-failure surfacing, early-return→ROLLBACK).

## Real-world results (live Postgres, operator-run on this host)

`tests/live.rs` extended (benchmark stack, SCRAM, seeded data):

- Parameterized insert bound `$1`(text)/`$2`(int) against an INT4
  column — width-matched binary binding; typed select-back returned
  `id = "item_1"`, `qty = 7` exactly.
- Unbound placeholder (`$2` missing) failed typed
  `ParamCountMismatch { placeholders: 2, bound: 1 }` before any wire
  traffic.
- Transaction COMMIT path persisted (`tx_commit` present);
  ROLLBACK path did not (`tx_rollback` absent) — asserted by ordered
  `SELECT id` after both.
- Stack torn down after the run. Test env-gated
  (`VELQU_PG_LIVE_TEST=1`); CI stays deterministic.

## Tests

- 9 new deterministic unit tests (query validation, placeholder scan,
  deadline bounds, transaction flows) — crate total 21 + 2 live.
- Full gates on this branch: `bun test` 383 pass / 0 fail; fmt/clippy
  clean; `./scripts/verify` ALL PASS.

## Cost

No new runtime cost for apps without the capability (the layer is
inside the same crate behind the same grant). Per-query cost is the
extended-protocol round trip plus one allocation per bound parameter —
no query planners, mappers, or ORMs in between (BETA-004-F).
