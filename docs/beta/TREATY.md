# Treaty client

Treaty is the typed client projection of the same route/schema contract used by
Velqu's compiler and runtime. The published client imports the generated
contract; it does not need server or compiler modules.

## Generate and use a client

Build the proof application first:

```bash
bun install --frozen-lockfile
bun packages/cli/src/index.ts build --project examples/proof
```

A consumer can use the generated contract with a transport:

```ts
import { treaty } from "@velqu/treaty";
import type { Api } from "./dist/contract";

const client = treaty<Api>(fetch, { baseUrl: "http://127.0.0.1:3000" });
const health = await client.api.health.live.get();
if (health.error) throw new Error(`health failed: ${health.error.status}`);
console.log(health.data.status);
```

Navigation follows route IDs. Path parameters are supplied before the HTTP
method; query, headers, and body options are checked against the route's
schema:

```ts
const result = await client.api.users.get({ id: "usr_1" }).get({
  headers: { authorization: "Bearer q-demo-token" },
});

const created = await client.api.users.create({}).post({
  name: "Ada",
  email: "ada@example.org",
});
```

The generated response union distinguishes success from declared failures.
For the proof route, a missing/invalid session is a typed 401 problem rather
than an untyped exception. Undeclared methods, missing required path values,
and invalid route options fail at compile time where TypeScript can prove them
and at runtime at the client boundary.

## Transport modes

The repository provides three deliberately distinct testing/client modes:

- **Unit-local:** direct in-process dispatch for fast deterministic route tests;
  it does not prove the HTTP runtime.
- **Runtime-local:** starts the actual Rust/QuickJS runtime and drives the
  compiled QPack over HTTP. This is the proof of runtime wiring.
- **Remote:** uses an injected `fetch` against an already deployed endpoint;
  callers own endpoint availability and network errors are represented as a
  status-0 network result.

Mode labels are exposed by the testing helpers (`__mode`) so tests cannot
silently confuse a unit shortcut with runtime evidence.

## Contract and client boundaries

The generated `contract.d.ts` is derived from the canonical build artifacts.
Treaty is portable: its transport requires only a Fetch-compatible function;
Bun-only extensions such as `preconnect` are not part of the client contract.
Use `treatyRoutes` when publishing a client for an explicit route allowlist so
unused routes can be tree-shaken.

## Verify the example

From the repository root:

```bash
bun test conformance/treaty examples/proof
bun run typecheck
bun packages/cli/src/index.ts build --project examples/proof
```

These tests cover generated type projections, path/query/body/status typing,
unit/runtime/remote parity, runtime-local actual binary execution, and client
bundle isolation. This is private-alpha evidence, not a production-readiness
claim; the generated proof credentials are fixtures only.
