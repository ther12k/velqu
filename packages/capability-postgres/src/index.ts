/**
 * First-party Postgres capability SDK (BETA-004-A).
 *
 * The application-facing surface of the `runtime:postgres` capability.
 * Identity and versioning mirror the ABI model in
 * `crates/q-capabilities/src/postgres.rs` — a pack that grants
 * `postgres` carries an exact `runtime:postgres` v1 requirement, and a
 * runtime that does not provide it fails closed before serving.
 *
 * Cost posture (parent guardrail): importing this module constructs
 * nothing — no pool, no sockets, no timers, no background work. Every
 * native operation routes through `globalThis.__velquPostgres*`
 * bindings, which exist only when the host links the capability. When
 * the binding is absent every method throws the same typed
 * `PostgresCapabilityUnavailable` — fail closed, never a silent
 * fallback, never a JS-side reimplementation.
 *
 * Queries are parameterized by construction: `sql()` takes the
 * statement text and a positional parameter array; there is no
 * string-concatenation API to reach for (BETA-004-C pins the wire
 * behavior; BETA-004-F keeps this an ORM-free surface).
 */

/** Capability identity — must equal `POSTGRES_CAPABILITY_ID` in q-capabilities. */
export const POSTGRES_CAPABILITY_ID = "runtime:postgres";

/** Exact required version — must equal `POSTGRES_CAPABILITY_VERSION`. */
export const POSTGRES_CAPABILITY_VERSION = 1;

/** Grant name handlers use via `ctx.native.postgres`. */
export const POSTGRES_GRANT = "postgres";

/** Deadline ceiling (ms) — must equal `MAX_POSTGRES_OP_DEADLINE_MS`. */
export const MAX_POSTGRES_DEADLINE_MS = 120_000;

/**
 * Thrown when the host has not linked `runtime:postgres`. Typed so the
 * compiler's declared error status can map it; never a silent `null`.
 */
export class PostgresCapabilityUnavailable extends Error {
  constructor(operation: string) {
    super(
      `postgres capability unavailable: native binding for '${operation}' is not linked ` +
        "(pack must grant postgres and the runtime must provide runtime:postgres)",
    );
    this.name = "PostgresCapabilityUnavailable";
  }
}

/** A single positional parameter value (BETA-004-C: no nested arrays/objects). */
export type SqlParam = string | number | boolean | null;

/** One validated query result row (plain JSON-compatible object). */
export type SqlRow = Record<string, string | number | boolean | null>;

export interface SqlResult {
  rows: SqlRow[];
  /** Number of rows affected for DML; 0 for SELECT-shaped results. */
  affectedRows: number;
}

/**
 * BWASM-C-002: the statement did not settle within its deadline. The
 * native side cancelled the round trip and released the connection back
 * to the pool before this rejection reaches the handler.
 */
export class PostgresDeadlineExceeded extends Error {
  readonly deadlineMs: number | null;
  constructor(nativeReason: string) {
    super(`postgres.sql deadline exceeded: ${nativeReason}`);
    this.name = "PostgresDeadlineExceeded";
    const m = /within (\d+)ms/.exec(nativeReason);
    this.deadlineMs = m ? Number(m[1]) : null;
  }
}

/**
 * BWASM-C-002: the statement failed for a non-deadline reason (backend
 * error, bound violation, param mismatch, value-conversion limit). The
 * native reason text is preserved verbatim — it is the stable diagnostic
 * surface; callers must not parse it beyond the deadline classifier.
 */
export class PostgresQueryError extends Error {
  readonly nativeReason: string;
  constructor(nativeReason: string) {
    super(`postgres.sql failed: ${nativeReason}`);
    this.name = "PostgresQueryError";
    this.nativeReason = nativeReason;
  }
}

function requireNativeBinding(operation: string): (...args: unknown[]) => unknown {
  const binding = (globalThis as Record<string, unknown>)[`__velquPostgres${operation}`];
  if (typeof binding !== "function") {
    throw new PostgresCapabilityUnavailable(operation);
  }
  return binding as (...args: unknown[]) => unknown;
}

/**
 * The `runtime:postgres` capability object. Constructing this object
 * allocates a plain record only; the pool and every connection belong
 * to the host and are created lazily on first demand after the
 * capability reaches `Ready` (ABI G-004). Apps that never import or
 * grant it pay nothing at build, pack-load, or run time.
 */
export const postgres = {
  capabilityId: POSTGRES_CAPABILITY_ID,
  capabilityVersion: POSTGRES_CAPABILITY_VERSION,

  /**
   * Run one parameterized statement. `text` must use positional
   * parameters (`$1, $2, ...`); `params` values bind in order.
   * `deadlineMs` is bounded by MAX_POSTGRES_DEADLINE_MS and cancels
   * the round trip, releasing the connection back to the pool.
   *
   * BWASM-C-002 (async contract, locked before browser adapters depend
   * on it): ALWAYS returns a Promise. Argument validation stays
   * synchronous (TypeError/RangeError thrown from the call). Rejections
   * are typed: {@link PostgresDeadlineExceeded} when the statement did
   * not settle within its deadline (the native side cancelled the round
   * trip and released the connection), {@link PostgresQueryError} for
   * every other failure (native reason preserved verbatim).
   *
   * v1 semantics (frozen — see docs/beta/POSTGRES_ASYNC_MIGRATION.md):
   * single-statement autocommit per call; no transaction pinning; rows
   * carry only the bounded JSON-compatible value set; `affectedRows`
   * counts DML changes and is 0 for SELECT-shaped results.
   */
  sql(text: string, params: readonly SqlParam[] = [], deadlineMs = 5_000): Promise<SqlResult> {
    if (typeof text !== "string" || text.length === 0) {
      throw new TypeError("postgres.sql: statement text is required");
    }
    if (typeof deadlineMs !== "number" || deadlineMs <= 0 || deadlineMs > MAX_POSTGRES_DEADLINE_MS) {
      throw new RangeError(`postgres.sql: deadline must be 1..${MAX_POSTGRES_DEADLINE_MS}ms`);
    }
    // Fail-closed binding check stays SYNCHRONOUS: an unlinked capability
    // throws PostgresCapabilityUnavailable from the call itself (never a
    // rejected promise a caller might swallow in a non-awaited chain).
    const query = requireNativeBinding("Query");
    return Promise.resolve()
      .then(() => query(text, [...params], deadlineMs))
      .then((result) => result as SqlResult)
      .catch((cause: unknown) => {
        const reason = cause instanceof Error ? cause.message : String(cause);
        if (/did not settle within \d+ms/.test(reason)) {
          throw new PostgresDeadlineExceeded(reason);
        }
        throw new PostgresQueryError(reason);
      });
  },
};
