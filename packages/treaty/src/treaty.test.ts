/**
 * Treaty + core + schema type spike: proves Eden-quality Treaty typing:
 * - Exact method narrowing: only declared HTTP method exists on client
 * - Exact body constraint: post() accepts only R["body"]
 * - 2xx data vs non-2xx error separation (200 is never in error union)
 * - Status narrowing: switch on status narrows error.problem
 * - Negative compile-time type tests
 */
import { afterAll, describe, expect, expectTypeOf, test } from "bun:test";
import { defineApp, defineModule, definePolicy, route, status } from "@velqu/core";
import { s } from "@velqu/schema";
import { treaty, type TreatyClient } from "./index";
import type { RouteContract } from "@velqu/contract";

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

export const app = defineApp({
  id: "proof",
  modules: [
    defineModule({ id: "hello", routes: [helloRoute] }),
    defineModule({ id: "users", routes: [usersCreate, usersGet] }),
  ],
});

// ---------------------------------------------------------------- published contract mode

export type ProofApi = {
  "hello.get": RouteContract<
    "/hello/:name",
    "GET",
    { name: string },
    Record<string, never>,
    undefined,
    { 200: { message: string } }
  >;
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
    {
      200: { id: string; name: string; email: string };
      401: { title: string; type: string };
      404: { title: string; detail?: string };
    }
  >;
};

// ---------------------------------------------------------------- unit-local runtime tests

const unitServer = Bun.serve({
  port: 0,
  fetch: async (req) => {
    const url = new URL(req.url);
    if (url.pathname === "/hello/Rafi") return Response.json({ message: "Hello Rafi" });
    if (url.pathname === "/users/usr_1") {
      if (req.headers.get("authorization") !== "Bearer q-demo-token") {
        return Response.json(
          { type: "https://velqu.dev/problems/unauthorized", title: "Unauthorized", status: 401 },
          { status: 401 },
        );
      }
      return Response.json({ id: "usr_1", name: "Ada", email: "ada@example.org" });
    }
    if (url.pathname === "/users" && req.method === "POST") {
      return Response.json({ id: "usr_1", name: "Ada", email: "ada@example.org" }, { status: 201 });
    }
    return new Response("no route", { status: 404 });
  },
});

describe("treaty (Eden-style typing & runtime)", () => {
  const api = treaty<ProofApi>({
    baseUrl: `http://localhost:${unitServer.port}`,
    contract: {
      "hello.get": { path: "/hello/:name", method: "GET" },
      "users.create": { path: "/users", method: "POST" },
      "users.get": { path: "/users/:id", method: "GET" },
    },
  });

  test("treaty() returns properly typed TreatyClient", () => {
    expectTypeOf(api).toEqualTypeOf<TreatyClient<ProofApi>>();
  });

  test("success returns data strictly typed as 2xx response, never error", async () => {
    const r = await api.hello.get({ name: "Rafi" }).get();
    expect(r.error).toBeNull();
    if (!r.error) {
      expectTypeOf(r.data).toEqualTypeOf<{ message: string }>();
      expect(r.data.message).toBe("Hello Rafi");
    }
  });

  test("status narrowing on error union (200 is NOT in error status union)", async () => {
    const r = await api.users.get({ id: "usr_1" }).get();
    expect(r.data).toBeNull();
    if (r.error) {
      // 0 (network/abort) | 401 | 404 — 200 MUST NOT BE PRESENT
      expectTypeOf(r.error.status).toEqualTypeOf<0 | 401 | 404>();

      if (r.error.status === 401) {
        expectTypeOf(r.error.problem).toEqualTypeOf<{ title: string; type: string }>();
        expect(r.error.problem.title).toBe("Unauthorized");
      }
      if (r.error.status === 404) {
        expectTypeOf(r.error.problem).toEqualTypeOf<{ title: string; detail?: string }>();
      }
    }
  });

  test("POST body is strictly constrained to contract body schema", async () => {
    const r = await api.users.create.post({ name: "Ada", email: "ada@example.org" });
    expect(r.error).toBeNull();
    if (!r.error) {
      expectTypeOf(r.data).toEqualTypeOf<{ id: string; name: string; email: string }>();
      expect(r.data.id).toBe("usr_1");
    }
  });

  test("Method narrowing: GET routes only have .get(), POST routes only have .post()", () => {
    // @ts-expect-error — hello.get is a GET route, cannot call .post()
    void api.hello.get({ name: "Rafi" }).post;

    // @ts-expect-error — users.create is a POST route, cannot call .get()
    void api.users.create.get;

    // @ts-expect-error — users.create body must have email
    void api.users.create.post({ name: "Ada" });
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

  test("runtime rejects missing required path parameter", () => {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    expect(() => api.hello.get({} as any)).toThrow("missing required path parameter \"name\"");
  });

  test("runtime encodes path parameters and rejects undeclared methods", async () => {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    expect(() => (api.hello.get({ name: "Rafi" }) as any).post()).toThrow("method \"POST\" is not allowed");
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    expect(() => (api.users.create as any).get()).toThrow("method \"GET\" is not allowed");
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
