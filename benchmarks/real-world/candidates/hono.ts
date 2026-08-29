/** W4 candidate: Hono on Bun + Bun-native fetch (pinned hono@4.13.5). */
import { Hono } from "hono";
import { PORT, UPSTREAM, validateMs } from "./shared";

const app = new Hono();
app.get("/api/bench/io", async (c) => {
  const ms = validateMs(c.req.query("ms"));
  if (ms === null) {
    return c.json({ error: "invalid ms" }, 400);
  }
  const upstream = await fetch(`${UPSTREAM}/io?ms=${ms}`);
  const body = await upstream.text();
  return c.body(body, upstream.status, {
    "content-type": upstream.headers.get("content-type") ?? "application/json",
  });
});
app.all("*", (c) => c.json({ error: "not found" }, 404));

const server = Bun.serve({
  hostname: "127.0.0.1",
  port: PORT,
  fetch: app.fetch,
});
console.log(JSON.stringify({ event: "candidate.ready", candidate: "hono", port: server.port }));
