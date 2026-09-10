# Velqu Browser-WASM — Master Plan

Status: proposed GitHub issue packet  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Program ID: `BWASM`  
Target label: `milestone:browser-wasm`  
Issue count: **38 total / 36 mandatory / 2 optional**

## 1. Executive decision

Velqu should gain a **second deployment target** for browser-local execution. The target must be real WebAssembly, but it must not attempt to compile the existing native `q-runtime` wholesale into a browser.

The selected architecture is:

```text
Velqu source
    |
    v
Velqu compiler --target browser-wasm
    |
    +-- q-browser-kernel.wasm
    +-- browser handler bundle
    +-- route/schema/capability manifest
    +-- contract and source maps
    +-- integrity-bound loader/bootstrap
    |
    v
Static HTTPS hosting
    |
    v
Service Worker or injected Fetch adapter
    |
    v
BrowserRuntime.fetch(Request) -> Promise<Response>
    |
    +-- Rust/WASM kernel:
    |     artifact/manifest verification
    |     route matching and params
    |     schema validation
    |     capability authorization
    |     canonical problem mapping
    |
    +-- isolated Worker:
          generated TypeScript handler execution
          browser-safe capability adapters
          cancellation and hard recovery
```

The existing native deployment remains:

```text
QPack -> native Rust runtime -> QuickJS-NG -> Hyper/Tokio -> native capabilities
```

This is not an accidental fork of Velqu. Browser and native targets share contract IDs, route semantics, schema IR, capability declarations, status/problem behavior, and conformance fixtures. They intentionally do not pretend to share TCP listening, Hyper backpressure, OS signals, native filesystem/process behavior, real server secrets, or native PostgreSQL connections.

## 2. Product outcome

A user can:

1. author a normal supported Velqu application;
2. build it with `velqu build --target browser-wasm`;
3. publish the generated directory to generic static HTTPS hosting;
4. load the application in a supported browser;
5. navigate or call routes without a running Velqu application server;
6. use browser-local state and approved browser capabilities;
7. receive a structured `deployment-required` result when the application needs production-only infrastructure;
8. move to native Velqu deployment without rewriting route/schema contracts.

For an AI app builder, this creates a clear commercial boundary:

```text
Free/local prototype:
  static hosting + Browser-WASM + local persistence

Paid/real deployment:
  public backend + secrets + durable multi-user database
  + email/payment/webhook/queue/cron integrations
  + native Velqu runtime
```

The pricing/control-plane decision is outside Velqu core. Velqu supplies machine-readable deployment requirements so the product can make that decision honestly.

## 3. What “deployed in the browser using WASM” means

The release claim is valid only when all of the following are true:

- a Rust-produced `.wasm` kernel is on the mandatory request path;
- route selection, request/response schema validation, capability authorization, and canonical problem mapping cannot silently bypass that kernel;
- emitted application artifacts run on static HTTPS hosting;
- no Velqu application server, TCP listener, or native process is needed after deployment;
- handlers execute outside the editor/UI realm;
- compatibility and browser claims are tied to real-browser evidence;
- static hosting and any external model/API gateway are disclosed rather than described as “zero infrastructure”.

The MVP does **not** require QuickJS-NG inside the browser. Generated handlers execute as browser JavaScript in an isolated Worker. `BWASM-X-001` separately determines whether QuickJS-NG-in-WASM is worth the payload, startup, maintenance, and security cost.

## 4. Current-repository fit

The plan deliberately builds on current Velqu boundaries instead of replacing them:

- `packages/core` already contains authoring contracts and should remain free from browser-host I/O.
- `packages/treaty` already uses a portable Fetch-shaped boundary and direct dispatch concepts.
- `packages/testing` demonstrates in-process/direct handler dispatch, but it is a test helper and contains Bun-specific behavior; it must not become the shipped browser runtime by renaming it.
- `crates/q-schema-runtime` is a strong portable-WASM candidate.
- `crates/q-router` currently couples routing to native engine/pack types and needs a host-independent core.
- `crates/q-pack` contains both byte-slice verification logic and native file/mmap concerns; these must be split.
- `crates/q-engine` mixes portable IDs/models with Tokio-backed execution concerns; the portable model belongs in a smaller crate.
- `crates/q-runtime`, `q-http`, and `q-engine-quickjs` remain native because their HTTP, Tokio, OS-thread, signal, filesystem/process, and embedded-engine responsibilities are not browser primitives.

## 5. Required runtime profiles

### 5.1 Worker/injected-Fetch profile

The first profile and development baseline:

```ts
const runtime = await createBrowserRuntime(artifacts);

const response = await runtime.fetch(
  new Request("https://preview.invalid/api/todos"),
);
```

It works in a dedicated Worker or preview iframe and does not depend on Service Worker lifecycle. React or another UI can inject this Fetch implementation directly.

### 5.2 Static Service Worker profile

The deployment profile:

- a scoped Service Worker intercepts only application-owned requests;
- it loads and verifies one content-addressed build;
- it forwards requests to `BrowserRuntime.fetch`;
- it returns browser `Response` objects;
- it supports deterministic installation, activation, update, offline, and rollback behavior;
- it never controls the editor/control-plane scope.

### 5.3 Native production profile

The existing Velqu profile remains the production path for:

- real server secrets;
- remote/native PostgreSQL;
- public webhooks;
- payment and email providers;
- scheduled jobs and durable queues;
- production multi-user state;
- native ingress, lifecycle, and operational controls.

## 6. Portable Rust/WASM boundary

The program should produce or extract approximately these boundaries:

```text
q-runtime-model
  route IDs
  schema IDs
  capability IDs
  manifest/contract models
  status/problem models
  serde-only, host independent

q-pack-core
  verify_from_slice
  canonical manifest parsing
  integrity and bounds checks
  no filesystem/mmap/process assumptions

q-pack-native
  filesystem/mmap/loading/signing integrations

q-router-core
  compiled route table
  precedence and parameter extraction
  method resolution
  no engine/HTTP/runtime dependency

q-schema-runtime
  host-independent schema validation
  explicit limits
  native + wasm tests

q-browser-kernel
  wasm-bindgen ABI
  artifact/manifest verification
  router + schema + capability authorization
  request/response validation plan
  canonical problem mapping
```

Names are proposals. The issue acceptance criteria, not these exact names, define the boundary.

The WASM ABI must be versioned, bounded, deterministic, and tested in an actual browser. It must reject malformed, oversized, stale, mixed-version, or unsupported artifacts before handler execution.

## 7. JavaScript/browser runtime boundary

`@velqu/browser-runtime` owns:

- artifact loading and verification orchestration;
- `Request -> Promise<Response>`;
- body/header/query normalization;
- invocation of the Rust/WASM kernel;
- isolated Worker handler execution;
- cancellation and Worker replacement;
- browser capability registry;
- Treaty transport/direct dispatch without semantic bypass;
- structured diagnostics;
- optional Service Worker integration.

It must not import:

- `Bun.*`;
- `node:*`;
- native addons;
- `q-http` or `q-runtime`;
- editor/control-plane credentials;
- production provider secrets.

## 8. Compiler and artifact contract

`velqu build --target browser-wasm` emits a self-contained, deterministic artifact directory such as:

```text
dist/browser-wasm/
  index.html
  velqu-sw.js
  velqu-bootstrap.js
  velqu-loader.js
  q-browser-kernel.<digest>.wasm
  handlers.<digest>.js
  app-manifest.<digest>.json
  contract.<digest>.json
  schemas.<digest>.json
  source-maps/...
  asset-manifest.json
```

The canonical manifest binds:

- target and ABI versions;
- build ID;
- route/schema/capability contract versions;
- URLs, byte lengths, media types, and SHA-256 digests;
- import-policy version;
- support-profile requirements;
- optional capability chunks;
- source-map policy.

The loader verifies bytes before activation. An N+1 build cannot execute with N's handler, schema, or WASM bytes.

The compiler may remain a native/Bun tool for the MVP. “Browser deployment” applies to the output runtime, not necessarily to compilation itself.

## 9. Capability policy

Every capability is classified:

| State | Meaning |
|---|---|
| `browser` | Implemented only for the browser target |
| `browser-and-native` | Contract supported by both targets |
| `simulated` | Explicit opt-in preview simulation; never presented as a real side effect |
| `deployment-required` | Requires native/hosted infrastructure |
| `forbidden` | Not allowed for this target/security profile |

Mandatory browser baseline:

- timer with cancellation/deadline behavior;
- approved Web Crypto operations;
- bounded/redacted logging;
- restricted outbound fetch, default deny;
- memory + namespaced IndexedDB KV persistence.

Optional:

- PGlite-backed local SQL subset;
- QuickJS-NG-in-WASM parity profile.

Production/deployment-required by default:

- secrets;
- real remote PostgreSQL credentials;
- payment side effects;
- email delivery;
- public webhooks;
- cron and durable queues;
- shared multi-user persistence.

Unavailable capability use must fail before side effects and produce a stable result similar to:

```json
{
  "type": "urn:velqu:problem:deployment-required",
  "code": "VELQU_DEPLOYMENT_REQUIRED",
  "capability": "secrets",
  "routeId": "checkout.create",
  "reason": "native-runtime-required"
}
```

Exact fields are frozen in `BWASM-D-001` and `BWASM-C-005`.

## 10. Database contract decision

The current Postgres authoring contract must become asynchronous before Browser-WASM API freeze:

```ts
const result = await ctx.native.postgres.sql(
  "select * from todos where done = $1",
  [false],
);
```

A browser database implementation is asynchronous. Preserving a synchronous public contract would force target-specific application code or unsafe blocking tricks. `BWASM-C-002` therefore updates the shared contract first; `BWASM-C-003` adds PGlite only as an optional adapter.

IndexedDB KV is the mandatory local-persistence baseline because the core runtime should not impose a database-WASM payload on every project.

## 11. Security model

A public app builder changes the threat model from trusted application code to potentially untrusted generated/user code. The program must not imply that WebAssembly or a Worker automatically creates a secure hostile-code sandbox.

Required shape:

```text
editor/control plane origin
    |
    | validated postMessage only
    v
separate preview origin
    |
    +-- sandboxed iframe
    +-- strict CSP and Permissions Policy
    +-- scoped Service Worker
    +-- isolated execution Worker
    +-- network default deny
    +-- no provider keys or production secrets
```

Controls include:

- separate origin and Service Worker scope;
- schema-, origin-, project-, and invocation-validated messages;
- bounded request/body/response/log/message sizes;
- Worker deadline, termination, replacement, and stale-message rejection;
- restricted imports and outbound network;
- no ambient credentials;
- redacted diagnostics;
- independent security review before a beta claim.

Worker termination is a hard recovery mechanism, not proof of a hard heap quota. Any hostile-code sandbox claim needs separate evidence and owner acceptance.

## 12. Compatibility and conformance

The project needs one shared fixture corpus executed against native and browser targets.

Each behavior is classified:

| Class | Meaning |
|---|---|
| Exact parity | Canonical outputs are identical |
| Equivalent by contract | Host-level details differ but public semantics match |
| Browser-only | Explicit Browser-WASM feature |
| Native-only | Explicit native-runtime feature |
| Unsupported | Rejected at build/load/runtime with stable diagnostic |

Required coverage includes:

- route precedence, params, methods, 404/405/OPTIONS/HEAD policy;
- query, headers, supported body forms, and limits;
- request/response schemas;
- declared status and problem behavior;
- Treaty typing and decoding;
- capability authorization and deployment-required behavior;
- abort/cancellation;
- artifact verification and mixed-version rejection.

Browser support claims require blocking real-browser CI lanes. A standards-compliant design is not itself support evidence.

## 13. Performance and release budgets

`BWASM-D-004` freezes thresholds before release qualification. Budgets must cover:

- core compressed and uncompressed artifact size;
- optional SQL/engine chunk isolation;
- cold/warm load;
- verification and WASM instantiation;
- first request and steady request;
- Worker restart;
- update activation;
- memory growth under repeated requests, errors, aborts, and restarts.

Browser-local numbers must not be marketed as comparable to native network-server throughput.

## 14. Program phases

| Phase | Purpose | Issues |
|---|---|---:|
| 00 Program | Epic, ownership, decision tracking | 1 |
| 01 Design | Freeze product, portability, security, support contracts | 4 |
| 02 Kernel | Extract portable Rust and build real WASM kernel | 6 |
| 03 Runtime | Fetch dispatcher, Workers, Treaty, capability registry | 6 |
| 04 Build/deploy | Compiler target, manifests, Service Worker, CLI, rollback | 6 |
| 05 Capabilities | Browser adapters, async Postgres, local persistence | 5 |
| 06 Quality/release | Differential tests, browser CI, security, budgets, cleanroom | 8 |
| 07 Optional parity | QuickJS-NG-in-WASM GO/NO-GO spike | 1 |
| 08 Gate | Exact-candidate beta decision | 1 |

See [`TASK_INDEX.md`](./TASK_INDEX.md) for every issue and [`DEPENDENCY_GRAPH.md`](./DEPENDENCY_GRAPH.md) for sequencing.

## 15. Gate policy

The Browser-WASM MVP is GO only when:

- all mandatory design decisions are frozen;
- portable kernel evidence is complete;
- browser runtime evidence is complete;
- static activation/update/rollback evidence is complete;
- deployment-required behavior is complete;
- native/browser conformance is reviewed;
- every claimed browser has blocking evidence;
- independent security review is complete;
- size/startup/latency/leak budgets pass;
- docs and migration are current;
- an external cleanroom static deployment succeeds;
- one exact candidate has checksums, SBOM, provenance, and claim-to-evidence mapping;
- zero unresolved P0 remains;
- every remaining P1 is explicitly accepted or makes the result NO-GO.

`BWASM-C-003` and `BWASM-X-001` are optional by default. They become mandatory only through a recorded owner decision before candidate freeze.

## 16. Non-goals

- Compile all of `q-runtime`, Hyper, Tokio networking, signals, filesystem/process, native Postgres, and current QuickJS host into a browser.
- Open a TCP listening server from a browser tab.
- Run provider secrets in the browser.
- Claim real shared multi-user persistence from IndexedDB or PGlite.
- Claim Worker/WASM isolation as a proven hostile-code sandbox.
- Make the compiler run in the browser in the MVP.
- Replace native Velqu deployment.
- Promise byte-for-byte JavaScript-engine parity before the optional spike passes.
- Put billing/pricing policy inside the framework.

## 17. Critical risks

| Risk | Consequence | Required mitigation |
|---|---|---|
| Full-runtime port scope explosion | Beta never ships | Enforce target boundary in D-001/D-002 |
| JavaScript-only preview drift | False confidence before native deployment | Mandatory Rust/WASM kernel + Q-001 differential suite |
| Untrusted code escapes preview scope | Credential/data compromise | D-003, R-004, Q-003 independent review |
| Mixed cached build | Wrong handler/schema/kernel combination | B-002 and B-006 atomic activation |
| Capability silently mocked | User believes real side effect occurred | C-005 fail-closed classification |
| SQL payload bloats all apps | Poor startup/mobile UX | PGlite optional and lazy loaded |
| Browser support overclaim | Production failures | D-004 and Q-002 evidence-bound matrix |
| Async database break hidden | Target-specific APIs and unresolved promises | C-002 explicit migration |
| QuickJS-WASM becomes mandatory prematurely | Payload and maintenance burden | X-001 optional GO/NO-GO |
| Native runtime regression | Existing product damaged | Native regression and Q-001 shared fixtures |

## 18. Recommended registration sequence

Do not create and assign all 38 issues as if they can run concurrently.

Open first:

1. `BWASM-EPIC`
2. `BWASM-D-001`
3. `BWASM-D-002`
4. `BWASM-D-003`
5. `BWASM-D-004`

After those decisions are accepted, register the kernel phase, then runtime/build/capability work in dependency order. Quality and gate issues can be pre-registered for visibility but should not be treated as executable until their dependencies exist.

The included script defaults to dry-run and supports phase-filtered creation.

## 19. Completion definition

Completion is not “the demo loaded once.” It is the exact-candidate definition in [`DEFINITION_OF_DONE.md`](./DEFINITION_OF_DONE.md), followed by an explicit GO in `BWASM-GATE`.
