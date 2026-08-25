/**
 * Controlled upstream service (BETA-001-A, workload W4).
 *
 * Deterministic latency source for the real-world benchmark: `GET /io?ms=N`
 * responds with a fixed JSON body after a timer-based delay of N milliseconds.
 * Candidates proxy this service so every framework faces identical upstream
 * latency without a database in the path.
 *
 * Endpoints:
 *   GET /health   -> {"status":"ok"} (no delay)
 *   GET /io?ms=N  -> {"status":"ok","ms":N} after N ms (N: 0..1000, integer)
 *
 * Invalid requests fail fast with RFC-9457-shaped problems (400/404) and are
 * never silently delayed.
 */

const PORT = Number(process.env.PORT ?? 8791);
const MAX_MS = 1000;

const httpServer = Bun.serve({
  hostname: "127.0.0.1",
  port: PORT,
  fetch(req) {
    const url = new URL(req.url);
    if (url.pathname === "/health") {
      return Response.json({ status: "ok" });
    }
    if (url.pathname !== "/io") {
      return problem(404, `unknown path: ${url.pathname}`);
    }
    const msRaw = url.searchParams.get("ms") ?? "";
    if (!/^\d{1,4}$/.test(msRaw)) {
      return problem(400, `ms must be a non-negative integer, got: ${JSON.stringify(msRaw)}`);
    }
    const ms = Number(msRaw);
    if (ms > MAX_MS) {
      return problem(400, `ms must be <= ${MAX_MS}, got: ${ms}`);
    }
    const started = performance.now();
    return new Promise<Response>((resolve) => {
      setTimeout(() => {
        resolve(
          Response.json({
            status: "ok",
            ms,
            actualMs: Math.round((performance.now() - started) * 1000) / 1000,
          }),
        );
      }, ms);
    });
  },
});

function problem(status: number, detail: string): Response {
  return Response.json(
    { type: "https://velqu.dev/problems/upstream", title: "Upstream request rejected", status, detail },
    { status },
  );
}

console.log(JSON.stringify({ event: "upstream.ready", port: httpServer.port, pid: process.pid }));

function shutdown() {
  httpServer.stop(true);
  process.exit(0);
}
process.on("SIGINT", shutdown);
process.on("SIGTERM", shutdown);
