/**
 * @velqu/browser-pglite — BWASM-C-003 optional browser-local SQL.
 *
 * PGlite-backed adapter exposing the frozen async Postgres operation
 * subset (C-002) as `runtime:local-sql` v1. Browser-local prototype
 * data only: single connection, one database per origin+namespace,
 * lazy engine load, bounded ops, typed fail-closed errors.
 *
 * This is NOT the native `postgres` capability — that grant stays
 * `deployment-required` (C-005) and is never satisfied here. No
 * multi-user, production-durability, or native-performance claims.
 */
export {
  LOCAL_SQL_CAPABILITY_ID,
  LOCAL_SQL_CAPABILITY_VERSION,
  DEFAULT_OP_DEADLINE_MS,
  MAX_OP_DEADLINE_MS,
  LocalSqlError,
  createLocalSql,
  storageName,
} from "./local-sql";

export type {
  LocalSqlPersistence,
  LocalSqlErrorCode,
  PgliteLoader,
  PgliteModule,
  PgliteInstance,
  PgliteResults,
  PgliteTx,
  CreateLocalSqlOptions,
  QueryOptions,
  LocalSqlRow,
  QueryResult,
  LocalSqlDescribe,
  LocalSql,
} from "./local-sql";

export { classifyStatement, MULTI_SESSION_STATEMENTS } from "./subset";
export type { SubsetVerdict } from "./subset";
