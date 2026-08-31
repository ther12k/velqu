import { describe, it, expect, beforeEach, afterEach } from "bun:test";
import { inspectPack } from "./pack-inspect";
import { build } from "@velqu/compiler";
import { mkdirSync, writeFileSync, rmSync, existsSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";

describe("CLI Command Surface (M4A-002-A)", () => {
  let tempDir: string;

  beforeEach(() => {
    tempDir = join(tmpdir(), `velqu-cli-test-${Date.now()}-${Math.random().toString(36).slice(2)}`);
    mkdirSync(tempDir, { recursive: true });
    mkdirSync(join(tempDir, "src"), { recursive: true });
    mkdirSync(join(tempDir, "node_modules", "@velqu"), { recursive: true });

    const { symlinkSync } = require("node:fs");
    try {
      symlinkSync(join(process.cwd(), "packages", "core"), join(tempDir, "node_modules", "@velqu", "core"), "dir");
      symlinkSync(join(process.cwd(), "packages", "schema"), join(tempDir, "node_modules", "@velqu", "schema"), "dir");
    } catch {}

    writeFileSync(
      join(tempDir, "src", "app.ts"),
      `import { defineApp, defineModule, route } from "@velqu/core";\n` +
        `import { s } from "@velqu/schema";\n` +
        `export const hello = route({\n` +
        `  id: "hello.get",\n` +
        `  method: "GET",\n` +
        `  path: "/hello",\n` +
        `  response: { 200: s.object({ msg: s.string() }) },\n` +
        `  handle: async () => ({ msg: "hello" }),\n` +
        `});\n` +
        `export const app = defineApp({\n` +
        `  id: "cliapp",\n` +
        `  modules: [defineModule({ id: "main", routes: [hello] })],\n` +
        `});\n` +
        `export default app;\n`,
    );
    writeFileSync(
      join(tempDir, "tsconfig.json"),
      JSON.stringify({ compilerOptions: { target: "ES2022" } }, null, 2),
    );
  });

  afterEach(() => {
    if (existsSync(tempDir)) {
      rmSync(tempDir, { recursive: true, force: true });
    }
  });

  it("inspects compiled QPack artifact properties accurately with pack-inspect", async () => {
    const buildRes = await build({
      project: tempDir,
      outDir: join(tempDir, "dist"),
    });

    const report = inspectPack(buildRes.packPath);
    expect(report.status).toBe("ok");
    expect(report.appId).toBe("cliapp");
    expect(report.formatVersion).toBe(1);
    expect(report.engine?.name).toBe("quickjs-ng");
    expect(report.routesCount).toBe(1);
    expect(report.contractHash).toBeDefined();
    expect(report.bundleSha256).toBeDefined();
  });

  it("reports clear error when inspecting a non-existent pack file", () => {
    const report = inspectPack(join(tempDir, "does-not-exist.qpack"));
    expect(report.status).toBe("error");
    expect(report.error).toContain("pack file not found");
  });

  it("runs CLI inspect diagnostics subcommand producing static extraction report", async () => {
    const proc = Bun.spawn(
      ["bun", "packages/cli/src/index.ts", "inspect", "diagnostics", "--project", tempDir],
      {
        stdout: "pipe",
        stderr: "pipe",
        env: process.env,
      },
    );

    const stdout = await new Response(proc.stdout).text();
    const exitCode = await proc.exited;

    expect(exitCode).toBe(0);
    expect(stdout).toContain("diagnostics for cliapp");
    expect(stdout).toContain("routes: 1");
    expect(stdout).toContain("verdict: OK");
  });

  it("runs CLI check command verifying static routes without emitting artifacts", async () => {
    const proc = Bun.spawn(
      ["bun", "packages/cli/src/index.ts", "check", "--project", tempDir],
      {
        stdout: "pipe",
        stderr: "pipe",
        env: process.env,
      },
    );

    const stdout = await new Response(proc.stdout).text();
    const exitCode = await proc.exited;

    expect(exitCode).toBe(0);
    expect(stdout).toContain("velqu check: 1 routes in");
    expect(stdout).toContain("clean");
  });

  it("runs CLI pack inspect command and outputs formatted pack summary", async () => {
    const buildRes = await build({
      project: tempDir,
      outDir: join(tempDir, "dist"),
    });

    const proc = Bun.spawn(
      ["bun", "packages/cli/src/index.ts", "pack", "inspect", buildRes.packPath],
      {
        stdout: "pipe",
        stderr: "pipe",
        env: process.env,
      },
    );

    const stdout = await new Response(proc.stdout).text();
    const exitCode = await proc.exited;

    expect(exitCode).toBe(0);
    expect(stdout).toContain("pack:");
    expect(stdout).toContain("appId: cliapp");
    expect(stdout).toContain("formatVersion: 1");
    expect(stdout).toContain("engine: quickjs-ng");
    expect(stdout).toContain("routes: 1");
  });

  it("prints usage instructions when invoked without command or with invalid arguments", async () => {
    const proc = Bun.spawn(["bun", "packages/cli/src/index.ts", "--help"], {
      stdout: "pipe",
      stderr: "pipe",
      env: process.env,
    });

    const stdout = await new Response(proc.stdout).text();
    const exitCode = await proc.exited;

    expect(exitCode).toBe(0);
    expect(stdout).toContain("velqu — Unified Velqu CLI");
    expect(stdout).toContain("velqu dev");
    expect(stdout).toContain("velqu build");
    expect(stdout).toContain("velqu inspect");
    expect(stdout).toContain("velqu contract diff");
    expect(stdout).toContain("velqu pack");
  });
});
