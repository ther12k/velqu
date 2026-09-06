/**
 * BWASM-B-001 — the `browser-wasm` compiler target.
 *
 * Emits a deterministic, self-contained browser artifact directory from
 * an application project, WITHOUT touching the native QPack emission
 * (`build()` output is unchanged; the native artifacts are consumed as
 * inputs and carried into the browser set byte-identically).
 *
 * Emitted set (all paths inside `<outDir>/browser/`):
 * - `app.browser.js` — handler-bundle ES module registering through the
 *   narrow R-003 API (`defineBrowserHandlers`); NO ambient globals.
 * - `app.qpack` — the verified pack bytes (byte-identical to native).
 * - `browser-manifest.json` — target identity, ABI versions, pack hash,
 *   sorted handler/status table (deterministic; no wall clock, no
 *   absolute paths).
 * - `contract.json`, `schema-manifest.json`, `capability-manifest.json`
 *   carried from the native build (self-contained directory).
 *
 * Build-time diagnostics (fail before any browser run, acceptance 4):
 * - native-liveness routes (RUN-009) are native-only surfaces —
 *   rejected with a source-located CompileError (ADR-0037 §1).
 * - workspace paths are sanitized out of emitted metadata (acceptance 5).
 */

import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { join, relative } from "node:path";
import { extractApp, CompileError, type ExtractedApp } from "./extract";

/** Browser target identity carried in the manifest. */
export const BROWSER_TARGET = "browser-wasm" as const;

/** ABI versions this emitter produces against (R-003 / K-005). */
export const EMITTED_HANDLER_ABI_VERSION = 1;
export const EMITTED_KERNEL_ABI_VERSION = 1;

export interface BrowserWasmBuildOptions {
  /** path to the app entry (app.ts) or its directory */
  project: string;
  /** build output root; browser set lands in `<outDir>/browser` */
  outDir: string;
}

export interface BrowserWasmBuildResult {
  readonly outDir: string;
  readonly files: Record<string, string>;
  readonly routes: number;
}

/** Host-path sanitizer: no workspace prefix crosses into artifacts. */
function sanitizeSourceLocation(raw: string): string {
  const markers = ["/src/", "/packages/", "/examples/", "/modules/"];
  let path = raw.replace(/\\/g, "/").replace(/\/+/g, "/");
  for (const marker of markers) {
    const idx = path.indexOf(marker);
    if (idx > 0) {
      path = path.slice(idx + 1);
      break;
    }
  }
  return path.replace(/^\/+/, "");
}

/** Emit the deterministic browser artifact set for one app project. */
export function buildBrowserWasmArtifacts(
  opts: BrowserWasmBuildOptions,
  app: ExtractedApp,
  nativeOutDir: string,
): BrowserWasmBuildResult {
  // Build-time diagnostics FIRST: native-only constructs must fail HERE,
  // before any artifact directory is created (nothing is emitted on the
  // diagnostic path).
  for (const r of app.routes) {
    if (r.liveness) {
      throw new CompileError(
        `browser-wasm target: route "${r.id}" uses native liveness (RUN-009), ` +
          `which the browser kernel does not provide — remove it or split it ` +
          `into a native-only service (${sanitizeSourceLocation(r.sourceFile)})`,
      );
    }
  }

  const browserDir = join(opts.outDir, "browser");
  mkdirSync(browserDir, { recursive: true });

  // Handler keys/statuses come from the app's declared routes (the same
  // table the pack manifest and the kernel verify against).
  const routeRows = app.routes.map((r) => ({
    id: r.id,
    statuses: Object.keys(r.responses)
      .map((s) => Number.parseInt(s, 10))
      .sort((a, b) => a - b),
    sourceFile: sanitizeSourceLocation(r.sourceFile),
    bindingName: r.bindingName,
    sourceFileRaw: r.sourceFile,
  }));

  // --- app.browser.js (handler bundle entry; ES module source form —
  // the consumer's bundler packs it; B-005 adds the CLI workflow) ---
  const sourceFiles = [...new Set(routeRows.map((r) => r.sourceFileRaw))].sort();
  const nsByFile = new Map(sourceFiles.map((f, i) => [f, `m${i}`] as const));
  const importLines = sourceFiles.map((f, i) => {
    // Relative from the emitted file's directory; the emitted path is
    // deterministic given the project layout.
    const rel = relative(browserDir, f).replace(/\\/g, "/");
    return `import * as m${i} from ${JSON.stringify(rel.startsWith(".") ? rel : `./${rel}`)};`;
  });
  const registrationLines = routeRows.map((r) => {
    const ns = nsByFile.get(r.sourceFileRaw)!;
    const isDefault = r.bindingName === "__default_export";
    const access = `${ns}.${isDefault ? "default" : r.bindingName}`;
    const defaultStatus = r.statuses[0] ?? 200;
    return [
      `  {`,
      `    handlerKey: ${JSON.stringify(r.id)},`,
      `    statuses: [${r.statuses.join(", ")}],`,
      `    source: ${JSON.stringify(r.sourceFile)},`,
      `    invoke: async (ctx) => {`,
      `      const result = await (${access}).handle(ctx);`,
      `      if (result && typeof result === "object" && "kind" in result) return result;`,
      `      return { kind: "response", status: ${defaultStatus}, headers: [["content-type", "application/json"]], body: result ?? {} };`,
      `    },`,
      `  },`,
    ].join("\n");
  });
  const expectationLines = [
    `  requiredHandlerKeys: [${routeRows.map((r) => JSON.stringify(r.id)).sort().join(", ")}],`,
    `  declaredStatuses: {`,
    ...routeRows.map((r) => `    ${JSON.stringify(r.id)}: [${r.statuses.join(", ")}],`),
    `  },`,
    `  handlerAbiVersion: ${EMITTED_HANDLER_ABI_VERSION},`,
  ].join("\n");
  const appBrowserJs = [
    "// @generated by @velqu/compiler (browser-wasm target) — do not edit",
    `// Handler ABI v${EMITTED_HANDLER_ABI_VERSION}; kernel ABI v${EMITTED_KERNEL_ABI_VERSION} (ADR-0037)`,
    'import { defineBrowserHandlers } from "@velqu/browser-runtime";',
    ...importLines,
    "",
    "const registrations = [",
    registrationLines.join("\n"),
    "];",
    "",
    "export const handlers = defineBrowserHandlers(registrations, {",
    expectationLines,
    "});",
    "",
  ].join("\n");

  // --- browser-manifest.json (deterministic: sorted, hashed, no paths) ---
  const packBytes = readFileSync(join(nativeOutDir, "app.qpack"));
  const packSha256 = createHash("sha256").update(packBytes).digest("hex");
  const manifest = {
    formatVersion: 1,
    target: BROWSER_TARGET,
    handlerAbiVersion: EMITTED_HANDLER_ABI_VERSION,
    kernelAbiVersion: EMITTED_KERNEL_ABI_VERSION,
    appId: app.appId,
    packSha256,
    handlers: routeRows
      .map((r) => ({ handlerKey: r.id, statuses: r.statuses, source: r.sourceFile }))
      .sort((a, b) => (a.handlerKey < b.handlerKey ? -1 : 1)),
  };

  // --- self-contained set: carry the verified pack + shared manifests ---
  const carried = ["app.qpack", "schema-manifest.json", "capability-manifest.json", "contract.json"];

  const files: Record<string, string> = {
    "app.browser.js": appBrowserJs,
    "browser-manifest.json": `${JSON.stringify(manifest, null, 2)}\n`,
  };
  for (const name of carried) {
    files[name] = readFileSync(join(nativeOutDir, name), "utf8");
  }

  for (const [name, content] of Object.entries(files)) {
    writeFileSync(join(browserDir, name), content);
  }

  return {
    outDir: browserDir,
    files: Object.fromEntries(
      Object.entries(files).map(([name, content]) => [
        name,
        createHash("sha256").update(content).digest("hex"),
      ]),
    ),
    routes: app.routes.length,
  };
}
