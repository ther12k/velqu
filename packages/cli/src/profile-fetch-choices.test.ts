/**
 * M4A-003-D: Optional fetch and service profile choices for scaffolding.
 */

import { describe, it, expect, beforeEach, afterEach } from "bun:test";
import { generateStarterProject, VALID_SERVICE_PROFILES } from "./scaffold";
import { extractApp, build } from "@velqu/compiler";
import { ExitCode } from "./exit-codes";
import { mkdirSync, rmSync, existsSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";

describe("Optional fetch & profile scaffolding choices (M4A-003-D)", () => {
  let tempDir: string;

  beforeEach(() => {
    tempDir = join(tmpdir(), `velqu-profile-fetch-${Date.now()}-${Math.random().toString(36).slice(2)}`);
    mkdirSync(tempDir, { recursive: true });
  });

  afterEach(() => {
    if (existsSync(tempDir)) {
      rmSync(tempDir, { recursive: true, force: true });
    }
  });

  it("configures default serverless profile when no profile is specified", () => {
    const files = generateStarterProject({ name: "default-app" });
    const pkg = JSON.parse(files["package.json"]);

    expect(pkg.scripts.dev).toBe("velqu dev");
    expect(pkg.scripts.build).toBe("velqu build");
    expect(pkg.velqu.profile).toBe("serverless");
    expect(pkg.velqu.capabilities).toEqual([]);
    expect(files["src/app.ts"]).toContain("Configured runtime profile: serverless");
  });

  it("configures dynamic multi-worker service profile when requested", () => {
    const files = generateStarterProject({ name: "service-app", profile: "service" });
    const pkg = JSON.parse(files["package.json"]);

    expect(pkg.scripts.dev).toBe("velqu dev --profile service");
    expect(pkg.scripts.build).toBe("velqu build --profile service");
    expect(pkg.velqu.profile).toBe("service");
    expect(files["src/app.ts"]).toContain("Configured runtime profile: service");
  });

  it("configures pinned-worker throughput profile when requested", () => {
    const files = generateStarterProject({ name: "throughput-app", profile: "throughput" });
    const pkg = JSON.parse(files["package.json"]);

    expect(pkg.scripts.dev).toBe("velqu dev --profile throughput");
    expect(pkg.scripts.build).toBe("velqu build --profile throughput");
    expect(pkg.velqu.profile).toBe("throughput");
    expect(files["src/app.ts"]).toContain("Configured runtime profile: throughput");
  });

  it("fails closed on invalid profile option with actionable error naming valid options", () => {
    expect(() =>
      generateStarterProject({ name: "bad-app", profile: "unknown" as any }),
    ).toThrow(/Invalid service profile 'unknown'/);
  });

  it("generates upstream fetch module and updates Treaty client when withFetch is true", async () => {
    const files = generateStarterProject({ name: "fetch-app", withFetch: true });

    expect(files["src/modules/upstream/routes.ts"]).toBeDefined();
    expect(files["src/modules/upstream/routes.test.ts"]).toBeDefined();

    const pkg = JSON.parse(files["package.json"]);
    expect(pkg.velqu.capabilities).toEqual(["fetch"]);

    const appCode = files["src/app.ts"];
    expect(appCode).toContain('import { upstreamRoutes } from "./modules/upstream/routes";');
    expect(appCode).toContain('defineModule({ id: "upstream", routes: upstreamRoutes })');

    const clientCode = files["src/client.ts"];
    expect(clientCode).toContain('"upstream.quote"');
    expect(clientCode).toContain('path: "/upstream/quote"');
    expect(clientCode).toContain('"upstream.quote": { path: "/upstream/quote", method: "GET" }');

    // Write files and test static app extraction
    for (const [relPath, content] of Object.entries(files)) {
      const fullPath = join(tempDir, relPath);
      mkdirSync(join(tempDir, relPath, ".."), { recursive: true });
      writeFileSync(fullPath, content);
    }

    mkdirSync(join(tempDir, "node_modules", "@velqu"), { recursive: true });
    const { symlinkSync } = require("node:fs");
    try {
      symlinkSync(join(process.cwd(), "packages", "core"), join(tempDir, "node_modules", "@velqu", "core"), "dir");
      symlinkSync(join(process.cwd(), "packages", "schema"), join(tempDir, "node_modules", "@velqu", "schema"), "dir");
      symlinkSync(join(process.cwd(), "packages", "treaty"), join(tempDir, "node_modules", "@velqu", "treaty"), "dir");
    } catch {}

    const app = extractApp(join(tempDir, "src", "app.ts"));
    expect(app.appId).toBe("fetch-app");
    expect(app.modules).toEqual(["health", "greetings", "upstream"]);
    expect(app.routes.length).toBe(4); // health.live, greetings.get, greetings.create, upstream.quote

    // Build the app:
    const buildRes = await build({
      project: tempDir,
      outDir: join(tempDir, "dist"),
    });
    expect(buildRes.routes).toBe(4);
    expect(existsSync(buildRes.packPath)).toBeTrue();
  });

  it("CLI init accepts --profile and --with-fetch flags and writes configured project", async () => {
    const target = join(tempDir, "cli-service-fetch");

    const proc = Bun.spawn(
      [
        "bun",
        "packages/cli/src/index.ts",
        "init",
        target,
        "--name",
        "cli-service-app",
        "--profile",
        "service",
        "--with-fetch",
      ],
      {
        stdout: "pipe",
        stderr: "pipe",
        env: process.env,
      },
    );

    const stdout = await new Response(proc.stdout).text();
    const exitCode = await proc.exited;

    expect(exitCode).toBe(ExitCode.SUCCESS);
    expect(stdout).toContain("profile: service");
    expect(stdout).toContain("fetch: enabled");

    const pkg = JSON.parse(await Bun.file(join(target, "package.json")).text());
    expect(pkg.scripts.dev).toBe("velqu dev --profile service");
    expect(pkg.velqu.profile).toBe("service");
    expect(pkg.velqu.capabilities).toEqual(["fetch"]);
    expect(existsSync(join(target, "src", "modules", "upstream", "routes.ts"))).toBeTrue();
  });

  it("CLI init --json outputs structured profile and fetch capability details", async () => {
    const target = join(tempDir, "cli-json-choices");

    const proc = Bun.spawn(
      [
        "bun",
        "packages/cli/src/index.ts",
        "init",
        target,
        "--name",
        "json-choices-app",
        "--profile",
        "throughput",
        "--fetch",
        "--json",
      ],
      {
        stdout: "pipe",
        stderr: "pipe",
        env: process.env,
      },
    );

    const stdout = await new Response(proc.stdout).text();
    const exitCode = await proc.exited;

    expect(exitCode).toBe(ExitCode.SUCCESS);
    const parsed = JSON.parse(stdout);
    expect(parsed.status).toBe("ok");
    expect(parsed.profile).toBe("throughput");
    expect(parsed.withFetch).toBe(true);
    expect(parsed.filesCount).toBe(12);
  });

  it("CLI init rejects invalid service profile with error exit code", async () => {
    const target = join(tempDir, "cli-bad-profile");

    const proc = Bun.spawn(
      [
        "bun",
        "packages/cli/src/index.ts",
        "init",
        target,
        "--name",
        "bad-profile-app",
        "--profile",
        "invalid-profile",
      ],
      {
        stdout: "pipe",
        stderr: "pipe",
        env: process.env,
      },
    );

    const stderr = await new Response(proc.stderr).text();
    const exitCode = await proc.exited;

    expect(exitCode).toBe(ExitCode.GENERAL_ERROR);
    expect(stderr).toContain("invalid service profile 'invalid-profile'");
  });
});
