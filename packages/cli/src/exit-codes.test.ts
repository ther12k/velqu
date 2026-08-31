import { describe, it, expect, beforeEach, afterEach } from "bun:test";
import { ExitCode } from "./exit-codes";
import { build } from "@velqu/compiler";
import { mkdirSync, writeFileSync, rmSync, existsSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";

describe("CLI Stable Exit Codes (M4A-002-B)", () => {
  let tempDir: string;

  beforeEach(() => {
    tempDir = join(tmpdir(), `velqu-exit-test-${Date.now()}-${Math.random().toString(36).slice(2)}`);
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
        `  id: "exitapp",\n` +
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

  it("exits 0 on successful build", async () => {
    const proc = Bun.spawn(
      ["bun", "packages/cli/src/index.ts", "build", "--project", tempDir, "--out", join(tempDir, "dist")],
      {
        stdout: "pipe",
        stderr: "pipe",
        env: process.env,
      },
    );
    const exitCode = await proc.exited;
    expect(exitCode).toBe(ExitCode.SUCCESS);
  });

  it("exits 0 on clean contract diff (no changes)", async () => {
    // Build first to produce contract.lock.json:
    await build({ project: tempDir, outDir: join(tempDir, "dist") });

    const proc = Bun.spawn(
      ["bun", "packages/cli/src/index.ts", "contract", "diff", "--project", tempDir],
      {
        stdout: "pipe",
        stderr: "pipe",
        env: process.env,
      },
    );
    const exitCode = await proc.exited;
    expect(exitCode).toBe(ExitCode.SUCCESS);
  });

  it("exits 2 on breaking contract diff (e.g. route removed / changed)", async () => {
    // Build initial lock:
    await build({ project: tempDir, outDir: join(tempDir, "dist") });

    // Modify route to introduce breaking change (delete /hello, add /hello-v2):
    writeFileSync(
      join(tempDir, "src", "app.ts"),
      `import { defineApp, defineModule, route } from "@velqu/core";\n` +
        `import { s } from "@velqu/schema";\n` +
        `export const helloV2 = route({\n` +
        `  id: "hello.v2",\n` +
        `  method: "GET",\n` +
        `  path: "/hello-v2",\n` +
        `  response: { 200: s.object({ msg: s.string() }) },\n` +
        `  handle: async () => ({ msg: "hello" }),\n` +
        `});\n` +
        `export const app = defineApp({\n` +
        `  id: "exitapp",\n` +
        `  modules: [defineModule({ id: "main", routes: [helloV2] })],\n` +
        `});\n` +
        `export default app;\n`,
    );

    // Rebuild without updating lock so dist/ differs from contract.lock.json:
    await build({ project: tempDir, outDir: join(tempDir, "dist"), updateLock: false });

    const proc = Bun.spawn(
      ["bun", "packages/cli/src/index.ts", "contract", "diff", "--project", tempDir],
      {
        stdout: "pipe",
        stderr: "pipe",
        env: process.env,
      },
    );
    const exitCode = await proc.exited;
    expect(exitCode).toBe(ExitCode.BREAKING_CONTRACT);
  });

  it("exits 1 on compilation error (syntax/extraction failure)", async () => {
    // Write broken code:
    writeFileSync(
      join(tempDir, "src", "app.ts"),
      `import { defineApp } from "@velqu/core";\nexport const app = defineApp({ INVALID_SYNTAX\n`,
    );

    const proc = Bun.spawn(
      ["bun", "packages/cli/src/index.ts", "build", "--project", tempDir],
      {
        stdout: "pipe",
        stderr: "pipe",
        env: process.env,
      },
    );
    const exitCode = await proc.exited;
    expect(exitCode).toBe(ExitCode.GENERAL_ERROR);
  });

  it("exits 1 on unknown command or missing file", async () => {
    const proc = Bun.spawn(
      ["bun", "packages/cli/src/index.ts", "nonexistent-cmd"],
      {
        stdout: "pipe",
        stderr: "pipe",
        env: process.env,
      },
    );
    const exitCode = await proc.exited;
    expect(exitCode).toBe(ExitCode.GENERAL_ERROR);
  });

  it("exits 0 on help command or --help flag", async () => {
    const proc1 = Bun.spawn(["bun", "packages/cli/src/index.ts", "--help"], {
      stdout: "pipe",
      stderr: "pipe",
      env: process.env,
    });
    expect(await proc1.exited).toBe(ExitCode.SUCCESS);

    const proc2 = Bun.spawn(["bun", "packages/cli/src/index.ts", "help"], {
      stdout: "pipe",
      stderr: "pipe",
      env: process.env,
    });
    expect(await proc2.exited).toBe(ExitCode.SUCCESS);
  });
});
