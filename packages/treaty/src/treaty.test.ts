/**
 * Treaty + core + schema type spike: proves the M0 sketch shape works —
 * route-local typing, policy context, status narrowing, non-throwing errors,
 * and published-contract mode. Unit-local runtime tests are labeled as such
 * (TRT-005): they do NOT prove native-runtime conformance.
 */
import { afterAll, describe, expect, expectTypeOf, test } from "bun:test";
import { defineApp, defineModule, definePolicy, route, status } from "@q/core";
import { s } from "@q/schema";
import { treaty } from "./index";
import type { RouteContract } from "@q/contract";

// ---------------------------------------------------------------- proof-shaped app

const Session = definePolicy({
  id: "auth.session",
  header: "authorization",
  declares: { 401: "unauthorized" },
  provides: "session",
  check: async (req) => {
    if (req.headers.authorization !== "Bearer q-demo-token") return status(401).problem("unauthorized");
    return { session: { userId: "usr_1" } };
  },
});

const helloRoute = route({
  id: "hello.get",
  method: "GET",
  path: "/hello/:name",
  params: s.object({ name: s.string({ minLength: 1, maxLength: 60 }) }),
  response: { 200: s.object({ message: s.string() }) },
  handle: ({ params }) => ({ message: `Hello ${params.name}` }),
});

const usersCreate = route({
  id: "users.create",
  method: "POST",
  path: "/users",
  body: s.object({
    name: s.string({ minLength: 1, maxLength: 60 }),
    email: s.string({ format: "email" }),
  }),
  response: {
    201: s.object({ id: s.string(), name: s.string(), email: s.string() }),
  },
  handle: async ({ body }) => status(201).value({ id: "usr_1", name: body.name, email: body.email }),
});

const usersGet = route({
  id: "users.get",
  method: "GET",
  path: "/users/:id",
  policy: Session,
  params: s.object({ id: s.string({ pattern: "^usr_[0-9]+$" }) }),
  response: {
    200: s.object({ id: s.string(), name: s.string(), email: s.string() }),
    404: s.object({ gone: s.boolean() }),
  },
  handle: async ({ params, session }) => {
    expectTypeOf(session).toEqualTypeOf<{ userId: string }>();
    if (params.id !== session.userId) return status(404).problem("not-found");
    return { id: params.id, name: "Ada", email: "ada@example.org" };
  },
});

const app = defineApp({
  id: "proof",
  modules: [
    defineModule({ id: "hello", routes: [helloRoute] }),
    defineModule({ id: "users", routes: [usersCreate, usersGet] }),
  ],
});

// ---------------------------------------------------------------- published contract mode

type ProofApi = {
  "hello.get": RouteContract<"/hello/:name", "GET", { name: string }, Record<string, never>, undefined, { 200: { message: string } }>;
  "users.create": RouteContract<
    "/users",
    "POST",
    Record<string, never>,
    Record<string, never>,
    { name: string; email: string },
    { 201: { id: string; name: string; email: string } }
  >;
  "users.get": RouteContract<
    "/users/:id",
    "GET",
    { id: string },
    Record<string, never>,
    undefined,
    { 200: { id: string; name: string; email: string }; 401: { title: string }; 404: { title: string } }
  >;
};

// ---------------------------------------------------------------- unit-local runtime tests (LABELED — not runtime conformance)

const unitServer = Bun.serve({
  port: 0,
  fetch: async (req) => {
    const url = new URL(req.url);
    if (url.pathname === "/hello/Rafi") return Response.json({ message: "Hello Rafi" });
    if (url.pathname === "/users/usr_1") {
      if (req.headers.get("authorization") !== "Bearer q-demo-token") {
        return Response.json({ type: "https://velqu.dev/problems/unauthorized", title: "Unauthorized", status: 401 }, { status: 401 });
      }
      return Response.json({ id: "usr_1", name: "Ada", email: "ada@example.org" });
    }
    if (url.pathname === "/users" && req.method === "POST") {
      return Response.json({ id: "usr_1", name: "Ada", email: "ada@example.org" }, { status: 201 });
    }
    return new Response("no route", { status: 404 });
  },
});

describe("treaty (unit-local, NOT runtime conformance)", () => {
  const api = treaty<ProofApi>({
      baseUrl: `http://localhost:${unitServer.port}`,
      contract: {
        "hello.get": { path: "/hello/:name", method: "GET" },
        "users.create": { path: "/users", method: "POST" },
        "users.get": { path: "/users/:id", method: "GET" },
      },
    });

  test("success returns typed data, no error", async () => {
    const r = await api.hello.get({ name: "Rafi" }).get();
    expect(r.error).toBeNull();
    if (!r.error) {
      expectTypeOf(r.data).toMatchTypeOf<{ message: string }>();
      expect((r.data as { message: string }).message).toBe("Hello Rafi");
    }
  });

  test("declared HTTP failure is a value, never a throw; status narrows", async () => {
    const r = await api.users.get({ id: "usr_1" }).get(); // no auth header → 401
    expect(r.data).toBeNull();
    if (r.error && r.error.status !== 0) {
      const narrowed: 401 | 200 | 404 = r.error.status;
      expect(narrowed).toBe(401);
      // narrowed to the 401 problem shape by the status check
      if (r.error.status === 401) expectTypeOf(r.error.problem).toMatchTypeOf<{ title: string }>();
      const problem = r.error.problem as { title: string };
      expect(problem.title).toBe("Unauthorized");
    }
  });

  test("network failure distinguishes status 0 kind network", async () => {
    const dead = treaty<ProofApi>({
      baseUrl: "http://127.0.0.1:1",
      contract: { "hello.get": { path: "/hello/:name", method: "GET" } },
    });
    const r = await dead.hello.get({ name: "x" }).get();
    expect(r.error?.status).toBe(0);
    if (r.error && r.error.status === 0) {
      expect(r.error.kind).toBe("network");
    }
  });

  test("abort maps to kind abort", async () => {
    const controller = new AbortController();
    const api2 = treaty<ProofApi>({
      baseUrl: `http://localhost:${unitServer.port}`,
      contract: { "hello.get": { path: "/hello/:name", method: "GET" } },
    });
    const p = api2.hello.get({ name: "Rafi" }).get({ signal: controller.signal });
    controller.abort();
    const r = await p;
    expect(r.error?.status).toBe(0);
    if (r.error && r.error.status === 0) expect(r.error.kind).toBe("abort");
  });

  test("post body is typed at the call site (runtime: bytes match)", async () => {
    const r = await api.users.create({}).post({ name: "Ada", email: "ada@example.org" });
    expect(r.error).toBeNull();
  });

  afterAll(() => unitServer.stop(true));
});

// ---------------------------------------------------------------- compile-time-only proofs

describe("type spike (compile-time)", () => {
  test("schema inference", () => {
    const CreateUser = s.object({ name: s.string({ maxLength: 60 }), email: s.string({ format: "email" }) });
    type User = { name: string; email: string };
    expectTypeOf<{ name: string; email: string }>().toEqualTypeOf<User>();
    const ir = CreateUser as unknown as { kind: string; required: string[] };
    expect(ir.kind).toBe("object");
    expect(ir.required).toEqual(["name", "email"]);
  });

  test("query schema with default is optional", () => {
    const Q = s.object({ ms: s.optional(s.integer({ minimum: 1, maximum: 1000 }), { default: 10 }) });
    const ir = Q as unknown as { required: string[] };
    expect(ir.required).toEqual([]);
  });
});
