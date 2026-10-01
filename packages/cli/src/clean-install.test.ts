/**
 * Clean Install Packet Verification (M4A-010-A, revised for the
 * published-beta posture — #1398).
 *
 * Verifies that a developer can take a scaffolded project and, using the
 * `velqu` bin exactly the way the scaffold's `bun run` scripts do:
 *   init -> check -> test -> build
 * `@velqu/*` resolution is simulated offline by symlinking the workspace
 * packages (plus the node_modules/.bin/velqu link a registry install
 * creates); the real public-registry path is exercised end-to-end by
 * registry-install.test.ts under VELQU_REGISTRY_E2E=1.
 */

import { describe, it, expect, beforeEach, afterEach } from "bun:test";
import { mkdirSync, rmSync, existsSync, symlinkSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";
import { createServer } from "node:net";

function freePort(): Promise<number> {
  return new Promise((resolve, reject) => {
    const srv = createServer();
    srv.listen(0, "127.0.0.1", () => {
      const addr = srv.address();
      const port = typeof addr === "object" && addr !== null ? addr.port : 0;
      srv.close(() => resolve(port));
    });
    srv.on("error", reject);
  });
}

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

  it("completes the developer workflow (init -> check -> test -> build) through the velqu bin", async () => {
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

    // 2. Offline @velqu/* resolution: symlink the workspace packages the
    // way a registry install materializes them, plus the bin link that
    // makes the scaffold's `velqu …` scripts executable via `bun run`.
    mkdirSync(join(appDir, "node_modules/@velqu"), { recursive: true });
    for (const pkg of ["core", "compiler", "schema", "treaty", "contract", "cli", "testing", "browser-runtime"]) {
      const pkgPath = join(worktreeDir, "packages", pkg);
      if (existsSync(pkgPath)) {
        symlinkSync(pkgPath, join(appDir, "node_modules/@velqu", pkg), "dir");
      }
    }
    symlinkSync(join(worktreeDir, "node_modules/typescript"), join(appDir, "node_modules/typescript"), "dir");
    mkdirSync(join(appDir, "node_modules/.bin"), { recursive: true });
    symlinkSync(join(worktreeDir, "packages/cli/bin/velqu.js"), join(appDir, "node_modules/.bin/velqu"));

    // The bin imports the built dist output; regenerate it if absent
    // (dist/ is gitignored — scripts/build-packages.ts is the source).
    if (!existsSync(join(worktreeDir, "packages/cli/dist/index.js"))) {
      const buildPkgs = Bun.spawn(["bun", "scripts/build-packages.ts"], {
        cwd: worktreeDir,
        stdout: "pipe",
        stderr: "pipe",
        env: process.env,
      });
      const buildPkgsCode = await buildPkgs.exited;
      expect(buildPkgsCode).toBe(0);
    }

    // 3. velqu check (through the bin, exactly as `bun run check` does)
    const checkProc = Bun.spawn(["bun", "run", "check"], {
      cwd: appDir,
      stdout: "pipe",
      stderr: "pipe",
      env: process.env,
    });
    const checkOut = await new Response(checkProc.stdout).text();
    const checkCode = await checkProc.exited;
    expect(checkCode).toBe(0);
    expect(checkOut).toContain("velqu check: 3 routes");

    // 4. bun test — deterministic: point the runtime-local client tests at
    // an ephemeral free port (nothing listens there, so they exercise the
    // documented status-0 skip path instead of colliding with whatever
    // occupies 3000 on the host).
    const devPort = String(await freePort());
    const testEnv = { ...process.env, VELQU_DEV_PORT: devPort };
    const testProc = Bun.spawn(["bun", "test"], {
      cwd: appDir,
      stdout: "pipe",
      stderr: "pipe",
      env: testEnv,
    });
    const testOut = await new Response(testProc.stdout).text();
    const testErr = await new Response(testProc.stderr).text();
    const testCode = await testProc.exited;
    expect(testCode).toBe(0);
    // bun routes test console output to stderr.
    expect(testOut + testErr).toContain("skipping: no dev server on 127.0.0.1:" + devPort);

    // 5. velqu build (through the bin)
    const buildProc = Bun.spawn(["bun", "run", "build"], {
      cwd: appDir,
      stdout: "pipe",
      stderr: "pipe",
      env: process.env,
    });
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
  }, 120_000);
});
