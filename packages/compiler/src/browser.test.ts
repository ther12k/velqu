/**
 * BWASM-B-001 — browser-wasm compiler target tests: golden emission,
 * native-unchanged, two-build reproducibility, build-time diagnostics.
 */
import { describe, it, expect, afterEach } from "bun:test";
import { mkdirSync, rmSync, writeFileSync, existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { buildBrowserWasmArtifacts, CompileError, extractApp } from "../src/index";

const root = join(import.meta.dir, "..", "..", "..");
const demoDir = join(root, "examples", "browser-demo");
const demoEntry = join(demoDir, "src", "app.ts");
let cleanup: string[] = [];
afterEach(() => {
  for (const dir of cleanup.splice(0)) rmSync(dir, { recursive: true, force: true });
});

describe("B-001 browser-wasm target (golden emission)", () => {
  it("emits a self-contained browser set for the demo project", () => {
    const app = extractApp(demoEntry);
    const nativeOut = join(demoDir, "dist");
    // The browser set carries the verified pack: produce it with the
    // native build (the native path is unchanged — covered separately).
    const { execSync } = require("node:child_process");
    execSync("bun packages/cli/src/index.ts build --project examples/browser-demo", {
      cwd: root,
      stdio: "pipe",
    });
    const result = buildBrowserWasmArtifacts(
      { project: demoEntry, outDir: nativeOut },
      app,
      nativeOut,
    );
    cleanup.push(join(demoDir, "dist", "browser"));
    const names = Object.keys(result.files).sort();
    expect(names).toEqual([
      "app.browser.js",
      "app.qpack",
      "browser-manifest.json",
      "capability-manifest.json",
      "contract.json",
      "schema-manifest.json",
    ]);
    const js = readFileSync(join(result.outDir, "app.browser.js"), "utf8");
    expect(js).toContain("defineBrowserHandlers");
    expect(js).toContain('"hello.get"');
    expect(js).toContain('"echo.post"');
    expect(js).not.toContain("globalThis.__velquFunctionManifest"); // no ambient globals
    const manifest = JSON.parse(readFileSync(join(result.outDir, "browser-manifest.json"), "utf8"));
    expect(manifest.target).toBe("browser-wasm");
    expect(manifest.handlerAbiVersion).toBe(1);
    expect(manifest.kernelAbiVersion).toBe(1);
    expect(manifest.packSha256).toMatch(/^[0-9a-f]{64}$/);
    // carried pack is byte-identical to the native build
    expect(readFileSync(join(result.outDir, "app.qpack"))).toEqual(
      readFileSync(join(nativeOut, "app.qpack")),
    );
  });

  it("native build output is unchanged by the browser emission", () => {
    const { execSync } = require("node:child_process");
    const before = readFileSync(join(demoDir, "dist", "app.qpack"));
    const app = extractApp(demoEntry);
    const result = buildBrowserWasmArtifacts(
      { project: demoEntry, outDir: join(demoDir, "dist") },
      app,
      join(demoDir, "dist"),
    );
    const after = readFileSync(join(demoDir, "dist", "app.qpack"));
    expect(after.equals(before)).toBeTrue();
    // browser dir is separate; no native artifact name collides
    expect(existsSync(join(result.outDir, "route-manifest.json"))).toBeFalse();
  });
});

describe("B-001 reproducibility", () => {
  it("two builds produce byte-identical browser artifact hashes", () => {
    const appA = extractApp(demoEntry);
    const appB = extractApp(demoEntry);
    const out = join(demoDir, "dist");
    const a = buildBrowserWasmArtifacts({ project: demoEntry, outDir: out }, appA, out);
    const b = buildBrowserWasmArtifacts({ project: demoEntry, outDir: out }, appB, out);
    expect(a.files).toEqual(b.files);
    for (const [name, hash] of Object.entries(a.files)) {
      const bytes = readFileSync(join(b.outDir, name));
      expect(createHash(bytes)).toBe(hash);
    }
  });
});

function createHash(bytes: Uint8Array): string {
  // eslint-disable-next-line -- Bun exposes node:crypto via global in tests
  const { createHash } = require("node:crypto");
  return createHash("sha256").update(bytes).digest("hex");
}

describe("B-001 build-time diagnostics", () => {
  it("native-liveness routes are rejected with a source-located error", () => {
    const dir = join(root, ".tmp-b001-liveness");
    cleanup.push(dir);
    rmSync(dir, { recursive: true, force: true }); // clear stale runs
    mkdirSync(join(dir, "src"), { recursive: true });
    writeFileSync(
      join(dir, "src", "app.ts"),
      [
        `import { route } from "@velqu/core";`,
        `import { s } from "@velqu/schema";`,
        ``,
        `export const health = route({`,
        `  id: "health.live",`,
        `  method: "GET",`,
        `  path: "/health/live",`,
        `  response: { 200: s.object({ status: s.string() }) },`,
        `  handle: () => ({ status: "ok" }),`,
        `});`,
        ``,
        `export const app = { routes: [health] };`,
        ``,
      ].join("\n"),
    );
    const app = extractApp(join(dir, "src", "app.ts"));
    const route = app.routes.find((r) => r.id === "health.live");
    expect(route?.liveness).not.toBeNull(); // fixture sanity: this IS native-liveness
    const nativeOut = join(dir, "dist");
    mkdirSync(nativeOut, { recursive: true });
    // The pack build requires a complete valid pack; the diagnostic must
    // fire before any artifact is read — pass a stub native dir.
    try {
      buildBrowserWasmArtifacts({ project: join(dir, "src", "app.ts"), outDir: dir }, app, nativeOut);
      throw new Error("unreachable");
    } catch (e) {
      expect(e).toBeInstanceOf(CompileError);
      expect((e as CompileError).message).toContain("browser-wasm target");
      expect((e as CompileError).message).toContain("native liveness");
      expect((e as CompileError).message).toContain("src/app.ts"); // source-located
    }
    // No browser directory was created on the diagnostic path.
    expect(existsSync(join(dir, "browser"))).toBeFalse();
  });
});
