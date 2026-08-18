import { route } from "@velqu/core";
import { s } from "@velqu/schema";

/** C5: native timer capability through a JS promise. */
export const timer = route({
  id: "async.timer",
  method: "GET",
  path: "/async",
  query: s.object({
    ms: s.optional(s.integer({ minimum: 1, maximum: 1000 }), { default: 10 }),
  }),
  response: { 200: s.object({ waited: s.integer() }) },
  handle: async ({ query, native }) => {
    const waited = await native.timer.delay(query.ms);
    return { waited };
  },
});

export default timer;
