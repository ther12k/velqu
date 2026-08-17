---
type: Reference
title: LLRT, QuickJS, QuickJS-NG, and rquickjs Notes
description: Architecture lessons and limits from LLRT and the selected QuickJS-family/Rust
  integration sources.
tags:
- llrt
- quickjs
- quickjs-ng
- rquickjs
- reference
status: stable
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
sources:
- id: aws-llrt
  resource: https://github.com/awslabs/llrt
  title: AWS LLRT
- id: quickjs
  resource: https://bellard.org/quickjs/quickjs.html
  title: QuickJS documentation
- id: quickjs-ng
  resource: https://quickjs-ng.github.io/quickjs/diff/
  title: QuickJS-NG differences and goals
- id: rquickjs
  resource: https://github.com/DelSkayn/rquickjs
  title: rquickjs Rust bindings
---

# AWS LLRT

AWS LLRT is a useful reference for a Rust runtime built around a QuickJS-family engine with selected native modules and limited Node/Web compatibility.

The relevant lesson is architectural:

```text
small JavaScript engine
+ Rust asynchronous host
+ selected native APIs
+ explicit compatibility limits
```

It is not proof that Project Q will beat Elysia as a general HTTP framework. LLRT has different product constraints and remains an experimental runtime with a deliberately incomplete compatibility surface.

# Upstream QuickJS

QuickJS provides a compact embeddable ECMAScript engine with:

- runtime/context APIs;
- memory and stack limits;
- interrupt handling;
- module and bytecode tooling;
- small-engine characteristics.

Its documented engine lifecycle timing is not equivalent to a full Project Q process-to-first-response result.

Bytecode is engine-version-specific and is not a safe untrusted exchange format.

# QuickJS-NG

QuickJS-NG is an actively optimized fork/family member that documents interpreter and allocation/parser improvements. It is the initial engine target through an adapter, not a permanent public framework dependency.

QuickJS-family interpreters do not use a general optimizing JIT like JavaScriptCore/V8. CPU-heavy JavaScript may therefore favor Bun or Node even when cold start/memory favors Project Q.

# rquickjs

`rquickjs` provides Rust bindings and asynchronous integration primitives. It is the proposed first bridge integration layer.

Project Q still owns:

- worker/event-loop policy;
- request-handle lifetime;
- route/pipeline manifest;
- capability API;
- cancellation;
- error/source maps;
- limits and observability.

Using a binding does not solve those product semantics automatically.

# Design conclusions

1. Build a framework/runtime for its own API, not a general compatibility runtime.
2. Keep native modules minimal and explicit.
3. Treat engine selection as replaceable internally.
4. Measure complete load and request behavior.
5. Treat same-process application code as trusted.
6. Pin bytecode to exact runtime/engine versions.
7. Record async and FFI ownership carefully.
8. Do not infer HTTP throughput from engine startup numbers.

# Source verification

At implementation time, pin exact versions/commits and review license, maintenance, supported features, and security advisories.
