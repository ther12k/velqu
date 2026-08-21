import { route, defineApp, defineModule } from "@velqu/core";
import { s } from "@velqu/schema";

export const standard = route({
  id: "std.get",
  method: "GET",
  path: "/standard",
  response: {
    200: s.object({ ok: s.boolean() }),
  },
  handle: async () => ({ ok: true }),
});

export const fallbackBody = route({
  id: "fb.body",
  method: "POST",
  path: "/fb-body",
  body: s.fallback("unsupported-transform", s.object({ data: s.string() })),
  response: {
    200: s.object({ received: s.boolean() }),
  },
  handle: async () => ({ received: true }),
});

export const fallbackResp = route({
  id: "fb.resp",
  method: "GET",
  path: "/fb-resp",
  response: {
    200: s.fallback("measured", s.object({ items: s.array(s.integer()) })),
  },
  handle: async () => ({ items: [1, 2, 3] }),
});

export const fallbackQuery = route({
  id: "fb.query",
  method: "GET",
  path: "/fb-query",
  query: s.fallback("explicit", s.object({ filter: s.string() })),
  response: {
    200: s.object({ matched: s.integer() }),
  },
  handle: async () => ({ matched: 42 }),
});

export default defineApp({
  id: "fallbackapp",
  modules: [defineModule({ id: "fallbackmod", routes: [standard, fallbackBody, fallbackResp, fallbackQuery] })],
});
