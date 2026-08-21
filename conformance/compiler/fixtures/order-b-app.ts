import { route, defineApp, defineModule } from "@velqu/core";
import { s } from "@velqu/schema";

// M25-001-C fixture: the SAME declarations as order-a-app.ts but every option
// literal is written in reversed field order. Both must compile to identical
// canonical hashes (sorted-key canonical form, ADR-0023).

export const ordered = route({
  id: "order.get",
  method: "GET",
  path: "/order/:id",
  params: s.object({ id: s.string({ format: "uuid", minLength: 1, maxLength: 32 }) }),
  query: s.object({
    page: s.optional(s.integer({ minimum: 1, maximum: 500 }), { default: 1 }),
    flags: s.array(s.enum(["a", "b"]), { maxItems: 4, minItems: 0 }),
  }),
  response: {
    200: s.object({
      score: s.number({ minimum: 0.0, maximum: 1.5 }),
      mode: s.enum(["fast", "slow"]),
    }),
  },
  handle: async () => ({ score: 0.5, mode: "fast" as const }),
});

export default defineApp({
  id: "orderapp",
  modules: [defineModule({ id: "orderapp", routes: [ordered] })],
});
