/**
 * lugas baseline: LugasJS on Bun (pinned lugas@0.1.0-beta.5).
 * Implements the frozen fixture contract (benchmarks/fixtures/fixture-contract.json).
 *
 * Timed routes (C0-C3) use lugas's declared-schema fast path. POST /users
 * parses the body manually because the fixture requires malformed JSON to be
 * 422 while lugas's built-in body pipeline answers 400 (MALFORMED_JSON).
 */

import { defineApp, route, json, text, empty } from "lugas";
import { z } from "zod";

const PORT = parseInt(process.env.PORT ?? "3000", 10);

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

const userSchema = z.object({
  name: z.string().min(1).max(60),
  email: z.string().email(),
});

const internalProblem = () =>
  json(500, {
    type: "https://velqu.dev/problems/internal",
    title: "Internal Server Error",
    status: 500,
  });
const unauthorizedProblem = () =>
  json(401, {
    type: "https://velqu.dev/problems/unauthorized",
    title: "Unauthorized",
    status: 401,
  });
const notFoundProblem = () =>
  json(404, {
    type: "https://velqu.dev/problems/not-found",
    title: "Not Found",
    status: 404,
  });
const validationProblem = (field: string, code: string) =>
  json(422, {
    type: "https://velqu.dev/problems/validation",
    title: "Validation failed",
    status: 422,
    errors: [{ path: field, code }],
  });
const methodNotAllowed = (allow: string) =>
  json(
    405,
    { type: "https://velqu.dev/problems/method", title: "Method Not Allowed", status: 405 },
    { headers: { allow } },
  );

const app = defineApp({
  // every handler throw lands here: redacted, no message, no stack (SEC-004)
  onError: () => internalProblem(),
  routes: {
    "/health/live": {
      GET: route({ handler: () => json(200, { status: "ok" }) }),
      HEAD: route({ handler: () => empty(200) }),
      POST: route({ handler: () => methodNotAllowed("GET, HEAD") }),
    },
    "/js-text": {
      GET: route({ handler: () => text(200, "plain") }),
      POST: route({ handler: () => methodNotAllowed("GET, HEAD") }),
    },
    "/js-json": {
      GET: route({ handler: () => json(200, { ok: true }) }),
      POST: route({ handler: () => methodNotAllowed("GET, HEAD") }),
    },
    "/hello/:name": {
      GET: route({
        params: z.object({ name: z.string().min(1).max(60) }),
        handler: (ctx) => json(200, { message: `Hello ${ctx.params.name}` }),
      }),
      POST: route({ handler: () => methodNotAllowed("GET, HEAD") }),
    },
    "/users": {
      POST: route({
        handler: async (ctx) => {
          let raw: unknown;
          try {
            raw = await ctx.request.json();
          } catch {
            return validationProblem("body", "malformedJSON");
          }
          const parsed = userSchema.safeParse(raw);
          if (!parsed.success) {
            const issue = parsed.error.issues[0];
            return validationProblem(String(issue?.path?.[0] ?? "body"), issue?.code ?? "invalid");
          }
          const id = `usr_${nextUser++}`;
          const u = { id, name: parsed.data.name, email: parsed.data.email };
          usersService().set(id, u);
          return json(201, u);
        },
      }),
      GET: route({ handler: () => methodNotAllowed("POST") }),
    },
    "/users/:id": {
      GET: route({
        params: z.object({ id: z.string().regex(/^usr_[0-9]+$/) }),
        handler: (ctx) => {
          if (ctx.request.headers.get("authorization") !== "Bearer q-demo-token") {
            return unauthorizedProblem();
          }
          const u = usersService().get(ctx.params.id);
          if (!u) return notFoundProblem();
          return json(200, u);
        },
      }),
    },
    "/async": {
      GET: route({
        query: z.object({ ms: z.coerce.number().int().min(1).max(1000).default(10) }),
        handler: async (ctx) => {
          await Bun.sleep(ctx.query.ms);
          return json(200, { waited: ctx.query.ms });
        },
      }),
      POST: route({ handler: () => methodNotAllowed("GET, HEAD") }),
    },
    "/cancel": {
      GET: route({
        query: z.object({ ms: z.coerce.number().int().min(1).max(5000).default(1000) }),
        handler: async (ctx) => {
          await Bun.sleep(ctx.query.ms);
          return json(200, { cancelled: false, waited: ctx.query.ms });
        },
      }),
      POST: route({ handler: () => methodNotAllowed("GET, HEAD") }),
    },
    "/throw": {
      GET: route({
        handler: () => {
          throw new Error("secret-boom");
        },
      }),
      POST: route({ handler: () => methodNotAllowed("GET, HEAD") }),
    },
  },
});

app.serve({ port: PORT, hostname: "127.0.0.1" });
console.log(`lugas ready port=${PORT}`);
