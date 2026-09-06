/**
 * BWASM-C-005 — capability portability classification, build-time
 * refusal, recorded states, and the deployment-requirements report.
 */
import { describe, it, expect, afterEach } from "bun:test";
import { mkdirSync, rmSync, writeFileSync, existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import {
  classifyCapability,
  portabilityReport,
  CAPABILITY_PORTABILITY_REGISTRY,
  buildBrowserWasmArtifacts,
  extractApp,
  build,
} from "../src/index";

const root = join(import.meta.dir, "..", "..", "..");
let cleanup: string[] = [];
afterEach(() => {
  for (const dir of cleanup.splice(0)) rmSync(dir, { recursive: true, force: true });
});

function writeProject(dir: string, appTs: string): string {
  mkdirSync(join(dir, "src"), { recursive: true });
  mkdirSync(join(dir, "node_modules", "@velqu"), { recursive: true });
  const { symlinkSync } = require("node:fs");
  for (const pkg of ["core", "schema", "capability-postgres"]) {
    const target = join(root, "packages", pkg);
    try {
      symlinkSync(target, join(dir, "node_modules", "@velqu", pkg), "dir");
    } catch {}
  }
  writeFileSync(join(dir, "src", "app.ts"), appTs);
  return join(dir, "src", "app.ts");
}

const PG_APP = [
  `import { route } from "@velqu/core";`,
  `import { s } from "@velqu/schema";`,
  `export const users = route({`,
  `  id: "users.list",`,
  `  method: "GET",`,
  `  path: "/users",`,
  `  response: { 200: s.object({ rows: s.array(s.string()) }) },`,
  `  handle: async (ctx) => {`,
  `    const r = await ctx.native.postgres.sql("SELECT id FROM users");`,
  `    return { rows: r.rows.map((row) => String(row.id)) };`,
  `  },`,
  `});`,
  `export const app = { routes: [users] };`,
  ``,
].join("\n");

describe("C-005 portability classification (goldens, fail closed)", () => {
  it("classifies every registered capability", () => {
    expect(classifyCapability("timer").state).toBe("browser-and-native");
    expect(classifyCapability("postgres").state).toBe("deployment-required");
    expect(classifyCapability("kv").state).toBe("browser");
    expect(classifyCapability("payments").state).toBe("forbidden");
    expect(classifyCapability("queues").state).toBe("forbidden");
  });

  it("UNKNOWN names fail closed as forbidden", () => {
    expect(classifyCapability("totally-unknown").state).toBe("forbidden");
    expect(classifyCapability("totally-unknown").remediation).toContain("fail closed");
    // every reserved production-integration name is forbidden
    for (const name of ["payments", "email", "webhooks", "cron", "queues"]) {
      expect(classifyCapability(name).state).toBe("forbidden");
    }
  });

  it("registry entries carry safe remediation (no secret-looking values)", () => {
    for (const [name, entry] of Object.entries(CAPABILITY_PORTABILITY_REGISTRY)) {
      expect(entry.state).toBeTruthy();
      expect(entry.remediation).not.toMatch(/password|secret=|api[_-]?key\s*[:=]/i);
      void name;
    }
  });
});

describe("C-005 portability report (machine-readable requirements)", () => {
  it("flags deployment-required on the browser target with routes and remediation", () => {
    const report = portabilityReport([
      { id: "users.list", capabilities: ["postgres"] },
      { id: "health.live", capabilities: ["timer"] },
    ]);
    expect(report.deploymentRequired).toBeTrue();
    const pg = report.entries.find((e) => e.capability === "postgres")!;
    expect(pg.state).toBe("deployment-required");
    expect(pg.routes).toEqual(["users.list"]);
    expect(pg.remediation).toContain("native Velqu runtime");
    const timer = report.entries.find((e) => e.capability === "timer")!;
    expect(timer.state).toBe("browser-and-native");
  });

  it("native target does not flag deployment-required (that is its job); forbidden always flags", () => {
    const routes = [{ id: "users.list", capabilities: ["postgres", "payments"] }];
    expect(portabilityReport(routes, { target: "native" }).deploymentRequired).toBeTrue(); // payments forbidden
    const onlyPg = [{ id: "users.list", capabilities: ["postgres"] }];
    expect(portabilityReport(onlyPg, { target: "native" }).deploymentRequired).toBeFalse();
    expect(portabilityReport(onlyPg, { target: "browser-wasm" }).deploymentRequired).toBeTrue();
  });

  it("unknown grants surface as forbidden in the report", () => {
    const report = portabilityReport([{ id: "r", capabilities: ["nope"] }]);
    expect(report.entries[0]!.state).toBe("forbidden");
    expect(report.deploymentRequired).toBeTrue();
  });
});

describe("C-005 build-time refusal (browser-wasm target)", () => {
  it("a postgres-declaring route fails the browser build before any artifact", () => {
    const dir = join(root, `.tmp-c005-pg`);
    cleanup.push(dir);
    rmSync(dir, { recursive: true, force: true });
    const entry = writeProject(dir, PG_APP);
    const app = extractApp(entry);
    try {
      buildBrowserWasmArtifacts(
        { project: entry, outDir: join(dir, "dist") },
        app,
        join(dir, "dist"),
      );
      throw new Error("unreachable");
    } catch (e) {
      expect(String(e)).toContain("deployment-required");
      expect(String(e)).toContain("postgres");
      expect(String(e)).toContain("users.list");
      expect(String(e)).toContain("src/app.ts");
    }
    expect(existsSync(join(dir, "dist", "browser"))).toBeFalse();
  });

  it("an explicit simulation profile records the state (never a mock implementation)", () => {
    const dir = join(root, `.tmp-c005-sim`);
    cleanup.push(dir);
    rmSync(dir, { recursive: true, force: true });
    const entry = writeProject(dir, PG_APP);
    // the native build produces the pack the browser set carries
    bun_native_build(dir);
    const app = extractApp(entry);
    const result = buildBrowserWasmArtifacts(
      { project: entry, outDir: join(dir, "dist"), simulate: true },
      app,
      join(dir, "dist"),
    );
    const manifest = JSON.parse(readFileSync(join(result.outDir, "browser-manifest.json"), "utf8"));
    expect(manifest.simulatedCapabilities).toEqual(["postgres"]);
  }, 60_000);

  it("forbidden capability names fail closed even with simulate", () => {
    const dir = join(root, `.tmp-c005-forbidden`);
    cleanup.push(dir);
    rmSync(dir, { recursive: true, force: true });
    // a handler that touches ctx.native.payments — extraction records the
    // grant; classification is forbidden regardless of simulate
    const entry = writeProject(
      dir,
      [
        `import { route } from "@velqu/core";`,
        `import { s } from "@velqu/schema";`,
        `export const pay = route({`,
        `  id: "pay.create",`,
        `  method: "POST",`,
        `  path: "/pay",`,
        `  response: { 402: s.object({}) },`,
        `  handle: async (ctx) => { void ctx.native.payments; return {}; },`,
        `});`,
        `export const app = { routes: [pay] };`,
        ``,
      ].join("\n"),
    );
    const app = extractApp(entry);
    try {
      buildBrowserWasmArtifacts(
        { project: entry, outDir: join(dir, "dist"), simulate: true },
        app,
        join(dir, "dist"),
      );
      throw new Error("unreachable");
    } catch (e) {
      expect(String(e)).toContain("never simulated");
      expect(String(e)).toContain("payments");
    }
  });

  it("the native build records portability states in capability-manifest.json", async () => {
    const dir = join(root, `.tmp-c005-record`);
    cleanup.push(dir);
    rmSync(dir, { recursive: true, force: true });
    const entry = writeProject(dir, PG_APP);
    await build({ project: entry, outDir: join(dir, "dist") });
    const manifest = JSON.parse(readFileSync(join(dir, "dist", "capability-manifest.json"), "utf8"));
    expect(manifest.portability.postgres).toEqual({
      state: "deployment-required",
      remediation: expect.any(String),
    });
  }, 60_000);
});

function bun_native_build(dir: string): void {
  const { execFileSync } = require("node:child_process");
  execFileSync("bun", ["packages/cli/src/index.ts", "build", "--project", dir], {
    cwd: root,
    stdio: "pipe",
  });
}
