/**
 * Pagination math + SQL text for the notes CRUD — pure functions here so
 * the unit battery can pin them; handlers keep only wiring + typed
 * failure mapping. All statements are single-statement autocommit with
 * positional parameters only (the capability's whole API surface).
 */

export const DEFAULT_PAGE = 1;
export const DEFAULT_PAGE_SIZE = 20;
export const MAX_PAGE_SIZE = 50;

export interface ResolvedPage {
  page: number;
  pageSize: number;
  offset: number;
}

/** Defaults + clamping contract lives in the schema (min/max), not here:
 * this only computes OFFSET from already-validated values. */
export function resolvePage(page: number | undefined, pageSize: number | undefined): ResolvedPage {
  const p = page ?? DEFAULT_PAGE;
  const size = pageSize ?? DEFAULT_PAGE_SIZE;
  return { page: p, pageSize: size, offset: (p - 1) * size };
}

export const SCHEMA_DDL =
  "CREATE TABLE IF NOT EXISTS notes (" +
  "id text PRIMARY KEY DEFAULT ('nts_' || substr(replace(gen_random_uuid()::text, '-', ''), 1, 12)), " +
  "title text NOT NULL, " +
  "body text, " +
  "created_at timestamptz NOT NULL DEFAULT now())";

export const INSERT_NOTE =
  "INSERT INTO notes (title, body) VALUES ($1, $2) RETURNING id, title, created_at::text AS created_at";

export const COUNT_NOTES = "SELECT count(*)::int AS total FROM notes";

export const SELECT_PAGE =
  "SELECT id, title, created_at::text AS created_at FROM notes ORDER BY created_at DESC, id LIMIT $1 OFFSET $2";

export const SELECT_ONE =
  "SELECT id, title, body, created_at::text AS created_at FROM notes WHERE id = $1";

// DELETE returns the deleted id via RETURNING — the native sql() surface
// returns rows only (no affectedRows), so existence = rows.length > 0.
export const DELETE_ONE = "DELETE FROM notes WHERE id = $1 RETURNING id";
