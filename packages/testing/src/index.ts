/**
 * @velqu/testing — Treaty local test adapters. The modes are LABELED and
 * reported separately (TRT-005): unit-local executes handlers in-process
 * (direct dispatcher or loopback transport) under Bun; runtime-local drives
 * the ACTUAL velqu-runtime binary over HTTP.
 */
import {
  treaty,
  type AnyRouteContract,
  type DispatchOutcome,
  type DispatchRequest,
  type TreatyClient,
  type TreatyFetch,
} from "@velqu/treaty";

// ---------------------------------------------------------------- unit-local DIRECT (M4A-004-A)

/** A route as seen by the direct unit-local dispatcher: handler + contract facts. */
export interface UnitDirectRoute {
  path: string;
  method: string;
  /**
   * Declared response statuses from the contract (route `response` keys).
   * A handler returning an undeclared status is a CONTRACT ERROR and fails
   * loud — the runtime itself can never emit an undeclared status.
   */
  responses: Record<number, unknown>;
  handle: (ctx: unknown) => Promise<unknown> | unknown;
}

export interface UnitDirectTreatyOptions {
  routes: Record<string, UnitDirectRoute>;
}

/** Raised when a unit-local handler produces a status the contract never declared. */
export class UndeclaredStatusError extends Error {
  readonly routeId: string;
  readonly status: number;
  readonly declared: readonly number[];
  constructor(routeId: string, status: number, declared: readonly number[]) {
    super(
      `treaty (unit-local): handler for "${routeId}" produced status ${status}, ` +
        `which the contract never declared (declared: ${declared.join(", ") || "none"}). ` +
        `Undeclared status is a contract error — declare it in the route response union.`,
    );
    this.name = "UndeclaredStatusError";
    this.routeId = routeId;
    this.status = status;
    this.declared = declared;
  }
}

/**
 * UNIT-LOCAL DIRECT mode — a generated in-process dispatcher, NO HTTP
 * transport at all: the Treaty client hands each call to the dispatcher,
 * which runs the handler in THIS process and status-splits the result by
 * the SAME contract machinery as the remote modes. Fast but NOT runtime
 * conformance; results must be labeled unit-local.
 */
export function unitTreatyDirect<Api extends Record<string, AnyRouteContract> = Record<string, AnyRouteContract>>(
  opts: UnitDirectTreatyOptions,
): {
  __mode: "unit-local (direct dispatcher, NOT runtime conformance)";
  close: () => void;
  api: TreatyClient<Api>;
} {
  const declaredOf = (routeId: string): readonly number[] =>
    Object.keys(opts.routes[routeId]?.responses ?? {}).map(Number);

  const dispatch = async (req: DispatchRequest): Promise<DispatchOutcome> => {
    const route = opts.routes[req.routeId];
    if (!route) {
      throw new Error(`treaty (unit-local): unknown route id "${req.routeId}" (not in declared routes)`);
    }
    if (route.method.toUpperCase() !== req.method) {
      throw new Error(
        `treaty (unit-local): method "${req.method}" is not allowed on route "${req.routeId}" ` +
          `(declared method: "${route.method}")`,
      );
    }
    const params: Record<string, string> = {};
    for (const seg of route.path.split("/").filter(Boolean)) {
      if (seg.startsWith(":")) {
        const name = seg.slice(1);
        const idx = route.path.split("/").filter(Boolean).indexOf(seg);
        const actual = req.path.split("/").filter(Boolean)[idx] ?? "";
        params[name] = decodeURIComponent(actual);
      }
    }
    const headers = Object.fromEntries(Object.entries(req.headers ?? {}).map(([k, v]) => [k.toLowerCase(), v]));
    const out = await route.handle({
      params,
      query: req.query ?? {},
      headers,
      body: req.body,
      json: () => req.body,
      native: { timer: { delay: (ms: number) => Bun.sleep(ms).then(() => ms) } },
    });
    const asRecord = out as Record<string, unknown> | null | undefined;
    let status: number;
    let value: unknown;
    if (asRecord && typeof asRecord === "object" && asRecord.__problem === true) {
      status = Number(asRecord.status ?? 500);
      value = asRecord;
    } else if (asRecord && typeof asRecord === "object" && asRecord.__ok === true) {
      status = Number(asRecord.status ?? 200);
      value = asRecord.value ?? null;
    } else {
      status = 200;
      value = out ?? null;
    }
    const declared = declaredOf(req.routeId);
    if (!declared.includes(status)) {
      throw new UndeclaredStatusError(req.routeId, status, declared);
    }
    return { kind: "response", status, bodyText: JSON.stringify(value ?? null) };
  };

  const contract = Object.fromEntries(
    Object.entries(opts.routes).map(([id, r]) => [id, { path: r.path, method: r.method }]),
  );
  const api = treaty<Api>({ baseUrl: "unit-local://direct", contract, dispatchImpl: dispatch });
  return {
    __mode: "unit-local (direct dispatcher, NOT runtime conformance)" as const,
    close: () => {},
    api,
  };
}

// ---------------------------------------------------------------- unit-local (loopback transport)

export interface UnitTreatyOptions {
  /** app route table to expose; handlers run in THIS process */
  routes: Record<string, { path: string; method: string; handle: (ctx: unknown) => Promise<unknown> | unknown }>;
  policies?: Record<string, (req: unknown) => Promise<unknown>>;
}

/**
 * UNIT-LOCAL mode — no native runtime involved. Fast but NOT runtime
 * conformance; results must be labeled unit-local.
 */
export function unitTreaty<Api extends Record<string, AnyRouteContract> = Record<string, AnyRouteContract>>(
  opts: UnitTreatyOptions,
): {
  __mode: "unit-local (NOT runtime conformance)";
  close: () => void;
  api: TreatyClient<Api>;
} {
  const server = Bun.serve({
    port: 0,
    fetch: async (req) => {
      const url = new URL(req.url);
      for (const [, route] of Object.entries(opts.routes)) {
        const pattern = new RegExp(
          "^" + route.path.replace(/:[A-Za-z]+/g, "([^/]+)") + "$",
        );
        const m = url.pathname.match(pattern);
        if (m && req.method === route.method) {
          const paramNames = [...route.path.matchAll(/:([A-Za-z]+)/g)].map((x) => x[1]);
          const params: Record<string, string> = {};
          paramNames.forEach((p, i) => (params[p] = m[i + 1]));
          const query = Object.fromEntries(url.searchParams.entries());
          const headers = Object.fromEntries(req.headers.entries());
          let body: unknown = undefined;
          if (route.method !== "GET" && route.method !== "HEAD") {
            const text = await req.text();
            try {
              body = text ? JSON.parse(text) : undefined;
            } catch {
              return new Response("malformed", { status: 422 });
            }
          }
          try {
            const out = await route.handle({ params, query, headers, body, json: () => body, native: { timer: { delay: (ms: number) => Bun.sleep(ms).then(() => ms) } } });
            const asRecord = out as Record<string, unknown> | null | undefined;
            if (asRecord && typeof asRecord === "object" && asRecord.__problem === true) {
              return Response.json(asRecord, { status: Number(asRecord.status ?? 500) });
            }
            if (asRecord && typeof asRecord === "object" && asRecord.__ok === true) {
              return Response.json(asRecord.value ?? null, { status: Number(asRecord.status ?? 200) });
            }
            if (typeof out === "string") return new Response(out, { status: 200 });
            return Response.json(out ?? null, { status: 200 });
          } catch {
            return new Response("internal", { status: 500 });
          }
        }
      }
      return new Response("not found", { status: 404 });
    },
  });
  const contract = Object.fromEntries(
    Object.entries(opts.routes).map(([id, r]) => [id, { path: r.path, method: r.method }]),
  );
  const base = `http://localhost:${server.port}`;
  const client = treaty<Api>({ baseUrl: base, contract });
  return {
    __mode: "unit-local (NOT runtime conformance)" as const,
    close: () => server.stop(true),
    api: client,
  };
}

// ---------------------------------------------------------------- remote

/** Options for a remote HTTP Treaty client. */
export interface RemoteTreatyOptions {
  baseUrl: string;
  contract: Record<string, { path: string; method: string }>;
  /** Inject a fetch implementation for tests; defaults to global fetch. */
  fetchImpl?: TreatyFetch;
}

/**
 * REMOTE mode — a thin adapter around the public Treaty HTTP transport.
 * The returned client is the same type/contract surface used by the
 * runtime-local adapter; no server/compiler code is imported.
 */
export function remoteTreaty<Api extends Record<string, AnyRouteContract> = Record<string, AnyRouteContract>>(
  opts: RemoteTreatyOptions,
): { __mode: "remote"; api: TreatyClient<Api> } {
  return {
    __mode: "remote" as const,
    api: treaty<Api>({ baseUrl: opts.baseUrl, contract: opts.contract, fetchImpl: opts.fetchImpl }),
  };
}

// ---------------------------------------------------------------- runtime-local

export interface RuntimeContractInfo {
  readonly path: string;
  readonly method: string;
}

/** Load the route table emitted in a published build's contract.json. */
export function contractFromBuild(dist: string): Record<string, RuntimeContractInfo> {
  const { readFileSync } = require("node:fs") as typeof import("node:fs");
  const { join } = require("node:path") as typeof import("node:path");
  const raw = JSON.parse(readFileSync(join(dist, "contract.json"), "utf8")) as {
    routes: Record<string, RuntimeContractInfo>;
  };
  return Object.fromEntries(Object.entries(raw.routes).map(([id, r]) => [id, { path: r.path, method: r.method }]));
}

export interface RuntimeTreatyOptions {
  packPath: string;
  qRuntimeBin?: string;
  port?: number;
  serviceProfile?: string;
  drainTimeoutMs?: number;
}

export interface RuntimeReadyInfo {
  appId?: string;
  routes?: number;
  serviceProfile?: string;
  engine?: string;
  startupMs?: number;
}

export interface RuntimeTreatyHandle<Api extends Record<string, AnyRouteContract>> {
  api: TreatyClient<Api>;
  port: number;
  ready: RuntimeReadyInfo | null;
  close: () => Promise<number>;
  __mode: "runtime-local";
}

/** RUNTIME-LOCAL mode — the actual Rust host + QuickJS worker over HTTP. */
export async function runtimeTreaty<Api extends Record<string, AnyRouteContract> = Record<string, AnyRouteContract>>(
  opts: RuntimeTreatyOptions,
  contract?: Record<string, RuntimeContractInfo>,
): Promise<RuntimeTreatyHandle<Api>> {
  const { resolve, dirname } = require("node:path") as typeof import("node:path");
  const { existsSync } = require("node:fs") as typeof import("node:fs");
  const packRoot = resolve(opts.packPath);
  const projectRoot = resolve(dirname(packRoot), "..", "..", "..");
  // The helper is commonly called from a package/conformance cwd rather than
  // the repository root. Search from the pack's checkout before cwd, while
  // still honoring the explicit env/option overrides.
  const packCandidates = [
    resolve(dirname(packRoot), "..", "..", "target/release/velqu-runtime"),
    resolve(dirname(packRoot), "..", "..", "target/debug/velqu-runtime"),
    resolve(projectRoot, "target/release/velqu-runtime"),
    resolve(projectRoot, "target/debug/velqu-runtime"),
  ];
  const candidates = [
    opts.qRuntimeBin,
    process.env.VELQU_RUNTIME,
    resolve("./target/release/velqu-runtime"),
    resolve("./target/debug/velqu-runtime"),
    resolve(process.cwd(), "target/release/velqu-runtime"),
    resolve(process.cwd(), "target/debug/velqu-runtime"),
    ...packCandidates,
  ].filter(Boolean);
  const bin = candidates.find((p) => existsSync(p!));
  if (!bin) throw new Error(`runtimeTreaty: velqu-runtime binary not found (looked in: ${candidates.join(", ")})`);
  const port = opts.port ?? freePort();
  const resolvedContract = contract ?? contractFromBuild(dirname(packRoot));
  const args = [bin, "--pack", opts.packPath, "--port", String(port)];
  if (opts.serviceProfile) args.push("--service-profile", opts.serviceProfile);
  const proc = Bun.spawn(args, { stdout: "pipe", stderr: "pipe", env: process.env });
  let ready: RuntimeReadyInfo | null = null;
  const consume = async (stream: ReadableStream<Uint8Array>) => {
    const reader = stream.getReader();
    const decoder = new TextDecoder();
    let buffer = "";
    for (;;) {
      const { done, value } = await reader.read();
      if (done) break;
      buffer += decoder.decode(value, { stream: true });
      let idx: number;
      while ((idx = buffer.indexOf("\n")) >= 0) {
        const line = buffer.slice(0, idx).trim();
        buffer = buffer.slice(idx + 1);
        if (ready === null && line.includes('"event":"ready"')) {
          try { ready = JSON.parse(line) as RuntimeReadyInfo; } catch { /* leave null */ }
        }
      }
    }
  };
  // The runtime writes the readiness identity to stdout and diagnostics to
  // stderr. Consume both bounded pipes so either stream can never deadlock.
  void Promise.all([
    consume(proc.stdout as ReadableStream<Uint8Array>),
    consume(proc.stderr as ReadableStream<Uint8Array>),
  ]).catch(() => {});
  const deadline = Date.now() + 10_000;
  for (;;) {
    try {
      const c = await Bun.connect({ hostname: "127.0.0.1", port, socket: { data() {}, open() {} } });
      c.end?.(); c.terminate?.(); break;
    } catch {
      if (Date.now() > deadline) throw new Error("velqu-runtime did not start");
      await Bun.sleep(10);
    }
  }
  const client = treaty<Api>({ baseUrl: `http://127.0.0.1:${port}`, contract: resolvedContract });
  return {
    __mode: "runtime-local",
    api: client,
    port,
    get ready() { return ready; },
    close: async () => {
      proc.kill("SIGTERM");
      const timer = setTimeout(() => {
        try { proc.kill("SIGKILL"); } catch { /* already gone */ }
      }, opts.drainTimeoutMs ?? 5_000);
      const code = await proc.exited;
      clearTimeout(timer);
      return code;
    },
  };
}

function freePort(): number {
  const l = Bun.listen({ hostname: "127.0.0.1", port: 0, socket: { data() {}, open() {} } });
  const port = l.port;
  l.stop(true);
  return port;
}
