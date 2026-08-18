import { readFileSync } from "node:fs";
import { route, defineApp, defineModule } from "@q/core";
import { s } from "@q/schema";

export const r = route({
  id: "bad.get",
  method: "GET",
  path: "/bad",
  response: { 200: s.object({}) },
  handle: async () => ({}),
});

export default defineApp({ id: "bad", modules: [defineModule({ id: "m", routes: [r] })] });
void readFileSync;
