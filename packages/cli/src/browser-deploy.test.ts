/**
 * BWASM-B-005 — CLI browser-wasm deployment workflow tests.
 *
 * Coverage (acceptance-mapped):
 * - kernel pin + deterministic browser-glue transform (fail-closed)
 * - compose: full artifact set, integrity, reproducibility, --clean
 * - inspect: clean, tampered, missing shell — nonzero-worthy problems
 * - export: exact deployment set, fail-closed on tamper
 * - preview: static bytes + diagnostics only; traversal/no-method guards
 * - static deployment smoke over real HTTP with a real module Worker
 * - CLI integration: --target validation, JSON schema-versioned output,
 *   nonzero exit for unsupported imports and failed integrity
 * - JSON fixtures (schema-pinned) under src/fixtures/browser-cli/
 */
import { describe, it, expect, beforeAll, afterAll } from "bun:test";
import {
  existsSync,
  mkdirSync,
  readFileSync,
  rmSync,
  statSync,
  writeFileSync,
} from "node:fs";
import { join } from "node:path";
import {
  BrowserDeployError,
  browserKernelModuleSource,
  composeBrowserDeployment,
  exportDeployment,
  inspectBrowserDeployment,
  resolveKernelArtifacts,
  startPreviewServer,
  PREVIEW_DIAGNOSTICS_PATH,
  type ComposeResult,
  type PreviewServer,
} from "./browser-deploy";
import { loadArtifacts, type KernelInstance, type KernelModule } from "@velqu/browser-runtime";

const root = join(import.meta.dir, "..", "..", "..");
const demoDir = join(root, "examples", "browser-demo");
const browserDir = join(demoDir, "dist", "browser");

let compose: ComposeResult;

beforeAll(async () => {
  compose = await composeBrowserDeployment({ project: join(demoDir, "src", "app.ts") });
});

// ---------------------------------------------------------------------------
// Kernel artifacts + browser glue transform
// ---------------------------------------------------------------------------

describe("B-005 kernel artifacts (vendored, pinned)", () => {
  it("resolves the pinned kernel and verifies digests", () => {
    const kernel = resolveKernelArtifacts();
    expect(kernel.kernelAbiVersion).toBe(1);
    expect(kernel.wasmBytes.byteLength).toBeGreaterThan(1_000_000);
    expect(kernel.wasmSha256).toMatch(/^[0-9a-f]{64}$/);
    expect(kernel.provenance.builtWith as string).toContain("wasm-bindgen 0.2.108");
  });

  it("fails closed when the glue pin does not match", () => {
    const kernelDir = join(root, "packages", "browser-runtime", "kernel");
    const pinPath = join(kernelDir, "kernel.json");
    const original = readFileSync(pinPath, "utf8");
    const pin = JSON.parse(original);
    try {
      writeFileSync(
        pinPath,
        JSON.stringify({ ...pin, glue: { ...pin.glue, sha256: "0".repeat(64) } }, null, 2),
      );
      expect(() => resolveKernelArtifacts()).toThrow(BrowserDeployError);
      expect(() => resolveKernelArtifacts()).toThrow(/glue does not match/);
    } finally {
      writeFileSync(pinPath, original);
    }
  });

  it("fails closed on an explicit kernel wasm that is missing", () => {
    expect(() => resolveKernelArtifacts("/nonexistent/kernel.wasm")).toThrow(
      /kernel wasm not found/,
    );
  });
});

describe("B-005 browser kernel module transform", () => {
  it("is deterministic and browser-shaped", () => {
    const glue = resolveKernelArtifacts().glueSource;
    const a = browserKernelModuleSource(glue);
    const b = browserKernelModuleSource(glue);
    expect(a).toBe(b);
    expect(a).toContain("export function initKernelSync");
    expect(a).toContain("export { WasmKernel, kernel_abi_version }");
    expect(a).not.toContain("readFileSync");
    expect(a).not.toContain("__dirname");
  });

  it("fails closed when the glue layout marker is not unique", () => {
    const glue = resolveKernelArtifacts().glueSource;
    expect(() => browserKernelModuleSource(glue.replace("const wasmPath = ", ""))).toThrow(
      BrowserDeployError,
    );
    expect(() =>
      browserKernelModuleSource(glue + "\nconst wasmPath = duplicate;\n"),
    ).toThrow(/layout changed/);
  });

  it("instantiates the vendored wasm through the transformed module (real WebAssembly)", async () => {
    const kernel = resolveKernelArtifacts();
    const tmp = join(root, ".tmp-b005-kernel");
    rmSync(tmp, { recursive: true, force: true });
    mkdirSync(tmp, { recursive: true });
    const modPath = join(tmp, "kernel.js");
    writeFileSync(modPath, browserKernelModuleSource(kernel.glueSource));
    writeFileSync(join(tmp, "kernel.wasm"), kernel.wasmBytes);
    const mod = await import(modPath);
    mod.initKernelSync(new Uint8Array(readFileSync(join(tmp, "kernel.wasm"))));
    expect(mod.kernel_abi_version()).toBe(1);
    const k = new mod.WasmKernel(new Uint8Array(readFileSync(join(browserDir, "app.qpack"))));
    const plan = JSON.parse(
      k.plan_request(JSON.stringify({ abiVersion: 1, method: "GET", path: "/hello/world" })),
    );
    expect(plan.kind).toBe("invoke");
    expect(plan.handlerKey).toBe("hello.get");
    k.dispose();
    rmSync(tmp, { recursive: true, force: true });
  });
});

// ---------------------------------------------------------------------------
// Compose (build --target browser-wasm)
// ---------------------------------------------------------------------------

describe("B-005 compose (browser-wasm build pipeline)", () => {
  it("emits the complete deployment set with a verified manifest", async () => {
    for (const name of [
      "app.qpack",
      "kernel.wasm",
      "kernel.js",
      "app.bundle.js",
      "browser-manifest.json",
      "contract.json",
      "schema-manifest.json",
      "capability-manifest.json",
      "velqu-artifacts.json",
      "index.html",
      "page.js",
      "worker.js",
      "service-worker.js",
    ]) {
      expect(existsSync(join(browserDir, name))).toBeTrue();
    }
    expect(existsSync(join(browserDir, "_entries"))).toBeFalse();
    // The B-002 manifest verifies over the emitted bytes (fs reader).
    const loaded = await loadArtifacts(
      readFileSync(join(browserDir, "velqu-artifacts.json"), "utf8"),
      async (url) => new Uint8Array(readFileSync(join(browserDir, url))),
    );
    expect(loaded.buildId).toBe(compose.buildId);
    expect(Object.keys(loaded.manifest.artifacts)).not.toContain("sourceMap");
    expect(loaded.manifest.artifacts.kernelWasm.bytes).toBe(1_731_509);
  });

  it("reproduces byte-identical deployments (same buildId and digests)", async () => {
    const second = await composeBrowserDeployment({
      project: join(demoDir, "src", "app.ts"),
    });
    expect(second.buildId).toBe(compose.buildId);
    expect(second.shellFiles).toEqual(compose.shellFiles);
    expect(second.manifest.artifacts).toEqual(compose.manifest.artifacts);
  });

  it("--clean removes stale files from previous builds", async () => {
    const stale = join(browserDir, "stale-from-old-build.js");
    writeFileSync(stale, "// stale");
    await composeBrowserDeployment({
      project: join(demoDir, "src", "app.ts"),
      clean: true,
    });
    expect(existsSync(stale)).toBeFalse();
    expect(existsSync(join(browserDir, "velqu-artifacts.json"))).toBeTrue();
  });

  it("rejects an invalid base path", async () => {
    try {
      await composeBrowserDeployment({
        project: join(demoDir, "src", "app.ts"),
        basePath: "/bad path/",
      });
      throw new Error("unreachable");
    } catch (e) {
      expect(e).toBeInstanceOf(BrowserDeployError);
      expect((e as BrowserDeployError).code).toBe("INVALID_OPTION");
    }
  });

  it("emits sourceMap role + linked maps with --source-map", async () => {
    const withMaps = await composeBrowserDeployment({
      project: join(demoDir, "src", "app.ts"),
      sourceMap: true,
    });
    expect(Object.keys(withMaps.manifest.artifacts)).toContain("sourceMap");
    expect(existsSync(join(browserDir, "app.bundle.js.map"))).toBeTrue();
    // restore the default (no maps) deployment for the remaining tests
    await composeBrowserDeployment({ project: join(demoDir, "src", "app.ts") });
  });

  it("substitutes the base path into the page SW scope and SW scope constant", async () => {
    await composeBrowserDeployment({
      project: join(demoDir, "src", "app.ts"),
      basePath: "/app/",
    });
    const sw = readFileSync(join(browserDir, "service-worker.js"), "utf8");
    expect(sw).toContain('"/app/"');
    const page = readFileSync(join(browserDir, "page.js"), "utf8");
    expect(page).toContain('scope: "/app/"');
    await composeBrowserDeployment({ project: join(demoDir, "src", "app.ts") });
  });
});

// ---------------------------------------------------------------------------
// Inspect (integrity)
// ---------------------------------------------------------------------------

describe("B-005 inspect (integrity + inventory)", () => {
  it("verifies a clean deployment", async () => {
    const report = await inspectBrowserDeployment(browserDir);
    expect(report.ok).toBeTrue();
    expect(report.problems).toEqual([]);
    expect(report.buildId).toBe(compose.buildId);
    expect(report.integrity.checkedArtifacts).toBe(7);
    expect(report.targetCompatibility).toEqual({
      target: "browser-wasm",
      handlerAbiVersion: 1,
      kernelAbiVersion: 1,
      cliSupports: true,
    });
    expect(report.artifactInventory.length).toBeGreaterThanOrEqual(13);
  });

  it("detects a tampered artifact (truncated contract)", async () => {
    const file = join(browserDir, "contract.json");
    const original = readFileSync(file);
    try {
      writeFileSync(file, Buffer.concat([original, Buffer.from(" ")]));
      const report = await inspectBrowserDeployment(browserDir);
      expect(report.ok).toBeFalse();
      expect(report.problems[0].artifact).toBe("contract");
      expect(report.problems[0].reason).toBe("truncated");
    } finally {
      writeFileSync(file, original);
    }
  });

  it("detects a mixed artifact set (pack swapped from a foreign build)", async () => {
    const packPath = join(browserDir, "app.qpack");
    const original = readFileSync(packPath);
    try {
      // Same-length substitution of the pack = foreign build content
      // under an unchanged manifest → digest mismatch (tampered), the
      // same detection path that names cross-build mixes at the loader.
      const foreign = Buffer.from(original);
      foreign[foreign.length - 1] ^= 0xff;
      writeFileSync(packPath, foreign);
      const report = await inspectBrowserDeployment(browserDir);
      expect(report.ok).toBeFalse();
      expect(report.problems.some((p) => p.artifact === "pack")).toBeTrue();
    } finally {
      writeFileSync(packPath, original);
    }
  });

  it("reports missing shell files as problems", async () => {
    const shell = join(browserDir, "index.html");
    const original = readFileSync(shell);
    rmSync(shell);
    try {
      const report = await inspectBrowserDeployment(browserDir);
      expect(report.ok).toBeFalse();
      expect(report.problems.some((p) => p.artifact === "index.html" && p.reason === "missing")).toBeTrue();
    } finally {
      writeFileSync(shell, original);
    }
  });

  it("surfaces deployment requirements honestly", async () => {
    const report = await inspectBrowserDeployment(browserDir);
    const text = report.deploymentRequirements.join("\n");
    expect(text).toContain("static hosting only");
    expect(text).toContain("BWASM-B-003");
    expect(text).toContain("timer"); // nativeOps surfaced, not hidden
  });

  it("fails closed when there is no browser deployment", async () => {
    expect(() => inspectBrowserDeployment(join(root, "examples", "proof", "dist", "browser"))).toThrow(
      /no velqu-artifacts\.json/,
    );
  });
});

// ---------------------------------------------------------------------------
// Export
// ---------------------------------------------------------------------------

describe("B-005 export (verified clean copy)", () => {
  const outDir = join(demoDir, "dist", "browser-export");

  it("copies exactly the deployment set (no build inputs)", async () => {
    rmSync(outDir, { recursive: true, force: true });
    const result = await exportDeployment(browserDir, outDir);
    expect(result.buildId).toBe(compose.buildId);
    const names = result.files.map((f) => f.file).sort();
    expect(names).toEqual([
      "app.bundle.js",
      "app.qpack",
      "browser-manifest.json",
      "capability-manifest.json",
      "contract.json",
      "index.html",
      "kernel.js",
      "kernel.wasm",
      "page.js",
      "schema-manifest.json",
      "service-worker.js",
      "velqu-artifacts.json",
      "worker.js",
    ]);
    expect(names).not.toContain("app.browser.js"); // build input, not deployed
    // The exported set self-verifies.
    const loaded = await loadArtifacts(
      readFileSync(join(outDir, "velqu-artifacts.json"), "utf8"),
      async (url) => new Uint8Array(readFileSync(join(outDir, url))),
    );
    expect(loaded.buildId).toBe(compose.buildId);
  });

  it("refuses to export a tampered set", async () => {
    const file = join(browserDir, "schema-manifest.json");
    const original = readFileSync(file);
    writeFileSync(file, original.toString().replace("{", "{ "));
    try {
      await exportDeployment(browserDir, join(demoDir, "dist", "browser-export-refused"));
      throw new Error("unreachable");
    } catch (e) {
      expect(e).toBeInstanceOf(BrowserDeployError);
      expect((e as BrowserDeployError).code).toBe("INTEGRITY_FAILED");
      expect(existsSync(join(demoDir, "dist", "browser-export-refused"))).toBeFalse();
    } finally {
      writeFileSync(file, original);
      rmSync(join(demoDir, "dist", "browser-export-refused"), { recursive: true, force: true });
    }
  });
});

// ---------------------------------------------------------------------------
// Preview (static bytes + diagnostics ONLY)
// ---------------------------------------------------------------------------

describe("B-005 preview (development static server)", () => {
  let server: PreviewServer;

  beforeAll(async () => {
    server = await startPreviewServer({ browserDir, port: 0 });
  });
  afterAll(async () => {
    await server.stop();
  });

  it("serves the deployment shell with preview markers", async () => {
    const res = await fetch(`${server.baseUrl}/index.html`);
    expect(res.status).toBe(200);
    expect(res.headers.get("content-type")).toContain("text/html");
    expect(res.headers.get("x-velqu-preview")).toBe("dev");
    expect(await res.text()).toContain("page.js");
  });

  it("serves artifacts with canonical media types", async () => {
    const wasm = await fetch(`${server.baseUrl}/kernel.wasm`);
    expect(wasm.status).toBe(200);
    expect(wasm.headers.get("content-type")).toBe("application/wasm");
    expect((await wasm.arrayBuffer()).byteLength).toBe(1_731_509);
    const manifest = await fetch(`${server.baseUrl}/velqu-artifacts.json`);
    expect(manifest.headers.get("content-type")).toContain("application/json");
    const artifacts = (await manifest.json()) as { buildId: string };
    expect(artifacts.buildId).toBe(compose.buildId);
  });

  it("serves schema-versioned diagnostics and never claims production", async () => {
    const res = await fetch(`${server.baseUrl}${PREVIEW_DIAGNOSTICS_PATH}`);
    expect(res.status).toBe(200);
    const body = (await res.json()) as Record<string, unknown>;
    expect(body.schemaVersion).toBe(1);
    expect(body.production).toBeFalse();
    expect(body.command).toBe("preview");
    expect(body.buildId).toBe(compose.buildId);
    expect(JSON.stringify(body.deploymentRequirements)).toContain("static hosting only");
  });

  it("blocks traversal, hidden entries, and non-GET methods", async () => {
    const traversal = await fetch(`${server.baseUrl}/..%2f..%2fpackage.json`);
    expect(traversal.status).toBe(404);
    const entries = await fetch(`${server.baseUrl}/_entries/worker.js`);
    expect(entries.status).toBe(404);
    const post = await fetch(`${server.baseUrl}/index.html`, { method: "POST" });
    expect(post.status).toBe(405);
  });

  it("stops cleanly (port released)", async () => {
    const temp = await startPreviewServer({ browserDir, port: 0 });
    const url = temp.baseUrl;
    await fetch(`${url}/index.html`);
    await temp.stop();
    let released = false;
    try {
      await fetch(`${url}/index.html`);
    } catch {
      released = true;
    }
    expect(released).toBeTrue();
  });
});

// ---------------------------------------------------------------------------
// Static deployment smoke (real HTTP + real module Worker)
// ---------------------------------------------------------------------------

describe("B-005 static deployment smoke (clean consumer)", () => {
  it("boots the runtime from HTTP-served verified bytes and executes a handler in a real Worker", async () => {
    const { createBrowserRuntime, WorkerHost } = await import("@velqu/browser-runtime");
    const kernelMod = await import(join(browserDir, "kernel.js"));
    const server = await startPreviewServer({ browserDir, port: 0 });
    try {
      const base = new URL(`${server.baseUrl}/`);
      const readArtifact = async (url: string) =>
        new Uint8Array(await (await fetch(new URL(url, base))).arrayBuffer());
      const manifestText = await (await fetch(new URL("velqu-artifacts.json", base))).text();
      const loaded = await loadArtifacts(manifestText, readArtifact);
      expect(loaded.buildId).toBe(compose.buildId);

      kernelMod.initKernelSync(loaded.bytes.kernelWasm);
      expect(kernelMod.kernel_abi_version()).toBe(1);

      const sessionId = crypto.randomUUID();
      const host = new WorkerHost(sessionId, () => {
        const worker = new Worker(join(browserDir, "worker.js"), { type: "module" });
        worker.postMessage({ type: "velqu-session", sessionId });
        return worker;
      });
      const VelquKernel = class implements KernelInstance {
        private readonly inner: InstanceType<typeof kernelMod.WasmKernel>;
        constructor(packBytes: Uint8Array) {
          this.inner = new kernelMod.WasmKernel(packBytes);
        }
        plan_request(requestJson: string): string {
          return this.inner.plan_request(requestJson);
        }
        complete_invocation(completionJson: string): string {
          return this.inner.complete_invocation(completionJson);
        }
        authorize_capability(name: string): string {
          return this.inner.authorize_capability(name);
        }
        dispose(): void {
          this.inner.dispose();
        }
      };
      const velquKernelModule: KernelModule = Object.assign(VelquKernel, {
        kernel_abi_version: kernelMod.kernel_abi_version,
      });
      const runtime = createBrowserRuntime({
        packBytes: loaded.bytes.pack,
        kernel: velquKernelModule,
        executeHandler: async (plan) => {
          const result = await host.execute(plan);
          if (result.kind !== "response") {
            // Handler-side problem: reject, the runtime maps failures to
            // typed problems (smoke wiring; the happy path is response-kind).
            throw new Error(`handler problem: ${result.problemId} ${result.detail ?? ""}`.trim());
          }
          return {
            kind: "response",
            status: result.status,
            headers: [...(result.headers ?? [])] as Array<[string, string]>,
            body: result.body,
          };
        },
      });

      const ok = await runtime.fetch(new Request("http://consumer.example/hello/world"));
      expect(ok.status).toBe(200);
      expect(await ok.json()).toEqual({ message: "Hello world" });

      const missing = await runtime.fetch(new Request("http://consumer.example/nope"));
      expect(missing.status).toBe(404);
      const problem = (await missing.json()) as { problemId: string; status: number };
      expect(problem.problemId).toBe("not-found");
      expect(missing.headers.get("content-type")).toContain("application/problem+json");
    } finally {
      await server.stop();
    }
  });
});

// ---------------------------------------------------------------------------
// CLI integration (subprocess; exit codes + JSON schema)
// ---------------------------------------------------------------------------

async function runCli(args: string[]): Promise<{ code: number; stdout: string; stderr: string }> {
  const proc = Bun.spawn(["bun", "packages/cli/src/index.ts", ...args], {
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

describe("B-005 CLI integration", () => {
  it("build --target browser-wasm --json exits 0 with schemaVersion 1", async () => {
    const r = await runCli([
      "build", "--target", "browser-wasm",
      "--project", "examples/browser-demo",
      "--json",
    ]);
    expect(r.code).toBe(0);
    const body = JSON.parse(r.stdout) as Record<string, unknown>;
    expect(body.schemaVersion).toBe(1);
    expect(body.status).toBe("ok");
    expect(body.target).toBe("browser-wasm");
    expect(body.buildId).toMatch(/^[0-9a-f]{64}$/);
    expect(body.kernel).toBeObject();
  }, 60_000);

  it("rejects an unsupported --target with exit 1", async () => {
    const r = await runCli(["build", "--target", "deno", "--project", "examples/browser-demo"]);
    expect(r.code).toBe(1);
    expect(r.stderr).toContain("unsupported --target 'deno'");
  });

  it("inspect browser exits 0 clean and 1 tampered (JSON schema-versioned)", async () => {
    const clean = await runCli([
      "inspect", "browser", "--project", "examples/browser-demo", "--json",
    ]);
    expect(clean.code).toBe(0);
    const cleanBody = JSON.parse(clean.stdout) as Record<string, unknown>;
    expect(cleanBody.schemaVersion).toBe(1);
    expect(cleanBody.status).toBe("ok");

    const file = join(browserDir, "app.qpack");
    const original = readFileSync(file);
    writeFileSync(file, Buffer.concat([original, Buffer.from("x")]));
    try {
      const tampered = await runCli([
        "inspect", "browser", "--project", "examples/browser-demo", "--json",
      ]);
      expect(tampered.code).toBe(1);
      const body = JSON.parse(tampered.stdout) as {
        status: string;
        problems: { artifact: string; reason: string }[];
      };
      expect(body.status).toBe("error");
      expect(body.problems[0].artifact).toBe("pack");
      expect(body.problems[0].reason).toBe("truncated");
    } finally {
      writeFileSync(file, original);
    }
  });

  it("export --json exits 0 with the exact deployment inventory", async () => {
    rmSync(join(demoDir, "dist", "browser-export"), { recursive: true, force: true });
    const r = await runCli([
      "export", "--project", "examples/browser-demo",
      "--out", "examples/browser-demo/dist/browser-export", "--json",
    ]);
    expect(r.code).toBe(0);
    const body = JSON.parse(r.stdout) as Record<string, unknown>;
    expect(body.schemaVersion).toBe(1);
    expect(body.status).toBe("ok");
    expect(body.totalBytes).toBeGreaterThan(1_700_000);
  });

  it("build --target browser-wasm exits nonzero for unsupported imports (B-003 policy)", async () => {
    const tmp = join(root, ".tmp-b005-policy");
    rmSync(tmp, { recursive: true, force: true });
    mkdirSync(join(tmp, "src"), { recursive: true });
    writeFileSync(
      join(tmp, "src", "app.ts"),
      [
        `import { route } from "@velqu/core";`,
        `import { s } from "@velqu/schema";`,
        `export const hello = route({`,
        `  id: "hello.get",`,
        `  method: "GET",`,
        `  path: "/hello",`,
        `  response: { 200: s.object({ msg: s.string() }) },`,
        `  handle: async () => ({ msg: eval("String(1 + 1)") }),`,
        `});`,
        `export const app = { routes: [hello] };`,
        ``,
      ].join("\n"),
    );
    try {
      const r = await runCli(["build", "--target", "browser-wasm", "--project", tmp]);
      expect(r.code).toBe(1);
      expect(r.stderr).toContain("BWASM-POLICY-DYNAMIC-CODE");
      // The diagnostic path emits nothing.
      expect(existsSync(join(tmp, "dist", "browser"))).toBeFalse();
    } finally {
      rmSync(tmp, { recursive: true, force: true });
    }
  }, 60_000);

  it("help lists the new workflows", async () => {
    const r = await runCli(["--help"]);
    expect(r.code).toBe(0);
    expect(r.stdout).toContain("browser-wasm");
    expect(r.stdout).toContain("velqu preview");
    expect(r.stdout).toContain("velqu export");
    expect(r.stdout).toContain("inspect browser");
  });
});

// ---------------------------------------------------------------------------
// JSON fixtures (schema-pinned)
// ---------------------------------------------------------------------------

describe("B-005 JSON output fixtures", () => {
  const fixturesDir = join(import.meta.dir, "fixtures", "browser-cli");

  /**
   * Digest-tolerant deep compare: 64-hex strings and "12hex…" forms match
   * by pattern; absolute paths match by suffix; everything else is exact.
   */
  function matchesFixture(actual: unknown, fixture: unknown, path = "$"): void {
    if (typeof fixture === "string") {
      if (/^[0-9a-f]{64}$/.test(fixture)) {
        expect(typeof actual).toBe("string");
        expect(actual as string).toMatch(/^[0-9a-f]{64}$/);
        return;
      }
      if (/^[0-9a-f]{12}…$/.test(fixture)) {
        expect(actual as string).toMatch(/^[0-9a-f]{12}…$/);
        return;
      }
      if (fixture.startsWith("/")) {
        expect((actual as string).endsWith(fixture.replace(/^\//, "/")) || actual === fixture).toBeTrue();
        return;
      }
      expect(actual).toBe(fixture);
      return;
    }
    if (Array.isArray(fixture)) {
      expect(Array.isArray(actual)).toBeTrue();
      expect((actual as unknown[]).length).toBe(fixture.length);
      fixture.forEach((f, i) => matchesFixture((actual as unknown[])[i], f, `${path}[${i}]`));
      return;
    }
    if (fixture && typeof fixture === "object") {
      expect(actual).toBeObject();
      const fa = fixture as Record<string, unknown>;
      const aa = actual as Record<string, unknown>;
      expect(Object.keys(aa).sort()).toEqual(Object.keys(fa).sort());
      for (const key of Object.keys(fa)) matchesFixture(aa[key], fa[key], `${path}.${key}`);
      return;
    }
    expect(actual).toBe(fixture);
  }

  it("inspect browser --json matches the committed fixture", async () => {
    const r = await runCli([
      "inspect", "browser", "--project", "examples/browser-demo", "--json",
    ]);
    const fixture = JSON.parse(readFileSync(join(fixturesDir, "inspect-browser.json"), "utf8"));
    matchesFixture(JSON.parse(r.stdout), fixture);
  });

  it("export --json matches the committed fixture", async () => {
    rmSync(join(demoDir, "dist", "browser-export"), { recursive: true, force: true });
    const r = await runCli([
      "export", "--project", "examples/browser-demo",
      "--out", "examples/browser-demo/dist/browser-export", "--json",
    ]);
    const fixture = JSON.parse(readFileSync(join(fixturesDir, "export.json"), "utf8"));
    matchesFixture(JSON.parse(r.stdout), fixture);
  });
});

afterAll(() => {
  // Keep the committed demo artifacts byte-stable for the B-001 suite.
  if (existsSync(browserDir) && statSync(browserDir).isDirectory()) {
    rmSync(join(demoDir, "dist", "browser-export"), { recursive: true, force: true });
  }
});
