/** W4 candidate: Bun runtime + Bun-native fetch (no framework). */
import { PORT, UPSTREAM, validateMs } from "./shared";

const server = Bun.serve({
  hostname: "127.0.0.1",
  port: PORT,
  async fetch(req) {
    const url = new URL(req.url);
    if (url.pathname !== "/api/bench/io") {
      return Response.json({ error: "not found", path: url.pathname }, { status: 404 });
    }
    const ms = validateMs(url.searchParams.get("ms"));
    if (ms === null) {
      return Response.json({ error: "invalid ms" }, { status: 400 });
    }
    const upstream = await fetch(`${UPSTREAM}/io?ms=${ms}`);
    const body = await upstream.text();
    return new Response(body, {
      status: upstream.status,
      headers: { "content-type": upstream.headers.get("content-type") ?? "application/json" },
    });
  },
});
console.log(JSON.stringify({ event: "candidate.ready", candidate: "bun-fetch", port: server.port }));
