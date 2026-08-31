/**
 * @velqu/cli — actual-runtime dev server and worker swap pipeline (M4A-001-C).
 *
 * Implements the safe reload loop: builds a temporary QPack, spawns and
 * verifies the new QuickJS worker runtime, and switches traffic ONLY after
 * the new worker is verified healthy and ready. If build or worker initialization
 * fails, the prior healthy worker continues serving traffic uninterrupted.
 */

import { existsSync } from "node:fs";
import { resolve, join } from "node:path";
import { IncrementalPackBuilder, type TempPackResult } from "@velqu/compiler";
import { ProjectWatcher, type WatchEvent } from "@velqu/compiler";

export interface DevServerOptions {
  /** Path to the project entry file or directory */
  project: string;
  /** Public gateway port (default: 0 for free port, or specified port) */
  port?: number;
  /** Debounce milliseconds for file watching (default: 50) */
  debounceMs?: number;
  /** Path to velqu-runtime binary (defaults to searching target/release and target/debug) */
  qRuntimeBin?: string;
  /** Service profile for the worker (default: "serverless") */
  serviceProfile?: string;
  /** Auto-start file watcher on start (default: false in programmatic mode) */
  watch?: boolean;
  /** Reload callback */
  onReload?: (result: ReloadResult) => void;
  /** Log callback */
  onLog?: (msg: string) => void;
}

export interface WorkerInstance {
  id: string;
  generation: number;
  port: number;
  proc: ReturnType<typeof Bun.spawn>;
  packPath: string;
  packSha256: string;
  contractHash: string;
  createdAt: number;
  readyAt: number;
}

export interface ReloadResult {
  success: boolean;
  switched: boolean;
  generation: number;
  totalMs: number;
  compileMs: number;
  workerInitMs: number;
  retainedPriorWorker: boolean;
  error?: string;
  contractChanged?: boolean;
}

export class DevServer {
  private readonly project: string;
  private readonly requestedPort: number;
  private readonly debounceMs: number;
  private readonly runtimeBin: string;
  private readonly serviceProfile: string;
  private readonly onReload?: (result: ReloadResult) => void;
  private readonly onLog?: (msg: string) => void;

  private builder: IncrementalPackBuilder;
  private watcher: ProjectWatcher | null = null;
  private gateway: ReturnType<typeof Bun.serve> | null = null;
  private activeWorker: WorkerInstance | null = null;
  private drainingWorkers: WorkerInstance[] = [];
  private generation = 0;
  private running = false;
  private reloading = false;

  constructor(opts: DevServerOptions) {
    this.project = resolve(opts.project);
    this.requestedPort = opts.port ?? 0;
    this.debounceMs = opts.debounceMs ?? 50;
    this.serviceProfile = opts.serviceProfile ?? "serverless";
    this.onReload = opts.onReload;
    this.onLog = opts.onLog;
    this.runtimeBin = this.findRuntimeBinary(opts.qRuntimeBin);
    this.builder = new IncrementalPackBuilder({ project: this.project });
  }

  /**
   * Start the dev server: compile initial QPack, start worker 1, verify ready,
   * start public gateway, and optionally start file watcher.
   */
  public async start(enableWatcher = false): Promise<{ port: number; generation: number }> {
    if (this.running) {
      return { port: this.gateway?.port ?? 0, generation: this.generation };
    }

    this.log(`building initial temporary QPack for ${this.project}...`);
    const packResult = await this.builder.build();

    this.log(`spawning initial worker generation 1...`);
    const initialWorker = await this.spawnAndVerifyWorker(packResult, 1);
    if (!initialWorker) {
      throw new Error(`failed to start initial worker runtime from pack ${packResult.packPath}`);
    }

    this.activeWorker = initialWorker;
    this.generation = 1;

    // Start public HTTP gateway:
    this.gateway = Bun.serve({
      port: this.requestedPort,
      hostname: "127.0.0.1",
      fetch: async (req) => {
        return this.proxyRequest(req);
      },
    });

    this.running = true;
    this.log(`dev server listening at http://127.0.0.1:${this.gateway.port} (worker gen 1 on port ${initialWorker.port})`);

    if (enableWatcher) {
      await this.startWatcher();
    }

    return { port: this.gateway.port ?? 0, generation: this.generation };
  }

  /**
   * Safely recompile and reload: loads the new worker before switching traffic.
   * If compilation or worker startup fails, retains the prior healthy worker.
   */
  public async reload(): Promise<ReloadResult> {
    if (!this.running || this.reloading) {
      return {
        success: false,
        switched: false,
        generation: this.generation,
        totalMs: 0,
        compileMs: 0,
        workerInitMs: 0,
        retainedPriorWorker: this.activeWorker != null,
        error: "server not running or reload already in progress",
      };
    }

    this.reloading = true;
    const t0 = performance.now();
    const nextGen = this.generation + 1;
    this.log(`[reload:gen-${nextGen}] building updated temporary QPack...`);

    let packResult: TempPackResult;
    try {
      packResult = await this.builder.build();
    } catch (e) {
      const err = e instanceof Error ? e.message : String(e);
      this.log(`[reload:gen-${nextGen}] compile failed: ${err} — retaining prior worker gen ${this.generation}`);
      this.reloading = false;
      const res: ReloadResult = {
        success: false,
        switched: false,
        generation: this.generation,
        totalMs: Math.round(performance.now() - t0),
        compileMs: Math.round(performance.now() - t0),
        workerInitMs: 0,
        retainedPriorWorker: true,
        error: `compile error: ${err}`,
      };
      if (this.onReload) this.onReload(res);
      return res;
    }

    const compileMs = packResult.buildMs;
    this.log(`[reload:gen-${nextGen}] spawning candidate worker runtime...`);
    const workerT0 = performance.now();
    const candidateWorker = await this.spawnAndVerifyWorker(packResult, nextGen);

    if (!candidateWorker) {
      this.log(`[reload:gen-${nextGen}] worker initialization failed — retaining prior worker gen ${this.generation}`);
      this.reloading = false;
      const res: ReloadResult = {
        success: false,
        switched: false,
        generation: this.generation,
        totalMs: Math.round(performance.now() - t0),
        compileMs,
        workerInitMs: Math.round(performance.now() - workerT0),
        retainedPriorWorker: true,
        error: "candidate worker failed health check before traffic switch",
      };
      if (this.onReload) this.onReload(res);
      return res;
    }

    const workerInitMs = Math.round(performance.now() - workerT0);

    // Atomic traffic switch:
    const oldWorker = this.activeWorker;
    this.activeWorker = candidateWorker;
    this.generation = nextGen;

    if (oldWorker) {
      this.drainingWorkers.push(oldWorker);
      // Initiate drain of old worker:
      this.drainWorker(oldWorker);
    }

    const totalMs = Math.round(performance.now() - t0);
    this.log(`[reload:gen-${nextGen}] successfully switched traffic to worker gen ${nextGen} (in ${totalMs}ms)`);
    this.reloading = false;

    const res: ReloadResult = {
      success: true,
      switched: true,
      generation: nextGen,
      totalMs,
      compileMs,
      workerInitMs,
      retainedPriorWorker: false,
      contractChanged: packResult.contractChanged,
    };
    if (this.onReload) this.onReload(res);
    return res;
  }

  public getPort(): number {
    return this.gateway?.port ?? 0;
  }

  public getGeneration(): number {
    return this.generation;
  }

  public getActiveWorker(): WorkerInstance | null {
    return this.activeWorker;
  }

  public isHealthy(): boolean {
    return this.running && this.activeWorker != null;
  }

  /**
   * Stop the server, kill all active/draining workers, and close watchers.
   */
  public async stop(): Promise<void> {
    this.running = false;
    if (this.watcher) {
      this.watcher.close();
      this.watcher = null;
    }
    if (this.gateway) {
      this.gateway.stop(true);
      this.gateway = null;
    }
    if (this.activeWorker) {
      try {
        this.activeWorker.proc.kill();
        await this.activeWorker.proc.exited;
      } catch {}
      this.activeWorker = null;
    }
    for (const w of this.drainingWorkers) {
      try {
        w.proc.kill();
        await w.proc.exited;
      } catch {}
    }
    this.drainingWorkers = [];
    this.builder.dispose();
  }

  private async proxyRequest(req: Request): Promise<Response> {
    const worker = this.activeWorker;
    if (!worker) {
      return new Response(
        JSON.stringify({
          type: "https://velqu.dev/problems/overload",
          title: "Service Unavailable",
          status: 503,
          detail: "no active worker runtime serving traffic",
        }),
        { status: 503, headers: { "content-type": "application/problem+json" } },
      );
    }

    const targetUrl = new URL(req.url);
    targetUrl.protocol = "http:";
    targetUrl.hostname = "127.0.0.1";
    targetUrl.port = String(worker.port);

    try {
      const headers = new Headers(req.headers);
      headers.set("x-forwarded-for", "127.0.0.1");
      headers.set("x-forwarded-proto", "http");

      let body: ReadableStream | ArrayBuffer | undefined = undefined;
      if (req.method !== "GET" && req.method !== "HEAD") {
        body = await req.arrayBuffer();
      }

      const res = await fetch(targetUrl.toString(), {
        method: req.method,
        headers,
        body,
        // @ts-expect-error Bun fetch duplex option for streaming
        duplex: "half",
      });

      return res;
    } catch {
      return new Response(
        JSON.stringify({
          type: "https://velqu.dev/problems/internal",
          title: "Bad Gateway",
          status: 502,
          detail: "worker failed to answer request",
        }),
        { status: 502, headers: { "content-type": "application/problem+json" } },
      );
    }
  }

  private async spawnAndVerifyWorker(
    packResult: TempPackResult,
    generation: number,
  ): Promise<WorkerInstance | null> {
    if (!packResult.packPath || !existsSync(packResult.packPath)) {
      return null;
    }

    const port = this.findFreePort();
    const t0 = performance.now();

    const proc = Bun.spawn(
      [
        this.runtimeBin,
        "--pack",
        packResult.packPath,
        "--port",
        String(port),
        "--service-profile",
        this.serviceProfile,
        "--log",
        "errors",
      ],
      {
        stdout: "pipe",
        stderr: "pipe",
        env: process.env,
      },
    );

    // Verify readiness via TCP connect with timeout:
    const deadline = Date.now() + 5_000;
    let ready = false;

    while (Date.now() < deadline) {
      if (proc.exitCode !== null) {
        // Process exited prematurely (load failure / invalid pack)
        return null;
      }
      try {
        const conn = await Bun.connect({
          hostname: "127.0.0.1",
          port,
          socket: {
            data() {},
            open() {},
            error() {},
          },
        });
        conn.end?.();
        conn.terminate?.();
        ready = true;
        break;
      } catch {
        await Bun.sleep(10);
      }
    }

    if (!ready) {
      try {
        proc.kill();
        await proc.exited;
      } catch {}
      return null;
    }

    return {
      id: `worker-gen-${generation}-${port}`,
      generation,
      port,
      proc,
      packPath: packResult.packPath,
      packSha256: packResult.packSha256,
      contractHash: packResult.contractHash,
      createdAt: t0,
      readyAt: performance.now(),
    };
  }

  private drainWorker(worker: WorkerInstance): void {
    // SIGTERM signals the worker's DrainGate to refuse new admissions,
    // drain in-flight connections (M3-007-C), and shutdown deterministically.
    setTimeout(async () => {
      try {
        worker.proc.kill("SIGTERM");
        await worker.proc.exited;
      } catch {}
      const idx = this.drainingWorkers.indexOf(worker);
      if (idx !== -1) this.drainingWorkers.splice(idx, 1);
    }, 100);
  }

  private async startWatcher(): Promise<void> {
    this.watcher = new ProjectWatcher({
      project: this.project,
      debounceMs: this.debounceMs,
      onChange: async (events: WatchEvent[]) => {
        this.log(`[watch] detected changes in ${events.map((e) => e.file).join(", ")} — reloading...`);
        await this.reload();
      },
      onError: (err) => {
        this.log(`[watch:error] ${err.message}`);
      },
    });
    await this.watcher.start();
  }

  private findFreePort(): number {
    const l = Bun.listen({
      hostname: "127.0.0.1",
      port: 0,
      socket: { data() {}, open() {} },
    });
    const p = l.port;
    l.stop(true);
    return p;
  }

  private findRuntimeBinary(customPath?: string): string {
    const candidates = [
      customPath,
      process.env.VELQU_RUNTIME,
      resolve("./target/release/velqu-runtime"),
      resolve("./target/debug/velqu-runtime"),
      resolve(process.cwd(), "target/release/velqu-runtime"),
      resolve(process.cwd(), "target/debug/velqu-runtime"),
    ].filter(Boolean);

    for (const c of candidates) {
      if (existsSync(c!)) return c!;
    }
    throw new Error(`velqu-runtime binary not found (looked in: ${candidates.join(", ")})`);
  }

  private log(msg: string): void {
    if (this.onLog) this.onLog(msg);
  }
}
