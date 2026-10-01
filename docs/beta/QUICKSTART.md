# Velqu Quickstart (public beta)

The shortest supported path from checkout to a running typed service.
Every command and response below was executed end-to-end against this
repository; shown bodies are the actual ones.

Production execution is the Rust runtime loading a compiled QPack; Bun
is used for development, package, and test tooling only.

> **Beta notice:** the supported beta target is Linux x86_64 glibc
> (macOS works for development only). The `@velqu/*` packages are
> published to npm as `0.1.0-beta.1` under the `beta` dist-tag (OD-010),
> so scaffolded projects install from the registry; the Rust runtime
> binary is built from a Velqu checkout and is required only for
> `velqu dev` and serving. The walkthrough below runs from a checkout
> because that is also how you build the runtime.

## Prerequisites

- Linux x86_64 (beta target)
- Bun `1.4.0` (build/dev tooling — exact pin; the build refuses other
  versions by design)
- Rust toolchain from the repository lockfile (runtime binary only)
- A checkout of this repository

From the repository root:

```bash
bun install --frozen-lockfile
cargo build --release -p velqu-runtime
```

## Create a starter project

```bash
bun packages/cli/src/index.ts create hello-velqu --name hello-velqu
```

The scaffold contains a health route, a greetings module with a service
and a test, a Treaty client example, and tooling config. The default
`serverless` profile runs one worker and is the least surprising
development profile; a multi-worker service uses the explicit grammar
`service:N` (`N` from 1 through 64; bare `service` is invalid):

```bash
bun packages/cli/src/index.ts create hello-svc --name hello-svc --profile service:4
```

The scaffold pins `@velqu/*@0.1.0-beta.1` and declares `@velqu/cli` in
devDependencies, with scripts that call its `velqu` bin — so inside the
generated project, `bun install` resolves everything from npm and no
monorepo linking is required:

```bash
cd hello-velqu
bun install
bun run check        # → velqu check: 3 routes in . — clean
```

## Build a production QPack

```bash
bun packages/cli/src/index.ts build --project hello-velqu
```

emits `hello-velqu/dist/app.qpack` (deterministic, byte-identical for
identical source and toolchain) plus the route, schema, capability,
contract, OpenAPI, lock, and build-report artifacts. Inspect the
compiled route plan without executing handlers:

```bash
bun packages/cli/src/index.ts inspect --project hello-velqu --json
```

## Run and call it

```bash
./target/release/velqu-runtime --pack hello-velqu/dist/app.qpack --port 8080
```

in another shell:

```bash
curl -sf http://127.0.0.1:8080/health/live
# → {"status":"ok"}
curl -sf http://127.0.0.1:8080/greetings/world
# → {"message":"Hello, world!"}
```

The runtime binds `127.0.0.1` by default (reverse-proxy posture;
`docs/beta/INSTALL.md` covers direct and container modes) and fails
closed before ready on any config, pack, or engine mismatch.

## Develop, check, and test

From the repository root, the development reload loop serves the
scaffold while watching for changes (production startup never performs
route/schema compilation or TypeScript transpilation — the dev loop is
the only place compilation happens per change):

```bash
bun packages/cli/src/index.ts dev --project hello-velqu --port 8084
```

```
curl -sf http://127.0.0.1:8084/health/live
# → {"status":"ok"}
curl -sf http://127.0.0.1:8084/greetings/dev
# → {"message":"Hello, dev!"}
```

Inside the scaffold, `bun run check`, `bun run test`, and
`bun run build` map to the same CLI commands.

## What this quickstart does not promise

- This is not a production-readiness claim and carries no SLA. The
  beta release line is `0.1.0-beta.1`; see
  [the beta definition](01_BETA_DEFINITION.md).
- `app.qpack` embeds QuickJS bytecode: it improves startup and enables
  strict verification, but it is not native-machine-code JIT
  compilation.
- Same-process QuickJS executes trusted application code only; it is
  not a hostile-code sandbox.
- `defer` is bounded, in-memory best-effort work, not a durable job
  queue.
- Performance numbers are not implied by this walkthrough. Claims
  require matched raw samples and p50/p95/p99 evidence under
  `benchmarks/raw/`.

## Next steps

- Read the [scope matrix](02_SCOPE_MATRIX.md) for supported and
  deferred capabilities.
- Installation modes (shared/standalone/container):
  [INSTALL.md](INSTALL.md).
- Read the generated project's `README.md` for its route and Treaty
  examples.
- Run the repository gate before sharing changes:

```bash
bun run verify
```
