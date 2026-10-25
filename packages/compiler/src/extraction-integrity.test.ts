/**
 * #1421 — static-extraction integrity. Three fail-open defects, all fixed
 * in extract.ts:
 *
 *  1. route() object literals silently DROPPED spread (and shorthand)
 *     properties — `...{ policy: Session }` compiled to a pack with no
 *     policy, no security, no policyHandlerId, and no diagnostic.
 *  2. policy references resolved by binding NAME through one app-wide map —
 *     two modules exporting same-named policies collided and the last one
 *     visited silently won every route; aliased imports failed outright.
 *     Resolution is now TypeScript symbol identity.
 *  3. every syntactic route() declaration was packed, regardless of
 *     defineApp/defineModule membership — removing a route from composition
 *     did not remove the endpoint. With defineApp present, exactly the
 *     module graph's routes ship; without it, legacy collect-all behavior
 *     is preserved.
 */
import { describe, it, expect, afterEach } from "bun:test";
import { mkdirSync, rmSync, writeFileSync, readFileSync, symlinkSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";
import { extractApp, build, CompileError } from "../src/index";

const root = join(import.meta.dir, "..", "..", "..");
let cleanup: string[] = [];
afterEach(() => {
  for (const dir of cleanup.splice(0)) rmSync(dir, { recursive: true, force: true });
});

/** Multi-file project fixture: files map is relative to the project root. */
function writeProject(name: string, files: Record<string, string>): { dir: string; entry: string } {
  const dir = join(tmpdir(), `velqu-extraction-${name}-${Date.now()}-${Math.random().toString(36).slice(2)}`);
  mkdirSync(join(dir, "src"), { recursive: true });
  mkdirSync(join(dir, "node_modules", "@velqu"), { recursive: true });
  for (const pkg of ["core", "schema"]) {
    try {
      symlinkSync(join(root, "packages", pkg), join(dir, "node_modules", "@velqu", pkg), "dir");
    } catch {}
  }
  for (const [rel, content] of Object.entries(files)) {
    const target = join(dir, rel);
    mkdirSync(join(target, ".."), { recursive: true });
    writeFileSync(target, content);
  }
  cleanup.push(dir);
  return { dir, entry: join(dir, "src", "app.ts") };
}

const routeBody = (id: string, path: string) => `
  id: "${id}",
  method: "GET",
  path: "${path}",
  response: { 200: s.object({ ok: s.boolean() }) },
  handle: async () => ({ ok: true }),
`;

// two modules that BOTH bind the name `Session` to different policies
const MOD_STRICT = `
import { definePolicy, route } from "@velqu/core";
import { s } from "@velqu/schema";

export const Session = definePolicy({
  id: "strict",
  header: "authorization",
  declares: { 401: "unauthorized" },
  provides: "session",
  check: async () => ({ session: { ok: true } }),
});

export const protectedRoute = route({
${routeBody("a.protected", "/a")}
  policy: Session,
});

export default [protectedRoute];
`;

const MOD_PREVIEW = `
import { definePolicy, route } from "@velqu/core";
import { s } from "@velqu/schema";

export const Session = definePolicy({
  id: "preview",
  header: "x-preview",
  declares: { 401: "unauthorized" },
  provides: "session",
  check: async () => ({ session: { ok: true } }),
});

export const previewRoute = route({
${routeBody("b.preview", "/b")}
  policy: Session,
});

export default [previewRoute];
`;

const COLLISION_APP = `
import { defineApp, defineModule } from "@velqu/core";
import aRoutes from "./a";
import bRoutes from "./b";

export const app = defineApp({
  id: "collision",
  modules: [
    defineModule({ id: "a", routes: [...aRoutes] }),
    defineModule({ id: "b", routes: [...bRoutes] }),
  ],
});
export default app;
`;

describe("route literal fail-closed (#1421.1)", () => {
  it("rejects spread properties instead of silently dropping them", () => {
    const { entry } = writeProject("spread", {
      "src/app.ts": `
import { definePolicy, route } from "@velqu/core";
import { s } from "@velqu/schema";
export const Session = definePolicy({ id: "p", declares: { 401: "unauthorized" }, provides: "session", check: async () => ({ session: {} }) });
export const r = route({
${routeBody("spread.get", "/spread")}
  ...{ policy: Session },
});
export default r;
`,
    });
    expect(() => extractApp(entry)).toThrow(/does not support spread properties/);
  });

  it("rejects shorthand properties (the value expression must be statically visible)", () => {
    const { entry } = writeProject("shorthand", {
      "src/app.ts": `
import { definePolicy, route } from "@velqu/core";
import { s } from "@velqu/schema";
export const Session = definePolicy({ id: "p", declares: { 401: "unauthorized" }, provides: "session", check: async () => ({ session: {} }) });
const policy = Session;
export const r = route({
${routeBody("short.get", "/short")}
  policy,
});
export default r;
`,
    });
    expect(() => extractApp(entry)).toThrow(/shorthand property 'policy'/);
  });

  it("rejects spreads inside definePolicy literals", () => {
    const { entry } = writeProject("policy-spread", {
      "src/app.ts": `
import { definePolicy, route } from "@velqu/core";
import { s } from "@velqu/schema";
export const P = definePolicy({ id: "p", ...{ header: "x-p" }, declares: { 401: "unauthorized" }, provides: "session", check: async () => ({ session: {} }) });
export const r = route({
${routeBody("ps.get", "/ps")}
  policy: P,
});
export default r;
`,
    });
    expect(() => extractApp(entry)).toThrow(/definePolicy\(\) object literal does not support spread/);
  });
});

describe("policy resolution by symbol (#1421.2)", () => {
  it("same-named policies in different modules each bind their own routes", () => {
    const { entry } = writeProject("collision", {
      "src/app.ts": COLLISION_APP,
      "src/a.ts": MOD_STRICT,
      "src/b.ts": MOD_PREVIEW,
    });
    const app = extractApp(entry);
    const a = app.routes.find((r) => r.id === "a.protected");
    const b = app.routes.find((r) => r.id === "b.preview");
    expect(a?.policyId).toBe("strict");
    expect(b?.policyId).toBe("preview");
  });

  it("the emitted pack carries each route's OWN policy header and handler (not the last module's)", async () => {
    const { dir, entry } = writeProject("collision-pack", {
      "src/app.ts": COLLISION_APP,
      "src/a.ts": MOD_STRICT,
      "src/b.ts": MOD_PREVIEW,
    });
    expect(extractApp(entry).routes.length).toBe(2);
    const res = await build({ project: dir, outDir: join(dir, "dist") });
    const pack = JSON.parse(readFileSync(res.packPath, "utf8"));
    const a = pack.routes.find((r: { id: string }) => r.id === "a.protected");
    const b = pack.routes.find((r: { id: string }) => r.id === "b.preview");
    expect(a.security).toEqual([{ scheme: "bearer", header: "authorization", problemStatus: 401 }]);
    expect(b.security).toEqual([{ scheme: "bearer", header: "x-preview", problemStatus: 401 }]);
    expect(a.plan.policyHandlerId).not.toBe(b.plan.policyHandlerId);
    expect(a.plan.policyHandlerId).not.toBeNull();
    expect(b.plan.policyHandlerId).not.toBeNull();
  });

  it("aliased policy imports resolve (name-matching failed on these before)", () => {
    const { entry } = writeProject("alias", {
      "src/app.ts": `
import { defineApp, defineModule } from "@velqu/core";
import { Session as Auth } from "./a";
import { protectedRoute } from "./a";
export const app = defineApp({ id: "alias", modules: [defineModule({ id: "a", routes: [protectedRoute] })] });
export default app;
`,
      "src/a.ts": MOD_STRICT.replace('export default [protectedRoute];', "export { protectedRoute };"),
    });
    const app = extractApp(entry);
    expect(app.routes.find((r) => r.id === "a.protected")?.policyId).toBe("strict");
  });

  it("a policy reference that resolves to no definePolicy fails closed", () => {
    const { entry } = writeProject("unknown-policy", {
      "src/app.ts": `
import { defineApp, defineModule, route } from "@velqu/core";
import { s } from "@velqu/schema";
// no definePolicy anywhere: the identifier has no policy symbol to resolve
const Session = { not: "a policy" };
export const r = route({
${routeBody("u.get", "/u")}
  policy: Session,
});
export const app = defineApp({ id: "u", modules: [defineModule({ id: "m", routes: [r] })] });
export default app;
`,
    });
    expect(() => extractApp(entry)).toThrow(/references unknown policy 'Session'/);
  });
});

describe("app-graph membership (#1421.3)", () => {
  it("an exported route omitted from defineApp modules stays OUT of the pack", async () => {
    const { dir, entry } = writeProject("membership", {
      "src/app.ts": `
import { defineApp, defineModule, route } from "@velqu/core";
import { s } from "@velqu/schema";
export const pub = route({ ${routeBody("member.public", "/public")} });
export const omittedAdmin = route({ ${routeBody("member.omitted", "/admin")} });
export const app = defineApp({ id: "member", modules: [defineModule({ id: "m", routes: [pub] })] });
export default app;
`,
    });
    const app = extractApp(entry);
    expect(app.routes.map((r) => r.id)).toEqual(["member.public"]);
    const res = await build({ project: dir, outDir: join(dir, "dist") });
    const pack = JSON.parse(readFileSync(res.packPath, "utf8"));
    expect(pack.routes.map((r: { id: string }) => r.id)).toEqual(["member.public"]);
  });

  it("resolves the real-world forms: default single route, named array, spread of default array", () => {
    const { entry } = writeProject("forms", {
      "src/app.ts": `
import { defineApp, defineModule } from "@velqu/core";
import health from "./health/routes";
import { greetingsRoutes } from "./greetings/routes";
import usersRoutes from "./users/routes";
export const app = defineApp({
  id: "forms",
  modules: [
    defineModule({ id: "health", routes: [health] }),
    defineModule({ id: "greetings", routes: greetingsRoutes }),
    defineModule({ id: "users", routes: [...usersRoutes] }),
  ],
});
export default app;
`,
      "src/health/routes.ts": `
import { route } from "@velqu/core";
import { s } from "@velqu/schema";
export const live = route({ ${routeBody("health.live", "/health/live")} });
export default live;
`,
      "src/greetings/routes.ts": `
import { route } from "@velqu/core";
import { s } from "@velqu/schema";
export const get = route({ ${routeBody("greet.get", "/greet")} });
export const create = route({ ${routeBody("greet.create", "/greet/create")} });
export const greetingsRoutes = [get, create];
`,
      "src/users/routes.ts": `
import { route } from "@velqu/core";
import { s } from "@velqu/schema";
export const list = route({ ${routeBody("users.list", "/users")} });
export const detail = route({ ${routeBody("users.detail", "/users/:id")} });
export default [list, detail] as const;
`,
    });
    const app = extractApp(entry);
    expect(app.routes.map((r) => r.id).sort()).toEqual([
      "greet.create",
      "greet.get",
      "health.live",
      "users.detail",
      "users.list",
    ]);
  });

  it("module references to unknown or dynamic bindings fail closed", () => {
    const unknown = writeProject("unknown-route", {
      "src/app.ts": `
import { defineApp, defineModule } from "@velqu/core";
import { real } from "./real";
export const app = defineApp({
  id: "x",
  modules: [defineModule({ id: "ok", routes: [real] }), defineModule({ id: "m", routes: [doesNotExist] })],
});
export default app;
`,
      "src/real.ts": `
import { route } from "@velqu/core";
import { s } from "@velqu/schema";
export const real = route({ ${routeBody("real.get", "/real")} });
`,
    });
    expect(() => extractApp(unknown.entry)).toThrow(/does not resolve to an exported route or array of routes|references 'doesNotExist'/);

    const dynamic = writeProject("dynamic-route", {
      "src/app.ts": `
import { defineApp, defineModule, route } from "@velqu/core";
import { s } from "@velqu/schema";
export const r = route({ ${routeBody("d.get", "/d")} });
const pick = Math.random() > 0.5 ? [r] : [];
export const app = defineApp({ id: "x", modules: [defineModule({ id: "m", routes: pick })] });
export default app;
`,
    });
    expect(() => extractApp(dynamic.entry)).toThrow(/dynamic expression|not an exported route/);
  });

  it("without defineApp, every discovered route still ships (legacy behavior preserved)", () => {
    const { entry } = writeProject("legacy", {
      "src/app.ts": `
import { route } from "@velqu/core";
import { s } from "@velqu/schema";
export const one = route({ ${routeBody("legacy.one", "/one")} });
export const two = route({ ${routeBody("legacy.two", "/two")} });
`,
    });
    const app = extractApp(entry);
    expect(app.routes.map((r) => r.id).sort()).toEqual(["legacy.one", "legacy.two"]);
  });
});
