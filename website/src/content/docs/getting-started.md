---
title: Getting started
description: Install Velqu from npm, declare your first route, and run it on the Rust runtime.
---

## Install

Velqu is published under the **`beta`** dist-tag — `latest` is intentionally
unset while the project is pre-GA.

```bash
bun add @velqu/core@beta @velqu/schema@beta
bun add -g @velqu/cli@beta          # the `velqu` CLI (requires Bun 1.4+)
```

Production execution is the Rust `velqu-runtime` binary (Linux x86_64 in
beta); Bun is the authoring and build toolchain.

## 30-second example

Declare a route — one schema drives validation, types, and the client:

```ts
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
```

Compile it to a QPack and run it on the Rust host:

```bash
velqu build --project my-app
cargo build --release -p velqu-runtime   # or use a released runtime binary
./target/release/velqu-runtime --pack my-app/dist/app.qpack --port 8080
```

```text
$ curl -s http://127.0.0.1:8080/hello/world
{"message":"Hello world"}

$ curl -s http://127.0.0.1:8080/no-such-route
{"type":".../problems/not-found","status":404,...}
```

Expected failures are typed RFC 9457 problems at declared statuses — 404/405
are answered by the host router and never reach a handler; validation
failures are typed 422s. The compiler never executes your application code:
route discovery is static, so `velqu build` cannot run your handlers.

## Authoring rules

- Route bindings must be **exported** from their source module
  (`export const hello = route({...})`) — the compiler fails the build
  otherwise.
- `/health/ready` is reserved for the runtime readiness probe.
- Route metadata must be statically evaluable.

## Typed client (Treaty)

`velqu build` emits `contract.json` / `contract.d.ts`; clients import only
the contract — never server source:

```ts
import { treaty, type TreatyClient } from "@velqu/treaty";
import type { Api } from "./contract";     // generated types
import contract from "./contract.json";    // generated runtime table

export const api: TreatyClient<Api> =
  treaty<Api>({ baseUrl: "http://127.0.0.1:8080", contract });

const res = await api.hello({ name: "world" }).get();
if (res.data) console.log(res.data.message); // { message: string } at 200
```

Dot-navigation is typed down to the declared method — calling `.post()` on a
GET route is a compile-time type error and a named runtime error; path
parameters are required and URI-encoded by the client.

## Next steps

- Deploy the two-artifact way: [Deployment](/velqu/deployment/)
- Run the same app in a browser: [Browser-WASM](/velqu/browser-wasm/)
- The honest boundaries: [Known limitations](/velqu/known-limitations/)
