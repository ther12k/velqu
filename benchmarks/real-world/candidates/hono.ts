/**
 * Candidate: Hono on Bun + Bun-native fetch (pinned hono@4.13.5).
 * Implements W1..W4 matched contract (BETA-002-A).
 */
import { Hono } from "hono";
import { PORT, UPSTREAM, validateMs, validateFanout, validateOps, cpuWork } from "./shared";
import { DeterministicStore, verifyAuthHeader, type OrderItemInput } from "./matched";

const app = new Hono();
const store = new DeterministicStore();

// W1: Authenticated Single-Record Lookup
app.get("/api/users/:id", (c) => {
  const auth = verifyAuthHeader(c.req.header("authorization"));
  if (!auth.ok) return c.json({ error: auth.error }, 401);

  const user = store.getUser(c.req.param("id"));
  if (!user) return c.json({ error: "not found" }, 404);

  return c.json({
    id: user.id,
    name: user.name,
    email: user.email,
    role: user.role,
    createdAt: user.created_at,
  });
});

// W2: Authenticated Write Transaction
app.post("/api/orders", async (c) => {
  const auth = verifyAuthHeader(c.req.header("authorization"));
  if (!auth.ok) return c.json({ error: auth.error }, 401);

  let body: { items?: OrderItemInput[] };
  try {
    body = await c.req.json();
  } catch {
    return c.json({ error: "malformed JSON body" }, 400);
  }

  const res = store.createOrder(auth.user.id, body.items ?? []);
  if (!res.ok) return c.json({ error: res.error }, res.status as any);

  return c.json(res.order, 201);
});

// W3: Paginated List with Aggregation
app.get("/api/products", (c) => {
  const category = c.req.query("category") ?? "electronics";
  const page = Math.max(1, Number(c.req.query("page") ?? 1));
  const limit = Math.min(50, Math.max(1, Number(c.req.query("limit") ?? 20)));

  const res = store.getProducts(category, page, limit);
  return c.json(res);
});

// W4: Controlled I/O & Fan-out
async function proxyIo(ms: number): Promise<boolean> {
  const upstream = await fetch(`${UPSTREAM}/io?ms=${ms}`);
  return upstream.status === 200;
}

app.get("/api/bench/io", async (c) => {
  const ms = validateMs(c.req.query("ms"));
  if (ms === null) return c.json({ error: "invalid ms" }, 400);
  const upstream = await fetch(`${UPSTREAM}/io?ms=${ms}`);
  const body = await upstream.text();
  return c.body(body, upstream.status, {
    "content-type": upstream.headers.get("content-type") ?? "application/json",
  });
});

app.get("/api/bench/mixed", async (c) => {
  const mode = c.req.query("mode") ?? "";
  if (mode === "timeout") {
    try {
      await fetch(`${UPSTREAM}/io?ms=500`, { signal: AbortSignal.timeout(100) });
    } catch {
      return c.json({ mode, handled: "timeout" }, 504);
    }
    return c.json({ mode, handled: "timeout-unexpected" }, 500);
  }
  if (mode === "malformed") {
    const res = await fetch(`${UPSTREAM}/bad`);
    const text = await res.text();
    try {
      JSON.parse(text);
      return c.json({ mode, handled: "malformed-unexpected" }, 500);
    } catch {
      return c.json(
        { mode, handled: "malformed", problem: "upstream response was not valid JSON" },
        502,
      );
    }
  }
  const upstream = await fetch(`${UPSTREAM}/io?ms=5`);
  const body = await upstream.text();
  return c.body(body, upstream.status, { "content-type": "application/json" });
});

app.get("/api/bench/fanout", async (c) => {
  const n = validateFanout(c.req.query("n"));
  const ms = validateMs(c.req.query("ms") ?? "5");
  if (n === null || ms === null) return c.json({ error: "invalid n or ms" }, 400);
  const results = await Promise.all(Array.from({ length: n }, () => proxyIo(ms)));
  return c.json({ n, ms, ok: results.every(Boolean) });
});

// CPU operation levels (BETA-003-A): deterministic in-handler work, no I/O
app.get("/api/bench/cpu", (c) => {
  const ops = validateOps(c.req.query("ops"));
  if (ops === null) return c.json({ error: "invalid ops" }, 400);
  return c.json({ ops, checksum: cpuWork(ops) });
});

app.all("*", (c) => c.json({ error: "not found" }, 404));

const server = Bun.serve({ hostname: "127.0.0.1", port: PORT, fetch: app.fetch });
console.log(JSON.stringify({ event: "candidate.ready", candidate: "hono", port: server.port }));
