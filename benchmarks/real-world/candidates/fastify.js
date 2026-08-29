/** W4 candidate: Fastify on Node (pinned fastify@5.12.1) + Node global fetch. */
const fastify = require("fastify")({ logger: false });
const { PORT, UPSTREAM, validateMs } = require("./shared.cjs");

fastify.get("/api/bench/io", async (request, reply) => {
  const ms = validateMs(request.query.ms ?? null);
  if (ms === null) {
    return reply.code(400).send({ error: "invalid ms" });
  }
  const upstream = await fetch(`${UPSTREAM}/io?ms=${ms}`);
  const body = await upstream.text();
  reply.code(upstream.status).header(
    "content-type",
    upstream.headers.get("content-type") ?? "application/json",
  );
  return body;
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
