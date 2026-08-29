/** W4/Fanout candidate: Bun runtime + Bun-native fetch (no framework). */
import { PORT, UPSTREAM, validateMs, validateFanout } from "./shared";

async function proxyIo(ms: number): Promise<{ status: number; body: string }> {
  const upstream = await fetch(`${UPSTREAM}/io?ms=${ms}`);
  return { status: upstream.status, body: await upstream.text() };
}

const server = Bun.serve({
  hostname: "127.0.0.1",
  port: PORT,
  async fetch(req) {
    const url = new URL(req.url);
    if (url.pathname === "/api/bench/io") {
      const ms = validateMs(url.searchParams.get("ms"));
      if (ms === null) return Response.json({ error: "invalid ms" }, { status: 400 });
      const r = await proxyIo(ms);
      return new Response(r.body, {
        status: r.status,
        headers: { "content-type": "application/json" },
      });
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
    return Response.json({ error: "not found", path: url.pathname }, { status: 404 });
  },
});
console.log(JSON.stringify({ event: "candidate.ready", candidate: "bun-fetch", port: server.port }));
