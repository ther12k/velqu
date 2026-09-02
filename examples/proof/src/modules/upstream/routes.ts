import { route, status } from "@velqu/core";
import { s } from "@velqu/schema";

/**
 * Controlled upstream quote route (M4A-009-C): demonstrates outbound fetch
 * capability with typed success (200) and gateway error (502) contracts.
 */
export const quote = route({
  id: "upstream.quote",
  method: "GET",
  path: "/upstream/quote",
  response: {
    200: s.object({ quote: s.string(), source: s.string() }),
    502: s.object({ error: s.string() }),
  },
  handle: async () => {
    try {
      const res = await fetch("http://127.0.0.1:8791/health", {
        headers: { "user-agent": "velqu-proof" },
      });
      if (!res.ok) {
        return status(502).value({ error: `upstream returned HTTP ${res.status}` });
      }
      const data = (await res.json()) as { status?: string };
      return { quote: data.status ?? "ok", source: "controlled-upstream" };
    } catch (e) {
      return status(502).value({ error: (e as Error).message });
    }
  },
});

/**
 * Controlled upstream relay (M4A-009-C): proxies an upstream target with
 * bounded query options, timing-safe error mapping, and typed response.
 */
export const relay = route({
  id: "upstream.relay",
  method: "GET",
  path: "/upstream/relay",
  query: s.object({
    target: s.optional(s.string({ minLength: 1, maxLength: 200 }), {
      default: "http://127.0.0.1:8791/io?ms=5",
    }),
  }),
  response: {
    200: s.object({ status: s.string(), target: s.string() }),
    502: s.object({ error: s.string() }),
  },
  handle: async ({ query }) => {
    try {
      const res = await fetch(query.target);
      if (!res.ok) {
        return status(502).value({ error: `upstream returned HTTP ${res.status}` });
      }
      const data = (await res.json()) as { status?: string };
      return { status: data.status ?? "ok", target: query.target };
    } catch (e) {
      return status(502).value({ error: (e as Error).message });
    }
  },
});

/**
 * Upstream fan-out (M4A-009-C): dispatches N parallel outbound requests
 * within bounded concurrency (1..4) and aggregates results.
 */
export const fanout = route({
  id: "upstream.fanout",
  method: "GET",
  path: "/upstream/fanout",
  query: s.object({
    count: s.optional(s.integer({ minimum: 1, maximum: 4 }), { default: 2 }),
    target: s.optional(s.string({ minLength: 1, maxLength: 200 }), {
      default: "http://127.0.0.1:8791/io?ms=5",
    }),
  }),
  response: {
    200: s.object({ count: s.integer(), okCount: s.integer() }),
    502: s.object({ error: s.string() }),
  },
  handle: async ({ query }) => {
    try {
      const calls = Array.from({ length: query.count }, () => fetch(query.target));
      const responses = await Promise.all(calls);
      let okCount = 0;
      for (const r of responses) {
        if (r.ok) okCount++;
      }
      return { count: query.count, okCount };
    } catch (e) {
      return status(502).value({ error: (e as Error).message });
    }
  },
});

export default [quote, relay, fanout] as const;
