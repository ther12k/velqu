---
type: Design Principles
title: Project Q Design Principles
description: Non-negotiable product and engineering principles governing the framework.
tags:
- principles
- architecture
- performance
- typing
- compatibility
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
sources:
- id: elysia-2
  resource: https://elysiajs.com/blog/elysia-20
  title: Elysia 2 beta announcement and AOT design
- id: elysia-best-practice
  resource: https://elysiajs.com/essential/best-practice
  title: Elysia best-practice guide
- id: quickjs
  resource: https://bellard.org/quickjs/quickjs.html
  title: QuickJS documentation
---

# Design principles

## 1. Cold start is end-to-end

A JavaScript engine initialization number is not framework cold start. Measure process spawn, runtime initialization, application load, socket readiness, routing, validation, handler execution, and first response separately.

## 2. Static knowledge leaves the hot path

Routes, schemas, policy graphs, response metadata, defaults, OpenAPI, and client contracts are prepared before production startup whenever practical.

## 3. No application dry-run

The compiler must not execute application setup to discover routes. Build-time analysis operates on a constrained declarative contract and handler bindings.

## 4. One source of truth

A route schema supplies runtime validation rules, TypeScript inference, response contracts, OpenAPI metadata, Treaty inputs, and semantic API diff data.

## 5. Business logic remains TypeScript

Rust owns infrastructure, not product-specific domain behavior. Developers should not need to write Rust for normal routes.

## 6. Native work must earn its boundary cost

Moving a task to Rust is not automatically faster. Every native stage must include the cost of data conversion, allocation, scheduling, and result materialization.

## 7. Lazy materialization by default

Do not construct full `Request`, `Headers`, URL, query, params, cookies, or body objects unless the handler or policy actually requires them.

## 8. Explicit composition beats order-dependent mutation

Policies, interceptors, plugins, and capabilities declare their dependencies and outputs. Duplicate or conflicting composition fails at build time.

## 9. Local types, compact contracts

Route types should remain module-local. Frontend consumers may import a compact flattened contract instead of the complete server implementation.

## 10. Compatibility is a budget

Every Node, Bun, Web, or npm-compatible API increases binary size, maintenance, tests, and attack surface. Compatibility is added only for demonstrated use cases.

## 11. Trusted code and hostile code are different products

QuickJS limits improve resilience. They are not a substitute for process or OS isolation when executing untrusted code.

## 12. Observable performance

Every route should be inspectable for native stages, JavaScript calls, materialization points, fallback validation, and linked capabilities.

## 13. Correctness before microbenchmarks

HTTP parsing, cancellation, backpressure, schema fidelity, error contracts, and cleanup must be correct before throughput optimization.

## 14. Fair comparison

Benchmarks use matched behavior, pinned versions, published hardware and commands, multiple route counts, and statistical distributions.

## 15. Small public API during proof

The first milestones expose only abstractions required by the proof application. Speculative convenience APIs wait.

## 16. Failure is evidence

If the bridge or cold-start advantage does not materialize, document the result and revise the thesis rather than hiding the comparison.

## 17. Build output is a contract

An application pack includes runtime version, engine version, schema IR version, route manifest version, capability set, contract hash, and source-map metadata.

## 18. Feature-based application structure

Models/contracts, services, policies, and route adapters stay close to their feature while business services remain framework-independent.

## 19. Defer is not a durable queue

After-response work is bounded and observable. Durable jobs require an external queue capability.

## 20. Naming and public commitments remain reversible

The working name, package namespace, license, governance, repository ownership, and release date require explicit owner decisions.
