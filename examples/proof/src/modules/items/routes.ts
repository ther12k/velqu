import { route, status } from "@velqu/core";
import { s } from "@velqu/schema";
import { resolve } from "./service";

/**
 * Paginated item listing (M4A-009-A): cursor-based pagination over the lazy
 * in-memory store, with every query bound validated before the handler runs.
 */
export const list = route({
  id: "items.list",
  method: "GET",
  path: "/items",
  query: s.object({
    limit: s.optional(s.integer({ minimum: 1, maximum: 50 }), { default: 5 }),
    cursor: s.optional(s.string({ pattern: "^[0-9]+$" }), { default: "0" }),
  }),
  response: {
    200: s.object({
      items: s.array(s.object({ id: s.string({ pattern: "^itm_[0-9]+$" }), name: s.string({ minLength: 1, maxLength: 60 }), tags: s.array(s.string({ minLength: 1, maxLength: 24 }), { maxItems: 8 }) }), { maxItems: 50 }),
      nextCursor: s.nullable(s.string({ pattern: "^[0-9]+$" })),
    }),
  },
  handle: async ({ query }) => resolve().list(query.limit, Number(query.cursor)),
});

export const create = route({
  id: "items.create",
  method: "POST",
  path: "/items",
  body: s.object({
    name: s.string({ minLength: 1, maxLength: 60 }),
    tags: s.array(s.string({ minLength: 1, maxLength: 24 }), { minItems: 0, maxItems: 8 }),
  }),
  response: {
    201: s.object({ id: s.string({ pattern: "^itm_[0-9]+$" }), name: s.string({ minLength: 1, maxLength: 60 }), tags: s.array(s.string({ minLength: 1, maxLength: 24 }), { maxItems: 8 }) }),
  },
  handle: async ({ body }) => status(201).value(resolve().create(body.name, body.tags)),
});

export const get = route({
  id: "items.get",
  method: "GET",
  path: "/items/:id",
  params: s.object({ id: s.string({ pattern: "^itm_[0-9]+$" }) }),
  response: {
    200: s.object({ id: s.string({ pattern: "^itm_[0-9]+$" }), name: s.string({ minLength: 1, maxLength: 60 }), tags: s.array(s.string({ minLength: 1, maxLength: 24 }), { maxItems: 8 }) }),
    404: s.object({ missing: s.boolean() }),
  },
  handle: async ({ params }) => {
    const item = resolve().get(params.id);
    if (!item) return status(404).problem("not-found", { detail: "item not found" });
    return item;
  },
});

export const update = route({
  id: "items.update",
  method: "PATCH",
  path: "/items/:id",
  params: s.object({ id: s.string({ pattern: "^itm_[0-9]+$" }) }),
  body: s.object({
    name: s.optional(s.string({ minLength: 1, maxLength: 60 })),
    tags: s.optional(s.array(s.string({ minLength: 1, maxLength: 24 }), { maxItems: 8 })),
  }),
  response: {
    200: s.object({ id: s.string({ pattern: "^itm_[0-9]+$" }), name: s.string({ minLength: 1, maxLength: 60 }), tags: s.array(s.string({ minLength: 1, maxLength: 24 }), { maxItems: 8 }) }),
    404: s.object({ missing: s.boolean() }),
  },
  handle: async ({ params, body }) => {
    const item = resolve().update(params.id, body.name, body.tags);
    if (!item) return status(404).problem("not-found", { detail: "item not found" });
    return item;
  },
});

export const remove = route({
  id: "items.delete",
  method: "DELETE",
  path: "/items/:id",
  params: s.object({ id: s.string({ pattern: "^itm_[0-9]+$" }) }),
  response: {
    200: s.object({ deleted: s.boolean(), id: s.string({ pattern: "^itm_[0-9]+$" }) }),
    404: s.object({ missing: s.boolean() }),
  },
  handle: async ({ params }) => {
    const item = resolve().remove(params.id);
    if (!item) return status(404).problem("not-found", { detail: "item not found" });
    return { deleted: true, id: item.id };
  },
});

export default [list, create, get, update, remove] as const;
