/**
 * M4A-003-D/V: Optional fetch and service profile choices for scaffolding.
 *
 * Profile choices mirror the runtime's fail-closed grammar
 * (`serverless` | `service:N`, N = 1..64): the scaffold must never emit a
 * script the runtime itself would reject (verified live in M4A-003-V).
 */

import { describe, it, expect, beforeEach, afterEach } from "bun:test";
import {
  generateStarterProject,
  resolveServiceProfile,
  SERVICE_PROFILE_USAGE,
} from "./scaffold";
import { extractApp, build } from "@velqu/compiler";
import { ExitCode } from "./exit-codes";
import { mkdirSync, rmSync, existsSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";

describe("Optional fetch & profile scaffolding choices (M4A-003-D/V)", () => {
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

    expect(pkg.scripts.dev).toBe("bun node_modules/@velqu/cli/src/index.ts dev --project .");
    expect(pkg.scripts.build).toBe("bun node_modules/@velqu/cli/src/index.ts build --project .");
    expect(pkg.velqu.profile).toBe("serverless");
    expect(pkg.velqu.capabilities).toEqual([]);
    expect(files["src/app.ts"]).toContain("Configured runtime profile: serverless");
  });

  it("configures explicit-count service profile (service:N) matching the runtime grammar", () => {
    const files = generateStarterProject({ name: "service-app", profile: "service:4" });
    const pkg = JSON.parse(files["package.json"]);

    expect(pkg.scripts.dev).toBe("bun node_modules/@velqu/cli/src/index.ts dev --project . --profile service:4");
    expect(pkg.scripts.build).toBe("bun node_modules/@velqu/cli/src/index.ts build --project . --profile service:4");
    expect(pkg.velqu.profile).toBe("service:4");
    expect(files["src/app.ts"]).toContain("Configured runtime profile: service:4");
  });

  it("accepts every in-bounds service:N worker count and rejects out-of-bounds counts", () => {
    expect(resolveServiceProfile("service:1")).toMatchObject({ ok: true, profile: "service:1" });
    expect(resolveServiceProfile("service:64")).toMatchObject({ ok: true, profile: "service:64" });
    expect(resolveServiceProfile("service:0").ok).toBeFalse();
    expect(resolveServiceProfile("service:65").ok).toBeFalse();
  });

  it("fails closed on bare 'service' naming the explicit-count requirement", () => {
    const resolved = resolveServiceProfile("service");
    expect(resolved.ok).toBeFalse();
    if (!resolved.ok) {
      expect(resolved.error).toContain("requires an explicit worker count");
      expect(resolved.error).toContain("service:N");
    }
    expect(() => generateStarterProject({ name: "bad-app", profile: "service" as any })).toThrow(
      /explicit worker count/,
    );
  });

  it("fails closed on unknown profile names (e.g. 'throughput') with the accepted grammar", () => {
    const resolved = resolveServiceProfile("throughput");
    expect(resolved.ok).toBeFalse();
    if (!resolved.ok) {
      expect(resolved.error).toContain("throughput");
      expect(resolved.error).toContain(SERVICE_PROFILE_USAGE);
    }
    expect(() =>
      generateStarterProject({ name: "bad-app", profile: "throughput" as any }),
    ).toThrow(/invalid service profile 'throughput'/);
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

  it("CLI init accepts --profile service:N and --with-fetch flags and writes configured project", async () => {
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
        "service:4",
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
    expect(stdout).toContain("profile: service:4");
    expect(stdout).toContain("fetch: enabled");

    const pkg = JSON.parse(await Bun.file(join(target, "package.json")).text());
    expect(pkg.scripts.dev).toBe("bun node_modules/@velqu/cli/src/index.ts dev --project . --profile service:4");
    expect(pkg.velqu.profile).toBe("service:4");
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
        "service:2",
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
    expect(parsed.profile).toBe("service:2");
    expect(parsed.withFetch).toBe(true);
    expect(parsed.filesCount).toBe(12);
  });

  it("CLI init rejects bare 'service' profile with the explicit-count guidance", async () => {
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
        "service",
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
    expect(stderr).toContain("explicit worker count");
    expect(stderr).toContain("service:N");
  });
});
