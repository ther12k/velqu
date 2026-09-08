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

/**
 * #1292 — non-exported route bindings must fail the browser build.
 * The emitted app.browser.js references each handler through a namespace
 * import of its source module; a non-exported const folds to `undefined`
 * in the consumer's bundler and every invocation 500s at runtime.
 */
describe("#1292 non-exported handler binding guard", () => {
  function writeProject(
    dir: string,
    routeDecl: string,
    tail: string[] = [],
  ): string {
    rmSync(dir, { recursive: true, force: true });
    mkdirSync(join(dir, "src"), { recursive: true });
    writeFileSync(
      join(dir, "src", "app.ts"),
      [
        `import { route } from "@velqu/core";`,
        `import { s } from "@velqu/schema";`,
        ``,
        routeDecl,
        ...tail,
        ``,
      ].join("\n"),
    );
    return join(dir, "src", "app.ts");
  }

  const tickDecl = [
    `const tick = route({`,
    `  id: "sys.tick",`,
    `  method: "GET",`,
    `  path: "/sys/tick",`,
    `  response: { 200: s.object({ ms: s.integer() }) },`,
    `  handle: (ctx) => ({ ms: 0 }), // ctx param → not native-liveness`,
    `});`,
  ].join("\n");

  it("fails the build when the route const is not exported (source-located)", () => {
    const dir = join(root, ".tmp-1292-guard");
    cleanup.push(dir);
    const entry = writeProject(dir, tickDecl, [`export const app = { routes: [tick] };`]);
    const app = extractApp(entry);
    const route = app.routes.find((r) => r.id === "sys.tick");
    expect(route?.exported).toBeFalse(); // fixture sanity
    // Stub native dir: the diagnostic must fire before any artifact read.
    mkdirSync(join(dir, "dist"), { recursive: true });
    try {
      buildBrowserWasmArtifacts({ project: entry, outDir: dir }, app, join(dir, "dist"));
      throw new Error("unreachable");
    } catch (e) {
      expect(e).toBeInstanceOf(CompileError);
      expect((e as CompileError).message).toContain("browser-wasm target");
      expect((e as CompileError).message).toContain('"sys.tick"');
      expect((e as CompileError).message).toContain('"tick"');
      expect((e as CompileError).message).toContain("not exported");
      expect((e as CompileError).message).toContain("src/app.ts"); // source-located
    }
    // No browser directory was created on the diagnostic path.
    expect(existsSync(join(dir, "browser"))).toBeFalse();
  });

  it("a same-module `export { tick }` re-export satisfies the guard and emits", () => {
    const dir = join(root, ".tmp-1292-reexport");
    cleanup.push(dir);
    const entry = writeProject(dir, tickDecl, [
      `export const app = { routes: [tick] };`,
      `export { tick };`,
    ]);
    const app = extractApp(entry);
    const route = app.routes.find((r) => r.id === "sys.tick");
    expect(route?.exported).toBeTrue(); // re-export joins the export surface
    // Stub native artifacts: the B-001 emission hashes/copies bytes only.
    const nativeOut = join(dir, "dist");
    mkdirSync(nativeOut, { recursive: true });
    writeFileSync(join(nativeOut, "app.qpack"), "stub-pack");
    for (const name of ["schema-manifest.json", "capability-manifest.json", "contract.json"]) {
      writeFileSync(join(nativeOut, name), "{}\n");
    }
    const result = buildBrowserWasmArtifacts({ project: entry, outDir: dir }, app, nativeOut);
    expect(result.routes).toBe(1);
    const js = readFileSync(join(result.outDir, "app.browser.js"), "utf8");
    expect(js).toContain(".tick)"); // namespace access reaches the binding
  });

  it("default-export and `export const` routes are extracted as exported", () => {
    const dir = join(root, ".tmp-1292-forms");
    cleanup.push(dir);
    const entry = writeProject(
      dir,
      [
        `export default route({`,
        `  id: "root.get",`,
        `  method: "GET",`,
        `  path: "/",`,
        `  response: { 200: s.object({ ok: s.boolean() }) },`,
        `  handle: (ctx) => ({ ok: true }),`,
        `});`,
      ].join("\n"),
      [
        `export const listed = route({`,
        `  id: "listed.get",`,
        `  method: "GET",`,
        `  path: "/listed",`,
        `  response: { 200: s.object({ ok: s.boolean() }) },`,
        `  handle: (ctx) => ({ ok: true }),`,
        `});`,
        `const hidden = route({`,
        `  id: "hidden.get",`,
        `  method: "GET",`,
        `  path: "/hidden",`,
        `  response: { 200: s.object({ ok: s.boolean() }) },`,
        `  handle: (ctx) => ({ ok: true }),`,
        `});`,
        `export const app = { routes: [hidden] };`,
      ],
    );
    const app = extractApp(entry);
    const byId = new Map(app.routes.map((r) => [r.id, r]));
    expect(byId.get("root.get")?.exported).toBeTrue(); // export default
    expect(byId.get("listed.get")?.exported).toBeTrue(); // export const
    expect(byId.get("hidden.get")?.exported).toBeFalse(); // plain const
  });
});
