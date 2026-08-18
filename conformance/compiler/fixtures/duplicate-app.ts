import { route, defineApp, defineModule } from "@velqu/core";
import { s } from "@velqu/schema";

const a = route({ id: "dup.a", method: "GET", path: "/dup/:id", response: { 200: s.object({}) }, handle: async () => ({}) });
const b = route({ id: "dup.b", method: "GET", path: "/dup/:other", response: { 200: s.object({}) }, handle: async () => ({}) });

export default defineApp({ id: "dup", modules: [defineModule({ id: "m", routes: [a, b] })] });
