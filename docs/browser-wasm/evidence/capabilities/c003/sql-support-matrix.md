# BWASM-C-003 — SQL support matrix

Source commit at implementation: `2e4ffc0` (base) + the C-003 packet
commit (see git log). Engine: `@electric-sql/pglite` **0.5.8** (exact
pin; reports `PostgreSQL 18.3 (PGlite 0.5.8) on wasm32`).

The adapter's contract is a documented SUBSET of what the engine can
physically execute: an application may rely only on the Supported
surface; everything else rejects with `UnsupportedStatement` BEFORE
execution (stable code + actionable remediation).

## Supported (contract surface, single connection)

| Family | Verified by | Notes |
| --- | --- | --- |
| DDL: CREATE/ALTER/DROP TABLE, INDEX | `local-sql.test.ts` "DDL" | information_schema round-trip |
| INSERT with RETURNING | "DML" | `command` + returned rows |
| UPDATE / DELETE with bound params | "DML" | rowCount assertions |
| Joins (INNER tested) | "joins/aggregates" | |
| Aggregates (sum, count) | "joins/aggregates" | |
| CTE (WITH) | "joins/aggregates" | |
| Window functions (rank) | "joins/aggregates" | |
| Bound parameters ($1..$n) | throughout | |
| Transactions BEGIN/COMMIT/ROLLBACK | "transactions" | engine-native unit; rollback verified |
| EXPLAIN, subqueries, FOR UPDATE, views/sequences/schemas | engine capability | within single-connection semantics |

## Unsupported (reject with `UnsupportedStatement` + remediation)

| Statement | Why | Remediation recorded |
| --- | --- | --- |
| LISTEN / NOTIFY | needs multiple server sessions | poll a table / native service |
| COPY ... FROM STDIN | server streaming protocol | batch INSERT VALUES |
| CREATE EXTENSION | no extensions in browser build | core SQL surface only |
| ALTER SYSTEM | mutates server config | config fixed for local engine |
| CREATE / DROP DATABASE | the database IS the namespace | another namespace / reset() |
| DECLARE / FETCH / CLOSE (cursors) | cannot span calls | LIMIT/OFFSET or keyset pages |
| advisory locks | coordinate multiple sessions | exactly one connection exists |
| VACUUM / REINDEX | superuser housekeeping | not in the adapter contract |

## Explicitly not promised (even where the engine tolerates them)

- Concurrent sessions — there is exactly one connection per handle.
- Multi-user durability, production availability, native Postgres
  performance — never claimed; the native `postgres` grant remains
  `deployment-required` (C-005) and this adapter never satisfies it.
- Deadline preemption — deadlines reject the caller fail-closed; the
  WASM engine shares the caller's thread and is never preempted
  (documented; `lazy.test.ts` proves the caller-side rejection on a
  yielding wait).
