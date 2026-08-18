/**
 * @velqu/testing — Treaty local test adapters. The two modes are LABELED and
 * reported separately (TRT-005): unit-local executes handlers in-process
 * under Bun; runtime-local drives the ACTUAL velqu-runtime binary over HTTP.
 */
import { treaty } from "@velqu/treaty";

// ---------------------------------------------------------------- unit-local

export interface UnitTreatyOptions {
  /** app route table to expose; handlers run in THIS process */
  routes: Record<string, { path: string; method: string; handle: (ctx: unknown) => Promise<unknown> | unknown }>;
  policies?: Record<string, (req: unknown) => Promise<unknown>>;
}

/**
 * UNIT-LOCAL mode — no native runtime involved. Fast but NOT runtime
 * conformance; results must be labeled unit-local.
 */
export function unitTreaty<Api extends Record<string, never> = Record<string, never>>(
  opts: UnitTreatyOptions,
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
): any {
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
  const client = treaty<never>({ baseUrl: base, contract });
  return {
    __mode: "unit-local (NOT runtime conformance)" as const,
    close: () => server.stop(true),
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    api: client as any,
  };
}

// ---------------------------------------------------------------- runtime-local

export interface RuntimeTreatyOptions {
  packPath: string;
  qRuntimeBin?: string;
  port?: number;
}

/**
 * RUNTIME-LOCAL mode — spawns the actual velqu-runtime binary with the pack and
 * drives real HTTP. THIS is native-runtime conformance evidence.
 */
export async function runtimeTreaty<Api extends Record<string, never> = Record<string, never>>(
  opts: RuntimeTreatyOptions,
  contract: Record<string, { path: string; method: string }>,
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
): Promise<{ api: any; close: () => Promise<void>; __mode: "runtime-local" }> {
  const { resolve } = require("node:path");
  const { existsSync } = require("node:fs");
  const candidates = [
    opts.qRuntimeBin,
    process.env.VELQU_RUNTIME,
    resolve("./target/release/velqu-runtime"),
    resolve("./target/debug/velqu-runtime"),
    resolve(process.cwd(), "target/release/velqu-runtime"),
    resolve(process.cwd(), "target/debug/velqu-runtime"),
  ].filter(Boolean);
  const bin = candidates.find((p) => existsSync(p!));
  if (!bin) {
    throw new Error(`runtimeTreaty: velqu-runtime binary not found (looked in: ${candidates.join(", ")})`);
  }
  const port = opts.port ?? freePort();
  const proc = Bun.spawn([bin, "--pack", opts.packPath, "--port", String(port)], {
    stdout: "ignore",
    stderr: "ignore",
    env: process.env,
  });
  // wait ready
  const deadline = Date.now() + 10_000;
  for (;;) {
    try {
      const c = await Bun.connect({ hostname: "127.0.0.1", port, socket: { data() {}, open() {} } });
      c.end?.();
      c.terminate?.();
      break;
    } catch {
      if (Date.now() > deadline) throw new Error("velqu-runtime did not start");
      await Bun.sleep(10);
    }
  }
  const client = treaty<never>({ baseUrl: `http://127.0.0.1:${port}`, contract });
  return {
    __mode: "runtime-local",
    api: client,
    close: async () => {
      proc.kill();
      await proc.exited;
    },
  };
}

function freePort(): number {
  const l = Bun.listen({ hostname: "127.0.0.1", port: 0, socket: { data() {}, open() {} } });
  const port = l.port;
  l.stop(true);
  return port;
}
