---
type: Project Charter
title: Project Q Charter
description: Mission, problem, users, constraints, and evidence-based definition of
  success for Project Q.
tags:
- project-q
- charter
- cold-start
- typescript
- rust
- quickjs
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
sources:
- id: elysia-2
  resource: https://elysiajs.com/blog/elysia-20
  title: Elysia 2 beta announcement and AOT design
- id: eden-treaty
  resource: https://elysiajs.com/eden/treaty/overview
  title: Eden Treaty overview
- id: aws-llrt
  resource: https://github.com/awslabs/llrt
  title: AWS LLRT
---

# Mission

Build a small, deterministic TypeScript server framework that treats cold start, contract integrity, and end-to-end client typing as first-class product requirements.

Project Q combines:

- Bun-first package management, testing, and TypeScript development;
- a static contract compiler;
- a Rust HTTP and capability host;
- a QuickJS-family engine for application logic;
- an Eden Treaty-inspired typed client.

## Problem statement

Modern TypeScript backend frameworks often optimize developer experience and hot-path throughput while carrying one or more costs:

- production startup performs route registration, schema compilation, plugin resolution, or application evaluation;
- frontend type safety depends on importing a large server implementation type;
- plugin and middleware order can silently alter behavior;
- general Node.js compatibility expands binary size and runtime surface;
- framework benchmarks omit process startup, first validated request, idle memory, and type-check cost.

Elysia 2 substantially improves this area through modularity and build-time AOT. Project Q starts from that stronger baseline rather than comparing itself with older framework behavior.

## Product hypothesis

A static route contract plus a small interpreter can outperform a general JavaScript runtime in cold-start and idle-memory-sensitive workloads while retaining excellent TypeScript ergonomics.

The hypothesis is valid only if the Rust-to-QuickJS bridge remains bounded and measurable.

## Primary users

- TypeScript developers deploying APIs to serverless, scale-to-zero, CLI, edge-like, or bursty environments.
- Platform teams that need deterministic route inventories, typed client contracts, and semantic API diffs.
- Framework and library authors who want a minimal native capability interface without Node.js compatibility obligations.
- Teams already comfortable with Elysia-style schema-first routes and Eden Treaty-style clients.

## Stakeholder outcomes

### Application developer

- Write ordinary TypeScript business logic.
- Receive inferred request and response types from schemas.
- Use feature-based modules and framework-independent services.
- Get clear build errors for unsupported runtime APIs and dynamic contracts.

### Client developer

- Navigate an object-like API with autocomplete.
- Receive typed path, query, header, and body inputs.
- Narrow success and failure values by HTTP status.
- Consume either source types or a published compact contract package.

### Platform engineer

- Inspect all routes, policies, capabilities, response statuses, and owners.
- Enforce security coverage and semantic contract checks in CI.
- Ship a reproducible artifact with pinned runtime and engine metadata.
- Measure cold start, memory, and bridge cost with transparent tooling.

## Constraints

1. The first implementation is Rust-only on the native side.
2. Production execution uses QuickJS-NG initially, behind an engine adapter.
3. Bun is not the production engine.
4. No full Node.js, Bun runtime, Express, or Elysia compatibility is promised.
5. Static route and schema metadata are required for optimized production builds.
6. Dynamic route creation is outside the first public contract.
7. Performance claims require fair, published, reproducible evidence.
8. Same-process QuickJS execution is for trusted application code, not hostile multi-tenant code.

## Definition of success

The project earns continuation beyond M2 only when evidence demonstrates all of the following:

- process-to-first-response cold start is materially better than Elysia 2 AOT on matched fixtures;
- idle memory is lower on the same reference platform;
- the empty QuickJS handler boundary does not consume an unacceptable share of request cost;
- a 1,000-route application does not scale startup linearly through route registration or schema compilation;
- Treaty-style typing remains responsive at large route counts;
- the build output and runtime contract are deterministic and diagnosable;
- unsupported claims and compatibility gaps are explicit.

Failure to meet one target does not automatically terminate the project. It requires an ADR explaining whether the architecture, scope, or positioning changes.
