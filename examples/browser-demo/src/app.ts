/**
 * BWASM-B-001 sample project — builds a self-contained browser artifact
 * set via the compiler's browser-wasm target. Handlers USE the context
 * (no literal-returning native-liveness routes: those are native-only
 * surfaces per ADR-0037 §1).
 */
import { route } from "@velqu/core";
import { s } from "@velqu/schema";

export const hello = route({
  id: "hello.get",
  method: "GET",
  path: "/hello/:name",
  params: s.object({ name: s.string({ minLength: 1, maxLength: 60 }) }),
  response: { 200: s.object({ message: s.string() }) },
  handle: ({ params }) => ({ message: `Hello ${params.name}` }),
});

export const echo = route({
  id: "echo.post",
  method: "POST",
  path: "/echo",
  body: s.object({ text: s.string({ maxLength: 200 }) }),
  response: { 200: s.object({ echoed: s.string() }), 400: s.object({ missing: s.boolean() }) },
  handle: ({ body }) => ({ echoed: body.text }),
});

export const app = { routes: [hello, echo] };
