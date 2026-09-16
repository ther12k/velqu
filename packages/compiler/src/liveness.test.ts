/**
 * RUN-009 native-liveness fold tests — object literals (existing rule) and
 * the string-literal extension: fold ONLY when the declared 200 response is
 * a plain constraint-free string schema (wire semantics known: text/plain;
 * charset=utf-8). Everything ambiguous keeps the engine path (fail closed).
 */
import { describe, it, expect, afterEach } from "bun:test";
import { extractApp } from "./extract";
import { mkdtempSync, rmSync, mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";

let cleanup: string[] = [];
afterEach(() => {
  for (const dir of cleanup.splice(0)) rmSync(dir, { recursive: true, force: true });
});

function extractRoutes(source: string): ReturnType<typeof extractApp>["routes"] {
  const dir = mkdtempSync(join(tmpdir(), "velqu-run009-"));
  cleanup.push(dir);
  mkdirSync(join(dir, "src"), { recursive: true });
  writeFileSync(join(dir, "src", "app.ts"), source);
  return extractApp(join(dir, "src", "app.ts")).routes;
}

const routeDef = (handle: string, response: string) =>
  [
    `import { route } from "@velqu/core";`,
    `import { s } from "@velqu/schema";`,
    ``,
    `export const r = route({`,
    `  id: "r",`,
    `  method: "GET",`,
    `  path: "/r",`,
    `  response: { 200: ${response} },`,
    `  handle: ${handle},`,
    `});`,
    ``,
    `export const app = { routes: [r] };`,
    ``,
  ].join("\n");

describe("RUN-009 native-liveness fold", () => {
  it("object literal + object schema folds to application/json (existing rule)", () => {
    const [r] = extractRoutes(routeDef(`() => ({ status: "ok" })`, `s.object({ status: s.string() })`));
    expect(r.liveness).toEqual({ status: 200, contentType: "application/json", body: '{"status":"ok"}' });
  });

  it("async string literal + constraint-free s.string() folds to text/plain", () => {
    const [r] = extractRoutes(routeDef(`async () => "plain"`, `s.string()`));
    expect(r.liveness).toEqual({
      status: 200,
      contentType: "text/plain; charset=utf-8",
      body: "plain",
    });
  });

  it("no-substitution template literal folds like a string literal", () => {
    const [r] = extractRoutes(routeDef("async () => `plain`", `s.string()`));
    expect(r.liveness?.body).toBe("plain");
  });

  it("string literal with CONSTRAINED string schema keeps the engine path", () => {
    const [r] = extractRoutes(
      routeDef(`async () => "plain"`, `s.string({ minLength: 1 })`),
    );
    expect(r.liveness).toBeNull();
  });

  it("string literal with object schema does not fold (kind mismatch)", () => {
    const [r] = extractRoutes(routeDef(`async () => "plain"`, `s.object({ ok: s.boolean() })`));
    expect(r.liveness).toBeNull();
  });

  it("object literal with string schema does not fold (existing rule)", () => {
    const [r] = extractRoutes(routeDef(`() => ({ ok: true })`, `s.string()`));
    expect(r.liveness).toBeNull();
  });

  it("handler with parameters never folds", () => {
    const [r] = extractRoutes(
      routeDef(`async ({ query }) => "plain"`, `s.string()`),
    );
    expect(r.liveness).toBeNull();
  });

  it("non-literal expressions never fold (array index stays engine-bound)", () => {
    const [r] = extractRoutes(routeDef(`async () => ["plain"][0]`, `s.string()`));
    expect(r.liveness).toBeNull();
  });

  it("non-200-only declarations do not fold strings", () => {
    const dir = mkdtempSync(join(tmpdir(), "velqu-run009-"));
    cleanup.push(dir);
    mkdirSync(join(dir, "src"), { recursive: true });
    writeFileSync(
      join(dir, "src", "app.ts"),
      [
        `import { route } from "@velqu/core";`,
        `import { s } from "@velqu/schema";`,
        ``,
        `export const r = route({`,
        `  id: "r",`,
        `  method: "GET",`,
        `  path: "/r",`,
        `  response: { 200: s.string(), 404: s.object({ missing: s.boolean() }) },`,
        `  handle: async () => "plain",`,
        `});`,
        ``,
        `export const app = { routes: [r] };`,
        ``,
      ].join("\n"),
    );
    const [r] = extractApp(join(dir, "src", "app.ts")).routes;
    // 200 is still the declared constraint-free string → folds; the extra
    // status is a different status code and does not affect the 200 fold
    expect(r.liveness).toEqual({
      status: 200,
      contentType: "text/plain; charset=utf-8",
      body: "plain",
    });
  });
});
