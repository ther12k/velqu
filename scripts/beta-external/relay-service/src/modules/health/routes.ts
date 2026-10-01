import { route } from "@velqu/core";
import { s } from "@velqu/schema";

export const live = route({
  id: "health.live",
  method: "GET",
  path: "/health/live",
  response: {
    200: s.object({ status: s.string() }),
  },
  handle: async () => ({ status: "ok" }),
});

export const healthRoutes = [live];
