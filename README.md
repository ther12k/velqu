# Velqu (VelquJS)

**A Rust HTTP runtime that runs your TypeScript handlers — with one schema
contract driving types, validation, clients, OpenAPI, and the contract
lock.**

[Docs](docs/beta/INDEX.md) ·
[Browser-WASM guide](docs/beta/BROWSER_WASM.md) ·
[Beta plan](docs/beta/README.md) ·
[Architecture decisions](docs/okf/decisions/) ·
[Contributing](docs/beta/program/WORKFLOW.md)

![status](https://img.shields.io/badge/channel-beta-orange)
![license](https://img.shields.io/badge/license-MIT-blue)
![bun](https://img.shields.io/badge/Bun-1.4.0-black)
![typescript](https://img.shields.io/badge/TypeScript-5.9.3-3178C6)
![engine](https://img.shields.io/badge/quickjs--ng-0.15.1-8A2BE2)
![rust](https://img.shields.io/badge/Rust-1.96-DEA584)

The Rust host routes by method/path and enforces bounded queues, bodies,
heap, stack, and deadlines **before any JavaScript runs**. Handlers are
authored in TypeScript and executed by quickjs-ng 0.15.1 (via
rquickjs 0.12.2) embedded in the host. A single schema contract produces
generated types, runtime validation, the Treaty client, OpenAPI, and the
contract lock — one source of truth, no drift.

> **Honesty line:** same-process QuickJS executes *trusted application
> code only*. Velqu is not a hostile-code sandbox, and no PostgreSQL-parity
> or native-performance-parity claim is made without matched, reproducible
> evidence.

## Status & install

Pre-beta development toward **`0.1.0-beta.1`** (ADR-0020,
[`docs/beta/`](docs/beta/)). Milestone state:
[`docs/beta/program/STATUS.md`](docs/beta/program/STATUS.md).
Material decisions live in [`docs/okf/decisions/`](docs/okf/decisions/);
open owner decisions in
[`docs/open-decisions.md`](docs/open-decisions.md).

The `@velqu/*` packages are publish-ready (`0.1.0-beta.1`, npm org
created) — actual publication is owner-gated (OD-050,
[`docs/beta/PUBLISHING.md`](docs/beta/PUBLISHING.md)). Today you build
from source with the pinned toolchain (Bun 1.4.0, TypeScript 5.9.3,
stable Rust):

```bash
bun install
cargo build --release -p velqu-runtime
bun run verify        # the one command over the authorized scope
```

Deployment artifacts and authenticity checks:
[`docs/beta/INSTALL.md`](docs/beta/INSTALL.md).

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
bun packages/cli/src/index.ts build --project my-app
cargo build --release -p velqu-runtime
./target/release/velqu-runtime --pack my-app/dist/app.qpack --port 8080
```

```text
$ curl -s http://127.0.0.1:8080/hello/world
{"message":"Hello world"}

$ curl -s http://127.0.0.1:8080/no-such-route
{"type":".../problems/not-found","status":404,...}
```

Expected failures are typed RFC 9457 problems at declared statuses —
404/405 are answered by the host router and never reach a handler;
validation failures are typed 422s. The compiler never executes your
application code: route discovery is static, so `velqu build` cannot run
your handlers.

## Why Velqu?

- **One contract, no drift.** The same schema IR drives generated
  TypeScript types, kernel validation, Treaty clients, `openapi.json`,
  and the contract lock. Change a schema and every surface changes
  together — or the build fails.
- **Rust routes before JavaScript.** Method/path dispatch, body limits,
  queue bounds, and schema validation happen in the host. 404/405 never
  reach a handler, and validation failures are typed 422s.
- **Bounded by construction.** Bodies, headers, queues, heap, stack,
  handler deadlines, and pending operations all have enforced limits.
- **Typed failures.** Expected HTTP failures are typed values with
  declared statuses; unexpected errors are redacted before leaving the
  host.
- **Deterministic artifacts.** QPack builds are byte-reproducible across
  independent builders; a pack only runs on the exact runtime build it
  was compiled against (SEC-001 engine match).
- **Browser-WASM target.** The same application builds to static assets
  that run in an ordinary browser — Rust kernel compiled to WebAssembly,
  generated handlers in an isolated Worker. No Velqu application server.
  See the [Browser-WASM guide](docs/beta/BROWSER_WASM.md).

## Typed client (Treaty)

`velqu build` emits `contract.json` / `contract.d.ts`; clients import
only the contract — never server source:

```ts
import { treaty, type TreatyClient } from "@velqu/treaty";
import type { Api } from "./contract";     // generated types
import contract from "./contract.json";    // generated runtime table

export const api: TreatyClient<Api> =
  treaty<Api>({ baseUrl: "http://127.0.0.1:8080", contract });

const res = await api.hello({ name: "world" }).get();
if (res.data) console.log(res.data.message); // { message: string } at 200
```

Dot-navigation is typed down to the declared method — calling `.post()`
on a GET route is a compile-time type error and a named runtime error;
path parameters are required and URI-encoded by the client.

## Features

| Capability | Status | Track |
| --- | --- | --- |
| Rust host routing (method/path, pre-JS enforcement) | Available | M0–M2 |
| quickjs-ng handler execution, single worker, bounded scheduler | Available | M2 |
| One schema contract → types / validation / OpenAPI / lock | Available | M2 |
| Typed RFC 9457 problems at declared statuses | Available | M2 |
| Treaty typed client (dot-navigation, abort/transport regressions) | Available | M2 |
| Deterministic QPack + independent-build reproducibility | Available | M2 / M26 |
| Scheduler correctness pass (owner-scoped microtasks, deadlines) | Available | M2.2.1 |
| Numeric RoutePlan (no string identities on the hot path) | Available | M2.3 |
| Zero-copy ingress/worker slab | Planned | M2.4 |
| Schema JSON codecs | Planned | M2.5 |
| Binary QPack v2 | Planned | M2.6 |
| Capabilities / WinterTC | Planned | M2.7 |
| Browser fetch boundary | Planned | M2.8 |
| Multi-worker service mode | Planned | M3 |
| Browser-WASM static deployment (Rust/WASM kernel + Worker handlers) | Release candidate — gate pending | BWASM |
| Alpha release (`0.1.0-beta.1`) | Planned | M4 / ADR-0020 |

## Examples

| Example | What it shows | Target |
| --- | --- | --- |
| [`examples/proof`](examples/proof/) | 13 routes across modules: validation, async timers, cancellation, redaction, upstream fetch | Native |
| [`examples/browser-demo`](examples/browser-demo/) | Minimal two-route app compiled to static browser assets | Browser-WASM |

## Compatibility

Verified against the pinned toolchain on Linux x86_64:

| Component | Version |
| --- | --- |
| Rust (release builds) | 1.96.0 |
| Bun (dev/package/test tooling only) | 1.4.0 |
| TypeScript | 5.9.3 |
| quickjs-ng (embedded, pinned) | 0.15.1 |
| wasm-bindgen (browser kernel ABI) | 0.2.108 |

Beta deployment target is Linux x86_64
([`docs/beta/INSTALL.md`](docs/beta/INSTALL.md)). Browser evidence lanes
run Chromium; other browsers are documented but untested
([`docs/beta/KNOWN-LIMITATIONS.md`](docs/beta/KNOWN-LIMITATIONS.md)).
Performance claims are evidence-bound — matched candidates, retained raw
samples, p50/p95/p99 (`benchmarks/raw/`, index in
[`benchmarks/manifest.json`](benchmarks/manifest.json)).

## Roadmap

- **Now:** forward track per ADR-0018 — M2.4 zero-copy ingress, M2.5
  schema JSON codecs, M2.6 binary QPack v2, M2.7 capabilities/WinterTC,
  M2.8 fetch, then M3 multi-worker.
- **Finish line:** `0.1.0-beta.1` under ADR-0020
  ([`docs/beta/`](docs/beta/)); the GA track (ADR-0019,
  [`docs/production/`](docs/production/)) follows post-beta.
- **Browser-WASM:** all eight program phases complete; the release
  candidate packet (checksums, SBOM, candidate index) is assembled and
  awaiting the recorded GO/NO-GO gate review.

## Documentation

| Document | Contents |
| --- | --- |
| [`docs/beta/INDEX.md`](docs/beta/INDEX.md) | Beta plan index — definition, scope matrix, critical path, task ledger |
| [`docs/beta/BROWSER_WASM.md`](docs/beta/BROWSER_WASM.md) | Browser-WASM developer guide: architecture, quickstart, hosting, capabilities, observability, migration |
| [`docs/beta/KNOWN-LIMITATIONS.md`](docs/beta/KNOWN-LIMITATIONS.md) | Recorded limitations (incl. browser preview boundaries 19–24) |
| [`docs/beta/INSTALL.md`](docs/beta/INSTALL.md) | Shared-mode deployment: runtime binary + QPack, update policy, limits |
| [`docs/okf/delivery/prd.md`](docs/okf/delivery/prd.md) | Product requirements (delivery frame) |
| [`docs/okf/decisions/`](docs/okf/decisions/) | Architecture decision records (ADRs) |
| [`docs/open-decisions.md`](docs/open-decisions.md) | Open and decided owner decisions (license, naming, publication) |
| [`docs/m0-m2-traceability.md`](docs/m0-m2-traceability.md) | Requirement → code/test/evidence links for completed P0s |
| [`benchmarks/manifest.json`](benchmarks/manifest.json) | Evidence-bound performance reports (raw samples retained, p50/p95/p99) |

## Contributing & security

Contributions follow the packet workflow: one branch per packet, PR with
`Closes #<issue>`, squash-merged — see
[`docs/beta/program/WORKFLOW.md`](docs/beta/program/WORKFLOW.md).
`bun run verify` must pass before any milestone checkpoint. Never weaken a
test or fixture to pass; failures are reported honestly.

Security-relevant constraints (threat model, isolation contract, claim
policy) live in ADR-0038 and
[`docs/beta/governance/`](docs/beta/governance/). Please do not open
public issues for unreported vulnerabilities — contact the repository
owner directly.

## License

[MIT](LICENSE) — decided OD-004
([`docs/open-decisions.md`](docs/open-decisions.md)).
