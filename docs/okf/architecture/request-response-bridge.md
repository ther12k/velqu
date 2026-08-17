---
type: Architecture Specification
title: Request and Response Bridge
description: Lazy native-backed request access, JSON strategy spikes, response encoding,
  handle safety, and bridge kill criteria.
tags:
- bridge
- ffi
- materialization
- json
- quickjs
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
sources:
- id: quickjs
  resource: https://bellard.org/quickjs/quickjs.html
  title: QuickJS documentation
- id: rquickjs
  resource: https://github.com/DelSkayn/rquickjs
  title: rquickjs Rust bindings
---

# Purpose

The bridge is the design's highest-risk performance boundary. It connects Rust-owned HTTP state to QuickJS-owned application values without eagerly duplicating the request or traversing more data than the handler uses.

# Core rule

```text
route in Rust
materialize on demand
cross the boundary as few times as practical
invalidate all request handles at completion
```

# Native-backed context

Conceptual TypeScript surface:

```ts
async function handle(ctx: Context<RouteContract>) {
  const id = ctx.params.id;
  const token = ctx.headers.get("authorization");
  const body = await ctx.body.json();

  return {
    id,
    accepted: body.accepted
  };
}
```

The implementation may optimize typed properties generated from route metadata, but the semantic contract remains ordinary typed values.

# Lazy categories

| Category | Initial state | Materialization trigger |
|---|---|---|
| method/route ID | tiny immediate metadata | handler invocation |
| path parameters | native slices/offsets | property access or validation strategy |
| query | raw native query bytes | query access/declared decode |
| headers | native header table | named lookup or full enumeration |
| cookies | raw cookie header | cookie capability access |
| body | stream/native bytes | declared decoder or explicit read |
| Web `Request` | absent | explicit `ctx.request` access |
| URL object | absent | explicit URL access |
| response wrapper | absent | raw response path |

# Handle safety

A JavaScript native-backed object contains an opaque handle with:

```text
runtime generation
worker ID
invocation generation
handle slot
capability kind
```

Every native access validates this identity.

After request settlement:

- handle slots are invalidated;
- async operations are cancelled or detached according to policy;
- retained JavaScript wrappers throw a stable `RequestExpiredError`;
- memory is not reused under the same generation without increment.

# Body strategies

The bridge spike compares:

## Strategy A — QuickJS parse

```text
Rust reads bounded bytes
→ one byte/string transfer
→ QuickJS JSON.parse
→ JavaScript validation or generated checks
```

Potential strength: simple and avoids deep Rust-to-JS object construction.

Potential weakness: duplicate parsing/validation work and less native control.

## Strategy B — Rust parse and convert

```text
Rust parses JSON
→ Rust validation
→ recursive materialization into QuickJS objects
```

Potential strength: one parser and early validation.

Potential weakness: conversion and allocation can dominate, especially for large nested values.

## Strategy C — generated direct decoder

```text
bounded input
→ generated schema-aware scanner/decoder
→ only requested/validated fields materialized
```

Potential strength: can combine parse, validation, coercion, and projection.

Potential weakness: high compiler/runtime complexity and a narrow schema subset.

M0/M1 SHALL not assume C is required. The simplest strategy that passes budgets wins.

# Header/query optimization

For declared schemas:

```ts
headers: s.object({
  authorization: s.optional(s.string()),
  "if-match": s.optional(s.string())
})
```

the compiled route can request only those header values. A full `Headers` object is created only for enumeration or raw compatibility.

Similarly, query schemas may decode only declared keys while retaining a raw-query fallback.

# JavaScript boundary-call accounting

Each route build report includes an estimate and measured trace support:

```text
route: users.get
native stages: route, params decode
javascript stages: auth policy, handler
expected JS calls: 2
request materialization: params.id, authorization
body strategy: none
response strategy: generated object encoder
```

A JavaScript middleware chain should not accidentally turn one handler call into ten host crossings without visibility.

# Result model

Normal result forms:

```ts
return value;                    // inferred default success status
return status(201, value);       // declared success
return problem(404, details);    // declared failure
return text("ok");
return bytes(buffer);
```

The runtime sees a tagged result or a route-specialized convention. Generic object introspection is avoided where generated tags can safely reduce ambiguity.

# Response strategies

The spike compares:

1. QuickJS `JSON.stringify` followed by one byte/string transfer;
2. Rust traversal of a QuickJS object and native serialization;
3. generated specialized encoder for declared response schemas;
4. handler-provided bytes or stream.

The result must include correct escaping, numeric edge cases, Unicode, null/optional semantics, and schema conformance—not only speed.

# Raw Web compatibility

`ctx.request` and a Web-compatible `Response` are supported as explicit fallback paths after the basic API is stable.

The build report labels them:

```text
request-wrapper: full-web
response-wrapper: full-web
optimization: generic
```

The product does not claim full browser or Node Web API compatibility until conformance fixtures prove each surface.

# Async bridge

Native capabilities return promises. Cancellation propagates through an invocation token.

Required races:

- native operation completes before cancellation;
- cancellation wins before completion;
- completion arrives after request settlement;
- timeout interrupts JavaScript while native work is pending;
- handler catches an abort error;
- worker shuts down with pending operations.

Each race receives deterministic tests.

# Microbenchmark matrix

Input shapes:

```text
no input
1 path parameter
5 path/query/header scalar values
small JSON object
nested JSON object
array of 100 records
large bounded string
invalid JSON
schema-invalid JSON
```

Output shapes:

```text
integer
short string
small object
nested object
array of 100 records
typed problem
pre-serialized bytes
```

Report:

- per-request latency;
- throughput;
- allocations in Rust;
- QuickJS heap delta;
- bytes copied;
- number of host calls;
- serialization/materialization time.

# Kill criteria

A redesign is required when, under fair release builds:

- an empty cached QuickJS handler retains less than 50% of matched native Rust throughput **and** route workloads do not recover enough product value;
- bridge overhead exceeds the complete request budget needed to beat the cold-start/latency target;
- native parse/validate/materialize is slower or uses materially more memory than QuickJS parsing for the target shape;
- request-handle safety requires pervasive copies that defeat the lazy model;
- asynchronous completion cannot be made race-safe without unbounded state.

These are design gates, not promised observed outcomes.
