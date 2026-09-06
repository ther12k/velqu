/**
 * BWASM-Q-001 — conformance fixture application.
 *
 * Routes pin compatibility-critical behaviors for the differential
 * suite. Handlers on the exact-parity paths consume NOTHING the browser
 * MVP worker context cannot provide (constants + body JSON); routes
 * that consume schema-validated params/query or ctx.native capabilities
 * are the NATIVE-ONLY classification set — the suite records their
 * native outcome and the classification instead of demanding parity
 * (support-matrix entries, never normalized away).
 */
import { route, status } from "@velqu/core";
import { s } from "@velqu/schema";

// exact-parity set: routing + validation + status + problem semantics
export const greet = route({
  id: "greet.get",
  method: "GET",
  path: "/greet/:name",
  query: s.object({ loud: s.optional(s.string()) }),
  response: { 200: s.object({ message: s.string() }) },
  handle: async (ctx) => {
    void ctx; // reference ctx: not statically evaluable (RUN-009 keeps this off native liveness)
    return { message: "Hello from conformance" };
  },
});

export const echo = route({
  id: "echo.post",
  method: "POST",
  path: "/echo",
  body: s.object({ text: s.string(), count: s.integer() }),
  response: { 200: s.object({ text: s.string(), count: s.integer() }), 422: s.object({}) },
  handle: async (ctx) => ({ text: ctx.body.text, count: ctx.body.count }),
});

export const maybe = route({
  id: "maybe.get",
  method: "GET",
  path: "/maybe/yes",
  response: { 200: s.object({ found: s.boolean() }) },
  handle: async (ctx) => {
    void ctx;
    return { found: true };
  },
});

export const absent = route({
  id: "absent.get",
  method: "GET",
  path: "/absent",
  response: { 200: s.object({}), 404: s.object({}) },
  handle: async (ctx) => {
    void ctx;
    return status(404).value({});
  },
});

// native-only set: consumes schema-validated params/query/capability
export const paramEcho = route({
  id: "param.echo",
  method: "GET",
  path: "/param/:name",
  params: s.object({ name: s.string() }),
  response: { 200: s.object({ name: s.string() }) },
  handle: async (ctx) => ({ name: ctx.params.name }),
});

export const queryEcho = route({
  id: "query.echo",
  method: "GET",
  path: "/query-echo",
  query: s.object({ loud: s.optional(s.string()) }),
  response: { 200: s.object({ loud: s.string() }) },
  handle: async (ctx) => ({ loud: ctx.query.loud ?? "unset" }),
});

export const timed = route({
  id: "timed.get",
  method: "GET",
  path: "/timed",
  response: { 200: s.object({ waited: s.boolean() }) },
  handle: async (ctx) => {
    await ctx.native.timer.delay(1);
    return { waited: true };
  },
});

export const app = { routes: [greet, echo, maybe, absent, paramEcho, queryEcho, timed] };
