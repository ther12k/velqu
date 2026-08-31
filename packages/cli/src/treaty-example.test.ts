import { describe, it, expect, beforeEach, afterEach } from "bun:test";
import { generateStarterProject } from "./scaffold";
import { treaty } from "@velqu/treaty";
import { build, extractApp } from "@velqu/compiler";
import { DevServer } from "./dev-server";
import { mkdirSync, writeFileSync, rmSync, existsSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";

describe("Treaty Client Example Scaffolding (M4A-003-B)", () => {
  let tempDir: string;

  beforeEach(() => {
    tempDir = join(tmpdir(), `velqu-treaty-scaffold-test-${Date.now()}-${Math.random().toString(36).slice(2)}`);
    mkdirSync(tempDir, { recursive: true });
  });

  afterEach(() => {
    if (existsSync(tempDir)) {
      rmSync(tempDir, { recursive: true, force: true });
    }
  });

  it("generates src/client.ts with type-safe Treaty client definition", () => {
    const files = generateStarterProject({ name: "client-test" });

    expect(files["src/client.ts"]).toBeDefined();
    const clientCode = files["src/client.ts"];

    expect(clientCode).toContain('import { treaty, type TreatyClient } from "@velqu/treaty";');
    expect(clientCode).toContain("export type StarterApi = {");
    expect(clientCode).toContain('"health.live"');
    expect(clientCode).toContain('"greetings.get"');
    expect(clientCode).toContain('"greetings.create"');
    expect(clientCode).toContain("export const contract");
    expect(clientCode).toContain("export function createClient");

    const pkg = JSON.parse(files["package.json"]);
    expect(pkg.dependencies["@velqu/treaty"]).toBeDefined();
    expect(pkg.scripts["client"]).toBe("bun run src/client.ts");
  });

  it("drives live DevServer with Treaty client using dot-navigation and exact method typing", async () => {
    const files = generateStarterProject({ name: "e2e-treaty-test" });
    for (const [relPath, content] of Object.entries(files)) {
      const fullPath = join(tempDir, relPath);
      mkdirSync(join(tempDir, relPath, ".."), { recursive: true });
      writeFileSync(fullPath, content);
    }

    // Link packages:
    mkdirSync(join(tempDir, "node_modules", "@velqu"), { recursive: true });
    const { symlinkSync } = require("node:fs");
    try {
      symlinkSync(join(process.cwd(), "packages", "core"), join(tempDir, "node_modules", "@velqu", "core"), "dir");
      symlinkSync(join(process.cwd(), "packages", "schema"), join(tempDir, "node_modules", "@velqu", "schema"), "dir");
      symlinkSync(join(process.cwd(), "packages", "treaty"), join(tempDir, "node_modules", "@velqu", "treaty"), "dir");
    } catch {}

    const server = new DevServer({
      project: tempDir,
    });

    const { port } = await server.start();
    expect(port).toBeGreaterThan(0);

    // Instantiate Treaty client pointing at the live dev server:
    // (type alias — interfaces lack implicit index signatures for Record<>)
    type StarterApi = {
      "health.live": {
        path: "/health/live";
        method: "GET";
        params: never;
        query: never;
        body: never;
        headers: never;
        responses: { 200: { status: string } };
      };
      "greetings.get": {
        path: "/greetings/:name";
        method: "GET";
        params: { name: string };
        query: never;
        body: never;
        headers: never;
        responses: { 200: { message: string } };
      };
      "greetings.create": {
        path: "/greetings";
        method: "POST";
        params: never;
        query: never;
        body: { name: string; customGreeting?: string };
        headers: never;
        responses: { 201: { name: string; greeting: string } };
      };
    }

    const contract = {
      "health.live": { path: "/health/live", method: "GET" },
      "greetings.get": { path: "/greetings/:name", method: "GET" },
      "greetings.create": { path: "/greetings", method: "POST" },
    } as const;

    const api = treaty<StarterApi>({
      baseUrl: `http://127.0.0.1:${port}`,
      contract,
    });

    // 1. Call GET /health/live
    const health = await api.health.live.get();
    expect(health.error).toBeNull();
    expect(health.data).toBeDefined();
    expect(health.data?.status).toBe("ok");

    // 2. Call POST /greetings
    const created = await api.greetings.create.post({
      name: "Grace",
      customGreeting: "Welcome to Velqu!",
    });
    expect(created.error).toBeNull();
    expect(created.data?.name).toBe("Grace");
    expect(created.data?.greeting).toBe("Welcome to Velqu!");

    // 3. Call GET /greetings/:name via apply-then-method form (params substituted)
    const greeting = await api.greetings.get({ name: "Grace" }).get();
    expect(greeting.error).toBeNull();
    expect(greeting.data?.message).toBe("Welcome to Velqu!");

    await server.stop();
  });
});
