/**
 * BWASM-B-005 — CLI browser-wasm deployment workflows.
 *
 * Turns the B-001..B-004 pieces into one coherent developer path:
 *
 *   velqu build --target browser-wasm   → static artifact set + B-002 manifest
 *   velqu preview                       → static bytes + dev diagnostics only
 *   velqu inspect browser               → integrity (tamper/mixed-set) + inventory
 *   velqu export                        → verified clean deployment copy
 *
 * Design constraints honored:
 * - The native target is untouched (the browser pipeline CONSUMES native
 *   build output read-only; B-001 regression stays green).
 * - The Service Worker never executes handlers (ADR-0037: handlers run in
 *   the page's isolated Worker); the SW is the B-004 asset/offline lane.
 * - The kernel WASM + wasm-bindgen glue are vendored, hash-pinned
 *   (`packages/browser-runtime/kernel/kernel.json`), and the browser glue
 *   is a deterministic fail-closed transform of the pinned nodejs glue.
 * - Every JSON output carries `schemaVersion: 1` and is fixture-tested.
 */
import {
  createHash,
} from "node:crypto";
import {
  existsSync,
  mkdirSync,
  readFileSync,
  readdirSync,
  rmSync,
  statSync,
  writeFileSync,
  copyFileSync,
} from "node:fs";
import { basename, join, relative, resolve } from "node:path";
import type {
  ArtifactRole,
  BrowserArtifactManifest,
} from "@velqu/browser-runtime";
import { emitArtifactManifest, loadArtifacts, workerBootstrapSource } from "@velqu/browser-runtime";
import {
  build as nativeBuild,
  buildBrowserWasmArtifacts,
  extractApp,
  type BuildResult,
} from "@velqu/compiler";

export const BROWSER_DEPLOY_JSON_SCHEMA_VERSION = 1;

/** Files the deployment needs beyond the B-002 manifest roles (shell). */
export const DEPLOY_SHELL_FILES = [
  "index.html",
  "page.js",
  "worker.js",
  "service-worker.js",
  "kernel.js",
] as const;

export class BrowserDeployError extends Error {
  readonly code:
    | "KERNEL_PIN_MISMATCH"
    | "KERNEL_GLUE_LAYOUT"
    | "MISSING_ARTIFACTS"
    | "BUNDLE_FAILED"
    | "INTEGRITY_FAILED"
    | "NOT_A_BROWSER_DEPLOYMENT"
    | "INVALID_OPTION";
  constructor(code: BrowserDeployError["code"], message: string) {
    super(`[velqu:browser-deploy:${code}] ${message}`);
    this.name = "BrowserDeployError";
    this.code = code;
  }
}

// ---------------------------------------------------------------------------
// Kernel artifacts (vendored, hash-pinned)
// ---------------------------------------------------------------------------

export interface KernelArtifacts {
  readonly wasmBytes: Uint8Array;
  readonly wasmSha256: string;
  readonly glueSource: string;
  readonly glueSha256: string;
  readonly kernelAbiVersion: number;
  readonly provenance: Record<string, unknown>;
}

function sha256Hex(bytes: Uint8Array | string): string {
  return createHash("sha256").update(bytes).digest("hex");
}

/**
 * Resolve the kernel artifacts. Resolution order (fail closed):
 * 1. explicit `--kernel <file>` (a q_browser_kernel_bg.wasm built from
 *    q-browser-kernel with the pinned wasm-bindgen; glue still vendored)
 * 2. the vendored, hash-pinned set in `packages/browser-runtime/kernel/`.
 */
export function resolveKernelArtifacts(explicitWasmPath?: string): KernelArtifacts {
  const kernelDir = join(import.meta.dir, "..", "..", "browser-runtime", "kernel");
  const pin = JSON.parse(readFileSync(join(kernelDir, "kernel.json"), "utf8")) as {
    kernelAbiVersion: number;
    provenance: Record<string, unknown>;
    wasm: { file: string; sha256: string; bytes: number };
    glue: { file: string; sha256: string; bytes: number };
  };
  const wasmPath = explicitWasmPath
    ? resolve(explicitWasmPath)
    : join(kernelDir, pin.wasm.file);
  if (!existsSync(wasmPath)) {
    throw new BrowserDeployError(
      "MISSING_ARTIFACTS",
      `kernel wasm not found at ${wasmPath} — pass --kernel <q_browser_kernel_bg.wasm> or restore packages/browser-runtime/kernel/`,
    );
  }
  const wasmBytes = new Uint8Array(readFileSync(wasmPath));
  if (explicitWasmPath) {
    // An explicit kernel bypasses the vendored wasm pin (owner-supplied
    // artifact); the glue pin still applies because the transform below
    // is only valid for the pinned glue layout.
    return {
      wasmBytes,
      wasmSha256: sha256Hex(wasmBytes),
      glueSource: readGlue(kernelDir, pin),
      glueSha256: pin.glue.sha256,
      kernelAbiVersion: pin.kernelAbiVersion,
      provenance: { ...pin.provenance, explicitWasmPath: basename(wasmPath) },
    };
  }
  if (wasmBytes.byteLength !== pin.wasm.bytes || sha256Hex(wasmBytes) !== pin.wasm.sha256) {
    throw new BrowserDeployError(
      "KERNEL_PIN_MISMATCH",
      `vendored kernel wasm does not match kernel.json pin (expected ${pin.wasm.sha256.slice(0, 12)}…, ${pin.wasm.bytes}B) — restore the pinned artifact or update the pin deliberately`,
    );
  }
  return {
    wasmBytes,
    wasmSha256: pin.wasm.sha256,
    glueSource: readGlue(kernelDir, pin),
    glueSha256: pin.glue.sha256,
    kernelAbiVersion: pin.kernelAbiVersion,
    provenance: pin.provenance,
  };
}

function readGlue(
  kernelDir: string,
  pin: { glue: { file: string; sha256: string; bytes: number } },
): string {
  const gluePath = join(kernelDir, pin.glue.file);
  if (!existsSync(gluePath)) {
    throw new BrowserDeployError("MISSING_ARTIFACTS", `kernel glue not found at ${gluePath}`);
  }
  const glue = readFileSync(gluePath, "utf8");
  if (glue.length !== pin.glue.bytes || sha256Hex(glue) !== pin.glue.sha256) {
    throw new BrowserDeployError(
      "KERNEL_PIN_MISMATCH",
      `vendored kernel glue does not match kernel.json pin (expected ${pin.glue.sha256.slice(0, 12)}…, ${pin.glue.bytes}B)`,
    );
  }
  return glue;
}

// ---------------------------------------------------------------------------
// Browser kernel module (deterministic transform of the pinned glue)
// ---------------------------------------------------------------------------

const GLUE_INSTANTIATION_MARKER = "const wasmPath = ";

const BROWSER_KERNEL_TAIL = `\
// -- velqu browser kernel init -------------------------------------------
// Deterministic transform of the pinned wasm-bindgen 0.2.108 nodejs glue
// (@velqu/cli browser-deploy, BWASM-B-005). The generated glue above this
// marker is kept verbatim (class, helpers, __wbg_get_imports); only the
// module-loading tail is replaced: instantiation happens from
// loader-verified bytes (BWASM-B-002) with no fs access.
let wasm = null;
export function initKernelSync(verifiedBytes) {
  if (wasm) throw new Error("[velqu:kernel] kernel already initialized");
  wasm = new WebAssembly.Instance(new WebAssembly.Module(verifiedBytes), __wbg_get_imports()).exports;
}
export { WasmKernel, kernel_abi_version };
`;

/**
 * Transform the pinned nodejs-target glue into a browser ESM module.
 * Fail closed: the layout marker must occur exactly once, or the glue
 * layout changed and the transform must be re-derived deliberately.
 */
export function browserKernelModuleSource(glueSource: string): string {
  const first = glueSource.indexOf(GLUE_INSTANTIATION_MARKER);
  if (first < 0 || glueSource.indexOf(GLUE_INSTANTIATION_MARKER, first + 1) >= 0) {
    throw new BrowserDeployError(
      "KERNEL_GLUE_LAYOUT",
      `kernel glue layout changed (marker "${GLUE_INSTANTIATION_MARKER.trim()}" not found exactly once) — re-derive the browser kernel transform against the new wasm-bindgen output`,
    );
  }
  const head = glueSource.slice(0, first);
  // The glue assigns CommonJS `exports.*`; neutralized in ESM module scope.
  return `var exports = {};\n${head}${BROWSER_KERNEL_TAIL}`;
}

// ---------------------------------------------------------------------------
// Deployment composition (build --target browser-wasm)
// ---------------------------------------------------------------------------

export interface ComposeOptions {
  /** project dir or entry file */
  readonly project: string;
  /** native dist root (default: compiler's <project>/dist) */
  readonly outDir?: string;
  /** deployment base path (default "/") — SW scope + preview mount */
  readonly basePath?: string;
  /** wipe <outDir>/browser before emission */
  readonly clean?: boolean;
  /** emit linked sourcemaps for the bundles (default false) */
  readonly sourceMap?: boolean;
  /** explicit kernel wasm override */
  readonly kernelWasmPath?: string;
  /**
   * B-006 self-probe: one declared route executed by the generated page
   * after boot, rendered into the status info (visible evidence of which
   * build served a request; dev-diagnostic shell surface only).
   */
  readonly probe?: ProbeRequest;
}

export interface ProbeRequest {
  readonly method: "GET" | "POST" | "PUT" | "PATCH" | "DELETE";
  readonly path: string;
}

export interface ComposeResult {
  readonly browserDir: string;
  readonly buildId: string;
  readonly manifest: BrowserArtifactManifest;
  readonly kernel: {
    readonly kernelAbiVersion: number;
    readonly wasmSha256: string;
    readonly wasmBytes: number;
    readonly provenance: Record<string, unknown>;
  };
  readonly shellFiles: Readonly<Record<string, { bytes: number; sha256: string }>>;
  readonly routes: number;
  readonly native: { outDir: string; buildMs: number };
}

function normalizeBasePath(raw: string | undefined): string {
  if (raw === undefined || raw === "") return "/";
  let p = raw.replace(/\\/g, "/");
  if (!p.startsWith("/")) p = `/${p}`;
  if (!p.endsWith("/")) p = `${p}/`;
  if (!/^\/[A-Za-z0-9._~/-]*$/.test(p)) {
    throw new BrowserDeployError("INVALID_OPTION", `invalid --base-path ${raw!}`);
  }
  return p;
}

/** Generated page bootstrap (bundled into page.js). */
function pageEntrySource(basePath: string, appId: string, probe: ProbeRequest | null): string {
  const probeLines = probe
    ? [
        "  // B-006 self-probe (build-time option): executes one declared route",
        "  // through the booted runtime and renders the result — visible",
        "  // evidence of WHICH build served the request (no ambient globals).",
        `  const probeRes = await runtime.fetch(new Request("http://velqu.local${probe.path}", { method: ${JSON.stringify(probe.method)} }));`,
        "  const probeBody = await probeRes.text();",
      ].join("\n")
    : "  const probeRes = null; const probeBody = null;";
  const probeInfo = probe
    ? `      probe: { method: ${JSON.stringify(probe.method)}, path: ${JSON.stringify(probe.path)}, status: probeRes && probeRes.status, body: probeBody },`
    : "      probe: null,";
  return `\
// @generated by velqu build --target browser-wasm (BWASM-B-005/B-006) — do not edit
// Boots the kernel-backed runtime from loader-verified artifacts and
// registers the Service Worker (B-004). Handlers execute in an isolated
// Worker (worker.js); this page exposes NO ambient globals.
//
// B-006 availability policy: the fresh deployment is tried first; when it
// is unreachable, interrupted, corrupt, or mixed, the page falls back to
// the newest FULLY VERIFIED cached build (loadArtifactsWithFallback) —
// the last known-good build stays usable; an unverified set never boots.
import {
  loadArtifactsWithFallback,
  createBrowserRuntime,
  WorkerHost,
  bootstrapServiceWorker,
} from "@velqu/browser-runtime";
import { WasmKernel, kernel_abi_version, initKernelSync } from "./kernel.js";

const base = new URL(".", new URL(import.meta.url));
const statusEl = document.getElementById("velqu-status");
const setStatus = (text) => { if (statusEl) statusEl.textContent = text; };

try {
  setStatus("loading verified artifacts…");
  const loaded = await loadArtifactsWithFallback({
    fetch: (url) => fetch(url),
    caches,
    manifestUrl: new URL("velqu-artifacts.json", base).href,
    baseUrl: base.href,
    appId: ${JSON.stringify(appId)},
  });

  setStatus("initializing kernel…");
  initKernelSync(loaded.bytes.kernelWasm);
  const sessionId = crypto.randomUUID();
  const host = new WorkerHost(sessionId, () => {
    const worker = new Worker(new URL("worker.js", base).href, { type: "module" });
    // FIFO-first message: registers the handler table for this session
    // (the R-004 protocol gates invokes on the session id).
    worker.postMessage({ type: "velqu-session", sessionId });
    return worker;
  });
  function VelquKernel(packBytes) { return new WasmKernel(packBytes); }
  VelquKernel.kernel_abi_version = kernel_abi_version;
  const runtime = createBrowserRuntime({
    packBytes: loaded.bytes.pack,
    kernel: VelquKernel,
    executeHandler: (plan) => host.execute(plan),
  });
${probeLines}

  const swOutcome = await bootstrapServiceWorker({
    scriptUrl: new URL("service-worker.js", base).href,
    scope: ${JSON.stringify(basePath)},
  });

  setStatus(
    "ready — build " + loaded.buildId.slice(0, 12) +
    " (" + loaded.source + ")" +
    " · worker " + (runtime.state) +
    " · sw: " + swOutcome.kind,
  );
  const info = document.getElementById("velqu-info");
  if (info) {
    info.textContent = JSON.stringify({
      buildId: loaded.buildId,
      source: loaded.source,
      appId: loaded.manifest.appId,
      handlerAbiVersion: loaded.manifest.handlerAbiVersion,
      kernelAbiVersion: loaded.manifest.kernelAbiVersion,
      serviceWorker: swOutcome.kind,
${probeInfo}
      reason: swOutcome.kind === "injected-fetch-fallback" ? swOutcome.reason : undefined,
    }, null, 2);
  }
} catch (cause) {
  setStatus("failed: " + String(cause && cause.message ? cause.message : cause));
}
`;
}
/** Generated worker entry (bundled into worker.js). */
function workerEntrySource(): string {
  return `\
// @generated by velqu build --target browser-wasm (BWASM-B-005) — do not edit
// Isolation honesty: a same-origin Worker is NOT a hostile-code sandbox;
// it executes trusted application handlers (ADR-0037/0038 posture).
import { handlers as handlerTable } from "./app.browser.js";
${workerBootstrapSource()}
// The R-004 bootstrap owns self.onmessage; wrap it to accept the host's
// first "velqu-session" message (FIFO before any invoke) which registers
// the handler table under the page's session id.
const bootstrapOnMessage = self.onmessage;
self.onmessage = (event) => {
  const msg = event.data;
  if (msg && msg.type === "velqu-session" && typeof msg.sessionId === "string") {
    self.velquRegisterHandlers(handlerTable, msg.sessionId);
    return;
  }
  if (typeof bootstrapOnMessage === "function") bootstrapOnMessage(event);
};
`;
}

/** Generated Service Worker (bundled into service-worker.js). */
function serviceWorkerEntrySource(basePath: string, appId: string, deploymentSha256: string): string {
  return `\
// @generated by velqu build --target browser-wasm (BWASM-B-005/B-006) — do not edit
// Deployment identity: ${deploymentSha256}  (sha256 of velqu-artifacts.json)
// The constant above makes the SW script bytes differ per deployment, so
// the browser's update check actually fires on every redeploy (B-006).
//
// B-004 composition, asset/offline lane. The SW NEVER executes handlers —
// ADR-0037 keeps handler execution in the page's isolated Worker;
// SW-served API execution is out of MVP scope (Q-002 owns the lanes).
//
// B-006 activation contract:
// - install prefetches the manifest plan WITH digest verification plus
//   the app shell; any failure throws and the new SW never activates —
//   interrupted or corrupt updates cannot displace the known-good build;
// - activate keeps this build's cache plus at most one previous build
//   cache (cachesToKeep) — bounded growth, one-step rollback, and no
//   active client's artifacts are touched (per-build isolation);
// - navigation is network-first with the cached shell as offline
//   fallback (apply-on-next-reload: a fresh online reload revalidates).
import {
  buildCachePlan,
  cacheNameFor,
  handleFetchEvent,
  precacheVerified,
  cachesToKeep,
  sha256Hex,
  SW_SHELL_FILES,
} from "@velqu/browser-runtime";

const DEPLOYMENT_SHA256 = ${JSON.stringify(deploymentSha256)};
const APP_ID = ${JSON.stringify(appId)};
const SCOPE = ${JSON.stringify(basePath)};
const BASE_URL = self.registration.scope;
const MANIFEST_URL = new URL("velqu-artifacts.json", BASE_URL);

// SW module state is per-instance: a restarted worker starts empty, so
// every lifecycle path rehydrates through ensureReady() — never trusting
// in-memory state across restarts (B-006).
let manifest = null;   // BrowserArtifactManifest of THIS deployment
let cache = null;      // its content-addressed cache

/**
 * Rehydrate this deployment's state. The SW is cryptographically
 * self-identifying: the cached manifest whose bytes hash to
 * DEPLOYMENT_SHA256 is THIS deployment's cache — an older SW instance
 * keeps serving its own build even after newer builds cached beside it
 * (no mixed state across generations). With no matching cache (first
 * run, or evicted), the manifest comes from the network.
 */
async function ensureReady() {
  if (manifest && cache) return;
  for (const name of await caches.keys()) {
    if (!name.startsWith("velqu:" + APP_ID + ":")) continue;
    const candidate = await caches.open(name);
    const cached = await candidate.match(MANIFEST_URL.href);
    if (!cached) continue;
    const bytes = new Uint8Array(await cached.clone().arrayBuffer());
    if ((await sha256Hex(bytes)) === DEPLOYMENT_SHA256) {
      manifest = JSON.parse(new TextDecoder().decode(bytes));
      cache = candidate;
      return;
    }
  }
  const response = await fetch(MANIFEST_URL);
  if (!response.ok) throw new Error("velqu sw: manifest HTTP " + response.status);
  manifest = await response.json();
  cache = await caches.open(cacheNameFor(manifest.appId, manifest.buildId));
}

async function prefetch(url, sha256) {
  await precacheVerified(url, sha256, { fetch: (u) => fetch(u), cache });
}

self.addEventListener("install", (event) => {
  event.waitUntil((async () => {
    await ensureReady();
    // Manifest plan: digest-verified (a mixed or corrupt artifact fails
    // the install; the previous SW stays active).
    for (const entry of buildCachePlan(manifest, BASE_URL)) {
      await prefetch(entry.url, entry.sha256);
    }
    // App shell: availability caching (outside the B-002 role enum, so
    // no per-file digest contract — the runtime's integrity boundary is
    // the manifest roles).
    for (const shell of SW_SHELL_FILES) {
      await prefetch(new URL(shell, BASE_URL).href, null);
    }
    // Navigation fallback: the shell under the scope root too, so an
    // offline reload of "/" (or the base path) hits the cache.
    await cache.put(BASE_URL, (await fetch(new URL("index.html", BASE_URL).href)).clone());
  })());
});

// Apply-on-next-reload (B-006 / ADR-0039 §4): the page posts
// VELQU_APPLY_UPDATE only after an explicit user/developer action — the
// waiting deployment activates immediately and takes over on the next
// load. Never a mid-session swap of a running build.
self.addEventListener("message", (event) => {
  if (event.data && event.data.type === "VELQU_APPLY_UPDATE") {
    self.skipWaiting();
  }
});

self.addEventListener("activate", (event) => {
  event.waitUntil((async () => {
    await ensureReady();
    const { drop } = cachesToKeep(APP_ID, manifest.buildId, await caches.keys());
    for (const name of drop) await caches.delete(name);
  })());
});

// Lazy cache facade: every use rehydrates first, so a restarted SW serves
// from the right deployment without module state.
const lazyCache = {
  match: async (url) => { await ensureReady(); return cache.match(url); },
  put: async (url, response) => { await ensureReady(); return cache.put(url, response); },
};

self.addEventListener("fetch", (event) => {
  handleFetchEvent(event, {
    scope: SCOPE,
    locationOrigin: self.location.origin,
    manifest,
    runtimeFetch: async (request) => {
      await ensureReady();
      if (request.mode === "navigate") {
        try {
          // Re-fetch a CONSTRUCTED request, never the navigation Request
          // itself: an offline re-fetch of a navigate-mode Request pends
          // forever in Chromium instead of rejecting (B-006 finding), so
          // the offline shell fallback would never run.
          return await fetch(new Request(request.url, { headers: request.headers }));
        } catch {
          const shell = (await cache.match(new URL("index.html", BASE_URL).href))
            ?? (await cache.match(BASE_URL));
          if (shell) return shell;
          throw new Error("offline and no cached shell for " + request.url);
        }
      }
      return fetch(request);
    },
    cache: lazyCache,
    baseUrl: BASE_URL,
  });
});
`;
}

function indexHtmlSource(): string {
  return `<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Velqu browser-wasm deployment</title>
<style>
  :root { color-scheme: light dark; }
  body { font: 14px/1.5 ui-monospace, monospace; margin: 0; padding: 2rem; }
  main { max-width: 46rem; margin: 0 auto; }
  h1 { font-size: 1.1rem; }
  #velqu-status { font-weight: bold; }
  pre { background: rgba(127,127,127,.12); padding: 1rem; overflow: auto; }
</style>
</head>
<body>
<main>
  <h1>Velqu browser-wasm deployment</h1>
  <p id="velqu-status">loading…</p>
  <pre><code id="velqu-info"></code></pre>
  <p>This shell is a development/status page generated by
  <code>velqu build --target browser-wasm</code>. Application UIs bundle
  their own page against the same artifacts; the runtime boundary is
  <code>fetch(Request) =&gt; Promise&lt;Response&gt;</code>.</p>
</main>
<script type="module" src="./page.js"></script>
</body>
</html>
`;
}

async function bundleEntries(
  browserDir: string,
  entries: ReadonlyArray<string>,
  sourceMap: boolean,
): Promise<void> {
  let result: Awaited<ReturnType<typeof Bun.build>>;
  try {
    result = await Bun.build({
      entrypoints: entries.map((name) => join(browserDir, name)),
      outdir: browserDir,
      target: "browser",
      format: "esm",
      sourcemap: sourceMap ? "linked" : "none",
      naming: "[name].[ext]",
    });
  } catch (cause) {
    const errors = (cause as { errors?: unknown[] }).errors ?? [];
    const details = [String(cause), ...errors.map((e) => String((e as { message?: string })?.message ?? e))].join("; ");
    throw new BrowserDeployError("BUNDLE_FAILED", `bundling deployment entries failed: ${details}`);
  }
  if (!result.success) {
    const details = result.logs.map((l) => String(l)).join("; ");
    throw new BrowserDeployError("BUNDLE_FAILED", `bundling deployment entries failed: ${details}`);
  }
}

/**
 * The full browser-wasm build pipeline (native build → B-001 emission →
 * kernel → bundles → shell → B-002 content-addressed manifest).
 */
export async function composeBrowserDeployment(opts: ComposeOptions): Promise<ComposeResult> {
  const basePath = normalizeBasePath(opts.basePath);
  const native: BuildResult = await nativeBuild({
    project: opts.project,
    outDir: opts.outDir,
  });
  const entry = resolveEntry(opts.project);
  const app = extractApp(entry);
  const b001 = buildBrowserWasmArtifacts(
    { project: entry, outDir: native.outDir },
    app,
    native.outDir,
  );
  const browserDir = b001.outDir;
  if (opts.clean && existsSync(browserDir)) rmSync(browserDir, { recursive: true, force: true });
  mkdirSync(browserDir, { recursive: true });
  if (!existsSync(join(browserDir, "app.browser.js"))) {
    // clean wiped the B-001 emission — re-emit deterministically
    buildBrowserWasmArtifacts({ project: entry, outDir: native.outDir }, app, native.outDir);
  }

  // --- kernel artifacts (pinned) ---
  const kernel = resolveKernelArtifacts(opts.kernelWasmPath);
  writeFileSync(join(browserDir, "kernel.wasm"), kernel.wasmBytes);
  writeFileSync(join(browserDir, "kernel.js"), browserKernelModuleSource(kernel.glueSource));

  // --- generated entries → bundles ---
  // Entries are written beside their imports (same-directory "./"
  // specifiers): Bun's bundler resolves parent-relative specifiers
  // through the invoking module's tsconfig context, which the test
  // runner overrides — same-dir specifiers resolve identically
  // everywhere. Each bundle output overwrites its own entry file
  // (entries never import each other), so the deployed names land
  // directly: page.js, worker.js, service-worker.js, app.bundle.js.
  const appId = app.appId;
  writeFileSync(join(browserDir, "page.js"), pageEntrySource(basePath, appId, opts.probe ?? null));
  writeFileSync(join(browserDir, "worker.js"), workerEntrySource());
  writeFileSync(
    join(browserDir, "app.bundle.js"),
    "// @generated by velqu build --target browser-wasm — handler bundle entry\nexport { handlers } from \"./app.browser.js\";\n",
  );
  await bundleEntries(
    browserDir,
    ["page.js", "worker.js", "app.bundle.js"],
    opts.sourceMap ?? false,
  );

  // --- B-002 content-addressed manifest over the deployed roles ---
  const roleUrls: ReadonlyArray<[ArtifactRole, string]> = opts.sourceMap
    ? [
        ["pack", "app.qpack"],
        ["kernelWasm", "kernel.wasm"],
        ["handlerBundle", "app.bundle.js"],
        ["manifest", "browser-manifest.json"],
        ["contract", "contract.json"],
        ["schemas", "schema-manifest.json"],
        ["capabilities", "capability-manifest.json"],
        ["sourceMap", "app.bundle.js.map"],
      ]
    : [
        ["pack", "app.qpack"],
        ["kernelWasm", "kernel.wasm"],
        ["handlerBundle", "app.bundle.js"],
        ["manifest", "browser-manifest.json"],
        ["contract", "contract.json"],
        ["schemas", "schema-manifest.json"],
        ["capabilities", "capability-manifest.json"],
      ];
  const artifacts = {} as Record<ArtifactRole, Uint8Array>;
  const urls = {} as Record<ArtifactRole, string>;
  for (const [role, url] of roleUrls) {
    const path = join(browserDir, url);
    if (!existsSync(path)) {
      throw new BrowserDeployError("MISSING_ARTIFACTS", `expected artifact missing after build: ${url}`);
    }
    artifacts[role] = new Uint8Array(readFileSync(path));
    urls[role] = url;
  }
  const browserManifest = JSON.parse(readFileSync(join(browserDir, "browser-manifest.json"), "utf8"));
  const { manifest, manifestJson } = await emitArtifactManifest({
    appId: browserManifest.appId as string,
    handlerAbiVersion: browserManifest.handlerAbiVersion as number,
    kernelAbiVersion: browserManifest.kernelAbiVersion as number,
    packSha256: browserManifest.packSha256 as string,
    // Partial role set is the loader contract (per-manifest role maps);
    // the emitter type overstates completeness — see B-002 loader keys().
    artifacts: artifacts as Record<ArtifactRole, Uint8Array>,
    urls,
  });
  writeFileSync(join(browserDir, "velqu-artifacts.json"), manifestJson);

  // --- Service Worker: bundled AFTER the manifest so its script bytes
  // embed the deployment identity (sha256 of velqu-artifacts.json) —
  // the browser update check only fires when SW bytes change (B-006).
  const deploymentSha256 = sha256Hex(new TextEncoder().encode(manifestJson));
  writeFileSync(
    join(browserDir, "service-worker.js"),
    serviceWorkerEntrySource(basePath, appId, deploymentSha256),
  );
  await bundleEntries(browserDir, ["service-worker.js"], opts.sourceMap ?? false);
  if (!opts.sourceMap) {
    for (const name of ["app.bundle.js", "page.js", "worker.js", "service-worker.js"]) {
      rmSync(join(browserDir, `${name}.map`), { force: true });
    }
  }
  writeFileSync(join(browserDir, "index.html"), indexHtmlSource());

  const shellFiles: Record<string, { bytes: number; sha256: string }> = {};
  for (const name of DEPLOY_SHELL_FILES) {
    const bytes = readFileSync(join(browserDir, name));
    shellFiles[name] = { bytes: bytes.byteLength, sha256: sha256Hex(new Uint8Array(bytes)) };
  }

  return {
    browserDir,
    buildId: manifest.buildId,
    manifest,
    kernel: {
      kernelAbiVersion: kernel.kernelAbiVersion,
      wasmSha256: kernel.wasmSha256,
      wasmBytes: kernel.wasmBytes.byteLength,
      provenance: kernel.provenance,
    },
    shellFiles,
    routes: b001.routes,
    native: { outDir: native.outDir, buildMs: native.buildMs },
  };
}

function resolveEntry(project: string): string {
  let st;
  try {
    st = statSync(project);
  } catch {
    throw new BrowserDeployError("INVALID_OPTION", `project path not found: ${project}`);
  }
  if (st.isDirectory()) {
    for (const c of ["src/app.ts", "app.ts", "src/index.ts"]) {
      const p = join(project, c);
      if (existsSync(p)) return p;
    }
    throw new BrowserDeployError("INVALID_OPTION", `no app entry found in ${project}`);
  }
  return project;
}

// ---------------------------------------------------------------------------
// Inspect (integrity + inventory)
// ---------------------------------------------------------------------------

export interface InspectProblem {
  readonly artifact: string;
  readonly reason: string;
  readonly detail: string;
}

export interface BrowserDeploymentInspect {
  readonly ok: boolean;
  readonly problems: ReadonlyArray<InspectProblem>;
  readonly buildId: string | null;
  readonly browserDir: string;
  readonly integrity: {
    readonly verified: boolean;
    readonly checkedArtifacts: number;
  };
  readonly targetCompatibility: {
    readonly target: string;
    readonly handlerAbiVersion: number | null;
    readonly kernelAbiVersion: number | null;
    readonly cliSupports: boolean;
  };
  readonly capabilities: {
    readonly declared: ReadonlyArray<string>;
    readonly perRoute: Record<string, ReadonlyArray<string>>;
    readonly nativeOps: Record<string, string>;
  };
  readonly deploymentRequirements: ReadonlyArray<string>;
  readonly artifactInventory: ReadonlyArray<{ file: string; bytes: number; sha256: string; role: string | "shell" | "build-input" }>;
}

function fsReader(root: string) {
  return async (url: string): Promise<Uint8Array> => {
    const path = resolve(root, url);
    if (!path.startsWith(resolve(root))) {
      throw new Error(`artifact url escapes deployment root: ${url}`);
    }
    return new Uint8Array(readFileSync(path));
  };
}

export async function inspectBrowserDeployment(
  browserDir: string,
): Promise<BrowserDeploymentInspect> {
  const manifestPath = join(browserDir, "velqu-artifacts.json");
  if (!existsSync(manifestPath)) {
    throw new BrowserDeployError(
      "NOT_A_BROWSER_DEPLOYMENT",
      `no velqu-artifacts.json in ${browserDir} — run 'velqu build --target browser-wasm' first`,
    );
  }
  const problems: InspectProblem[] = [];
  let buildId: string | null = null;
  let checked = 0;
  let manifest: BrowserArtifactManifest | null = null;
  try {
    const loaded = await loadArtifacts(readFileSync(manifestPath, "utf8"), fsReader(browserDir));
    buildId = loaded.buildId;
    manifest = loaded.manifest;
    checked = Object.keys(loaded.manifest.artifacts).length;
  } catch (cause) {
    const artifact = (cause as { artifact?: string }).artifact ?? "browser-manifest.json";
    const reason = (cause as { reason?: string }).reason ?? "tampered";
    problems.push({
      artifact,
      reason,
      detail: cause instanceof Error ? cause.message : String(cause),
    });
  }

  // Shell presence + digests (beyond the manifest roles).
  for (const name of DEPLOY_SHELL_FILES) {
    if (!existsSync(join(browserDir, name))) {
      problems.push({
        artifact: name,
        reason: "missing",
        detail: `deployment shell file missing: ${name}`,
      });
    }
  }

  const browserManifestPath = join(browserDir, "browser-manifest.json");
  const capsPath = join(browserDir, "capability-manifest.json");
  const browserManifest = existsSync(browserManifestPath)
    ? (JSON.parse(readFileSync(browserManifestPath, "utf8")) as {
        target: string;
        handlerAbiVersion: number;
        kernelAbiVersion: number;
      })
    : null;
  const caps = existsSync(capsPath)
    ? (JSON.parse(readFileSync(capsPath, "utf8")) as {
        declared: string[];
        perRoute: Record<string, string[]>;
        nativeOps: Record<string, string>;
      })
    : { declared: [], perRoute: {}, nativeOps: {} };

  const roleByUrl = new Map<string, string>();
  if (manifest) {
    for (const entry of Object.values(manifest.artifacts)) {
      roleByUrl.set(entry.url, entry.role);
    }
  }
  const shellSet = new Set<string>(DEPLOY_SHELL_FILES);
  const artifactInventory = readdirSync(browserDir)
    .filter((f) => statSync(join(browserDir, f)).isFile())
    .sort()
    .map((file) => {
      const bytes = readFileSync(join(browserDir, file));
      const role = file === "velqu-artifacts.json"
        ? "artifact-manifest"
        : roleByUrl.get(file) ?? (shellSet.has(file) ? "shell" : "build-input");
      return {
        file,
        bytes: bytes.byteLength,
        sha256: sha256Hex(new Uint8Array(bytes)),
        role,
      };
    });

  const deploymentRequirements: string[] = [
    "static hosting only — no Velqu application server is implied by this artifact set",
    "deployment-required imports were rejected at build time (BWASM-B-003 import policy)",
  ];
  if (caps.declared.length > 0) {
    deploymentRequirements.push(
      `declared capabilities require their host bridges: ${caps.declared.join(", ")}`,
    );
  }
  const nativeOpNames = Object.keys(caps.nativeOps);
  if (nativeOpNames.length > 0) {
    deploymentRequirements.push(
      `pack references native operations (${nativeOpNames.join(", ")}) — the browser lane provides them through the runtime host; native-only surfaces stay on the native runtime`,
    );
  }

  return {
    ok: problems.length === 0,
    problems,
    buildId,
    browserDir,
    integrity: { verified: problems.length === 0, checkedArtifacts: checked },
    targetCompatibility: {
      target: browserManifest?.target ?? "unknown",
      handlerAbiVersion: browserManifest?.handlerAbiVersion ?? null,
      kernelAbiVersion: browserManifest?.kernelAbiVersion ?? null,
      cliSupports: browserManifest?.target === "browser-wasm",
    },
    capabilities: {
      declared: caps.declared,
      perRoute: caps.perRoute,
      nativeOps: caps.nativeOps,
    },
    deploymentRequirements,
    artifactInventory,
  };
}

// ---------------------------------------------------------------------------
// Preview (static bytes + development diagnostics ONLY)
// ---------------------------------------------------------------------------

const PREVIEW_MEDIA_TYPES: ReadonlyArray<[RegExp, string]> = [
  [/\.html?$/, "text/html; charset=utf-8"],
  [/\.js$/, "text/javascript; charset=utf-8"],
  [/\.mjs$/, "text/javascript; charset=utf-8"],
  [/\.css$/, "text/css; charset=utf-8"],
  [/\.json$/, "application/json; charset=utf-8"],
  [/\.wasm$/, "application/wasm"],
  [/\.map$/, "application/json; charset=utf-8"],
  [/\.svg$/, "image/svg+xml"],
  [/\.ico$/, "image/x-icon"],
];

export const PREVIEW_DIAGNOSTICS_PATH = "/_velqu_preview/diagnostics";

export interface PreviewServer {
  readonly port: number;
  readonly baseUrl: string;
  stop(): Promise<void>;
}

/**
 * Development preview: serves ONLY the generated static bytes under
 * `browserDir` (mounted at basePath) plus one JSON diagnostics endpoint.
 * It is not a production runtime and adds no application behavior.
 */
export async function startPreviewServer(options: {
  browserDir: string;
  port: number;
  basePath?: string;
}): Promise<PreviewServer> {
  const basePath = normalizeBasePath(options.basePath);
  const inspect = await inspectBrowserDeployment(options.browserDir);
  const server = Bun.serve({
    port: options.port,
    async fetch(request): Promise<Response> {
      const headers = new Headers({ "x-velqu-preview": "dev", "cache-control": "no-store" });
      const url = new URL(request.url);
      let pathname: string;
      try {
        pathname = decodeURIComponent(url.pathname);
      } catch {
        return new Response("bad request", { status: 400, headers });
      }
      if (basePath !== "/" && pathname.startsWith(basePath)) {
        pathname = pathname.slice(basePath.length - 1);
      }
      if (request.method !== "GET" && request.method !== "HEAD") {
        return new Response("method not allowed", { status: 405, headers });
      }
      if (pathname === PREVIEW_DIAGNOSTICS_PATH) {
        const body = JSON.stringify(
          {
            schemaVersion: BROWSER_DEPLOY_JSON_SCHEMA_VERSION,
            status: "ok",
            command: "preview",
            production: false,
            note: "development preview — static generated bytes plus diagnostics only; not a Velqu application server",
            buildId: inspect.buildId,
            integrity: inspect.integrity,
            targetCompatibility: inspect.targetCompatibility,
            capabilities: { declared: inspect.capabilities.declared, nativeOps: inspect.capabilities.nativeOps },
            deploymentRequirements: inspect.deploymentRequirements,
          },
          null,
          2,
        );
        return new Response(body, { headers });
      }
      // Static files only; no traversal, no hidden files, no _entries.
      if (pathname.includes("..") || pathname.includes("\0") || pathname.startsWith("/_entries")) {
        return new Response("not found", { status: 404, headers });
      }
      const rel = pathname.replace(/^\/+/, "");
      const filePath = resolve(options.browserDir, rel);
      if (!filePath.startsWith(resolve(options.browserDir)) || !existsSync(filePath) || !statSync(filePath).isFile()) {
        return new Response(
          JSON.stringify({ problemId: "not-found", status: 404, title: "Not Found", detail: pathname }),
          { status: 404, headers: new Headers({ "content-type": "application/problem+json", "x-velqu-preview": "dev" }) },
        );
      }
      const media = PREVIEW_MEDIA_TYPES.find(([re]) => re.test(filePath))?.[1] ?? "application/octet-stream";
      headers.set("content-type", media);
      return new Response(readFileSync(filePath), { headers });
    },
  });
  return {
    port: server.port ?? options.port,
    baseUrl: `http://127.0.0.1:${server.port}${basePath === "/" ? "" : basePath.slice(0, -1)}`,
    stop: () => {
      server.stop(true);
      return Promise.resolve();
    },
  };
}

// ---------------------------------------------------------------------------
// Export (verified clean deployment copy)
// ---------------------------------------------------------------------------

export interface ExportResult {
  readonly sourceDir: string;
  readonly outDir: string;
  readonly buildId: string | null;
  readonly files: ReadonlyArray<{ file: string; bytes: number; sha256: string }>;
  readonly totalBytes: number;
}

/**
 * Copy the exact deployment set (manifest artifacts + shell) into a clean
 * directory. Fails closed: a tampered or mixed artifact set cannot be
 * exported — inspect runs first and must verify.
 */
export async function exportDeployment(
  browserDir: string,
  outDir: string,
): Promise<ExportResult> {
  const inspect = await inspectBrowserDeployment(browserDir);
  if (!inspect.ok) {
    throw new BrowserDeployError(
      "INTEGRITY_FAILED",
      `refusing to export: ${inspect.problems.map((p) => `${p.artifact}:${p.reason}`).join("; ")}`,
    );
  }
  const roleFiles = inspect.artifactInventory
    .filter((f) => f.role !== "build-input")
    .map((f) => f.file);
  mkdirSync(outDir, { recursive: true });
  const files: { file: string; bytes: number; sha256: string }[] = [];
  let total = 0;
  for (const file of roleFiles.sort()) {
    const src = join(browserDir, file);
    const dst = join(outDir, file);
    copyFileSync(src, dst);
    const bytes = statSync(src).size;
    files.push({ file, bytes, sha256: sha256Hex(new Uint8Array(readFileSync(src))) });
    total += bytes;
  }
  return {
    sourceDir: browserDir,
    outDir,
    buildId: inspect.buildId,
    files,
    totalBytes: total,
  };
}

// Re-export for CLI wiring so diagnostics stay in one place.
export function relativeTo(cwd: string, path: string): string {
  return relative(cwd, path) || path;
}
