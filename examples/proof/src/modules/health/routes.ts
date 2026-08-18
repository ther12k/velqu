import { route } from "@q/core";
import { s } from "@q/schema";

/** C0: statically evaluable handler → compiler emits native liveness. */
export const live = route({
  id: "health.live",
  method: "GET",
  path: "/health/live",
  response: { 200: s.object({ status: s.string() }) },
  handle: () => ({ status: "ok" }),
});

export default live;
