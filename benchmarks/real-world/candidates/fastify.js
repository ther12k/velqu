/** W4/Fanout candidate: Fastify on Node (pinned fastify@5.12.1) + Node global fetch. */
const fastify = require("fastify")({ logger: false });
const { PORT, UPSTREAM, validateMs, validateFanout } = require("./shared.cjs");

async function proxyIo(ms) {
  const upstream = await fetch(`${UPSTREAM}/io?ms=${ms}`);
  return upstream.status === 200;
}

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
