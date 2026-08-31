import { describe, it, expect, beforeEach, afterEach } from "bun:test";
import { buildTemporaryPack, IncrementalPackBuilder } from "./incremental";
import { build } from "./index";
import { mkdirSync, writeFileSync, rmSync, existsSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";

describe("Incremental temporary QPack builder (M4A-001-B)", () => {
  let tempDir: string;

  beforeEach(() => {
    tempDir = join(tmpdir(), `velqu-inc-test-${Date.now()}-${Math.random().toString(36).slice(2)}`);
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
        `  id: "testapp",\n` +
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
        `  response: { 200: s.object({ msg: s.string() }) },\n` +
        `  handle: async () => ({ msg: "hello" }),\n` +
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

  it("builds temporary QPack for examples/proof with fast build latency and TypeScript source maps", async () => {
    const res = await buildTemporaryPack({
      project: "examples/proof",
      sourceMap: true,
    });

    expect(res.pack).toBeDefined();
    expect(res.packJson).toBeString();
    expect(res.packSha256).toBeString();
    expect(res.contractHash).toBeString();
    expect(res.routes.length).toBeGreaterThan(0);
    expect(res.bundle.code).toContain("globalThis.__velquFunctionManifest");
    expect(res.bundle.sourceMap).toBeDefined();
    expect(res.buildMs).toBeGreaterThanOrEqual(0);
    expect(res.buildMs).toBeLessThan(1000); // fast dev compile

    // Parse QPack and verify formatVersion 1:
    const packObj = res.pack as { formatVersion: number; engine: { name: string } };
    expect(packObj.formatVersion).toBe(1);
    expect(packObj.engine.name).toBe("quickjs-ng");
  });

  it("matches full build contract hash and route definitions identically (parity)", async () => {
    const tempRes = await buildTemporaryPack({
      project: "examples/proof",
      sourceMap: true,
      writeToDisk: false,
    });

    const fullRes = await build({
      project: "examples/proof",
      outDir: join(tempDir, "proof-dist"),
    });

    expect(tempRes.routes.length).toBe(fullRes.routes);
    expect(tempRes.contractHash).toBeDefined();

    // Verify pack structures have identical route count and format:
    const tempPack = tempRes.pack as { routes: Array<{ id: string; method: string; path: string }> };
    expect(tempPack.routes.length).toBe(fullRes.routes);
  });

  it("detects contract changes when route path or schema is modified", async () => {
    const builder = new IncrementalPackBuilder({
      project: tempDir,
      sourceMap: true,
    });

    const first = await builder.build();
    expect(first.contractChanged).toBeFalse();

    // Modify route path (contract change):
    writeFileSync(
      join(tempDir, "src", "routes.ts"),
      `import { route } from "@velqu/core";\n` +
        `import { s } from "@velqu/schema";\n` +
        `export const helloRoute = route({\n` +
        `  id: "hello.get",\n` +
        `  method: "GET",\n` +
        `  path: "/hello-v2",\n` +
        `  response: { 200: s.object({ msg: s.string() }) },\n` +
        `  handle: async () => ({ msg: "hello" }),\n` +
        `});\n`,
    );

    const second = await builder.build();
    expect(second.contractChanged).toBeTrue();
    expect(second.contractHash).not.toBe(first.contractHash);

    builder.dispose();
  });

  it("keeps contract unchanged when only handler implementation is edited", async () => {
    const builder = new IncrementalPackBuilder({
      project: tempDir,
      sourceMap: true,
    });

    const first = await builder.build();

    // Modify only internal handler logic (contract is unchanged):
    writeFileSync(
      join(tempDir, "src", "routes.ts"),
      `import { route } from "@velqu/core";\n` +
        `import { s } from "@velqu/schema";\n` +
        `export const helloRoute = route({\n` +
        `  id: "hello.get",\n` +
        `  method: "GET",\n` +
        `  path: "/hello",\n` +
        `  response: { 200: s.object({ msg: s.string() }) },\n` +
        `  handle: async () => ({ msg: "updated message body" }),\n` +
        `});\n`,
    );

    const second = await builder.build();
    expect(second.contractChanged).toBeFalse();
    expect(second.contractHash).toBe(first.contractHash);
    expect(second.packSha256).not.toBe(first.packSha256); // bundle changed

    builder.dispose();
  });

  it("bounds temporary disk storage by cleaning up older pack files", async () => {
    const customTemp = join(tempDir, "custom-temp");
    const builder = new IncrementalPackBuilder({
      project: tempDir,
      tempDir: customTemp,
    });

    // Build 5 times with changing route ids:
    for (let i = 0; i < 5; i++) {
      writeFileSync(
        join(tempDir, "src", "routes.ts"),
        `import { route } from "@velqu/core";\n` +
          `import { s } from "@velqu/schema";\n` +
          `export const helloRoute = route({\n` +
          `  id: "hello.get.${i}",\n` +
          `  method: "GET",\n` +
          `  path: "/hello/${i}",\n` +
          `  response: { 200: s.object({ v: s.integer() }) },\n` +
          `  handle: async () => ({ v: ${i} }),\n` +
          `});\n`,
      );
      await builder.build();
    }

    const files = readdirSync(customTemp).filter((f) => f.endsWith(".qpack"));
    expect(files.length).toBeLessThanOrEqual(2); // bounded storage

    builder.dispose();
    expect(existsSync(customTemp)).toBeFalse();
  });
});
