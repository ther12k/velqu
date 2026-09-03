/**
 * Candidate: Elysia 2 (AOT) on Bun + native fetch (pinned elysia@2.0.0-beta.4).
 * Implements W1..W4 matched contract (BETA-002-A).
 */
import { Elysia } from "elysia";
import { PORT, UPSTREAM, validateMs, validateFanout, validateOps, cpuWork } from "./shared";
import { DeterministicStore, verifyAuthHeader, type OrderItemInput } from "./matched";

const store = new DeterministicStore();

async function proxyIo(ms: number): Promise<boolean> {
  const upstream = await fetch(`${UPSTREAM}/io?ms=${ms}`);
  return upstream.status === 200;
}

const app = new Elysia({ aot: true })
  // W1: Authenticated Single-Record Lookup
  .get("/api/users/:id", ({ params, headers, set }) => {
    const auth = verifyAuthHeader(headers.authorization);
    if (!auth.ok) {
      set.status = 401;
      return { error: auth.error };
    }
    const user = store.getUser(params.id);
    if (!user) {
      set.status = 404;
      return { error: "not found" };
    }
    return {
      id: user.id,
      name: user.name,
      email: user.email,
      role: user.role,
      createdAt: user.created_at,
    };
  })
  // W2: Authenticated Write Transaction
  .post("/api/orders", ({ body, headers, set }) => {
    const auth = verifyAuthHeader(headers.authorization);
    if (!auth.ok) {
      set.status = 401;
      return { error: auth.error };
    }
    const orderBody = body as { items?: OrderItemInput[] };
    const res = store.createOrder(auth.user.id, orderBody?.items ?? []);
    if (!res.ok) {
      set.status = res.status;
      return { error: res.error };
    }
    set.status = 201;
    return res.order;
  })
  // W3: Paginated List with Aggregation
  .get("/api/products", ({ query }) => {
    const category = query.category ?? "electronics";
    const page = Math.max(1, Number(query.page ?? 1));
    const limit = Math.min(50, Math.max(1, Number(query.limit ?? 20)));
    return store.getProducts(category, page, limit);
  })
  // W4: Controlled I/O & Fan-out
  .get("/api/bench/io", async ({ query, set }) => {
    const ms = validateMs(query.ms ?? null);
    if (ms === null) {
      set.status = 400;
      return { error: "invalid ms" };
    }
    const upstream = await fetch(`${UPSTREAM}/io?ms=${ms}`);
    const body = await upstream.text();
    set.status = upstream.status;
    set.headers["content-type"] =
      upstream.headers.get("content-type") ?? "application/json";
    return body;
  })
  .get("/api/bench/mixed", async ({ query, set }) => {
    const mode = query.mode ?? "";
    if (mode === "timeout") {
      try {
        await fetch(`${UPSTREAM}/io?ms=500`, { signal: AbortSignal.timeout(100) });
      } catch {
        set.status = 504;
        return { mode, handled: "timeout" };
      }
      set.status = 500;
      return { mode, handled: "timeout-unexpected" };
    }
    if (mode === "malformed") {
      const res = await fetch(`${UPSTREAM}/bad`);
      const text = await res.text();
      try {
        JSON.parse(text);
        set.status = 500;
        return { mode, handled: "malformed-unexpected" };
      } catch {
        set.status = 502;
        return { mode, handled: "malformed", problem: "upstream response was not valid JSON" };
      }
    }
    const upstream = await fetch(`${UPSTREAM}/io?ms=5`);
    const body = await upstream.text();
    set.status = upstream.status;
    set.headers["content-type"] = "application/json";
    return body;
  })
  .get("/api/bench/fanout", async ({ query, set }) => {
    const n = validateFanout(query.n ?? null);
    const ms = validateMs(query.ms ?? "5");
    if (n === null || ms === null) {
      set.status = 400;
      return { error: "invalid n or ms" };
    }
    const results = await Promise.all(Array.from({ length: n }, () => proxyIo(ms)));
    return { n, ms, ok: results.every(Boolean) };
  })
  // CPU operation levels (BETA-003-A): deterministic in-handler work, no I/O
  .get("/api/bench/cpu", ({ query, set }) => {
    const ops = validateOps(query.ops ?? null);
    if (ops === null) {
      set.status = 400;
      return { error: "invalid ops" };
    }
    return { ops, checksum: cpuWork(ops) };
  })
  // Unknown routes must answer the shared contract shape, not Elysia's
  // default RFC-9457 body (BETA-002-C contract verification).
  .all("/*", ({ set }) => {
    set.status = 404;
    return { error: "not found" };
  })
  .listen(PORT);

console.log(
  JSON.stringify({ event: "candidate.ready", candidate: "elysia2", port: app.server?.port }),
);
