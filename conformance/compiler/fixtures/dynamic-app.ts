import { route, defineApp, defineModule } from "@q/core";
import { s } from "@q/schema";

const dynamicPath = "/computed-" + "path";
export const r = route({ id: "dyn.get", method: "GET", path: dynamicPath, response: { 200: s.object({}) }, handle: async () => ({}) });

export default defineApp({ id: "dyn", modules: [defineModule({ id: "m", routes: [r] })] });
