/**
 * @velqu/compiler — incremental temporary QPack builder (M4A-001-B).
 *
 * Fast-path in-memory / temporary QPack compilation for the `velqu dev`
 * reload loop. Extracts AST metadata without evaluating application code,
 * bundles with source maps pointing to TypeScript, and emits a self-contained
 * temporary QPack for the worker reload pipeline.
 */

import { mkdirSync, writeFileSync, unlinkSync, rmSync, existsSync } from "node:fs";
import { createHash } from "node:crypto";
import { dirname, join, resolve } from "node:path";
import { tmpdir } from "node:os";
import * as ts from "typescript";
import { extractApp, type ExtractedApp, type RouteInfo } from "./extract";
import { bundleApp, buildPack, type BundleResult } from "./emit";
import { PINNED_TOOLCHAIN, assertPinnedToolchain } from "./toolchain";

export interface TempPackOptions {
  /** Path to the app entry file or project directory */
  project: string;
  /** Whether to generate source maps pointing to TypeScript (default: true) */
  sourceMap?: boolean;
  /** Custom directory for temporary pack files (default: OS temp directory) */
  tempDir?: string;
  /** Whether to write the temporary app.qpack to disk for native worker loading (default: true) */
  writeToDisk?: boolean;
}

export interface TempPackResult {
  packJson: string;
  pack: Record<string, unknown>;
  packSha256: string;
  contractHash: string;
  app: ExtractedApp;
  routes: RouteInfo[];
  bundle: BundleResult;
  packPath: string | null;
  buildMs: number;
  contractChanged: boolean;
}

/**
 * Fast-path build of an application into a temporary, verified QPack.
 */
export async function buildTemporaryPack(
  opts: TempPackOptions,
  previousContractHash?: string,
): Promise<TempPackResult> {
  const t0 = performance.now();
  assertPinnedToolchain({ bun: Bun.version!, typescript: ts.version });

  const entry = resolveEntry(opts.project);
  const app = extractApp(entry);
  const bundle = await bundleApp(app, { sourceMap: opts.sourceMap ?? true });
  const { packJson, pack } = buildPack(app, bundle, {
    compilerVersion: PINNED_TOOLCHAIN.compiler,
    typescriptVersion: ts.version,
  });

  const packSha256 = createHash("sha256").update(packJson).digest("hex");
  const contractHash = (pack as { contractHash: string }).contractHash;
  const contractChanged = previousContractHash != null && previousContractHash !== contractHash;

  let packPath: string | null = null;
  if (opts.writeToDisk !== false) {
    const dir = opts.tempDir ?? join(tmpdir(), "velqu-temp-packs");
    mkdirSync(dir, { recursive: true });
    packPath = join(dir, `temp-${app.appId}-${packSha256.slice(0, 12)}.qpack`);
    writeFileSync(packPath, packJson);

    // Also emit the debug source sidecar next to it so source maps resolve:
    const sourcesPath = join(dir, `temp-${app.appId}-${packSha256.slice(0, 12)}.qpack.sources.json`);
    writeFileSync(
      sourcesPath,
      JSON.stringify({
        formatVersion: 1,
        packSha256,
        bundleSource: bundle.code,
        sourceMap: bundle.sourceMap,
        modules: app.modules.map((m) => ({ id: m, file: m })),
      }),
    );
  }

  const buildMs = Math.round(performance.now() - t0);

  return {
    packJson,
    pack,
    packSha256,
    contractHash,
    app,
    routes: app.routes,
    bundle,
    packPath,
    buildMs,
    contractChanged,
  };
}

/**
 * State-holding incremental pack builder that tracks contract versions,
 * cleans up stale temporary files, and provides fast feedback for the
 * `velqu dev` loop.
 */
export class IncrementalPackBuilder {
  private readonly project: string;
  private readonly sourceMap: boolean;
  private readonly tempDir: string;
  private lastResult: TempPackResult | null = null;
  private writtenPacks: string[] = [];

  constructor(opts: TempPackOptions) {
    this.project = resolve(opts.project);
    this.sourceMap = opts.sourceMap ?? true;
    this.tempDir = opts.tempDir ?? join(tmpdir(), `velqu-incremental-${Date.now()}`);
    mkdirSync(this.tempDir, { recursive: true });
  }

  /**
   * Recompile project to temporary QPack and report if contract changed.
   */
  public async build(): Promise<TempPackResult> {
    const prevContract = this.lastResult?.contractHash;
    const result = await buildTemporaryPack(
      {
        project: this.project,
        sourceMap: this.sourceMap,
        tempDir: this.tempDir,
        writeToDisk: true,
      },
      prevContract,
    );

    if (result.packPath) {
      this.writtenPacks.push(result.packPath);
      // Clean up older temp packs to bound disk usage (keep last 2):
      while (this.writtenPacks.length > 2) {
        const old = this.writtenPacks.shift()!;
        try {
          if (existsSync(old)) unlinkSync(old);
          const sidecar = old.replace(/\.qpack$/, ".qpack.sources.json");
          if (existsSync(sidecar)) unlinkSync(sidecar);
        } catch {}
      }
    }

    this.lastResult = result;
    return result;
  }

  public getLastResult(): TempPackResult | null {
    return this.lastResult;
  }

  public getTempDir(): string {
    return this.tempDir;
  }

  public dispose(): void {
    try {
      if (existsSync(this.tempDir)) {
        rmSync(this.tempDir, { recursive: true, force: true });
      }
    } catch {}
  }
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
