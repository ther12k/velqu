import { mkdirSync, writeFileSync, readFileSync, existsSync } from "node:fs";
import { dirname, join } from "node:path";
import * as ts from "typescript";
import { extractApp, hash } from "./extract";
import { bundleApp, buildPack, contractFor, contractDts, openapiFor, diffContracts } from "./emit";
import { evaluateAppStrategies, selectRouteStrategies } from "./strategy";

export { CompileError } from "./extract";
export { diffContracts, PROBLEM_REGISTRY, type DiffEntry } from "./emit";
export {
  evaluateAppStrategies,
  selectRouteStrategies,
  type StrategyName,
  type FallbackReason,
  type FallbackDescriptor,
  type RouteStrategyDecision,
  type AppStrategyReport,
} from "./strategy";

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
          responses: Object.fromEntries(
            Object.entries(r.responses).map(([status, decl]) => [
              status,
              decl.ir ?? (decl.problem ? { problem: decl.problem } : {}),
            ]),
          ),
          security: r.policyId ?? null,
        },
      ]),
    ),
  };
  const openapi = openapiFor(app);
  const dts = contractDts(app, (pack as { contractHash: string }).contractHash);

  // route/schema/capability manifests
  // M25-007-D: the manifest carries the REAL per-route codec choices and
  // the bridge-crossing model — native routes cross once with pre-validated
  // values; fallback routes cross lazily per field access. Fallback
  // reasons come from the same decisions that tagged the RoutePlan.
  const routeManifest = app.routes.map((r) => {
    const decision = selectRouteStrategies(r);
    const validationFallbackReason =
      decision.validationStrategy === "js"
        ? (decision.fallbacks.find((f) => ["body", "query", "params"].includes(f.location))?.reason ?? "explicit")
        : null;
    const responseFallbackReason =
      decision.primaryResponseStrategy === "js"
        ? (decision.fallbacks.find((f) => f.location.startsWith("response."))?.reason ?? "explicit")
        : null;
    return {
      id: r.id,
      method: r.method,
      path: r.path,
      moduleId: r.moduleId,
      policy: r.policyId,
      capabilities: r.capabilities,
      nativeStage: r.liveness ? "native-liveness" : "engine",
      validationStrategy: decision.validationStrategy,
      validationFallbackReason,
      responseStrategy: decision.primaryResponseStrategy,
      responseFallbackReason,
      // codec choice: direct decoder/encoder programs vs the generic path
      validationCodec:
        decision.validationStrategy === "js" ? "generic-fallback" : "direct-decoder",
      responseCodec:
        decision.primaryResponseStrategy === "js" ? "engine-stringify" : "direct-encoder",
      // bridge crossings: 1 pre-validated crossing (native) vs lazy
      // per-field crossings (fallback) — visible cost, never hidden
      bridge:
        decision.validationStrategy === "js" || decision.primaryResponseStrategy === "js"
          ? "lazy-per-field"
          : "single-prevalidated",
      sourceFile: r.sourceFile,
    };
  });
  const schemaManifest = {
    schemaIrVersion: 1,
    schemas: Object.fromEntries(
      app.routes.flatMap((r) =>
        [
          r.paramsIr ? [`sch:${r.id}.params`, r.paramsIr] : null,
          r.queryIr ? [`sch:${r.id}.query`, r.queryIr] : null,
          r.bodyIr ? [`sch:${r.id}.body`, r.bodyIr] : null,
          ...Object.entries(r.responses).map(([status, decl]) =>
            decl.ir && !decl.problem ? [`sch:${r.id}.${status}`, decl.ir] : null,
          ),
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
  const lockPath = join(outDir, "contract.lock.json");
  const lockExists = existsSync(lockPath);
  const writeLock = !lockExists || (opts.updateLock ?? false);

  const lock = {
    formatVersion: 1,
    contractHash: (pack as { contractHash: string }).contractHash,
    lockedAt: new Date().toISOString(),
    routes: contract.routes,
  };

  // evaluate strategy selection from measured evidence
  const { report: strategyReport } = evaluateAppStrategies(app.routes);

  // build report
  const buildReport = {
    formatVersion: 1,
    builtAt: new Date().toISOString(),
    appId: app.appId,
    routes: routeManifest,
    schemas: schemaManifest,
    capabilities: capabilityManifest,
    strategies: strategyReport,
    nativeStages: app.routes.filter((r) => r.liveness).map((r) => ({ route: r.id, stage: "native-liveness" })),
    integrity: (pack as { integrity: unknown }).integrity,
    versions: (pack as { engine: unknown; runtimeAbi: number }).engine,
  };

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
  const strategies = (report.strategies ?? {}) as {
    notes?: string[];
    fallbacks?: Array<{ route: string; location: string; strategy: string; reason: string; estimatedOverheadUs: number; description: string }>;
    decisions?: Array<{ route: string; validationStrategy: string; responseStrategy: string }>;
  };
  const decisionMap = new Map((strategies.decisions ?? []).map((d) => [d.route, d]));
  const fallbacks = strategies.fallbacks ?? [];

  const lines = [
    `# Build report — ${report.appId}`,
    "",
    `Built: ${report.builtAt}`,
    "",
    "## Routes",
    "",
    "| Route | Method | Path | Stage | Policy | Caps | Validation | Response |",
    "|---|---|---|---|---|---|---|---|",
    ...routes.map((r) => {
      const d = decisionMap.get(r.id as string);
      const val = d?.validationStrategy ?? "native";
      const resp = d?.responseStrategy ?? "native";
      return `| ${r.id} | ${r.method} | ${r.path} | ${r.nativeStage} | ${r.policy ?? "—"} | ${(r.capabilities as string[]).join(",") || "—"} | ${val} | ${resp} |`;
    }),
    "",
    "## Strategies",
    "",
    ...(strategies.notes ?? []).map((n) => `- ${n}`),
    fallbacks.length === 0
      ? "- JS fallbacks used: **none** (SCHEMA-005)"
      : fallbacks.map((f) => `- FALLBACK: ${f.route} [${f.location}]: strategy=${f.strategy} reason=${f.reason} (+${f.estimatedOverheadUs}µs) — ${f.description}`),
    "",
    "## Artifacts",
    "",
    ...Object.entries(bytes).map(([k, v]) => `- ${k}: ${v} B`),
    "",
  ];
  return lines.join("\n");
}

export function contractDiff(outDir: string, lockPath?: string): ReturnType<typeof diffContracts> {
  const current = JSON.parse(readFileSync(join(outDir, "contract.json"), "utf8"));
  const lock = JSON.parse(readFileSync(lockPath ?? join(outDir, "contract.lock.json"), "utf8"));
  return diffContracts(current, lock);
}
