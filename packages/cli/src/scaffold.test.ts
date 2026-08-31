import { describe, it, expect, beforeEach, afterEach } from "bun:test";
import { generateStarterProject } from "./scaffold";
import { build, extractApp } from "@velqu/compiler";
import { ExitCode } from "./exit-codes";
import { mkdirSync, rmSync, existsSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";

describe("Starter API Scaffolding (M4A-003-A)", () => {
  let tempDir: string;

  beforeEach(() => {
    tempDir = join(tmpdir(), `velqu-scaffold-test-${Date.now()}-${Math.random().toString(36).slice(2)}`);
    mkdirSync(tempDir, { recursive: true });
  });

  afterEach(() => {
    if (existsSync(tempDir)) {
      rmSync(tempDir, { recursive: true, force: true });
    }
  });

  it("generates correct, complete starter project structure without credentials", () => {
    const files = generateStarterProject({ name: "my-service" });

    expect(files["package.json"]).toBeDefined();
    expect(files["tsconfig.json"]).toBeDefined();
    expect(files["README.md"]).toBeDefined();
    expect(files["src/app.ts"]).toBeDefined();
    expect(files["src/modules/health/routes.ts"]).toBeDefined();
    expect(files["src/modules/greetings/routes.ts"]).toBeDefined();
    expect(files["src/modules/greetings/service.ts"]).toBeDefined();

    const pkg = JSON.parse(files["package.json"]);
    expect(pkg.name).toBe("my-service");
    expect(pkg.dependencies["@velqu/core"]).toBeDefined();
    expect(pkg.dependencies["@velqu/schema"]).toBeDefined();

    // Verify no demo secrets or credentials in starter:
    const allContent = Object.values(files).join("\n");
    expect(allContent).not.toContain("password");
    expect(allContent).not.toContain("secret");
    expect(allContent).not.toContain("API_KEY");
  });

  it("statically compiles the generated starter project with clean extraction and parity", async () => {
    const files = generateStarterProject({ name: "starter-test" });
    for (const [relPath, content] of Object.entries(files)) {
      const fullPath = join(tempDir, relPath);
      mkdirSync(join(tempDir, relPath, ".."), { recursive: true });
      writeFileSync(fullPath, content);
    }

    // Symlink workspace packages:
    mkdirSync(join(tempDir, "node_modules", "@velqu"), { recursive: true });
    const { symlinkSync } = require("node:fs");
    try {
      symlinkSync(join(process.cwd(), "packages", "core"), join(tempDir, "node_modules", "@velqu", "core"), "dir");
      symlinkSync(join(process.cwd(), "packages", "schema"), join(tempDir, "node_modules", "@velqu", "schema"), "dir");
    } catch {}

    const app = extractApp(join(tempDir, "src", "app.ts"));
    expect(app.appId).toBe("starter-test");
    expect(app.routes.length).toBe(3); // health.live, greetings.get, greetings.create
    expect(app.modules).toEqual(["health", "greetings"]);

    // Full compilation:
    const buildRes = await build({
      project: tempDir,
      outDir: join(tempDir, "dist"),
    });

    expect(buildRes.routes).toBe(3);
    expect(existsSync(buildRes.packPath)).toBeTrue();
  });

  it("CLI init command scaffolds starter project directory", async () => {
    const target = join(tempDir, "scaffolded-app");

    const proc = Bun.spawn(
      ["bun", "packages/cli/src/index.ts", "init", target, "--name", "quick-starter"],
      {
        stdout: "pipe",
        stderr: "pipe",
        env: process.env,
      },
    );

    const stdout = await new Response(proc.stdout).text();
    const exitCode = await proc.exited;

    expect(exitCode).toBe(ExitCode.SUCCESS);
    expect(stdout).toContain("velqu init: created starter project 'quick-starter'");
    expect(existsSync(join(target, "src", "app.ts"))).toBeTrue();
    expect(existsSync(join(target, "package.json"))).toBeTrue();
  });

  it("CLI init --json outputs machine-readable project scaffolding receipt", async () => {
    const target = join(tempDir, "json-app");

    const proc = Bun.spawn(
      ["bun", "packages/cli/src/index.ts", "init", target, "--name", "json-service", "--json"],
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
    expect(parsed.command).toBe("init");
    expect(parsed.name).toBe("json-service");
    expect(parsed.filesCount).toBe(8);
  });
});
