/** W4/Fanout candidate: Elysia 2 (AOT) on Bun + native fetch (pinned elysia@2.0.0-beta.4). */
import { Elysia } from "elysia";
import { PORT, UPSTREAM, validateMs, validateFanout } from "./shared";

async function proxyIo(ms: number): Promise<boolean> {
  const upstream = await fetch(`${UPSTREAM}/io?ms=${ms}`);
  return upstream.status === 200;
}

const app = new Elysia({ aot: true })
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
  .listen(PORT);

console.log(
  JSON.stringify({ event: "candidate.ready", candidate: "elysia2", port: app.server?.port }),
);
