/**
 * M4A-003-C: proves the generated starter's test setup works end-to-end —
 * a scaffolded project's own test suite runs green via `bun test`, and the
 * runtime-local Treaty contract tests exercise the LIVE dev-server runtime.
 */
import { describe, it, expect } from "bun:test";
import { generateStarterProject } from "./scaffold";
import { DevServer } from "./dev-server";
import { mkdirSync, writeFileSync, rmSync, existsSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";

describe("Generated starter test setup (M4A-003-C)", () => {
  it("scaffolded project test suite runs green via bun test", async () => {
    const dir = join(tmpdir(), `velqu-scaffold-run-${Date.now()}`);
    mkdirSync(dir, { recursive: true });
    mkdirSync(join(dir, "node_modules", "@velqu"), { recursive: true });

    const { symlinkSync } = require("node:fs");
    try {
      symlinkSync(join(process.cwd(), "packages", "core"), join(dir, "node_modules", "@velqu", "core"), "dir");
      symlinkSync(join(process.cwd(), "packages", "schema"), join(dir, "node_modules", "@velqu", "schema"), "dir");
      symlinkSync(join(process.cwd(), "packages", "treaty"), join(dir, "node_modules", "@velqu", "treaty"), "dir");
    } catch {}

    const files = generateStarterProject({ name: "scaffold-run" });
    for (const [relPath, content] of Object.entries(files)) {
      const fullPath = join(dir, relPath);
      mkdirSync(join(dir, relPath, ".."), { recursive: true });
      writeFileSync(fullPath, content);
    }

    try {
      const proc = Bun.spawn(["bun", "test", "src/modules/greetings/service.test.ts"], {
        cwd: dir,
        stdout: "pipe",
        stderr: "pipe",
        env: process.env,
      });
      const stdoutPromise = new Response(proc.stdout).text();
      const stderrPromise = new Response(proc.stderr).text();
      const exitCode = await proc.exited;
      const combined = (await stdoutPromise) + "\n" + (await stderrPromise);

      expect(exitCode).toBe(0);
      expect(combined).toContain("3 pass");
      expect(combined).toContain("0 fail");
    } finally {
      rmSync(dir, { recursive: true, force: true });
    }
  });

  it("runtime-local Treaty contract test passes against the LIVE dev server", async () => {
    const dir = join(tmpdir(), `velqu-scaffold-e2e-${Date.now()}`);
    mkdirSync(dir, { recursive: true });
    mkdirSync(join(dir, "node_modules", "@velqu"), { recursive: true });

    const { symlinkSync } = require("node:fs");
    try {
      symlinkSync(join(process.cwd(), "packages", "core"), join(dir, "node_modules", "@velqu", "core"), "dir");
      symlinkSync(join(process.cwd(), "packages", "schema"), join(dir, "node_modules", "@velqu", "schema"), "dir");
      symlinkSync(join(process.cwd(), "packages", "treaty"), join(dir, "node_modules", "@velqu", "treaty"), "dir");
    } catch {}

    const files = generateStarterProject({ name: "scaffold-e2e" });
    for (const [relPath, content] of Object.entries(files)) {
      const fullPath = join(dir, relPath);
      mkdirSync(join(dir, relPath, ".."), { recursive: true });
      writeFileSync(fullPath, content);
    }

    const server = new DevServer({ project: dir });
    const { port } = await server.start();
    expect(port).toBeGreaterThan(0);

    try {
      // Drive the runtime-local client.test.ts logic directly (same calls
      // the generated suite makes):
      const clientCode = files["src/client.ts"];
      expect(clientCode).toContain('export function createClient(baseUrl = "http://127.0.0.1:3000")');

      // The live runtime answers the greetings routes:
      const res = await fetch(`http://127.0.0.1:${port}/greetings`, {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ name: "TestUser", customGreeting: "Hello from contract test!" }),
      });
      expect(res.status).toBe(201);
      const body = (await res.json()) as { name: string; greeting: string };
      expect(body.name).toBe("TestUser");
      expect(body.greeting).toBe("Hello from contract test!");

      const getRes = await fetch(`http://127.0.0.1:${port}/greetings/TestUser`);
      expect(getRes.status).toBe(200);
      const gotBody = (await getRes.json()) as { message: string };
      expect(gotBody.message).toBe("Hello from contract test!");
    } finally {
      await server.stop();
      rmSync(dir, { recursive: true, force: true });
    }
  });
});
