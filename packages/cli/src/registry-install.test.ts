/**
 * Opt-in registry E2E (#1398): proves the starter's PUBLIC dependency path
 * with no monorepo links at all —
 *
 *   scaffold (local CLI) -> bun install (@velqu/* from npm)
 *   ->  bun run check  ->  bun test  ->  bun run build      [registry CLI]
 *
 * The scaffold step runs the WORKTREE CLI because the published
 * 0.1.0-beta.1 predates this packet (it still emits `workspace:*`);
 * every later step runs the registry-installed `velqu` bin that a real
 * consumer gets. When a post-fix version is published, add the
 * create-via-published-bin assertion back (it is what this suite
 * originally asserted).
 *
 * Requires network access to registry.npmjs.org and the pinned toolchain
 * (Bun 1.4.0 — `velqu build` refuses other versions by design), so it is
 * deliberately NOT part of the offline verify battery. Run it with:
 *
 *   VELQU_REGISTRY_E2E=1 bun test packages/cli/src/registry-install.test.ts
 *
 * A transcript of a real run is attached to the PR that introduced this
 * file; when skipped, this suite prints a pointer to that command.
 */

import { describe, it, expect, beforeEach, afterEach } from "bun:test";
import { mkdirSync, rmSync, existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";
import { createServer } from "node:net";
import { PUBLISHED_BETA_VERSION } from "./scaffold";

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

const enabled = process.env.VELQU_REGISTRY_E2E === "1";

describe.skipIf(!enabled)("Registry E2E: starter installs from npm (opt-in, #1398)", () => {
  let testDir: string;
  const worktreeDir = process.cwd();

  beforeEach(() => {
    testDir = join(tmpdir(), `velqu-registry-e2e-${Date.now()}-${Math.random().toString(36).slice(2)}`);
    mkdirSync(testDir, { recursive: true });
  });

  afterEach(() => {
    if (existsSync(testDir)) {
      rmSync(testDir, { recursive: true, force: true });
    }
  });

  it("installs the CLI from npm, scaffolds, installs, tests, checks, and builds", async () => {
    const hostDir = join(testDir, "host");
    const appDir = join(testDir, "app");
    mkdirSync(hostDir, { recursive: true });

    // 1. The published CLI installs from the public registry with its bin
    // intact (consumers without a checkout start here).
    const addProc = Bun.spawn(["bun", "add", `@velqu/cli@${PUBLISHED_BETA_VERSION}`], {
      cwd: hostDir,
      stdout: "pipe",
      stderr: "pipe",
      env: process.env,
    });
    const addCode = await addProc.exited;
    expect(addCode).toBe(0);
    expect(existsSync(join(hostDir, "node_modules/.bin/velqu"))).toBe(true);

    // 2. Scaffold via the worktree CLI (see file header: the published
    // 0.1.0-beta.1 predates this packet).
    const createProc = Bun.spawn(
      ["bun", join(worktreeDir, "packages/cli/src/index.ts"), "create", appDir, "--name", "registry-app"],
      {
        cwd: hostDir,
        stdout: "pipe",
        stderr: "pipe",
        env: process.env,
      },
    );
    const createOut = await new Response(createProc.stdout).text();
    const createCode = await createProc.exited;
    expect(createCode).toBe(0);
    expect(createOut).toContain("created starter project 'registry-app'");

    // 3. The scaffold pins registry versions (no workspace:* anywhere).
    const pkg = JSON.parse(readFileSync(join(appDir, "package.json"), "utf8"));
    expect(pkg.dependencies["@velqu/core"]).toBe(PUBLISHED_BETA_VERSION);
    expect(JSON.stringify(pkg)).not.toContain("workspace:*");

    // 4. Real registry install of the scaffolded app.
    const installProc = Bun.spawn(["bun", "install"], {
      cwd: appDir,
      stdout: "pipe",
      stderr: "pipe",
      env: process.env,
    });
    const installCode = await installProc.exited;
    expect(installCode).toBe(0);
    expect(existsSync(join(appDir, "node_modules/@velqu/core"))).toBe(true);
    expect(existsSync(join(appDir, "node_modules/.bin/velqu"))).toBe(true);

    // 5. Static check through the scaffold's script.
    const checkProc = Bun.spawn(["bun", "run", "check"], {
      cwd: appDir,
      stdout: "pipe",
      stderr: "pipe",
      env: process.env,
    });
    const checkOut = await new Response(checkProc.stdout).text();
    expect(await checkProc.exited).toBe(0);
    expect(checkOut).toContain("velqu check: 3 routes");

    // 6. Tests: deterministic skip on an ephemeral port.
    const devPort = String(await freePort());
    const testProc = Bun.spawn(["bun", "test"], {
      cwd: appDir,
      stdout: "pipe",
      stderr: "pipe",
      env: { ...process.env, VELQU_DEV_PORT: devPort },
    });
    expect(await testProc.exited).toBe(0);

    // 7. Production build through the scaffold's script.
    const buildProc = Bun.spawn(["bun", "run", "build"], {
      cwd: appDir,
      stdout: "pipe",
      stderr: "pipe",
      env: process.env,
    });
    const buildOut = await new Response(buildProc.stdout).text();
    expect(await buildProc.exited).toBe(0);
    expect(buildOut).toContain("velqu build [serverless]: 3 routes");
    expect(existsSync(join(appDir, "dist/app.qpack"))).toBe(true);
    expect(existsSync(join(appDir, "dist/contract.d.ts"))).toBe(true);
  }, 240_000);
});

describe.skipIf(enabled)("Registry E2E (skipped)", () => {
  it("points at the opt-in command (network test, not part of verify)", () => {
    console.log(
      "registry E2E not enabled — run with: VELQU_REGISTRY_E2E=1 bun test packages/cli/src/registry-install.test.ts",
    );
    expect(true).toBe(true);
  });
});
