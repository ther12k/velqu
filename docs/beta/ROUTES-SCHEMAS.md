# Routes, schemas, policies, and services

This guide extends the [quickstart](QUICKSTART.md) with the four building
blocks used by the proof application. The examples below are source-backed by
`examples/proof/`; they are intentionally small and do not claim production
readiness.

## Route and schema contract

A route declares its method, path, input schemas, response statuses, and
handler in one object:

```ts
import { route, status } from "@velqu/core";
import { s } from "@velqu/schema";

export const create = route({
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
  handle: async ({ body }) => status(201).value({
    id: "usr_2",
    name: body.name,
    email: body.email,
  }),
});
```

The same schema drives TypeScript inference, runtime validation, Treaty
client types, OpenAPI, and the contract lock. Statuses not declared on the
route are not silently accepted.

The proof version of this route uses the same shape in
`examples/proof/src/modules/users/routes.ts` and delegates persistence to a
service rather than embedding it in the handler.

## Parameters and responses

Path parameters are schemas too. A route can restrict an identifier before the
handler runs and declare a typed not-found problem:

```ts
export const get = route({
  id: "users.get",
  method: "GET",
  path: "/users/:id",
  params: s.object({ id: s.string({ pattern: "^usr_[0-9]+$" }) }),
  response: {
    200: s.object({ id: s.string(), name: s.string(), email: s.string() }),
    404: "not-found",
  },
  handle: async ({ params }) => {
    const user = lookup(params.id);
    if (!user) return status(404).problem("not-found", { detail: "user not found" });
    return user;
  },
});
```

Keep response schemas bounded: object fields, arrays, strings, and numbers
should have explicit limits where untrusted input can reach them. Validation
happens in the host/runtime pipeline; the compiler does not execute handlers
or service factories to discover routes.

## Policies

Policies run before a route and can declare statuses and typed context:

```ts
import { definePolicy, status } from "@velqu/core";

export const sessionPolicy = definePolicy({
  id: "auth.session",
  header: "authorization",
  declares: { 401: "unauthorized" },
  provides: "session",
  check: async (req) => {
    if (req.headers.authorization !== "Bearer q-demo-token") {
      return status(401).problem("unauthorized");
    }
    return { session: { userId: "usr_1" } };
  },
});
```

This fixture token is only for the proof app's local conformance tests. Do not
copy it into a deployed service or treat it as authentication guidance. The
proof policy is at `examples/proof/src/policy/session.ts`.

Attach a policy with `policy: sessionPolicy` on a route. The route then has a
stable declared 401 failure and receives the policy-provided context according
to the runtime contract.

## Lazy services

Services are factories resolved at runtime, not compiler-time dependency
injection. Keep mutable state behind the service boundary:

```ts
import { defineService } from "@velqu/core";

export const usersService = defineService("users.service", () => {
  const users = new Map<string, User>();
  return {
    get: (id: string) => users.get(id),
  };
});

let instance: ReturnType<typeof usersService.factory> | null = null;
export function resolve() {
  return (instance ??= usersService.factory());
}
```

`resolve()` memoizes the service for the process. The factory is not run while
routes are compiled or inspected; it runs on first use in the application
worker. For durable state, use an explicitly supported external capability —
the in-memory example is only a learning fixture.

## Verify the examples

From the repository root:

```bash
bun install --frozen-lockfile
bun test examples/proof
bun run typecheck
bun packages/cli/src/index.ts build --project examples/proof
```

These commands test the proof application's routes, policy, service, and
health module. Generated artifacts are evidence of the current private-alpha
toolchain, not a production-readiness claim.
