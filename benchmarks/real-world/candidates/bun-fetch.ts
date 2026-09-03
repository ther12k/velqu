/**
 * Candidate: Bun runtime + Bun-native fetch (no framework).
 * Implements W1..W4 matched contract (BETA-002-A).
 */
import { PORT, UPSTREAM, validateMs, validateFanout } from "./shared";
import { DeterministicStore, verifyAuthHeader, type OrderItemInput } from "./matched";

const store = new DeterministicStore();

function mixedHandler() {
  return async (mode: string) => {
    if (mode === "timeout") {
      try {
        await fetch(`${UPSTREAM}/io?ms=500`, {
          signal: AbortSignal.timeout(100),
        });
      } catch {
        return Response.json({ mode, handled: "timeout" }, { status: 504 });
      }
      return Response.json({ mode, handled: "timeout-unexpected" }, { status: 500 });
    }
    if (mode === "malformed") {
      const res = await fetch(`${UPSTREAM}/bad`);
      const text = await res.text();
      try {
        JSON.parse(text);
        return Response.json({ mode, handled: "malformed-unexpected" }, { status: 500 });
      } catch {
        return Response.json(
          { mode, handled: "malformed", problem: "upstream response was not valid JSON" },
          { status: 502 },
        );
      }
    }
    const res = await fetch(`${UPSTREAM}/io?ms=5`);
    const body = await res.text();
    return new Response(body, {
      status: res.status,
      headers: { "content-type": "application/json" },
    });
  };
}

async function proxyIo(ms: number): Promise<{ status: number; body: string }> {
  const upstream = await fetch(`${UPSTREAM}/io?ms=${ms}`);
  return { status: upstream.status, body: await upstream.text() };
}

const server = Bun.serve({
  hostname: "127.0.0.1",
  port: PORT,
  async fetch(req) {
    const url = new URL(req.url);

    // W1: Authenticated Single-Record Lookup
    if (req.method === "GET" && url.pathname.startsWith("/api/users/")) {
      const auth = verifyAuthHeader(req.headers.get("authorization"));
      if (!auth.ok) return Response.json({ error: auth.error }, { status: 401 });

      const id = url.pathname.slice("/api/users/".length);
      const user = store.getUser(id);
      if (!user) return Response.json({ error: "not found" }, { status: 404 });

      return Response.json({
        id: user.id,
        name: user.name,
        email: user.email,
        role: user.role,
        createdAt: user.created_at,
      });
    }

    // W2: Authenticated Write Transaction
    if (req.method === "POST" && url.pathname === "/api/orders") {
      const auth = verifyAuthHeader(req.headers.get("authorization"));
      if (!auth.ok) return Response.json({ error: auth.error }, { status: 401 });

      let body: { items?: OrderItemInput[] };
      try {
        body = await req.json();
      } catch {
        return Response.json({ error: "malformed JSON body" }, { status: 400 });
      }

      const res = store.createOrder(auth.user.id, body.items ?? []);
      if (!res.ok) return Response.json({ error: res.error }, { status: res.status });

      return Response.json(res.order, { status: 201 });
    }

    // W3: Paginated List with Aggregation
    if (req.method === "GET" && url.pathname === "/api/products") {
      const category = url.searchParams.get("category") ?? "electronics";
      const page = Math.max(1, Number(url.searchParams.get("page") ?? 1));
      const limit = Math.min(50, Math.max(1, Number(url.searchParams.get("limit") ?? 20)));

      const res = store.getProducts(category, page, limit);
      return Response.json(res);
    }

    // W4: Controlled I/O & Fan-out
    if (url.pathname === "/api/bench/io") {
      const ms = validateMs(url.searchParams.get("ms"));
      if (ms === null) return Response.json({ error: "invalid ms" }, { status: 400 });
      const r = await proxyIo(ms);
      return new Response(r.body, {
        status: r.status,
        headers: { "content-type": "application/json" },
      });
    }
    if (url.pathname === "/api/bench/mixed") {
      const mode = url.searchParams.get("mode");
      const handler = mixedHandler();
      return handler(mode ?? "");
    }
    if (url.pathname === "/api/bench/fanout") {
      const n = validateFanout(url.searchParams.get("n"));
      const ms = validateMs(url.searchParams.get("ms") ?? "5");
      if (n === null || ms === null) {
        return Response.json({ error: "invalid n or ms" }, { status: 400 });
      }
      const results = await Promise.all(
        Array.from({ length: n }, () => proxyIo(ms)),
      );
      return Response.json({
        n,
        ms,
        ok: results.every((r) => r.status === 200),
      });
    }

    return Response.json({ error: "not found" }, { status: 404 });
  },
});
console.log(JSON.stringify({ event: "candidate.ready", candidate: "bun-fetch", port: server.port }));
