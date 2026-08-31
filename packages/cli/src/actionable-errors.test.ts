import { describe, it, expect, beforeEach, afterEach } from "bun:test";
import { formatActionableError, renderCodeFrame } from "./errors";
import { CompileError } from "@velqu/compiler";
import { mkdirSync, writeFileSync, rmSync, existsSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";

describe("CLI Helpful Actionable Errors (M4A-002-D)", () => {
  let tempDir: string;

  beforeEach(() => {
    tempDir = join(tmpdir(), `velqu-err-test-${Date.now()}-${Math.random().toString(36).slice(2)}`);
    mkdirSync(tempDir, { recursive: true });
    mkdirSync(join(tempDir, "src"), { recursive: true });
    mkdirSync(join(tempDir, "node_modules", "@velqu"), { recursive: true });

    const { symlinkSync } = require("node:fs");
    try {
      symlinkSync(join(process.cwd(), "packages", "core"), join(tempDir, "node_modules", "@velqu", "core"), "dir");
      symlinkSync(join(process.cwd(), "packages", "schema"), join(tempDir, "node_modules", "@velqu", "schema"), "dir");
    } catch {}

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

  it("renders clean code frame with line number, context lines, and caret pointing to column", () => {
    const file = join(tempDir, "src", "sample.ts");
    writeFileSync(
      file,
      `import { defineApp } from "@velqu/core";\n` +
        `import { foo } from "node:fs";\n` +
        `export const app = defineApp({ id: "test" });\n`,
    );

    const frame = renderCodeFrame(file, 2, 8);
    expect(frame).not.toBeNull();
    expect(frame!).toContain("--> " + file + ":2:8");
    expect(frame!).toContain(`2 | import { foo } from "node:fs";`);
    expect(frame!).toContain("^"); // caret indicator
  });

  it("formats compile errors with actionable diagnostics, code frames, and hints", () => {
    const file = join(tempDir, "src", "sample.ts");
    writeFileSync(
      file,
      `import { defineApp } from "@velqu/core";\n` +
        `import { foo } from "node:fs";\n` +
        `export const app = defineApp({ id: "test" });\n`,
    );

    const err = new CompileError(
      "unsupported import 'node:fs'",
      { file, line: 2, column: 8 },
      "Velqu apps run on QuickJS with no Node/Bun APIs (ADR-0003)",
    );

    const diag = formatActionableError(err, "compile-error");
    expect(diag.title).toBe("compile-error");
    expect(diag.message).toBe("unsupported import 'node:fs'");
    expect(diag.location?.file).toBe(file);
    expect(diag.hint).toContain("ADR-0003");
    expect(diag.raw).toContain("[velqu:compile-error]");
    expect(diag.raw).toContain("node:fs");
    expect(diag.raw).toContain("hint: Velqu apps run on QuickJS");
  });

  it("CLI build surfaces actionable error frame on unsupported import", async () => {
    writeFileSync(
      join(tempDir, "src", "app.ts"),
      `import { defineApp } from "@velqu/core";\n` +
        `import * as fs from "node:fs";\n` +
        `export const app = defineApp({ id: "bad", modules: [] });\n`,
    );

    const proc = Bun.spawn(
      ["bun", "packages/cli/src/index.ts", "build", "--project", tempDir],
      {
        stdout: "pipe",
        stderr: "pipe",
        env: process.env,
      },
    );

    const stderr = await new Response(proc.stderr).text();
    const exitCode = await proc.exited;

    expect(exitCode).toBe(1);
    expect(stderr).toContain("[velqu:build-error]");
    expect(stderr).toContain("unsupported import 'node:fs'");
    expect(stderr).toContain("app.ts:1:");
    expect(stderr).toContain("hint: Velqu apps run on QuickJS");
  });

  it("CLI check surfaces actionable error frame on invalid route structure", async () => {
    writeFileSync(
      join(tempDir, "src", "app.ts"),
      `import { defineApp, route } from "@velqu/core";\n` +
        `export const bad = route({ method: "INVALID_METHOD", path: "/x", handle: async () => ({}) });\n`,
    );

    const proc = Bun.spawn(
      ["bun", "packages/cli/src/index.ts", "check", "--project", tempDir],
      {
        stdout: "pipe",
        stderr: "pipe",
        env: process.env,
      },
    );

    const stderr = await new Response(proc.stderr).text();
    const exitCode = await proc.exited;

    expect(exitCode).toBe(1);
    expect(stderr).toContain("[velqu:check-error]");
  });
});
