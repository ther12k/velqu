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
 * - route bindings that are not exported from their source module are
 *   rejected (#1292): the emitted bundle imports handlers by name, so a
 *   non-exported const would silently become `undefined` downstream.
 * - workspace paths are sanitized out of emitted metadata (acceptance 5).
 */

import { existsSync, mkdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { join, relative } from "node:path";
import { extractApp, CompileError, type ExtractedApp } from "./extract";
import {
  scanImportPolicy,
  assertImportPolicyOk,
  IMPORT_POLICY_VERSION,
} from "./import-policy";
import { classifyCapability, CAPABILITY_PORTABILITY_REGISTRY } from "./capability-portability";

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
  /**
   * BWASM-C-005: explicit simulation profile. Default FALSE — routes
   * declaring deployment-required capabilities fail the browser-wasm
   * build (statically known impossible). With this flag the build
   * records the simulated state in the manifest instead of refusing;
   * it NEVER provides a mock implementation.
   */
  simulate?: boolean;
}

export interface BrowserWasmBuildResult {
  readonly outDir: string;
  readonly files: Record<string, string>;
  readonly routes: number;
}

function entryFileOf(project: string, app: ExtractedApp): string {
  // extractApp resolved the entry already; the first route's source file
  // is inside the app tree — use the project path forms the CLI accepts.
  const candidates = [
    project,
    join(project, "src", "app.ts"),
    join(project, "app.ts"),
    join(project, "src", "index.ts"),
  ];
  for (const c of candidates) {
    if (existsSync(c) && statSync(c).isFile()) return c;
  }
  // Fall back to the first route module's file (app trees always have one).
  return app.routes[0]?.sourceFile ?? project;
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
  // diagnostic path). BWASM-B-003: the browser import policy runs before
  // emission too — forbidden/transitive/deployment-required imports block
  // the build (fail closed; ADR-0038 §5 trusted-handler enforcement).
  assertImportPolicyOk(scanImportPolicy(entryFileOf(opts.project, app)));
  for (const r of app.routes) {
    if (r.liveness) {
      throw new CompileError(
        `browser-wasm target: route "${r.id}" uses native liveness (RUN-009), ` +
          `which the browser kernel does not provide — remove it or split it ` +
          `into a native-only service (${sanitizeSourceLocation(r.sourceFile)})`,
      );
    }
    // #1292: the emitted bundle references each handler binding through a
    // namespace import of its source module. A non-exported binding folds
    // to `undefined` at consumer-bundle time and every invocation of the
    // route fails — fail the build with a source-located diagnostic instead.
    if (!r.exported) {
      throw new CompileError(
        `browser-wasm target: route "${r.id}" is bound to "${r.bindingName}", ` +
          `which is not exported from its source module — the emitted handler ` +
          `bundle imports it by name and would receive \`undefined\` at runtime. ` +
          `Add 'export' to the declaration (or re-export it: export { ${r.bindingName} }) ` +
          `(${sanitizeSourceLocation(r.sourceFile)})`,
      );
    }
    // BWASM-C-005: fail at build time when capability usage is statically
    // known impossible for the selected target (deployment-required or
    // forbidden). Unknown names classify as forbidden (fail closed).
    for (const grant of r.capabilities) {
      const { state, remediation } = classifyCapability(grant);
      if (state === "deployment-required" && !opts.simulate) {
        throw new CompileError(
          `browser-wasm target: route "${r.id}" declares capability "${grant}", ` +
            `which is deployment-required — it cannot run in a browser. ` +
            `Remediation: ${remediation} (${sanitizeSourceLocation(r.sourceFile)})`,
        );
      }
      if (state === "forbidden") {
        throw new CompileError(
          `browser-wasm target: route "${r.id}" declares capability "${grant}", ` +
            `which does not exist and is never simulated — unknown or reserved ` +
            `classifications fail closed (${sanitizeSourceLocation(r.sourceFile)})`,
        );
      }
    }
  }
  void CAPABILITY_PORTABILITY_REGISTRY;

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
      `      // BWASM-Q-007 (D4): the @velqu/core result protocol crosses`,
      `      // status(...).value(...) / status(...).problem(...) / .raw(...)`,
      `      // as {__ok, status, value} / {__problem, ...} / {__velquRaw, ...}.`,
      `      // These map to their DECLARED statuses; wrapping them into the`,
      `      // default-status body made every non-default return a 500.`,
      `      if (result && typeof result === "object" && "__velquRaw" in result) {`,
      `        return {`,
      `          kind: "response",`,
      `          status: result.status,`,
      `          headers: Object.entries(result.headers ?? {}),`,
      `          body: result.body,`,
      `        };`,
      `      }`,
      `      if (result && typeof result === "object" && "__problem" in result) {`,
      `        return {`,
      `          kind: "problem",`,
      `          problemId: result.problem,`,
      `          status: result.status,`,
      `          ...(result.detail !== undefined ? { detail: result.detail } : {}),`,
      `          ...(result.errors !== undefined ? { errors: result.errors } : {}),`,
      `        };`,
      `      }`,
      `      if (result && typeof result === "object" && "__ok" in result) {`,
      `        return {`,
      `          kind: "response",`,
      `          status: result.status,`,
      `          headers: [["content-type", "application/json"]],`,
      `          body: result.value ?? {},`,
      `        };`,
      `      }`,
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
  const simulatedGrants = opts.simulate
    ? [...new Set(app.routes.flatMap((r) => r.capabilities))].filter(
        (g) => classifyCapability(g).state === "deployment-required",
      )
    : [];
  const manifest = {
    formatVersion: 1,
    target: BROWSER_TARGET,
    importPolicyVersion: IMPORT_POLICY_VERSION,
    handlerAbiVersion: EMITTED_HANDLER_ABI_VERSION,
    kernelAbiVersion: EMITTED_KERNEL_ABI_VERSION,
    appId: app.appId,
    packSha256,
    ...(simulatedGrants.length > 0 ? { simulatedCapabilities: simulatedGrants.sort() } : {}),
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
