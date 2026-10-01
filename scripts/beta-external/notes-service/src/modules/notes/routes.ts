import { route, status } from "@velqu/core";
import { s } from "@velqu/schema";
import { notesWritePolicy } from "../../policy/notes";
import { resolvePage, SCHEMA_DDL, INSERT_NOTE, COUNT_NOTES, SELECT_PAGE, SELECT_ONE, DELETE_ONE } from "./service";

// NOTE: each handler references `ctx.native.postgres` directly — that
// reference IS the capability grant (compiler-detected, puts an exact
// runtime:postgres v1 requirement in the pack). Hoisting it into a
// shared helper would hide it from static extraction.
//
// The native handler surface returns the ROWS ARRAY directly
// (`await ctx.native.postgres.sql(...)` -> SqlRow[]; the {rows,
// affectedRows} wrapper belongs to the SDK package, which is not part
// of the published set). DML detection therefore keys on RETURNING.
//
// All DB failures map to the declared 503 — typed, no error-text leaks
// (unexpected errors are redacted by the host anyway; we keep the typed
// surface on purpose).

interface NoteRow {
  id: string;
  title: string;
  body?: string | null;
  created_at: string;
}

export const create = route({
  id: "notes.create",
  method: "POST",
  path: "/notes",
  policy: notesWritePolicy,
  body: s.object({
    title: s.string({ minLength: 1, maxLength: 120 }),
    body: s.optional(s.string({ maxLength: 8192 })),
  }),
  response: {
    201: s.object({ id: s.string(), title: s.string(), createdAt: s.string() }),
    503: s.object({ error: s.string() }),
  },
  handle: async (ctx) => {
    const db = ctx.native.postgres;
    try {
      await db.sql(SCHEMA_DDL, [], 5000);
      const rows = (await db.sql(INSERT_NOTE, [ctx.body.title, ctx.body.body ?? null], 5000)) as NoteRow[];
      const row = rows[0];
      if (!row) return status(503).value({ error: "database unavailable" });
      return status(201).value({ id: row.id, title: row.title, createdAt: row.created_at });
    } catch {
      return status(503).value({ error: "database unavailable" });
    }
  },
});

export const list = route({
  id: "notes.list",
  method: "GET",
  path: "/notes",
  query: s.object({
    page: s.optional(s.integer({ minimum: 1, maximum: 100000 }), { default: 1 }),
    pageSize: s.optional(s.integer({ minimum: 1, maximum: 50 }), { default: 20 }),
  }),
  response: {
    200: s.object({
      rows: s.array(s.object({ id: s.string(), title: s.string(), createdAt: s.string() })),
      total: s.integer(),
      page: s.integer(),
      pageSize: s.integer(),
    }),
    503: s.object({ error: s.string() }),
  },
  handle: async (ctx) => {
    const db = ctx.native.postgres;
    const { page, pageSize, offset } = resolvePage(ctx.query.page, ctx.query.pageSize);
    try {
      await db.sql(SCHEMA_DDL, [], 5000);
      const counted = (await db.sql(COUNT_NOTES, [], 5000)) as { total: number }[];
      const paged = (await db.sql(SELECT_PAGE, [pageSize, offset], 5000)) as NoteRow[];
      const total = Number(counted[0]?.total ?? 0);
      const rows = paged.map((r) => ({ id: r.id, title: r.title, createdAt: r.created_at }));
      return { rows, total, page, pageSize };
    } catch {
      return status(503).value({ error: "database unavailable" });
    }
  },
});

export const get = route({
  id: "notes.get",
  method: "GET",
  path: "/notes/:id",
  params: s.object({ id: s.string({ pattern: "^nts_[0-9a-f]{12}$" }) }),
  response: {
    200: s.object({ id: s.string(), title: s.string(), body: s.string(), createdAt: s.string() }),
    503: s.object({ error: s.string() }),
  },
  handle: async (ctx) => {
    const db = ctx.native.postgres;
    try {
      await db.sql(SCHEMA_DDL, [], 5000);
      const rows = (await db.sql(SELECT_ONE, [ctx.params.id], 5000)) as (NoteRow & { body: string | null })[];
      const row = rows[0];
      if (!row) return status(404).problem("not-found", { detail: "note not found" });
      return { id: row.id, title: row.title, body: row.body ?? "", createdAt: row.created_at };
    } catch {
      return status(503).value({ error: "database unavailable" });
    }
  },
});

export const remove = route({
  id: "notes.delete",
  method: "DELETE",
  path: "/notes/:id",
  policy: notesWritePolicy,
  params: s.object({ id: s.string({ pattern: "^nts_[0-9a-f]{12}$" }) }),
  response: {
    200: s.object({ deleted: s.boolean() }),
    503: s.object({ error: s.string() }),
  },
  handle: async (ctx) => {
    const db = ctx.native.postgres;
    try {
      await db.sql(SCHEMA_DDL, [], 5000);
      const rows = (await db.sql(DELETE_ONE, [ctx.params.id], 5000)) as { id: string }[];
      if (rows.length === 0) return status(404).problem("not-found", { detail: "note not found" });
      return { deleted: true };
    } catch {
      return status(503).value({ error: "database unavailable" });
    }
  },
});

export const notesRoutes = [create, list, get, remove];
