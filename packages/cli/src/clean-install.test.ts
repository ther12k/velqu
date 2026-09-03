/**
 * Clean Install Packet Verification (M4A-010-A).
 *
 * Verifies that invited alpha developers can:
 * 1. Initialize a new starter project with `velqu init`.
 * 2. Run static validation with `velqu check`.
 * 3. Execute unit tests with `bun test`.
 * 4. Compile the production QPack bundle with `velqu build`.
 * 5. Verify the compiled bundle structure, route count, and manifest artifacts.
 */

import { describe, it, expect, beforeEach, afterEach } from "bun:test";
import { mkdirSync, rmSync, existsSync, writeFileSync, symlinkSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";

describe("Clean install packet verification (M4A-010-A)", () => {
  let testDir: string;
  const worktreeDir = process.cwd();

  beforeEach(() => {
    testDir = join(tmpdir(), `velqu-clean-install-${Date.now()}-${Math.random().toString(36).slice(2)}`);
    mkdirSync(testDir, { recursive: true });
  });

  afterEach(() => {
    if (existsSync(testDir)) {
      rmSync(testDir, { recursive: true, force: true });
    }
  });

  it("completes the full developer workflow (init -> check -> test -> build) on a clean project", async () => {
    const appDir = join(testDir, "starter-app");

    // 1. velqu init
    const initProc = Bun.spawn(
      ["bun", join(worktreeDir, "packages/cli/src/index.ts"), "init", appDir, "--name", "starter-app"],
      { stdout: "pipe", stderr: "pipe", env: process.env },
    );
    const initCode = await initProc.exited;
    expect(initCode).toBe(0);
    expect(existsSync(join(appDir, "package.json"))).toBe(true);
    expect(existsSync(join(appDir, "src/app.ts"))).toBe(true);

    // 2. Set up workspace package resolution (private alpha requirement)
    mkdirSync(join(appDir, "node_modules/@velqu"), { recursive: true });
    for (const pkg of ["core", "compiler", "schema", "treaty", "contract", "cli", "testing"]) {
      const pkgPath = join(worktreeDir, "packages", pkg);
      if (existsSync(pkgPath)) {
        symlinkSync(pkgPath, join(appDir, "node_modules/@velqu", pkg), "dir");
      }
    }
    // Link pinned typescript from monorepo dependencies
    symlinkSync(join(worktreeDir, "node_modules/typescript"), join(appDir, "node_modules/typescript"), "dir");

    // 3. velqu check
    const checkProc = Bun.spawn(
      ["bun", join(worktreeDir, "packages/cli/src/index.ts"), "check", "--project", appDir],
      { stdout: "pipe", stderr: "pipe", env: process.env },
    );
    const checkOut = await new Response(checkProc.stdout).text();
    const checkCode = await checkProc.exited;
    expect(checkCode).toBe(0);
    expect(checkOut).toContain("velqu check: 3 routes");

    // 4. bun test
    const testProc = Bun.spawn(
      ["bun", "test"],
      { cwd: appDir, stdout: "pipe", stderr: "pipe", env: process.env },
    );
    const testCode = await testProc.exited;
    expect(testCode).toBe(0);

    // 5. velqu build
    const buildProc = Bun.spawn(
      ["bun", join(worktreeDir, "packages/cli/src/index.ts"), "build", "--project", appDir, "--out", join(appDir, "dist")],
      { stdout: "pipe", stderr: "pipe", env: process.env },
    );
    const buildOut = await new Response(buildProc.stdout).text();
    const buildCode = await buildProc.exited;
    expect(buildCode).toBe(0);
    expect(buildOut).toContain("velqu build [serverless]: 3 routes");

    // 6. Verify generated bundle artifacts
    const distDir = join(appDir, "dist");
    expect(existsSync(join(distDir, "app.qpack"))).toBe(true);
    expect(existsSync(join(distDir, "contract.json"))).toBe(true);
    expect(existsSync(join(distDir, "contract.d.ts"))).toBe(true);
    expect(existsSync(join(distDir, "openapi.json"))).toBe(true);
    expect(existsSync(join(distDir, "published-manifest.json"))).toBe(true);

    const pack = JSON.parse(readFileSync(join(distDir, "app.qpack"), "utf8"));
    expect(pack.appId).toBe("starter-app");
    expect(pack.routes.length).toBe(3);
    expect(pack.runtimeAbi).toBe(1);
  });
});
