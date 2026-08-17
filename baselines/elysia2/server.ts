/**
 * elysia2 baseline: Elysia 2 AOT on Bun (pinned elysia@2.0.0-beta.4).
 * Implements the frozen fixture contract (benchmarks/fixtures/fixture-contract.json).
 */

import { Elysia, t } from "elysia";

const PORT = parseInt(process.env.PORT ?? "3000", 10);
const N_ROUTES = parseInt(process.env.N_ROUTES ?? "0", 10);

// lazy in-memory users service (first use seeds the fixture user usr_1)
let users: Map<string, { id: string; name: string; email: string }> | null = null;
let nextUser = 1;
function usersService() {
  if (users === null) {
    users = new Map();
    users.set("usr_1", { id: "usr_1", name: "Ada", email: "ada@example.org" });
  }
  return users;
}

const app = new Elysia({ aot: true })
  .error(({ error, set }) => {
    // Elysia 2 error objects carry `status` + `name`
    // (ValidationError=422, NotFound=404; ParseError=400 → fixture 422)
    const status = error?.status ?? 500;
    if (status === 422) {
      set.status = 422;
      return error; // validation detail: identifies property + keyword
    }
    if (status === 400) {
      // malformed JSON: a raw Response bypasses Elysia's status pinning
      return new Response(
        JSON.stringify({
          type: "https://velqu.dev/problems/validation",
          title: "Validation failed",
          status: 422,
          detail: "malformed JSON body",
        }),
        { status: 422, headers: { "content-type": "application/json" } },
      );
    }
    if (status === 404) {
      set.status = 404;
      return { type: "https://velqu.dev/problems/not-found", title: "Not Found", status: 404 };
    }
    // All other errors (including throw "secret-boom"): redact message and stack (SEC-004)
    set.status = 500;
    return {
      type: "https://velqu.dev/problems/internal",
      title: "Internal Server Error",
      status: 500,
    };
  })
  // C0: static liveness (GET and HEAD)
  .get("/health/live", () => ({ status: "ok" }))
  .head("/health/live", ({ set }) => {
    set.status = 200;
    return "";
  })
  // C1: plaintext
  .get("/js-text", ({ set }) => {
    set.headers["content-type"] = "text/plain; charset=utf-8";
    return "plain";
  })
  // C2: small JSON
  .get("/js-json", () => ({ ok: true }))
  // C3: hello with path param validation
  .get(
    "/hello/:name",
    {
      params: t.Object({
        name: t.String({ minLength: 1, maxLength: 60 }),
      }),
    },
    ({ params: { name } }) => ({ message: `Hello ${name}` }),
  )
  // C3: users.create with body schema validation
  .post(
    "/users",
    {
      body: t.Object({
        name: t.String({ minLength: 1, maxLength: 60 }),
        email: t.String({ format: "email" }),
      }),
    },
    ({ body, set }) => {
      const id = `usr_${nextUser++}`;
      const u = { id, name: body.name, email: body.email };
      usersService().set(id, u);
      set.status = 201;
      return u;
    },
  )
  // C4: users.get with session policy + param validation
  .get(
    "/users/:id",
    {
      params: t.Object({
        id: t.String({ pattern: "^usr_[0-9]+$" }),
      }),
      beforeHandle({ headers, set }) {
        if (headers["authorization"] !== "Bearer q-demo-token") {
          set.status = 401;
          return {
            type: "https://velqu.dev/problems/unauthorized",
            title: "Unauthorized",
            status: 401,
          };
        }
      },
    },
    ({ params: { id }, set }) => {
      const u = usersService().get(id);
      if (!u) {
        set.status = 404;
        return {
          type: "https://velqu.dev/problems/not-found",
          title: "Not Found",
          status: 404,
        };
      }
      return u;
    },
  )
  // async: timer route
  .get(
    "/async",
    {
      query: t.Object({
        ms: t.Optional(t.Numeric({ minimum: 1, maximum: 1000, default: 10 })),
      }),
    },
    async ({ query: { ms } }) => {
      const waited = ms ?? 10;
      await Bun.sleep(waited);
      return { waited };
    },
  )
  // cancel route
  .get("/cancel", async ({ query }) => {
    const ms = parseInt((query as Record<string, string>).ms ?? "1000", 10);
    await Bun.sleep(ms);
    return { cancelled: false, waited: ms };
  })
  // throw route
  .get("/throw", () => {
    throw new Error("secret-boom");
  });

// 405 Method Not Allowed helper for known paths
const knownGetPaths = ["/js-text", "/js-json", "/async", "/cancel", "/throw"];
for (const p of knownGetPaths) {
  app.post(p, ({ set }) => {
    set.status = 405;
    set.headers["allow"] = "GET, HEAD";
    return { type: "https://velqu.dev/problems/method", title: "Method Not Allowed", status: 405 };
  });
}
app.get("/users", ({ set }) => {
  set.status = 405;
  set.headers["allow"] = "POST";
  return { type: "https://velqu.dev/problems/method", title: "Method Not Allowed", status: 405 };
});

// route-count generated routes (if requested)
if (N_ROUTES > 0) {
  for (let i = 0; i < N_ROUTES; i++) {
    app.get(
      `/res${i}/item/:id`,
      {
        params: t.Object({
          id: t.Numeric({ minimum: 1, maximum: N_ROUTES }),
        }),
      },
      ({ params: { id } }) => ({ id: Number(id), n: N_ROUTES }),
    );
  }
}

app.listen({ port: PORT, hostname: "127.0.0.1" });
console.log(`elysia2 ready port=${PORT} routes=${N_ROUTES > 0 ? N_ROUTES : "fixture"}`);
