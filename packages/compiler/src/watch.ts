/**
 * @velqu/compiler — source and contract watcher (M4A-001-A).
 *
 * Discovers all source modules in the application's dependency graph,
 * monitors contract locks and config files, coalesces rapid edits with
 * configurable debouncing, and triggers typed watch events for the
 * actual-runtime dev loop.
 */

import { watch, type FSWatcher, existsSync, statSync } from "node:fs";
import { dirname, join, resolve, relative } from "node:path";
import * as ts from "typescript";

export type WatchEventKind = "source" | "contract" | "config";

export interface WatchEvent {
  kind: WatchEventKind;
  file: string;
  action: "change" | "rename" | "delete";
  timestamp: number;
  /** Latency from file event detection to dispatch in milliseconds */
  latencyMs: number;
}

export interface WatchOptions {
  /** Path to the app entry file (e.g. app.ts) or project directory */
  project: string;
  /** Debounce interval in milliseconds (default: 50ms) */
  debounceMs?: number;
  /** Additional files or directories to watch */
  extraWatchPaths?: string[];
  /** Callback fired when a change is detected */
  onChange?: (events: WatchEvent[]) => void | Promise<void>;
  /** Error callback */
  onError?: (error: Error) => void;
}

export interface DiscoveredFiles {
  entryFile: string;
  sourceFiles: string[];
  contractFiles: string[];
  configFiles: string[];
  allWatched: string[];
}

export class ProjectWatcher {
  private readonly project: string;
  private readonly debounceMs: number;
  private readonly extraPaths: string[];
  private readonly onChange?: (events: WatchEvent[]) => void | Promise<void>;
  private readonly onError?: (error: Error) => void;

  private watchers: Map<string, FSWatcher> = new Map();
  private pendingEvents: Map<string, { event: WatchEvent; timer: ReturnType<typeof setTimeout> }> = new Map();
  private discovered: DiscoveredFiles | null = null;
  private running = false;

  constructor(opts: WatchOptions) {
    this.project = resolve(opts.project);
    this.debounceMs = opts.debounceMs ?? 50;
    this.extraPaths = (opts.extraWatchPaths ?? []).map((p) => resolve(p));
    this.onChange = opts.onChange;
    this.onError = opts.onError;
  }

  /**
   * Statically discover all source modules, contract locks, and config files.
   * Does NOT evaluate any application code (COMP-002).
   */
  public discover(): DiscoveredFiles {
    const entryFile = this.resolveEntryPath(this.project);
    const projectDir = statSync(this.project).isDirectory() ? this.project : dirname(entryFile);

    const program = ts.createProgram([entryFile], {
      target: ts.ScriptTarget.ES2022,
      module: ts.ModuleKind.ESNext,
      moduleResolution: ts.ModuleResolutionKind.Bundler,
      strict: false,
      noEmit: true,
      allowImportingTsExtensions: true,
      baseUrl: projectDir,
    });

    const sourceFiles: string[] = [];
    for (const sf of program.getSourceFiles()) {
      const fn = resolve(sf.fileName);
      if (fn.includes("node_modules") || fn.includes(".git") || fn.includes("/dist/")) continue;
      if (fn.endsWith(".d.ts") && !fn.includes("contract.d.ts")) continue;
      sourceFiles.push(fn);
    }

    const contractFiles: string[] = [];
    for (const name of ["contract.lock.json", "contract.json", "contract.meta.json", "contract.d.ts", "openapi.json"]) {
      const candidates = [join(projectDir, name), join(projectDir, "dist", name)];
      for (const c of candidates) {
        if (existsSync(c)) contractFiles.push(resolve(c));
      }
    }

    const configFiles: string[] = [];
    for (const name of ["tsconfig.json", "package.json", "velqu.json"]) {
      const c = join(projectDir, name);
      if (existsSync(c)) configFiles.push(resolve(c));
    }

    for (const extra of this.extraPaths) {
      if (existsSync(extra)) {
        if (extra.endsWith(".ts") || extra.endsWith(".js")) {
          sourceFiles.push(extra);
        } else if (extra.includes("contract") || extra.endsWith(".json")) {
          contractFiles.push(extra);
        } else {
          configFiles.push(extra);
        }
      }
    }

    const allWatched = Array.from(new Set([...sourceFiles, ...contractFiles, ...configFiles]));
    this.discovered = {
      entryFile,
      sourceFiles: Array.from(new Set(sourceFiles)),
      contractFiles: Array.from(new Set(contractFiles)),
      configFiles: Array.from(new Set(configFiles)),
      allWatched,
    };
    return this.discovered;
  }

  /**
   * Start watching discovered files and project directories.
   */
  public async start(): Promise<DiscoveredFiles> {
    if (this.running) return this.discovered ?? this.discover();
    const discovered = this.discover();

    // Determine unique directories to watch (more efficient and handles new files):
    const dirsToWatch = new Set<string>();
    for (const f of discovered.allWatched) {
      dirsToWatch.add(dirname(f));
    }
    // Also watch project root:
    const projectDir = statSync(this.project).isDirectory() ? this.project : dirname(discovered.entryFile);
    dirsToWatch.add(projectDir);

    for (const dir of dirsToWatch) {
      if (!existsSync(dir)) continue;
      try {
        const watcher = watch(dir, { recursive: false }, (eventType, filename) => {
          if (!filename) return;
          const fullPath = resolve(dir, filename.toString());
          this.handleRawEvent(fullPath, eventType === "rename" ? "rename" : "change");
        });
        watcher.on("error", (err) => {
          if (this.onError) this.onError(err);
        });
        this.watchers.set(dir, watcher);
      } catch (e) {
        if (this.onError) this.onError(e as Error);
      }
    }

    this.running = true;
    return discovered;
  }

  /**
   * Stop watching and clear all active timers.
   */
  public close(): void {
    this.running = false;
    for (const watcher of this.watchers.values()) {
      watcher.close();
    }
    this.watchers.clear();
    for (const pending of this.pendingEvents.values()) {
      clearTimeout(pending.timer);
    }
    this.pendingEvents.clear();
  }

  public isWatching(): boolean {
    return this.running;
  }

  public watchedDirectoryCount(): number {
    return this.watchers.size;
  }

  public classifyFile(filePath: string): WatchEventKind | null {
    const norm = resolve(filePath);
    if (norm.includes("node_modules") || norm.includes("/.git/") || norm.includes("/dist/")) return null;
    if (norm.endsWith("contract.lock.json") || norm.endsWith("contract.meta.json") || norm.endsWith("contract.json")) {
      return "contract";
    }
    if (norm.endsWith("tsconfig.json") || norm.endsWith("velqu.json") || norm.endsWith("package.json")) {
      return "config";
    }
    if (norm.endsWith(".ts") || norm.endsWith(".js") || norm.endsWith(".tsx") || norm.endsWith(".jsx")) {
      return "source";
    }
    return null;
  }

  private handleRawEvent(filePath: string, action: "change" | "rename" | "delete"): void {
    const t0 = performance.now();
    const kind = this.classifyFile(filePath);
    if (!kind) return;

    // Check if the file still exists or was deleted:
    const finalAction: "change" | "rename" | "delete" = !existsSync(filePath) ? "delete" : action;

    const event: WatchEvent = {
      kind,
      file: filePath,
      action: finalAction,
      timestamp: Date.now(),
      latencyMs: 0,
    };

    // Coalesce rapid edits per file using debouncing:
    const existing = this.pendingEvents.get(filePath);
    if (existing) {
      clearTimeout(existing.timer);
    }

    const timer = setTimeout(() => {
      this.pendingEvents.delete(filePath);
      event.latencyMs = Math.round(performance.now() - t0);
      if (this.onChange) {
        try {
          this.onChange([event]);
        } catch (e) {
          if (this.onError) this.onError(e as Error);
        }
      }
    }, this.debounceMs);

    this.pendingEvents.set(filePath, { event, timer });
  }

  private resolveEntryPath(project: string): string {
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
}

/**
 * Convenience helper to start watching a project.
 */
export async function watchSourceAndContracts(opts: WatchOptions): Promise<ProjectWatcher> {
  const watcher = new ProjectWatcher(opts);
  await watcher.start();
  return watcher;
}
