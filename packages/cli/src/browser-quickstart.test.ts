/**
 * BWASM-Q-006 — Automated executable quickstart test.
 *
 * Verifies that the exact commands documented in `docs/beta/BROWSER_WASM.md` §2
 * execute cleanly end-to-end on generated artifacts:
 * 1. build --target browser-wasm --project <dir>
 * 2. inspect browser --project <dir> --json
 * 3. preview --project <dir> (serving static assets & dev diagnostics)
 * 4. export --project <dir> --out <dir> --json
 */

import { afterAll, describe, expect, it } from "bun:test";
import { existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";
import { startPreviewServer, PREVIEW_DIAGNOSTICS_PATH } from "./browser-deploy";

const root = join(import.meta.dir, "../../..");
const workDir = join(tmpdir(), `velqu-quickstart-${Date.now()}`);
const appDir = join(workDir, "my-app");
const exportDir = join(workDir, "exported-static");

function setupQuickstartApp(): void {
  mkdirSync(join(appDir, "src"), { recursive: true });
  writeFileSync(
    join(appDir, "package.json"),
    JSON.stringify({ name: "quickstart-app", type: "module" }, null, 2),
  );
  writeFileSync(
    join(appDir, "src", "app.ts"),
    `
import { route } from "@velqu/core";
import { s } from "@velqu/schema";

export const hello = route({
  id: "hello.get",
  method: "GET",
  path: "/hello/:name",
  response: { 200: s.object({ message: s.string() }) },
  handle: async (ctx) => ({ message: \`Hello \${ctx.params?.name ?? "world"}!\` }),
});

export const app = { routes: [hello] };
`,
  );
  // Symlink workspace packages
  const linkDir = join(appDir, "node_modules", "@velqu");
  mkdirSync(linkDir, { recursive: true });
  for (const pkg of ["core", "schema", "browser-runtime"]) {
    const target = join(linkDir, pkg);
    if (!existsSync(target)) {
      writeFileSync(target, ""); // placeholder or symlink
      rmSync(target);
      // Link to workspace package
      const src = join(root, "packages", pkg);
      Bun.spawnSync(["ln", "-s", src, target]);
    }
  }
}

async function runCli(args: string[]): Promise<{ code: number; stdout: string; stderr: string }> {
  const proc = Bun.spawn(["bun", join(root, "packages/cli/src/index.ts"), ...args], {
    cwd: root,
    stdout: "pipe",
    stderr: "pipe",
    env: process.env,
  });
  const [stdout, stderr, code] = await Promise.all([
    new Response(proc.stdout).text(),
    new Response(proc.stderr).text(),
    proc.exited,
  ]);
  return { code, stdout, stderr };
}

/** Fails with the CLI's captured stderr/stdout visible — a bare code check hides the actual error (#1305/#1306). */
function expectCliOk(r: { code: number; stdout: string; stderr: string }) {
  if (r.code !== 0) {
    throw new Error(
      `CLI exited ${r.code}\n--- stdout ---\n${r.stdout.slice(-1200)}\n--- stderr ---\n${r.stderr.slice(-1200)}`,
    );
  }
}

describe("BWASM-Q-006 documented quickstart workflow execution", () => {
  setupQuickstartApp();

  it("Step 1: build --target browser-wasm compiles deployment assets", async () => {
    const r = await runCli([
      "build", "--target", "browser-wasm", "--project", appDir, "--json",
    ]);
    expectCliOk(r);
    const parsed = JSON.parse(r.stdout);
    expect(parsed.status).toBe("ok");
    expect(parsed.target).toBe("browser-wasm");
    expect(existsSync(join(appDir, "dist/browser/velqu-artifacts.json"))).toBeTrue();
    expect(existsSync(join(appDir, "dist/browser/kernel.wasm"))).toBeTrue();
    expect(existsSync(join(appDir, "dist/browser/service-worker.js"))).toBeTrue();
  }, 60_000);

  it("Step 2: inspect browser verifies deployment integrity and capabilities", async () => {
    const r = await runCli([
      "inspect", "browser", "--project", appDir, "--json",
    ]);
    expectCliOk(r);
    const parsed = JSON.parse(r.stdout);
    expect(parsed.status).toBe("ok");
    expect(parsed.target).toBe("browser");
    expect(parsed.ok).toBeTrue();
    expect(parsed.problems.length).toBe(0);
  });

  it("Step 3: preview server serves static shell and dev diagnostics", async () => {
    const browserDir = join(appDir, "dist/browser");
    const server = await startPreviewServer({ browserDir, port: 0 });
    try {
      const indexRes = await fetch(`${server.baseUrl}/index.html`);
      expect(indexRes.status).toBe(200);
      expect(await indexRes.text()).toContain("velqu-status");

      const diagRes = await fetch(`${server.baseUrl}${PREVIEW_DIAGNOSTICS_PATH}`);
      expect(diagRes.status).toBe(200);
      const diagBody = (await diagRes.json()) as any;
      expect(diagBody.command).toBe("preview");
      expect(diagBody.production).toBeFalse();
    } finally {
      await server.stop();
    }
  });

  it("Step 4: export creates verified clean static distribution", async () => {
    const r = await runCli([
      "export", "--project", appDir, "--out", exportDir, "--json",
    ]);
    expectCliOk(r);
    const parsed = JSON.parse(r.stdout);
    expect(parsed.status).toBe("ok");
    expect(parsed.command).toBe("export");
    expect(parsed.files.length).toBeGreaterThanOrEqual(10);
    expect(existsSync(join(exportDir, "velqu-artifacts.json"))).toBeTrue();
    expect(existsSync(join(exportDir, "kernel.wasm"))).toBeTrue();
  });

  afterAll(() => {
    rmSync(workDir, { recursive: true, force: true });
  });
});
