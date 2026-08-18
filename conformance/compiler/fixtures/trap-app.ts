import { route, defineApp, defineModule, defineService } from "@velqu/core";
import { s } from "@velqu/schema";

// TRAP: this global records any module-scope side effect (must stay 0 hits
// for the route-count we compile; the compiler must never EXECUTE modules).
declare const globalThis: { __velquTrapSideEffects?: number };
globalThis.__velquTrapSideEffects = (globalThis.__velquTrapSideEffects ?? 0) + 1;

// TRAP: service factory that throws if invoked (COMP-002: factories must not
// run at build time)
export const trapService = defineService("trap.service", () => {
  throw new Error("SERVICE FACTORY RAN DURING BUILD (COMP-002 violation)");
});

export const trapped = route({
  id: "trap.get",
  method: "GET",
  path: "/trap",
  response: { 200: s.object({ ok: s.boolean() }) },
  handle: async () => ({ ok: true }),
});

export default defineApp({
  id: "trap",
  modules: [defineModule({ id: "trap", routes: [trapped] })],
});
