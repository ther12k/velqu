import { describe, it, expect, beforeEach, afterEach } from "bun:test";
import { ExitCode } from "./exit-codes";
import { build } from "@velqu/compiler";
import { mkdirSync, writeFileSync, rmSync, existsSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";

describe("CLI Machine-Readable JSON Output (M4A-002-C)", () => {
  let tempDir: string;

  beforeEach(() => {
    tempDir = join(tmpdir(), `velqu-json-test-${Date.now()}-${Math.random().toString(36).slice(2)}`);
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
        `  id: "jsonapp",\n` +
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

  it("outputs structured JSON on velqu build --json", async () => {
    const proc = Bun.spawn(
      ["bun", "packages/cli/src/index.ts", "build", "--project", tempDir, "--out", join(tempDir, "dist"), "--json"],
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
    expect(parsed.command).toBe("build");
    expect(parsed.routes).toBe(1);
    expect(parsed.artifactBytes).toBeDefined();
  });

  it("outputs structured JSON on velqu check --json", async () => {
    const proc = Bun.spawn(
      ["bun", "packages/cli/src/index.ts", "check", "--project", tempDir, "--json"],
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
    expect(parsed.command).toBe("check");
    expect(parsed.appId).toBe("jsonapp");
    expect(parsed.routesCount).toBe(1);
    expect(parsed.clean).toBeTrue();
  });

  it("outputs structured JSON on velqu inspect diagnostics --json", async () => {
    const proc = Bun.spawn(
      ["bun", "packages/cli/src/index.ts", "inspect", "diagnostics", "--project", tempDir, "--json"],
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
    expect(parsed.command).toBe("inspect");
    expect(parsed.target).toBe("diagnostics");
    expect(parsed.appId).toBe("jsonapp");
    expect(parsed.routesCount).toBe(1);
    expect(parsed.verdict).toBe("OK");
  });

  it("outputs structured JSON on velqu pack inspect --json", async () => {
    await build({ project: tempDir, outDir: join(tempDir, "dist") });

    const proc = Bun.spawn(
      ["bun", "packages/cli/src/index.ts", "pack", "inspect", join(tempDir, "dist", "app.qpack"), "--json"],
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
    expect(parsed.command).toBe("pack");
    expect(parsed.target).toBe("inspect");
    expect(parsed.appId).toBe("jsonapp");
    expect(parsed.routesCount).toBe(1);
  });

  it("outputs structured JSON on velqu contract diff --json", async () => {
    await build({ project: tempDir, outDir: join(tempDir, "dist") });

    const proc = Bun.spawn(
      ["bun", "packages/cli/src/index.ts", "contract", "diff", "--project", tempDir, "--json"],
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
    expect(parsed.command).toBe("contract");
    expect(parsed.target).toBe("diff");
    expect(parsed.changesCount).toBe(0);
    expect(parsed.breakingCount).toBe(0);
  });

  it("outputs structured error JSON when compilation fails with --json", async () => {
    writeFileSync(
      join(tempDir, "src", "app.ts"),
      `import { defineApp } from "@velqu/core";\nexport const app = defineApp({ BROKEN_SYNTAX\n`,
    );

    const proc = Bun.spawn(
      ["bun", "packages/cli/src/index.ts", "build", "--project", tempDir, "--json"],
      {
        stdout: "pipe",
        stderr: "pipe",
        env: process.env,
      },
    );
    const stdout = await new Response(proc.stdout).text();
    const exitCode = await proc.exited;

    expect(exitCode).toBe(ExitCode.GENERAL_ERROR);
    const parsed = JSON.parse(stdout);
    expect(parsed.status).toBe("error");
    expect(parsed.command).toBe("build");
    expect(parsed.error).toBeDefined();
  });
});
