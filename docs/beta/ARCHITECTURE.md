# Velqu architecture (public beta)

How Velqu executes your application: one Rust host process, one pinned
QuickJS engine, and a pre-compiled application pack. This page describes
structure and boundaries, not performance; measured claims require
matched raw evidence under `benchmarks/raw/` (none is implied here).

## The three artifacts

| artifact | role |
|---|---|
| schema contract (your `app.ts` + `defineApp/defineModule`) | single source of truth: types, runtime validation, Treaty clients, OpenAPI, contract lock, RoutePlan |
| `app.qpack` (QPack) | verified deployment unit: QuickJS bytecode + route plan + schema manifest + capability inventory, hash-bound and signed-checkable |
| `velqu-runtime` / `velqu-standalone` | static Rust binary embedding quickjs-ng 0.15.1 via rquickjs =0.12.2 (pinned) |

The compiler derives everything from the contract without dry-running
the application: route discovery has no side effects, no service
factories run, and no handler executes at compile time. Production
startup performs zero route/schema/OpenAPI/plugin compilation and zero
TypeScript transpilation — it loads and verifies the pack.

## Request path

```text
client → reverse proxy (public TLS)
       → velqu-runtime (127.0.0.1)
            1. TcpListener::accept        (TCP peer identity captured;
                                           ingress headers like Host and
                                           X-Forwarded-* are data, never identity)
            2. Rust route match           (method/path from the pre-compiled
                                           RoutePlan — no JS involved yet)
            3. schema validation          (request shape checked against the
                                           manifest; failures typed, declared statuses)
            4. handler call into QuickJS  (request data crosses lazily —
                                           unread fields are never materialized)
            5. response strategy          (matched responses written natively;
                                           schema-checked)
            6. structured completion log  (field allowlist, sampled)
```

Steps 1–3 and 5–6 never execute JavaScript. A request only enters the
engine for the handler body.

## The engine boundary

- Exactly one QuickJS worker serves all handlers (M1/M2 scope).
- The worker is created under a pre-eval lockdown (`NO_DYNAMIC_CODE_LOCKDOWN`):
  `eval`, `new Function`, and prototype-constructor routes
  throw a typed `TypeError` before application code first runs.
- Heap (32 MiB default) and stack (512 KiB default) are bounded; every
  handler runs under a deadline (5 s default) enforced by the host.
- Native handles crossing into JS are opaque, generation-checked, and
  expire at settlement — a stale handle cannot be replayed.
- Same-process QuickJS executes trusted application code only. Velqu
  is not a hostile-code sandbox and does not claim hostile multi-tenant
  isolation.

## Failure model

- Expected HTTP failures are typed values with declared statuses from
  the contract; they are part of the public surface.
- Error problems are RFC 9457-compatible.
- Unexpected errors are redacted before leaving the host — internal
  details never leak into responses or logs.

## Capabilities

Capability use (e.g. `runtime:postgres@1`) is declared in the contract,
carried in a hash-bound capability inventory inside the pack, and
resolved to host capability services at startup. The reference JWT
package (`@velqu/capability-auth-jwt`) is HS256-only and fails closed
through five verification gates. Requests never receive ambient
authority; anything privileged goes through a capability.

## Configuration and deployment

- Config layers: CLI > environment > file > defaults; files must
  declare `"configVersion": 1`; the `VELQU_*` environment namespace is
  closed (unknown names reject startup); secrets are redacted
  (`SecretString` — `Debug`/`Display` never reveal values).
- Deployment posture: loopback bind with `proxyMode: "reverse-proxy"`
  default; public binds require explicit `direct` mode; graceful drain
  completes in-flight work within a bounded budget; shared and
  standalone (pack-embedded) modes plus a container image are covered
  in [INSTALL.md](INSTALL.md).

## What the pack is (and is not)

`app.qpack` embeds QuickJS bytecode alongside the route plan and
manifests, all bound by digests and an exact runtime-fingerprint match
(a pack runs only on the exact runtime build it was compiled against).
Bytecode improves startup and enables strict verification; it is not
native-machine-code JIT compilation, and no universal performance
claim is made. This is a non-SLA public beta — see
[01_BETA_DEFINITION.md](01_BETA_DEFINITION.md) for scope and
[LIMITS-AND-NON-GOALS.md](LIMITS-AND-NON-GOALS.md) for boundaries.
