/**
 * Candidate: Fastify on Node (pinned fastify@5.12.1) + Node global fetch.
 * Implements W1..W4 matched contract (BETA-002-A).
 */
const fastify = require("fastify")({ logger: false });
const { PORT, UPSTREAM, validateMs, validateFanout } = require("./shared.cjs");
const { DeterministicStore, verifyAuthHeader } = require("./matched.cjs");

const store = new DeterministicStore();

async function proxyIo(ms) {
  const upstream = await fetch(`${UPSTREAM}/io?ms=${ms}`);
  return upstream.status === 200;
}

// W1: Authenticated Single-Record Lookup
fastify.get("/api/users/:id", async (request, reply) => {
  const auth = verifyAuthHeader(request.headers.authorization);
  if (!auth.ok) return reply.code(401).send({ error: auth.error });

  const user = store.getUser(request.params.id);
  if (!user) return reply.code(404).send({ error: "not found" });

  return {
    id: user.id,
    name: user.name,
    email: user.email,
    role: user.role,
    createdAt: user.created_at,
  };
});

// W2: Authenticated Write Transaction
fastify.post("/api/orders", async (request, reply) => {
  const auth = verifyAuthHeader(request.headers.authorization);
  if (!auth.ok) return reply.code(401).send({ error: auth.error });

  const body = request.body || {};
  const res = store.createOrder(auth.user.id, body.items || []);
  if (!res.ok) return reply.code(res.status).send({ error: res.error });

  return reply.code(201).send(res.order);
});

// W3: Paginated List with Aggregation
fastify.get("/api/products", async (request, reply) => {
  const category = request.query.category || "electronics";
  const page = Math.max(1, Number(request.query.page || 1));
  const limit = Math.min(50, Math.max(1, Number(request.query.limit || 20)));

  return store.getProducts(category, page, limit);
});

// W4: Controlled I/O & Fan-out
fastify.get("/api/bench/io", async (request, reply) => {
  const ms = validateMs(request.query.ms ?? null);
  if (ms === null) return reply.code(400).send({ error: "invalid ms" });
  const upstream = await fetch(`${UPSTREAM}/io?ms=${ms}`);
  const body = await upstream.text();
  reply.code(upstream.status).header(
    "content-type",
    upstream.headers.get("content-type") ?? "application/json",
  );
  return body;
});

fastify.get("/api/bench/mixed", async (request, reply) => {
  const mode = request.query.mode ?? "";
  if (mode === "timeout") {
    try {
      await fetch(`${UPSTREAM}/io?ms=500`, { signal: AbortSignal.timeout(100) });
    } catch {
      return reply.code(504).send({ mode, handled: "timeout" });
    }
    return reply.code(500).send({ mode, handled: "timeout-unexpected" });
  }
  if (mode === "malformed") {
    const res = await fetch(`${UPSTREAM}/bad`);
    const text = await res.text();
    try {
      JSON.parse(text);
      return reply.code(500).send({ mode, handled: "malformed-unexpected" });
    } catch {
      return reply.code(502).send({
        mode,
        handled: "malformed",
        problem: "upstream response was not valid JSON",
      });
    }
  }
  const upstream = await fetch(`${UPSTREAM}/io?ms=5`);
  const body = await upstream.text();
  reply.code(upstream.status).header("content-type", "application/json");
  return body;
});

fastify.get("/api/bench/fanout", async (request, reply) => {
  const n = validateFanout(request.query.n ?? null);
  const ms = validateMs(request.query.ms ?? "5");
  if (n === null || ms === null) return reply.code(400).send({ error: "invalid n or ms" });
  const results = await Promise.all(Array.from({ length: n }, () => proxyIo(ms)));
  return { n, ms, ok: results.every(Boolean) };
});

fastify.setNotFoundHandler((request, reply) => {
  reply.code(404).send({ error: "not found" });
});

fastify.listen({ port: PORT, host: "127.0.0.1" }, (err) => {
  if (err) {
    console.error(JSON.stringify({ event: "candidate.failed", error: String(err) }));
    process.exit(1);
  }
  const addr = fastify.server.address();
  const port = typeof addr === "object" && addr ? addr.port : PORT;
  console.log(JSON.stringify({ event: "candidate.ready", candidate: "fastify", port }));
});
