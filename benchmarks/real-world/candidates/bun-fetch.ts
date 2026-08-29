/** W4/Fanout candidate: Bun runtime + Bun-native fetch (no framework). */
import { PORT, UPSTREAM, validateMs, validateFanout } from "./shared";


// M28-011-C mixed-outcome route (added to the Bun-based candidates):
//   success   -> relay upstream 200
//   timeout   -> upstream 500ms vs a 100ms client deadline -> typed 504
//   malformed -> upstream /bad (200 + garbage) -> parse failure -> typed 502
function mixedHandler() {
  return async (mode: string) => {
    if (mode === "timeout") {
      try {
        await fetch(`${UPSTREAM}/io?ms=500`, {
          signal: AbortSignal.timeout(100),
        });
      } catch {
        // deadline abort: the typed 504 is the contract
        return Response.json({ mode, handled: "timeout" }, { status: 504 });
      }
      return Response.json({ mode, handled: "timeout-unexpected" }, { status: 500 });
    }
    if (mode === "malformed") {
      const res = await fetch(`${UPSTREAM}/bad`);
      const text = await res.text();
      try {
        JSON.parse(text);
        // The fixture is deterministic garbage; reaching here is a failure.
        return Response.json({ mode, handled: "malformed-unexpected" }, { status: 500 });
      } catch {
        return Response.json(
          { mode, handled: "malformed", problem: "upstream response was not valid JSON" },
          { status: 502 },
        );
      }
    }
    const res = await fetch(`${UPSTREAM}/io?ms=5`);
    const body = await res.text();
    return new Response(body, {
      status: res.status,
      headers: { "content-type": "application/json" },
    });
  };
}

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
    if (url.pathname === "/api/bench/mixed") {
      const mode = url.searchParams.get("mode");
      const handler = mixedHandler();
      return handler(mode ?? "");
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
