# Research Notes and Assumptions

Research baseline: public repository `ther12k/velqu`, commit `84740c54242a116ad8424dc4a14cca8d3af2dd93`, observed on 2026-09-04. Re-check every statement before implementation if `master` has moved.

This file separates **repository-derived observations**, **browser/platform constraints**, and **proposed architecture**. The proposal is not represented as already implemented.

## A. Repository-derived observations

### A.1 Native production runtime

Observed workspace and source paths show a native Rust runtime organized around crates including:

- `crates/q-runtime`
- `crates/q-http`
- `crates/q-engine`
- `crates/q-engine-quickjs`
- `crates/q-pack`
- `crates/q-router`
- `crates/q-schema-runtime`
- `crates/q-capabilities`
- `crates/q-capability-postgres`

The root workspace enables native-oriented Tokio/Hyper, filesystem/process/signal/network, mmap, QuickJS host, and Postgres dependencies. Therefore the packet does not assume that the existing `q-runtime` can be switched to `wasm32-unknown-unknown` unchanged.

### A.2 Existing portable foundations

At the baseline:

- `packages/core/src/index.ts` contains authoring contracts rather than a browser server.
- `packages/treaty/src/index.ts` uses portable Request/Response/Fetch-shaped concepts and direct dispatch support.
- `packages/testing/src/index.ts` contains a direct/in-process Treaty helper that avoids HTTP transport, but testing code also contains Bun-specific behavior and is not a production browser runtime.
- `crates/q-schema-runtime` has a relatively portable dependency surface and is a candidate for native+WASM reuse.
- `crates/q-pack` includes byte-slice verification functionality alongside native loading/mmap responsibilities.
- `crates/q-router` and `crates/q-engine` currently need dependency separation before a small host-independent routing kernel can be compiled cleanly.
- `crates/q-engine-quickjs` uses native execution machinery and is not assumed to be the MVP browser handler engine.
- the current TypeScript Postgres capability appears synchronous from the authoring surface, which conflicts with ordinary browser database APIs and motivates an explicit async migration decision.

These observations justify enhancing Velqu rather than replacing it with a second framework.

### A.3 Current issue-management pattern

The repository's existing public issues use atomic identifiers, explicit dependencies, mode/priority labels, read-first paths, steps, guardrails, commands, evidence, out-of-scope boundaries, and stop conditions. This packet follows that structure and adds Browser-WASM-specific labels.

The packet uses a label named `milestone:browser-wasm`, matching the repository's label-oriented milestone convention. It does not assume a GitHub Milestone object already exists.

## B. Browser/platform constraints

The following are platform constraints rather than Velqu defects:

- ordinary browsers do not expose a TCP listening socket equivalent to a native Hyper server;
- a Service Worker can intercept requests within its scope and answer with browser `Response` objects;
- `wasm32-unknown-unknown` does not provide general native filesystem/process/thread/network behavior;
- browser code and WebAssembly still require a host environment and usually static HTTPS delivery;
- Web Workers can be terminated for recovery but do not alone prove a hostile-code security boundary;
- IndexedDB and browser database engines are asynchronous;
- browser headers, cookies, redirects, credentials, streaming, and cache behavior differ from native server ingress;
- real-browser tests are necessary; a successful Rust compile alone does not prove browser support.

## C. Proposed architecture, not current implementation

This packet proposes:

1. a host-independent Rust model/router/pack/schema boundary;
2. `q-browser-kernel` compiled to WebAssembly;
3. `@velqu/browser-runtime` exposing `fetch(Request): Promise<Response>`;
4. generated handlers running as JavaScript in an isolated Worker for MVP;
5. a content-addressed static artifact manifest and loader;
6. an optional scoped Service Worker deployment adapter;
7. namespaced IndexedDB KV as mandatory local persistence;
8. PGlite as optional SQL convenience;
9. a machine-readable `deployment-required` capability state;
10. an optional QuickJS-NG-in-WASM spike, not an implicit MVP requirement.

Names are provisional until `BWASM-D-001` and `BWASM-D-002` are accepted.

## D. External references to verify during implementation

Use primary documentation and pin versions at implementation time:

- Rust `wasm32-unknown-unknown` platform support documentation.
- `wasm-bindgen` and `wasm-bindgen-test` documentation.
- Web Platform Service Worker, Fetch, Web Worker, CSP, Permissions Policy, IndexedDB, and Web Crypto specifications/documentation.
- selected browser automation tooling documentation.
- PGlite documentation if `BWASM-C-003` proceeds.
- QuickJS-NG, `rquickjs`, and candidate browser-WASM toolchain sources if `BWASM-X-001` proceeds.

Do not copy a current external version assumption from this planning packet into a support claim without re-verification.

## E. Assumptions requiring owner ratification

- Browser-WASM beta prioritizes prototype/static applications, not every native Velqu workload.
- Worker JavaScript handler execution is acceptable for MVP.
- compatibility-critical semantics must still flow through Rust/WASM.
- compiler execution can remain native/Bun-hosted.
- separate preview origin is mandatory for untrusted app-builder code.
- IndexedDB KV is mandatory; PGlite is optional.
- Postgres capability becomes async before API freeze.
- QuickJS-WASM is optional and evidence-gated.
- native Velqu remains the deployment path for production-only capabilities.

These are tracked in `OWNER_DECISIONS.md`; they are not silently treated as final.
