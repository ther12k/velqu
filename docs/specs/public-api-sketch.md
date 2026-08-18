# Velqu Public API Sketch — frozen for M0/M2

Status: M0 type-spike target shape. The compiler (M2) must accept exactly these
forms statically. Packages are `@velqu/*` (owner-decided; ADR-0016). Historical `@q/*` references in earlier docs refer to the same packages.

## Authoring (`@velqu/core` + `@velqu/schema`)

```ts
import { defineApp, defineModule, route, definePolicy, defineService, status } from "@velqu/core";
import { s } from "@velqu/schema";

// schema subset mirrors Schema IR v1 (docs/specs/pack-format-v1.md)
const CreateUser = s.object({
  name: s.string({ minLength: 1, maxLength: 60 }),
  email: s.string({ format: "email" }),
});

const hello = route({
  id: "hello.get",
  method: "GET",
  path: "/hello/:name",
  params: s.object({ name: s.string({ minLength: 1, maxLength: 60 }) }),
  response: { 200: s.object({ message: s.string() }) },
  handle: ({ params }) => ({ message: `Hello ${params.name}` }),
});
// route() returns Route<Id, Input, Output> — a pure static descriptor.
// handle is NEVER called by the compiler.

const sessionPolicy = definePolicy({
  id: "auth.session",
  header: "authorization",
  declares: { 401: "unauthorized" },   // statuses this policy can produce
  provides: "session",
  check: async (req) => {
    const token = req.headers["authorization"];
    if (token !== "Bearer q-demo-token") return status(401).problem("unauthorized");
    return { session: { userId: "usr_1" } };
  },
});

const userService = defineService("users.service", () => {
  // lazy factory: runs on first resolve() at runtime, never during compile
  return { async get(id: string) { /* ... */ } };
});

export default defineModule({ id: "hello", routes: [hello] });

export default defineApp({ id: "proof", modules: [helloModule, usersModule] });
// App type (type-level only) drives source-mode Treaty:
// type Api = typeof app (contract type: paths, inputs, status-narrowed outputs)
```

## Typed results and problems

```ts
status(200).value(data);            // default success
status(201).value(user);            // alternate success
status(404).problem("not-found", { detail: "user not found" });
status(422).problem("validation", { errors: [{ path: "name", code: "maxLength", message: "..." }] });
```

Handler return type = union over declared `response` keys; undeclared status is
a compile error and a controlled runtime contract failure.

## Treaty (`@velqu/treaty`)

```ts
import { treaty } from "@velqu/treaty";
import type { Api } from "@velqu/contract-source";      // source mode
// or: import type { Api } from "./dist/contract";  // published mode (generated .d.ts)

const api = treaty<Api>({ baseUrl: "http://localhost:3000" });

const r = await api.users({ id: "usr_1" }).get({}, { headers: { authorization: "Bearer q-demo-token" } });
if (r.error) {
  if (r.error.status === 401) { /* r.error.problem typed as unauthorized */ }
} else {
  r.data; // typed 200 body
}
// r.error?.status is 0 for network failure, "abort" for AbortError — never throws for HTTP.
```

- Path segments from route paths: `api.hello({ name }) === GET /hello/:name`.
- Method suffix: `.get(paramsAndOptions?)`, `.post(body, options?)`, ...
- Query/headers passed in first argument alongside path params.
- Result: `{ data, error: null } | { data: null, error: { status, problem } | { status: 0, kind: "network" | "abort" } }`.

## Local test adapters (`@velqu/testing`)

```ts
import { unitTreaty } from "@velqu/testing";     // executes handlers in-process (Bun) — labeled unit-local
import { runtimeTreaty } from "@velqu/testing";  // spawns the actual q-runtime binary — labeled runtime-local
```

TRT-005: unit-local results are never reported as native-runtime conformance.

## Non-goals respected

No decorators, no classes, no middleware chains, no plugin registration side
effects. Everything above is data the compiler can read without executing.
