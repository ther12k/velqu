/** W4/Fanout candidate: Hono on Bun + Bun-native fetch (pinned hono@4.13.5). */
import { Hono } from "hono";
import { PORT, UPSTREAM, validateMs, validateFanout } from "./shared";

const app = new Hono();

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
app.all("*", (c) => c.json({ error: "not found" }, 404));

const server = Bun.serve({ hostname: "127.0.0.1", port: PORT, fetch: app.fetch });
console.log(JSON.stringify({ event: "candidate.ready", candidate: "hono", port: server.port }));
