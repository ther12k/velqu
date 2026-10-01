/**
 * #1401 — definePolicy's declared `header` must drive the route plan's
 * declared-header set and the pack's security metadata. Before the fix,
 * extraction dropped the field and emit hardcoded "authorization", so a
 * policy reading any other header always saw an empty headers record.
 */
import { describe, it, expect, afterEach } from "bun:test";
import { mkdirSync, rmSync, writeFileSync, readFileSync, symlinkSync, existsSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";
import { extractApp, build } from "../src/index";

const root = join(import.meta.dir, "..", "..", "..");
let cleanup: string[] = [];
afterEach(() => {
  for (const dir of cleanup.splice(0)) rmSync(dir, { recursive: true, force: true });
});

function writeProject(name: string, appTs: string): { dir: string; entry: string } {
  const dir = join(tmpdir(), `velqu-policy-header-${name}-${Date.now()}-${Math.random().toString(36).slice(2)}`);
  mkdirSync(join(dir, "src"), { recursive: true });
  mkdirSync(join(dir, "node_modules", "@velqu"), { recursive: true });
  for (const pkg of ["core", "schema"]) {
    try {
      symlinkSync(join(root, "packages", pkg), join(dir, "node_modules", "@velqu", pkg), "dir");
    } catch {}
  }
  writeFileSync(join(dir, "src", "app.ts"), appTs);
  cleanup.push(dir);
  return { dir, entry: join(dir, "src", "app.ts") };
}

const APP = (header: string | null) => [
  `import { definePolicy, route } from "@velqu/core";`,
  `import { s } from "@velqu/schema";`,
  `export const tokenPolicy = definePolicy({`,
  `  id: "hook.token",`,
  ...(header ? [`  header: "${header}",`] : []),
  `  declares: { 401: "unauthorized" },`,
  `  provides: "hook",`,
  `  check: async (req) => {`,
  `    const headers = req.headers as Record<string, string>;`,
  `    const token = headers[${JSON.stringify(header ?? "authorization")}];`,
  `    if (token !== "t") return ({ __problem: true, status: 401 }) as never;`,
  `    return { hook: { ok: true } };`,
  `  },`,
  `});`,
  `export const secured = route({`,
  `  id: "hooks.receive",`,
  `  method: "POST",`,
  `  path: "/hooks",`,
  `  policy: tokenPolicy,`,
  `  body: s.object({ n: s.integer() }),`,
  `  response: { 201: s.object({ ok: s.boolean() }) },`,
  `  handle: async () => ({ ok: true }),`,
  `});`,
].join("\n");

describe("policy declared header (#1401)", () => {
  it("extraction captures the declared policy header", () => {
    const { entry } = writeProject("extract", APP("x-hook-token"));
    const app = extractApp(entry);
    const pol = app.policies.find((p) => p.id === "hook.token");
    expect(pol?.header).toBe("x-hook-token");
  });

  it("plan declared-header set carries the custom header (pack + security)", async () => {
    const { dir, entry } = writeProject("custom", APP("x-hook-token"));
    expect(extractApp(entry).routes.find((r) => r.id === "hooks.receive")?.policyId).toBe("hook.token");
    const res = await build({ project: dir, outDir: join(dir, "dist") });
    const pack = JSON.parse(readFileSync(res.packPath, "utf8"));
    const table: string[] = pack.headerNameTable;
    expect(table).toContain("x-hook-token");
    const route = pack.routes.find((r: { id: string }) => r.id === "hooks.receive");
    const declared = route.plan.headerNameIds.map((i: number) => table[i]);
    expect(declared).toContain("x-hook-token");
    expect(declared).not.toContain("authorization");
    expect(route.security).toEqual([{ scheme: "bearer", header: "x-hook-token", problemStatus: 401 }]);
  });

  it("omitted header defaults to authorization (existing packs byte-stable)", async () => {
    const { dir } = writeProject("default", APP(null));
    const res = await build({ project: dir, outDir: join(dir, "dist") });
    const pack = JSON.parse(readFileSync(res.packPath, "utf8"));
    const table: string[] = pack.headerNameTable;
    expect(table).toContain("authorization");
    const route = pack.routes.find((r: { id: string }) => r.id === "hooks.receive");
    const declared = route.plan.headerNameIds.map((i: number) => table[i]);
    expect(declared).toEqual(["authorization"]);
    expect(route.security).toEqual([{ scheme: "bearer", header: "authorization", problemStatus: 401 }]);
  });
});
