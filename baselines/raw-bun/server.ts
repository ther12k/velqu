/**
 * raw-bun baseline: idiomatic Bun.serve, zero npm dependencies.
 * Implements the frozen fixture contract exactly (benchmarks/fixtures/fixture-contract.json).
 * PORT env (default 3000); N_ROUTES env (0 = canonical fixture; N>0 adds the
 * generated item routes for PERF-005).
 */

const PORT = parseInt(process.env.PORT ?? "3000", 10);
const N_ROUTES = parseInt(process.env.N_ROUTES ?? "0", 10);

// lazy in-memory users service (first use seeds the fixture user)
let users: Map<string, { id: string; name: string; email: string }> | null = null;
let nextUser = 1;
function usersService() {
  if (users === null) {
    users = new Map();
    users.set("usr_1", { id: "usr_1", name: "Ada", email: "ada@example.org" });
  }
  return users;
}

function json(body: unknown, status = 200): Response {
  return new Response(JSON.stringify(body), {
    status,
    headers: { "content-type": "application/json" },
  });
}

function validationProblem(field: string, code: string, message: string): Response {
  return json(
    {
      type: "https://velqu.dev/problems/validation",
      title: "Validation failed",
      status: 422,
      errors: [{ path: field, code, message }],
    },
    422,
  );
}

const isEmail = (s: string) => /^[^@\s]+@[^@\s]+\.[^@\s]+$/.test(s);
const isUsrId = (s: string) => /^usr_[0-9]+$/.test(s);

async function handle(req: Request): Promise<Response> {
  const url = new URL(req.url);
  const path = url.pathname;
  const method = req.method.toUpperCase();

  // ---- generated item routes (route-count benchmark)
  if (N_ROUTES > 0 && path.startsWith("/res")) {
    const m = path.match(/^\/res(\d+)\/item\/(\d+)$/);
    if (m && method === "GET") {
      const n = parseInt(m[1], 10);
      const id = parseInt(m[2], 10);
      if (n >= 0 && n < N_ROUTES && id >= 1 && id <= N_ROUTES) {
        return json({ id, n: N_ROUTES });
      }
      return validationProblem("id", "minimum", "out of range");
    }
  }

  // ---- canonical fixture routes
  if (path === "/health/live" && (method === "GET" || method === "HEAD")) {
    return json({ status: "ok" });
  }
  if (path === "/js-text" && method === "GET") {
    return new Response("plain", { headers: { "content-type": "text/plain; charset=utf-8" } });
  }
  if (path === "/js-json" && method === "GET") {
    return json({ ok: true });
  }
  const hello = path.match(/^\/hello\/([^/]+)$/);
  if (hello && method === "GET") {
    const name = decodeURIComponent(hello[1]);
    if (name.length < 1 || name.length > 60) return validationProblem("name", "maxLength", "must be at most 60 characters");
    return json({ message: `Hello ${name}` });
  }
  if (path === "/users" && method === "POST") {
    let body: { name?: unknown; email?: unknown };
    try {
      body = await req.json();
    } catch {
      return json({ type: "https://velqu.dev/problems/validation", title: "Validation failed", status: 422, detail: "malformed JSON body" }, 422);
    }
    if (typeof body.name !== "string" || body.name.length < 1 || body.name.length > 60) {
      return validationProblem("name", "maxLength", "must be 1-60 characters");
    }
    if (typeof body.email !== "string" || !isEmail(body.email)) {
      return validationProblem("email", "format", "must be a valid email");
    }
    const id = `usr_${nextUser++}`;
    const u = { id, name: body.name, email: body.email };
    usersService().set(id, u);
    return json(u, 201);
  }
  const user = path.match(/^\/users\/([^/]+)$/);
  if (user && method === "GET") {
    const auth = req.headers.get("authorization");
    if (auth !== "Bearer q-demo-token") {
      return json({ type: "https://velqu.dev/problems/unauthorized", title: "Unauthorized", status: 401 }, 401);
    }
    const id = decodeURIComponent(user[1]);
    if (!isUsrId(id)) return validationProblem("id", "pattern", "must match ^usr_[0-9]+$");
    const u = usersService().get(id);
    if (!u) return json({ type: "https://velqu.dev/problems/not-found", title: "Not Found", status: 404 }, 404);
    return json(u);
  }
  if (path === "/async" && method === "GET") {
    const msRaw = url.searchParams.get("ms");
    const ms = msRaw === null ? 10 : parseInt(msRaw, 10);
    if (!Number.isInteger(ms) || ms < 1 || ms > 1000) {
      return validationProblem("ms", "maximum", "must be 1-1000");
    }
    await Bun.sleep(ms);
    return json({ waited: ms });
  }
  if (path === "/cancel" && method === "GET") {
    const msRaw = url.searchParams.get("ms");
    const ms = msRaw === null ? 1000 : parseInt(msRaw, 10);
    await Bun.sleep(ms);
    return json({ cancelled: false, waited: ms });
  }
  if (path === "/throw" && method === "GET") {
    throw new Error("secret-boom");
  }

  // 405 when the path exists under another method
  const knownPaths: [string, string][] = [
    ["/health/live", "GET"], ["/js-text", "GET"], ["/js-json", "GET"],
    ["/users", "POST"], ["/async", "GET"], ["/cancel", "GET"], ["/throw", "GET"],
  ];
  const matched = knownPaths.find(([p]) => p === path) ?? (hello || user ? [path, "GET"] : undefined);
  if (matched) {
    return new Response(
      JSON.stringify({ type: "https://velqu.dev/problems/method", title: "Method Not Allowed", status: 405 }),
      { status: 405, headers: { "content-type": "application/json", allow: matched[1] === "GET" ? "GET, HEAD" : "POST" } },
    );
  }
  return json({ type: "https://velqu.dev/problems/not-found", title: "Not Found", status: 404 }, 404);
}

const server = Bun.serve({
  port: PORT,
  hostname: "127.0.0.1",
  reusePort: false,
  async fetch(req) {
    try {
      return await handle(req);
    } catch {
      // redacted internal error: no message, no stack
      return json({ type: "https://velqu.dev/problems/internal", title: "Internal Server Error", status: 500 }, 500);
    }
  },
});

console.log(`raw-bun ready port=${server.port} routes=${N_ROUTES > 0 ? "generated" : "fixture"}`);
