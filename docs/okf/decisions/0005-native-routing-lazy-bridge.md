---
type: Architecture Decision
title: 'ADR-0005: Native Routing and Lazy Bridge'
description: Selects Rust route dispatch and demand-driven request materialization
  while keeping Web wrappers explicit.
tags:
- adr
- routing
- bridge
- lazy
- http
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
---

# ADR-0005: Native Routing and Lazy Request Bridge

## Decision state

Proposed baseline.

## Context

A universal JavaScript `fetch(Request)` handler is elegant but can force routing and Web object allocation through JavaScript. Conversely, eagerly converting every parsed Rust value into QuickJS objects can erase native advantages.

## Decision

Rust performs method/path routing before JavaScript. The handler receives native-backed lazy request access. Only fields used by the route/policy/handler are materialized.

A full Web `Request`/`Response` path exists only as an explicit fallback.

## Consequences

Positive:

- native 404/405/static route behavior;
- fewer unnecessary allocations;
- direct route/pipeline ID dispatch;
- visible request materialization.

Negative:

- bridge and lifetime safety are complex;
- exact Web compatibility is not automatic;
- native object access itself must be benchmarked.

## Important qualification

Native JSON parsing/validation is not required by this decision. Parsing and response strategies are selected through end-to-end bridge evidence.

## Rejected alternatives

- one global JavaScript fetch router as the only API;
- eager full request conversion;
- duplicate Rust and JavaScript routers;
- invisible fallback to generic wrappers.

## Validation

Bridge microbenchmarks, handle-lifetime tests, full wrapper fallback fixtures, and route inspection must pass.
