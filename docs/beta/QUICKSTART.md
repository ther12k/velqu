# Velqu Quickstart (private alpha)

This is the shortest supported learning path for the current Velqu private
alpha. Production execution is the Rust runtime loading a compiled QPack;
Bun is used for development, package, and test tooling only.

> **Private-alpha notice:** the `@velqu/*` packages currently resolve through
> the monorepo workspace and are not published to npm. Run the commands below
> from a Velqu checkout. A standalone generated project needs local workspace
> package links until the public beta packaging work is complete.

## Prerequisites

- Linux or macOS
- Bun `1.4.0`
- Rust toolchain supported by the repository lockfile
- A checkout of this repository

From the repository root:

```bash
bun install --frozen-lockfile
```

## Create a starter project

Generate a small app in a new directory. The default `serverless` profile is
one worker and is the least surprising development profile.

```bash
bun packages/cli/src/index.ts init /tmp/velqu-hello --name velqu-hello
cd /tmp/velqu-hello
```

The scaffold contains a health route, a greetings module, a Treaty client
example, and tests. It contains no credentials and does not provision a
 database or external service.

For a multi-worker service profile, use the explicit runtime grammar
`service:N` (`N` from 1 through 64):

```bash
bun /path/to/velqu/packages/cli/src/index.ts init . \
  --name velqu-service --profile service:4
```

Bare `service` is invalid; the worker count is required.

## Develop, check, and test

Run these from the generated project directory:

```bash
bun install
bun run check
bun run test
bun run dev
```

`bun run dev` starts the development reload loop. It compiles the application
when a change is detected; production startup does not perform route/schema
compilation or TypeScript transpilation.

The scaffold uses `workspace:*` dependencies in this private alpha. If the
standalone directory cannot resolve them, run it inside the monorepo or create
local links under `node_modules/@velqu/` as described in its generated
`README.md`.

## Build a production QPack

Build from the project directory:

```bash
bun run build
```

The output directory contains `app.qpack` plus the route, schema, capability,
contract, OpenAPI, lock, and build-report artifacts. Inspect the compiled
route plan without executing handlers:

```bash
bun /path/to/velqu/packages/cli/src/index.ts inspect --project . --json
```

Run the Rust runtime against the produced pack from the repository checkout:

```bash
cargo run --release -p velqu-runtime -- --pack /tmp/velqu-hello/dist/app.qpack
```

The exact runtime flags may evolve during private-alpha development; use
`velqu --help` and the generated build report for the current artifact paths.

## What this quickstart does not promise

- This is not a production-readiness claim. The forward finish line is
  `0.1.0-beta.1`; see [the beta definition](01_BETA_DEFINITION.md).
- Same-process QuickJS executes trusted application code only; it is not a
  hostile-code sandbox.
- `defer` is bounded, in-memory best-effort work, not a durable job queue.
- Performance numbers are not implied by this walkthrough. Claims require
  matched raw samples and p50/p95/p99 evidence under `benchmarks/raw/`.

## Next steps

- Read the [scope matrix](02_SCOPE_MATRIX.md) for supported and deferred
  capabilities.
- Read the generated project's `README.md` for its route and Treaty examples.
- Run the repository gate before sharing changes:

```bash
bun run verify
```
