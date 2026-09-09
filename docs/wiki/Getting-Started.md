# Getting Started

Velqu is not yet published to npm — build from source with the pinned
toolchain (Bun 1.4.0, TypeScript 5.9.3, stable Rust; production execution
is the Rust binary, Bun is dev/test tooling only).

```bash
git clone https://github.com/ther12k/velqu && cd velqu
bun install
cargo build --release -p velqu-runtime
bun run verify        # the one verification command over the authorized scope
```

## 30-second example

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

```bash
bun packages/cli/src/index.ts build --project my-app
./target/release/velqu-runtime --pack my-app/dist/app.qpack --port 8080
curl -s http://127.0.0.1:8080/hello/world     # {"message":"Hello world"}
```

Authoring rules that bite:

- Every route binding must be **exported** from its source module
  (`export const hello = route({...})`) — the compiler fails the build
  otherwise (#1292).
- `/health/ready` is reserved for the runtime readiness probe.
- Route metadata must be statically evaluable — the compiler never
  executes application code.

## Run the examples

| Example | Command | Target |
| --- | --- | --- |
| `examples/proof` | `bun packages/cli/src/index.ts build --project examples/proof` | Native |
| `examples/browser-demo` | `bun packages/cli/src/index.ts build --target browser-wasm --project examples/browser-demo` | Browser-WASM |

## Deploy (native, shared mode)

Two artifacts from a trusted build: `velqu-runtime` + `app.qpack`. A pack
only runs on the exact runtime build it was compiled against (SEC-001
engine match); upgrading the runtime means rebuilding both. Limits and
update policy: [INSTALL.md](https://github.com/ther12k/velqu/blob/master/docs/beta/INSTALL.md).

## Deploy (browser)

Static assets, no Velqu application server — see [[Browser-WASM]].
