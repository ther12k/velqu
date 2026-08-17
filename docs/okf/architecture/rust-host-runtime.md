---
type: Architecture Specification
title: Rust Host Runtime
description: Production host responsibilities, startup state machine, ownership, admission,
  scheduling, response, and shutdown.
tags:
- rust
- runtime
- hyper
- tokio
- http
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
sources:
- id: hyper
  resource: https://docs.rs/hyper
  title: hyper Rust HTTP library
- id: tokio
  resource: https://tokio.rs/
  title: Tokio asynchronous runtime
- id: rquickjs
  resource: https://github.com/DelSkayn/rquickjs
  title: rquickjs Rust bindings
---

# Purpose

The Rust host provides the minimum production substrate required to accept HTTP requests, enforce limits, dispatch routes, drive QuickJS work, and write responses.

The initial implementation favors correctness, measurability, and a narrow public surface over writing custom transport primitives.

# Initial technical baseline

Proposed M1 baseline:

```text
Rust stable
Tokio
hyper
rquickjs
QuickJS-NG feature/engine target
HTTP/1.1
one listener
one QuickJS worker
one versioned application pack
```

Use of Tokio and hyper is an implementation baseline, not an eternal public contract. Any replacement requires feature-equivalent conformance and comparative evidence.

Axum, Actix Web, or another full Rust web framework is not required in the core because routing and framework semantics are Project Q responsibilities. Small internal utilities may be used when justified and pinned.

# Host components

```text
runtime/
├── config
├── application_pack
├── listener
├── http
├── admission
├── router
├── bridge
├── engine
├── capabilities
├── scheduler
├── response
├── telemetry
└── shutdown
```

# Startup state machine

```text
process_started
  → config_validated
  → pack_verified
  → native_components_initialized
  → engine_worker_created
  → application_loaded
  → handlers_cached
  → listener_bound
  → ready
```

Failure before `ready` is terminal and diagnosable. The host does not start accepting traffic with a partially verified application.

# Request ownership

A request receives a native request handle scoped to its invocation.

The handle owns or references:

- request method and canonical URI;
- parsed path captures;
- header table;
- body state or stream;
- cancellation token;
- route pipeline ID;
- trace/request ID;
- response/deferred state.

JavaScript wrappers hold opaque handle identifiers, not raw Rust pointers. Every access checks worker and invocation generation so a retained object cannot read memory belonging to a later request.

# Admission and backpressure

Admission happens before expensive body or JavaScript allocations.

Limits include:

- maximum header bytes/count;
- maximum URI length;
- route-specific body limit;
- maximum queued requests;
- maximum in-flight requests per worker;
- maximum pending native operations per invocation;
- request deadline;
- optional connection-level limits.

When saturated, the host fails explicitly with a configured overload response rather than growing an unbounded queue.

# Scheduler boundary

The HTTP side submits a compact invocation message:

```rust
Invocation {
    request_handle,
    route_id,
    handler_id,
    deadline,
    trace_id,
}
```

The JavaScript worker executes only messages assigned to its owning runtime/context. Promise completions and native callbacks are returned to that same worker.

M1 uses one worker, making ownership unambiguous. Multi-worker scheduling is deferred to [Concurrency and Isolation](concurrency-and-isolation.md).

# Response ownership

A handler may return:

- a declared structured value;
- a typed status result;
- text/bytes;
- an approved stream handle;
- a raw response fallback.

The response encoder validates the returned status and strategy against the route manifest. Production behavior for an undeclared result is a controlled internal error; development includes precise diagnostics.

Headers become immutable once response body write begins.

# Shutdown

Graceful shutdown:

1. stops accepting new connections;
2. rejects new queued work;
3. waits for in-flight requests up to a bounded deadline;
4. cancels remaining native operations;
5. runs mandatory cleanup hooks where safe;
6. closes services and engine workers;
7. exits with an observable result.

Best-effort deferred work does not extend shutdown indefinitely.

# Failure classes

| Class | Behavior |
|---|---|
| malformed HTTP | protocol-appropriate client error or connection close |
| admission limit | explicit overload/body/header response |
| route decode/validation | declared problem response |
| policy denial | declared typed problem response |
| JavaScript exception | mapped error in development; redacted 500 in production |
| engine fatal/corruption | worker terminated, request failed, process policy applied |
| native capability error | typed capability error or internal failure |
| invalid application pack | fail before ready |
| bytecode version mismatch | fail before ready |

# Rust safety rules

- `unsafe` is prohibited by default outside reviewed FFI wrappers.
- Every FFI wrapper receives targeted tests and documented ownership.
- JavaScript values do not cross runtime/thread ownership boundaries directly.
- Opaque handles use generation counters and explicit invalidation.
- Panics do not become ordinary control flow.
- Secrets and raw authorization data are not formatted by default error paths.
- All externally controlled allocations have limits.
- Fuzzing targets HTTP/bridge/manifest parsing boundaries.

# M1 acceptance criteria

- process loads a signed/hash-verified development pack and reaches ready;
- actual HTTP/1.1 keep-alive requests work;
- route dispatch happens before QuickJS;
- cached handler invocation works for plaintext and JSON;
- cancellation and timeout are observable;
- body/header/queue limits fail predictably;
- no request handle remains usable after invocation completion;
- graceful shutdown closes listener, worker, and services;
- runtime-local tests use the actual binary over loopback.
