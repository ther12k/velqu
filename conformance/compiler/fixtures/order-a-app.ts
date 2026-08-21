import { route, defineApp, defineModule } from "@velqu/core";
import { s } from "@velqu/schema";

// M25-001-C fixture: option literals written in one field order (A).
// order-b-app.ts declares the SAME schemas with every option map in the
// opposite field order — canonical hashing must make both packs identical.

export const ordered = route({
  id: "order.get",
  method: "GET",
  path: "/order/:id",
  params: s.object({ id: s.string({ maxLength: 32, minLength: 1, format: "uuid" }) }),
  query: s.object({
    page: s.optional(s.integer({ maximum: 500, minimum: 1 }), { default: 1 }),
    flags: s.array(s.enum(["a", "b"]), { minItems: 0, maxItems: 4 }),
  }),
  response: {
    200: s.object({
      score: s.number({ maximum: 1.5, minimum: 0.0 }),
      mode: s.enum(["fast", "slow"]),
    }),
  },
  handle: async () => ({ score: 0.5, mode: "fast" as const }),
});

export default defineApp({
  id: "orderapp",
  modules: [defineModule({ id: "orderapp", routes: [ordered] })],
});
