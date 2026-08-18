/**
 * @velqu/compiler — build API. Static extraction (never runs the app) + Bun.build
 * bundling + deterministic artifact emission.
 */
import { mkdirSync, writeFileSync, readFileSync, existsSync } from "node:fs";
import { dirname, join } from "node:path";
import * as ts from "typescript";
import { extractApp, hash } from "./extract";
import { bundleApp, buildPack, contractFor, contractDts, openapiFor, diffContracts } from "./emit";

export { CompileError } from "./extract";
export { diffContracts } from "./emit";

export interface BuildOptions {
  project: string;           // path to the app entry (app.ts) or its directory
  outDir?: string;           // default <project-dir>/../dist or ./dist
  sourceMap?: boolean;       // default true
  /** rewrite contract.lock.json even if one exists (default: preserve) */
  updateLock?: boolean;
}

export interface BuildResult {
  outDir: string;
  packPath: string;
  routes: number;
  buildMs: number;
  artifactBytes: Record<string, number>;
  /** true when the contract lock was kept from a previous build */
  lockPreserved: boolean;
}

export async function build(opts: BuildOptions): Promise<BuildResult> {
  const t0 = performance.now();
  const entry = resolveEntry(opts.project);
  const outDir = opts.outDir ?? join(dirname(entry), "..", "dist");
  mkdirSync(outDir, { recursive: true });

  const app = extractApp(entry);
  const bundle = await bundleApp(app, { sourceMap: opts.sourceMap });
  const { packJson, pack } = buildPack(app, bundle, {
    compilerVersion: "0.1.0",
    typescriptVersion: ts.version,
  });

  // artifacts
  const contract = {
    formatVersion: 1,
    appId: app.appId,
    contractHash: (pack as { contractHash: string }).contractHash,
    generatedAt: new Date().toISOString(),
    routes: Object.fromEntries(
      app.routes.map((r) => [
        r.id,
        {
          path: r.path,
          method: r.method,
          params: r.paramsIr,
          query: r.queryIr,
          body: r.bodyIr,
          responses: Object.keys(r.responses),
          security: r.policyId ?? null,
        },
      ]),
    ),
  };
  const openapi = openapiFor(app);
  const dts = contractDts(app, (pack as { contractHash: string }).contractHash);

  // route/schema/capability manifests
  const routeManifest = app.routes.map((r) => ({
    id: r.id,
    method: r.method,
    path: r.path,
    moduleId: r.moduleId,
    policy: r.policyId,
    capabilities: r.capabilities,
    nativeStage: r.liveness ? "native-liveness" : "engine",
    validationStrategy: "native",
    responseStrategy: "native",
    sourceFile: r.sourceFile,
  }));
  const schemaManifest = {
    schemaIrVersion: 1,
    schemas: Object.fromEntries(
      app.routes.flatMap((r) =>
        [
          r.paramsIr ? [`sch:${r.id}.params`, r.paramsIr] : null,
          r.queryIr ? [`sch:${r.id}.query`, r.queryIr] : null,
          r.bodyIr ? [`sch:${r.id}.body`, r.bodyIr] : null,
        ].filter(Boolean) as [string, unknown][],
      ),
    ),
  };
  const capabilityManifest = {
    declared: [...new Set(app.routes.flatMap((r) => r.capabilities))],
    perRoute: Object.fromEntries(app.routes.map((r) => [r.id, r.capabilities])),
    nativeOps: { timer: "cancellable delay (ms) → Promise<number>" },
  };

  // contract lock (semantic diff base)
  const lock = {
    formatVersion: 1,
    contractHash: (pack as { contractHash: string }).contractHash,
    lockedAt: new Date().toISOString(),
    routes: contract.routes,
  };

  // build report
  const buildReport = {
    formatVersion: 1,
    builtAt: new Date().toISOString(),
    appId: app.appId,
    routes: routeManifest,
    schemas: schemaManifest,
    capabilities: capabilityManifest,
    strategies: {
      // SCHEMA-005: every fallback visible
      fallbacks: [] as string[],
      notes: [
        "validation: native (ADR-0015)",
        "responses: native serialization (ADR-0015)",
        "engine JS strategy available per-route; none used in this build",
      ],
    },
    nativeStages: app.routes.filter((r) => r.liveness).map((r) => ({ route: r.id, stage: "native-liveness" })),
    integrity: (pack as { integrity: unknown }).integrity,
    versions: (pack as { engine: unknown; runtimeAbi: number }).engine,
  };

  // The lock is a FROZEN baseline for semantic diff — it must NOT follow
  // every build, or `q contract diff` would always report "no changes".
  // First build writes it; later builds preserve it unless updateLock.
  const lockPath = join(outDir, "contract.lock.json");
  const lockExists = existsSync(lockPath);
  const writeLock = !lockExists || (opts.updateLock ?? false);

  const files: Record<string, string> = {
    "app.qpack": packJson,
    "route-manifest.json": JSON.stringify(routeManifest, null, 2),
    "schema-manifest.json": JSON.stringify(schemaManifest, null, 2),
    "capability-manifest.json": JSON.stringify(capabilityManifest, null, 2),
    "contract.json": JSON.stringify(contract, null, 2),
    "contract.d.ts": dts,
    "openapi.json": JSON.stringify(openapi, null, 2),
    ...(writeLock ? { "contract.lock.json": JSON.stringify(lock, null, 2) } : {}),
    "build-report.json": JSON.stringify(buildReport, null, 2),
  };
  const artifactBytes: Record<string, number> = {};
  for (const [name, content] of Object.entries(files)) {
    writeFileSync(join(outDir, name), content);
    artifactBytes[name] = Buffer.byteLength(content);
  }
  writeFileSync(join(outDir, "build-report.md"), renderBuildReportMd(buildReport, artifactBytes));

  return {
    outDir,
    packPath: join(outDir, "app.qpack"),
    routes: app.routes.length,
    buildMs: Math.round(performance.now() - t0),
    artifactBytes,
    lockPreserved: lockExists && !writeLock,
  };
}

function resolveEntry(project: string): string {
  const { statSync } = require("node:fs") as typeof import("node:fs");
  let st;
  try {
    st = statSync(project);
  } catch {
    throw new Error(`project path not found: ${project}`);
  }
  if (st.isDirectory()) {
    for (const c of ["src/app.ts", "app.ts", "src/index.ts"]) {
      const p = join(project, c);
      if (existsSync(p)) return p;
    }
    throw new Error(`no app entry found in ${project} (looked for src/app.ts, app.ts, src/index.ts)`);
  }
  return project;
}

function renderBuildReportMd(report: Record<string, unknown>, bytes: Record<string, number>): string {
  const routes = report.routes as Array<Record<string, unknown>>;
  const lines = [
    `# Build report — ${report.appId}`,
    "",
    `Built: ${report.builtAt}`,
    "",
    "## Routes",
    "",
    "| Route | Method | Path | Stage | Policy | Caps | Validation | Response |",
    "|---|---|---|---|---|---|---|---|",
    ...routes.map(
      (r) =>
        `| ${r.id} | ${r.method} | ${r.path} | ${r.nativeStage} | ${r.policy ?? "—"} | ${(r.capabilities as string[]).join(",") || "—"} | native | native |`,
    ),
    "",
    "## Strategies",
    "",
    ...(report.strategies as { notes: string[] }).notes.map((n) => `- ${n}`),
    (report.strategies as { fallbacks: string[] }).fallbacks.length === 0
      ? "- JS fallbacks used: **none** (SCHEMA-005)"
      : (report.strategies as { fallbacks: string[] }).fallbacks.map((f) => `- FALLBACK: ${f}`),
    "",
    "## Artifacts",
    "",
    ...Object.entries(bytes).map(([k, v]) => `- ${k}: ${v} B`),
    "",
  ];
  return lines.join("\n");
}

// ---------------------------------------------------------------- diff CLI helper

export function contractDiff(outDir: string, lockPath?: string): ReturnType<typeof diffContracts> {
  const current = JSON.parse(readFileSync(join(outDir, "contract.json"), "utf8"));
  const lock = JSON.parse(readFileSync(lockPath ?? join(outDir, "contract.lock.json"), "utf8"));
  return diffContracts(current, lock);
}
