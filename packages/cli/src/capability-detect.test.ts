import { describe, expect, test } from "bun:test";
import { writeFileSync, mkdirSync, rmSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";
import { extractApp } from "../../compiler/src/extract";

/** Extract a single-route fixture app and return its declared grants. */
function extractGrants(handlerSource: string): string[] {
  const dir = join(tmpdir(), `velqu-detect-${Date.now()}-${Math.random().toString(36).slice(2)}`);
  mkdirSync(join(dir, "src"), { recursive: true });
  writeFileSync(
    join(dir, "package.json"),
    JSON.stringify({ name: "fixture", type: "module", dependencies: { "@velqu/core": "*" } }),
  );
  writeFileSync(
    join(dir, "src", "app.ts"),
    `import { route } from "@velqu/core";\nimport { s } from "@velqu/schema";\n${handlerSource}\nexport default r;\n`,
  );
  try {
    return extractApp(join(dir, "src", "app.ts")).routes.flatMap((r) => r.capabilities);
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
}

describe("capability detection (M27-002-D regression)", () => {
  test("destructured native usage grants the timer capability", () => {
    // the proof-app shape — this produced NO grant before the fix
    const grants = extractGrants(`
      export const r = route({
        id: "t.timer",
        method: "GET",
        path: "/t",
        response: { 200: s.object({ ok: s.boolean() }) },
        handle: async ({ native }) => {
          await native.timer.delay(5);
          return { ok: true };
        },
      });`);
    expect(grants).toEqual(["timer"]);
  });

  test("aliased destructured native still grants", () => {
    const grants = extractGrants(`
      export const r = route({
        id: "t.alias",
        method: "GET",
        path: "/a",
        response: { 200: s.object({ ok: s.boolean() }) },
        handle: async ({ native: n }) => {
          await n.timer.delay(5);
          return { ok: true };
        },
      });`);
    expect(grants).toEqual(["timer"]);
  });

  test("ctx.native form keeps granting", () => {
    const grants = extractGrants(`
      export const r = route({
        id: "t.ctx",
        method: "GET",
        path: "/c",
        response: { 200: s.object({ ok: s.boolean() }) },
        handle: async (ctx) => {
          await ctx.native.timer.delay(5);
          return { ok: true };
        },
      });`);
    expect(grants).toEqual(["timer"]);
  });

  test("no native usage grants nothing", () => {
    const grants = extractGrants(`
      export const r = route({
        id: "t.plain",
        method: "GET",
        path: "/p",
        response: { 200: s.object({ v: s.integer() }) },
        handle: async ({ query }) => ({ v: query ?? 0 }),
      });`);
    expect(grants).toEqual([]);
  });
});
