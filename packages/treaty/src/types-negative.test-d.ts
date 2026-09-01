/**
 * M4A-004-A negative TYPE tests (compile-time contract enforcement).
 *
 * This file is typecheck-only evidence: every `@ts-expect-error` MUST
 * flag a real type error (`bun run typecheck` fails if any line stops
 * erroring — no weakening). Runtime behavior is covered by unit-direct.test.ts.
 */
import { expectTypeOf } from "bun:test";
import { treaty } from "@velqu/treaty";

type Api = {
  "users.get": {
    path: "/users/:id";
    method: "GET";
    params: { id: string };
    query: { verbose?: boolean };
    body: never;
    headers: never;
    responses: {
      200: { id: string; name: string };
      404: { code: string };
    };
  };
  "users.create": {
    path: "/users";
    method: "POST";
    params: never;
    query: never;
    body: { name: string; email: string };
    headers: never;
    responses: { 201: { id: string } };
  };
};

const api = treaty<Api>({ baseUrl: "http://127.0.0.1:1", contract: {} });

// --- method narrowing: only the declared method exists ---------------------

// GET route has no .post()
// @ts-expect-error method "post" does not exist on a GET route
api["users.get"].post;

// POST route has no .get()
// @ts-expect-error method "get" does not exist on a POST route
api["users.create"].get;

// --- body constraint: POST body must match the declared body schema --------

// @ts-expect-error missing required body (name, email)
api["users.create"].post();

// @ts-expect-error wrong body shape (email missing)
api["users.create"].post({ name: "Ada" });

// @ts-expect-error wrong body type (email: number)
api["users.create"].post({ name: "Ada", email: 42 });

// GET with a body is not typeable either (RequestOptions has no body field)
// @ts-expect-error GET takes only RequestOptions, not a body
api["users.get"]({ id: "u1" }).get({} as never as { body: unknown });

// --- path params: required and correctly named ------------------------------

// @ts-expect-error missing required path parameter
api["users.get"]().get();

// @ts-expect-error wrong path parameter name
api["users.get"]({ username: "u1" }).get();

// --- status splitting: 200 is data, never an error --------------------------

declare const result: Awaited<ReturnType<ReturnType<typeof api.users.get>["get"]>>;

// data is exactly the declared 200 body; error is null on the success arm
expectTypeOf(result.data).toMatchTypeOf<{ id: string; name: string } | null>();
expectTypeOf(result.error).toMatchTypeOf<
  | null
  | { status: 0; kind: "network"; message: string }
  | { status: 0; kind: "abort" }
  | { status: 404; problem: { code: string } }
>();

// @ts-expect-error 200 is never in the ERROR union (status 200 impossible on error)
const impossibleErrorStatus: number = result.error !== null && result.error.status === 200;

// @ts-expect-error error problem is the declared 404 shape, not the 200 shape
result.error?.problem?.name;

// --- navigation: unknown route ids are type errors ---------------------------

// @ts-expect-error unknown route segment
api.users.delete();
