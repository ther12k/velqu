import { route, defineApp, defineModule } from "@velqu/core";
import { s } from "@velqu/schema";

export const gone = route({
  id: "gone.get",
  method: "GET",
  path: "/gone",
  // a declared 404 response — extract auto-tags it with the "not-found"
  // registry problem, which must surface as the exact envelope
  response: {
    200: s.object({ ok: s.boolean() }),
    404: s.object({ missing: s.boolean() }),
  },
  handle: async () => ({ ok: true }),
});

export default defineApp({
  id: "problem-app",
  modules: [defineModule({ id: "gone", routes: [gone] })],
});
