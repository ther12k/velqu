import { route, defineApp, defineModule } from "@velqu/core";
import { s } from "@velqu/schema";

const dynamicPath = "/computed-" + "path";
export const r = route({ id: "dyn.get", method: "GET", path: dynamicPath, response: { 200: s.object({}) }, handle: async () => ({}) });

export default defineApp({ id: "dyn", modules: [defineModule({ id: "m", routes: [r] })] });
