import { describe, it, expect, beforeEach, afterEach } from "bun:test";
import { generateStarterProject, PUBLISHED_BETA_VERSION } from "./scaffold";
import { build, extractApp } from "@velqu/compiler";
import { ExitCode } from "./exit-codes";
import { mkdirSync, rmSync, existsSync, writeFileSync, readFileSync } from "node:fs";
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
    expect(pkg.dependencies["@velqu/core"]).toBe(PUBLISHED_BETA_VERSION);
    expect(pkg.dependencies["@velqu/schema"]).toBe(PUBLISHED_BETA_VERSION);
    expect(pkg.dependencies["@velqu/treaty"]).toBe(PUBLISHED_BETA_VERSION);
    expect(pkg.dependencies["@velqu/cli"]).toBeUndefined(); // the CLI is not a runtime dependency
    expect(pkg.devDependencies["@velqu/cli"]).toBe(PUBLISHED_BETA_VERSION); // scripts use its bin
    expect(pkg.devDependencies.typescript).toBe("5.9.3"); // exact toolchain pin
    expect(pkg.engines.bun).toBe("1.4.0");
    expect(pkg.scripts.check).toBe("velqu check --project .");
    expect(pkg.scripts.build).toBe("velqu build --project .");
    expect(pkg.scripts.dev).toBe("velqu dev --project .");

    // Verify no demo secrets or credentials in starter:
    const allContent = Object.values(files).join("\n");
    expect(allContent).not.toContain("password");
    expect(allContent).not.toContain("secret");
    expect(allContent).not.toContain("API_KEY");
  });

  it("README discloses the published-registry dependency posture (OD-010, #1398)", () => {
    const files = generateStarterProject({ name: "docs-disclosure" });

    const readme = files["README.md"];
    expect(readme).toContain("## Dependencies (public beta)");
    expect(readme).toContain(PUBLISHED_BETA_VERSION);
    expect(readme).toContain("bun install");
    expect(readme).toContain("VELQU_RUNTIME");
    expect(readme).not.toContain("workspace:*");
    expect(readme).not.toContain("not yet published");
  });

  it("PUBLISHED_BETA_VERSION matches the shipped package manifests (no scaffold drift)", () => {
    // Resolved from this test file (packages/cli/src), not process.cwd():
    // bun test runs from the repository root, and the monorepo root
    // manifest carries a different version than the packages.
    const versionOf = (rel: string) =>
      JSON.parse(readFileSync(join(import.meta.dir, rel), "utf8")).version as string;
    expect(PUBLISHED_BETA_VERSION).toBe(versionOf(join("..", "package.json")));
    for (const pkg of ["core", "schema", "treaty"]) {
      expect(PUBLISHED_BETA_VERSION).toBe(versionOf(join("..", "..", pkg, "package.json")));
    }
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

  it("--with-fetch adds the upstream route, test, and capability metadata", () => {
    const files = generateStarterProject({ name: "fetch-service", withFetch: true });
    const pkg = JSON.parse(files["package.json"]);
    expect(pkg.velqu.capabilities).toEqual(["fetch"]);
    expect(files["src/modules/upstream/routes.ts"]).toContain('id: "upstream.quote"');
    expect(files["src/modules/upstream/routes.test.ts"]).toContain("upstream module");
    expect(files["README.md"]).toContain("Outbound Fetch Capability");
    expect(files["README.md"]).toContain("enabled");
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
    expect(parsed.filesCount).toBe(10);
  });
});
