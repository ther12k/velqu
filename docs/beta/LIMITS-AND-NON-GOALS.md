# Limits and non-goals

Velqu's private alpha is intentionally narrow. This page is the boundary
reference for what the current beta path does **not** promise. A feature being
interesting or technically possible is not evidence that it belongs in the
release scope.

## Runtime and platform limits

- Production execution is Rust plus QuickJS-NG loading a verified QPack.
  Bun is development/package/test tooling, not the production engine.
- Same-process QuickJS executes **trusted application code only**. It is not a
  hostile-code or multi-tenant sandbox.
- The documented working platform is Linux x86_64 glibc; Linux ARM64 glibc is
  included when the build environment is available. Windows and macOS have no
  beta support promise.
- Runtime queues, request bodies, jobs, heap, stack, deadlines, fetch work,
  and deferred callbacks are bounded. A limit rejection is a designed
  fail-closed result, not an invitation to retry without a policy.
- **No dynamic code execution** (BETA-007-E): every production runtime
  context replaces the `eval` global (direct and indirect forms both
  resolve it) and the dynamic `Function`-construction routes (the
  `Function` global plus the function/async/generator prototype
  constructor routes) with a typed `TypeError`
  (`velqu: dynamic code execution is disabled (...)`) before any
  application code runs. Static definitions — classes, closures,
  generators — are unaffected; the compiler emits statically compiled
  bundles, so legitimate applications have no need for dynamic
  evaluation. This is hardening for trusted application code, not a
  sandbox boundary.

## Explicit non-goals

The beta does not include or promise:

- full Node.js or Bun compatibility, CommonJS, or native Node addons;
- Express/Elysia compatibility adapters or an ORM in core;
- WebSocket, SSE, server-side rendering, or a frontend framework;
- automatic cloud provisioning, managed queues, cron, object storage, or
  pub/sub;
- direct public runtime TLS/HTTP2 termination (TLS is reverse-proxy first);
- universal identity/authentication platform behavior;
- universal benchmark superiority, an SLA, GA stability, or a production-ready
  claim.

These are scope decisions, not hidden partial implementations. See the
[scope matrix](02_SCOPE_MATRIX.md) and [post-beta backlog](governance/POST_BETA_BACKLOG.md)
for the canonical included/deferred lists.

## API and contract limits

- A single schema contract drives types, runtime validation, Treaty, OpenAPI,
  and the lock; hand-written duplicate route contracts are not a supported
  escape hatch.
- Handlers returning undeclared HTTP statuses trigger a contract violation
  and are converted by the runtime to 500 internal errors to protect client
  type guarantees; all expected responses (including 400, 401, 404, etc.)
  must be explicitly declared in the route contract.
- Undeclared statuses, malformed service profiles, incompatible packs, and
  unsupported capabilities fail clearly rather than silently falling back.
- `defer` is bounded in-memory best-effort work, never a durable job system;
  use an external durable system for work that must survive process exit.
- Fetch is an explicit capability subject to runtime/deployment egress policy.
  The generated upstream route is an educational fixture, not an availability
  or SSRF-safety guarantee for arbitrary destinations. By default, private and
  loopback IP ranges are denied to prevent SSRF vulnerabilities.
- In-memory services in the proof app are learning fixtures, not durable
  persistence.

## Evidence and wording limits

Documentation examples are not production validation. Every code sample in
this documentation should correspond to a proof-app, conformance, or scaffold
test. Performance statements require matched retained raw samples and p50,
p95, and p99 values; a configuration page must not turn a target into a
measured result.

Use these words carefully:

| Say | Do not say |
| --- | --- |
| “private-alpha runtime” | “production-ready” |
| “reverse-proxy-first posture” | “native TLS support” |
| “bounded best-effort defer” | “durable background jobs” |
| “measured fixture result” | “universally faster” |
| “trusted application code” | “hostile-code sandbox” |

## Verify the boundary

From the repository root:

```bash
bun install --frozen-lockfile
bun test
bun run typecheck
bun packages/cli/src/index.ts build --project examples/proof
bun run verify
```

The current beta remains suitable for evaluation, staging, internal tools, and
non-critical services. It carries no SLA and may change between prerelease
versions.
