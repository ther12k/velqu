/**
 * Postgres capability pack-wiring test (BETA-004-A).
 *
 * End-to-end through the compiler CLI: an app whose handler touches
 * `native.postgres` produces a pack whose capability manifest declares
 * the grant (and therefore the exact `runtime:postgres` v1 requirement
 * via resolveLinkedModules), while an app that never touches postgres
 * declares no postgres grant anywhere — the zero-cost guardrail.
 */
import { describe, it, expect, beforeEach, afterEach } from "bun:test";
import { mkdirSync, rmSync, existsSync, writeFileSync, symlinkSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";

const worktreeDir = process.cwd();

interface RouteBody {
  handleParams: string;
  handleBody: string;
}

function makeProject(dir: string, routeBody: RouteBody): void {
  mkdirSync(join(dir, "src/modules/db"), { recursive: true });
  writeFileSync(
    join(dir, "package.json"),
    JSON.stringify(
      {
        name: "pg-fixture",
        private: true,
        version: "0.1.0",
        dependencies: {
          "@velqu/core": "workspace:*",
          "@velqu/schema": "workspace:*",
        },
      },
      null,
      2,
    ),
  );
  writeFileSync(
    join(dir, "src/modules/db/routes.ts"),
    `import { route } from "@velqu/core";
import { s } from "@velqu/schema";

export const dbCheck = route({
  id: "db.check",
  method: "GET",
  path: "/db",
  response: { 200: s.object({ ok: s.boolean() }) },
  handle: (${routeBody.handleParams}) => {
    ${routeBody.handleBody}
    return { ok: true };
  },
});

export default dbCheck;
`,
  );
  writeFileSync(
    join(dir, "src/app.ts"),
    `import { defineApp, defineModule } from "@velqu/core";
import dbRoutes from "./modules/db/routes";

export const app = defineApp({
  id: "pg-fixture",
  modules: [defineModule({ id: "db", routes: [dbRoutes] })],
});

export default app;
`,
  );
  // workspace package resolution (private alpha requirement)
  mkdirSync(join(dir, "node_modules/@velqu"), { recursive: true });
  for (const pkg of ["core", "schema"]) {
    symlinkSync(join(worktreeDir, "packages", pkg), join(dir, "node_modules/@velqu", pkg), "dir");
  }
  symlinkSync(join(worktreeDir, "node_modules/typescript"), join(dir, "node_modules/typescript"), "dir");
}

describe("postgres grant pack wiring (BETA-004-A)", () => {
  let testDir: string;

  beforeEach(() => {
    testDir = join(tmpdir(), `velqu-pg-grant-${Date.now()}-${Math.random().toString(36).slice(2)}`);
    mkdirSync(testDir, { recursive: true });
  });

  afterEach(() => {
    if (existsSync(testDir)) rmSync(testDir, { recursive: true, force: true });
  });

  it("a postgres-granting route declares the grant end-to-end through the CLI build", async () => {
    const appDir = join(testDir, "pg-app");
    makeProject(appDir, {
      handleParams: "{ native }",
      handleBody: "const rows = native.postgres.sql(\"SELECT 1 AS one\").rows;",
    });
    const build = Bun.spawn(
      ["bun", join(worktreeDir, "packages/cli/src/index.ts"), "build", "--project", appDir, "--out", join(appDir, "dist")],
      { stdout: "pipe", stderr: "pipe", env: process.env },
    );
    const buildErr = await new Response(build.stderr).text();
    const buildCode = await build.exited;
    expect(buildErr).toBe("");
    expect(buildCode).toBe(0);

    const manifest = JSON.parse(readFileSync(join(appDir, "dist/capability-manifest.json"), "utf8"));
    expect(manifest.declared).toContain("postgres");
    expect(manifest.perRoute["db.check"]).toContain("postgres");
  });

  it("a route that never touches postgres declares no grant (zero-cost default)", async () => {
    const appDir = join(testDir, "plain-app");
    makeProject(appDir, {
      handleParams: "{}",
      handleBody: "",
    });
    const build = Bun.spawn(
      ["bun", join(worktreeDir, "packages/cli/src/index.ts"), "build", "--project", appDir, "--out", join(appDir, "dist")],
      { stdout: "pipe", stderr: "pipe", env: process.env },
    );
    const buildCode = await build.exited;
    expect(buildCode).toBe(0);

    const manifest = JSON.parse(readFileSync(join(appDir, "dist/capability-manifest.json"), "utf8"));
    expect(manifest.declared).toEqual([]);
    expect(manifest.perRoute["db.check"]).toEqual([]);
  });
});
