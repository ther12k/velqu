/**
 * BWASM-C-003 — the documented local-SQL subset.
 *
 * PGlite is a real PostgreSQL build compiled to WASM, so the engine can
 * execute far more than this adapter promises. The SUBSET here is the
 * adapter's CONTRACT: what an application may rely on, what fails with
 * a stable actionable code, and what is deliberately out of scope for a
 * browser-local prototype database.
 *
 * Supported surface (single connection, single database):
 *   DDL:   CREATE / ALTER / DROP TABLE, INDEX, VIEW, SEQUENCE, SCHEMA
 *   DML:   INSERT / UPDATE / DELETE / SELECT ... (FOR UPDATE, RETURNING)
 *   joins, aggregates, CTEs (WITH), window functions, subqueries,
 *   PREPARE/EXECUTE via bound parameters, EXPLAIN
 *   transactions: BEGIN / COMMIT / ROLLBACK (serialized on the single
 *   connection; use LocalSql.transaction for scoped units)
 *
 * Unsupported (stable `UnsupportedStatement` with a remediation hint):
 *   LISTEN/NOTIFY, COPY ... FROM STDIN, CREATE EXTENSION / ALTER SYSTEM /
 *   superuser-only operations, pg_dump/pg_restore server programs,
 *   advisory locks, cursor operations across calls (DECLARE/FETCH/CLOSE),
 *   CREATE DATABASE / DROP DATABASE / template manipulation — the
 *   database IS the namespace; reset() replaces those.
 *
 * Explicitly not promised even where the engine tolerates them:
 *   concurrent sessions (there is exactly one connection), multi-user
 *   durability, production availability, or native Postgres performance.
 */

/** First keyword(s) that classify a statement out of the subset. */
const UNSUPPORTED_PREFIXES: ReadonlyArray<{ match: RegExp; reason: string; remediation: string }> = [
  { match: /^listen\b/i, reason: "LISTEN/NOTIFY needs a server connection with multiple sessions", remediation: "poll a table or use a deployed native service" },
  { match: /^notify\b/i, reason: "LISTEN/NOTIFY needs a server connection with multiple sessions", remediation: "poll a table or use a deployed native service" },
  { match: /^copy\s+[^]*\bfrom\s+stdin\b/i, reason: "COPY FROM STDIN is a server streaming protocol operation", remediation: "batch INSERT ... VALUES from your application instead" },
  { match: /^create\s+extension\b/i, reason: "extensions are not installed in the browser build", remediation: "the core SQL surface is available; extension-backed features are not" },
  { match: /^alter\s+system\b/i, reason: "ALTER SYSTEM mutates server configuration", remediation: "configuration is fixed for the browser-local engine" },
  { match: /^drop\s+database\b/i, reason: "the database IS the namespace", remediation: "use LocalSql.reset() to drop and recreate it" },
  { match: /^create\s+database\b/i, reason: "one database per namespace", remediation: "create another LocalSql instance with a distinct namespace" },
  { match: /^declare\s/i, reason: "server-side cursors cannot span calls on this contract", remediation: "fetch bounded pages with LIMIT/OFFSET or keyset pagination" },
  { match: /^fetch\s/i, reason: "server-side cursors cannot span calls on this contract", remediation: "fetch bounded pages with LIMIT/OFFSET or keyset pagination" },
  { match: /^close\s/i, reason: "server-side cursors cannot span calls on this contract", remediation: "fetch bounded pages with LIMIT/OFFSET or keyset pagination" },
  { match: /^pg_advisory_lock|^\s*select\s+pg_advisory/i, reason: "advisory locks coordinate multiple sessions", remediation: "there is exactly one connection; no cross-session coordination exists" },
  { match: /^vacuum\b/i, reason: "VACUUM/ANALYZE housekeeping is owned by the engine", remediation: "not part of the adapter contract; storage grows within browser quotas" },
  { match: /^reindex\b/i, reason: "REINDEX is superuser housekeeping", remediation: "not part of the adapter contract" },
];

/** Statements that only make sense with another live session. */
const MULTI_SESSION_HINTS = ["listen", "notify", "pg_advisory"];

export interface SubsetVerdict {
  readonly supported: boolean;
  readonly reason?: string;
  readonly remediation?: string;
}

/** Classify one SQL statement against the documented subset. */
export function classifyStatement(sql: string): SubsetVerdict {
  const head = sql.trimStart();
  for (const rule of UNSUPPORTED_PREFIXES) {
    if (rule.match.test(head)) {
      return { supported: false, reason: rule.reason, remediation: rule.remediation };
    }
  }
  return { supported: true };
}

/** Multi-session statement families, exported for evidence/matrix docs. */
export const MULTI_SESSION_STATEMENTS: readonly string[] = MULTI_SESSION_HINTS;
