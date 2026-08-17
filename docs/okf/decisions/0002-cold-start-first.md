---
type: Architecture Decision
title: 'ADR-0002: Cold-Start-First Product Priority'
description: Defines complete cold start as the primary product performance objective
  and constrains comparisons.
tags:
- adr
- cold-start
- performance
- serverless
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
sources:
- id: elysia-2
  resource: https://elysiajs.com/blog/elysia-20
  title: Elysia 2 beta announcement and AOT design
- id: quickjs
  resource: https://bellard.org/quickjs/quickjs.html
  title: QuickJS documentation
---

# ADR-0002: Cold-Start-First Product Priority

## Decision state

Proposed baseline.

## Context

A new framework cannot credibly promise universal speed. QuickJS is selected for small runtime characteristics, while Bun/JavaScriptCore may be stronger for long CPU-heavy JavaScript.

A clear optimization hierarchy is required.

## Decision

The primary performance objective is:

```text
complete process-to-first validated/policy response
```

Secondary objectives:

- idle memory;
- predictable resource limits;
- low route-count sensitivity;
- acceptable warm API latency;
- typed client and contract integrity.

Peak JavaScript loop throughput is not a primary objective.

## Measurement

Cold start includes process spawn, pack verification, engine/application load, handler caching, bind, request dispatch, validation/policy, and response.

Primary comparison uses matched validated and policy-protected routes, not only static responses.

## Consequences

- production compilation and lazy services are central;
- one initial worker is favored;
- broad compatibility is rejected when it harms startup;
- features must report startup and artifact impact;
- a feature can be optional even when common in Node runtimes.

## Rejected alternatives

- maximize requests per second at any startup/memory cost;
- quote bare engine initialization as application cold start;
- optimize only serverless and ignore service-mode correctness;
- market static native response numbers as framework performance.

## Validation

The thesis passes only with reproducible p95/p99 evidence against raw Bun and matched Elysia 2 AOT fixtures.
