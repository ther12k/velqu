import { describe, it, expect, beforeEach, afterEach } from "bun:test";
import { DevServer } from "./dev-server";
import { mkdirSync, writeFileSync, rmSync, existsSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";

describe("DevServer & Worker Swap Pipeline (M4A-001-C)", () => {
  let tempDir: string;

  beforeEach(() => {
    tempDir = join(tmpdir(), `velqu-dev-test-${Date.now()}-${Math.random().toString(36).slice(2)}`);
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
      `import { defineApp, defineModule } from "@velqu/core";\n` +
        `import { helloRoute } from "./routes";\n` +
        `export const app = defineApp({\n` +
        `  id: "devapp",\n` +
        `  modules: [defineModule({ id: "main", routes: [helloRoute] })],\n` +
        `});\n` +
        `export default app;\n`,
    );
    writeFileSync(
      join(tempDir, "src", "routes.ts"),
      `import { route } from "@velqu/core";\n` +
        `import { s } from "@velqu/schema";\n` +
        `export const helloRoute = route({\n` +
        `  id: "hello.get",\n` +
        `  method: "GET",\n` +
        `  path: "/hello",\n` +
        `  response: { 200: s.object({ version: s.integer(), msg: s.string() }) },\n` +
        `  handle: async () => ({ version: 1, msg: "hello-v1" }),\n` +
        `});\n`,
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

  it("starts dev server and proxies requests to QuickJS worker generation 1", async () => {
    const server = new DevServer({
      project: tempDir,
    });

    const { port, generation } = await server.start();
    expect(port).toBeGreaterThan(0);
    expect(generation).toBe(1);
    expect(server.isHealthy()).toBeTrue();

    const res = await fetch(`http://127.0.0.1:${port}/hello`);
    expect(res.status).toBe(200);
    const body = (await res.json()) as { version: number; msg: string };
    expect(body.version).toBe(1);
    expect(body.msg).toBe("hello-v1");

    await server.stop();
    expect(server.isHealthy()).toBeFalse();
  });

  it("loads candidate worker and verifies readiness before switching traffic on reload", async () => {
    const server = new DevServer({
      project: tempDir,
    });

    const { port } = await server.start();

    // Verify initial generation 1 response:
    const res1 = await fetch(`http://127.0.0.1:${port}/hello`);
    const body1 = (await res1.json()) as { version: number };
    expect(body1.version).toBe(1);

    // Update code to generation 2:
    writeFileSync(
      join(tempDir, "src", "routes.ts"),
      `import { route } from "@velqu/core";\n` +
        `import { s } from "@velqu/schema";\n` +
        `export const helloRoute = route({\n` +
        `  id: "hello.get",\n` +
        `  method: "GET",\n` +
        `  path: "/hello",\n` +
        `  response: { 200: s.object({ version: s.integer(), msg: s.string() }) },\n` +
        `  handle: async () => ({ version: 2, msg: "hello-v2" }),\n` +
        `});\n`,
    );

    const reloadRes = await server.reload();
    expect(reloadRes.success).toBeTrue();
    expect(reloadRes.switched).toBeTrue();
    expect(reloadRes.generation).toBe(2);
    expect(reloadRes.totalMs).toBeGreaterThanOrEqual(0);
    expect(reloadRes.totalMs).toBeLessThan(2000); // fast dev reload

    // Gateway now serves generation 2:
    const res2 = await fetch(`http://127.0.0.1:${port}/hello`);
    expect(res2.status).toBe(200);
    const body2 = (await res2.json()) as { version: number; msg: string };
    expect(body2.version).toBe(2);
    expect(body2.msg).toBe("hello-v2");

    await server.stop();
  });

  it("retains prior healthy worker when reload compilation fails (failed reload keeps prior app)", async () => {
    const server = new DevServer({
      project: tempDir,
    });

    const { port } = await server.start();
    expect(server.getGeneration()).toBe(1);

    // Write syntax error / broken file that fails extraction:
    writeFileSync(
      join(tempDir, "src", "routes.ts"),
      `import { route } from "@velqu/core";\n` +
        `export const broken = route({ INVALID_SYNTAX\n`,
    );

    const reloadRes = await server.reload();
    expect(reloadRes.success).toBeFalse();
    expect(reloadRes.switched).toBeFalse();
    expect(reloadRes.retainedPriorWorker).toBeTrue();
    expect(reloadRes.error).toBeDefined();

    // Generation remains 1:
    expect(server.getGeneration()).toBe(1);

    // Prior healthy worker continues serving requests without interruption:
    const res = await fetch(`http://127.0.0.1:${port}/hello`);
    expect(res.status).toBe(200);
    const body = (await res.json()) as { version: number; msg: string };
    expect(body.version).toBe(1);
    expect(body.msg).toBe("hello-v1");

    await server.stop();
  });

  it("drives proof fixture end-to-end through dev server gateway", async () => {
    const server = new DevServer({
      project: "examples/proof",
    });

    const { port } = await server.start();
    expect(port).toBeGreaterThan(0);

    // Call health.live on proof app:
    const res = await fetch(`http://127.0.0.1:${port}/health/live`);
    expect(res.status).toBe(200);
    const body = (await res.json()) as { status: string };
    expect(body.status).toBe("ok");

    await server.stop();
  });
});
