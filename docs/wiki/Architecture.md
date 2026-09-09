# Architecture

## The one-paragraph version

A Rust host embeds quickjs-ng 0.15.1 (via rquickjs 0.12.2, pinned) and
executes TypeScript-authored handlers from a compiled, verified QPack.
The Rust host routes by method/path and enforces all bounds **before any
JavaScript runs**; the compiler statically extracts routes from source
and never executes application code.

## Non-negotiable invariants

1. Production execution is Rust + a QuickJS-family engine; Bun is
   dev/package/test tooling only.
2. Rust routes by method/path before JavaScript handler execution.
3. Exactly one QuickJS worker (M1/M2; multi-worker is M3).
4. The compiler never dry-runs the application — no side effects during
   route discovery.
5. Production startup performs zero route/schema/OpenAPI/plugin
   compilation and zero TypeScript transpilation.
6. Request data crossing into JS is lazy; unread fields are never
   materialized.
7. Expected HTTP failures are typed values with declared statuses;
   problems are RFC 9457-compatible; unexpected errors are redacted
   before leaving the host.
8. One schema contract drives types, runtime, Treaty, OpenAPI, and the
   lock.
9. All queues, bodies, jobs, heap, stack, and deadlines are bounded.

Full list with rationale: [AGENTS.md](https://github.com/ther12k/velqu/blob/master/AGENTS.md)
and the ADRs under
[docs/okf/decisions/](https://github.com/ther12k/velqu/tree/master/docs/okf/decisions).

## Artifact flow

```text
app.ts (route/schema declarations)
   │  velqu build  (static extraction — never executes the app)
   ▼
app.qpack (verified, deterministic)   contract.json / contract.d.ts
   │                                          │ openapi.json, contract.lock.json
   ▼                                          ▼
velqu-runtime (Rust + quickjs-ng)      Treaty clients (import contract only)
```

- **Deterministic builds:** independent builders produce byte-identical
  artifacts (verified in CI-equivalent gates).
- **Engine match:** a pack only runs on the exact runtime build it was
  compiled against (SEC-001).
- **Browser-WASM variant:** the same pack runs against a Rust kernel
  compiled to WebAssembly — see [[Browser-WASM]].

## Key crates and packages

| Unit | Role |
| --- | --- |
| `crates/q-runtime` (`velqu-runtime`) | Host: config, scheduler, worker, bounds, deadlines |
| `crates/q-router` | Host-independent router core (wasm-clean) |
| `crates/q-schema-runtime` | Schema validation (wasm-qualified) |
| `crates/q-browser-kernel` | Rust→WASM browser kernel (routing/validation/verification) |
| `packages/core`, `packages/schema` | Authoring surface (`route()`, `s.*`) |
| `packages/compiler` | Static extraction, QPack, contract/OpenAPI emission, browser target |
| `packages/treaty` | Typed client (`treaty<Api>({baseUrl, contract})`) |
| `packages/browser-runtime` | Browser dispatcher, WorkerHost, Service Worker adapter |
