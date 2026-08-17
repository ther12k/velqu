---
type: Design Session Record
title: Project Q Design Session Decisions
description: Owner goals, QuickJS selection, Elysia/Eden priorities, retained ideas,
  corrections, risks, and requested handoff.
tags:
- design-session
- provenance
- quickjs
- elysia
- eden
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
---

# Purpose

This record preserves the owner direction and architectural corrections that led to the current bundle.

# Initial product question

The owner wanted to create a new backend stack from scratch, taking the strongest parts of Elysia 2 while retaining a minimal feature set and producing defensible advantages rather than a smaller clone.

# QuickJS direction

The owner then selected QuickJS as the application execution engine and proposed a hybrid architecture:

```text
Rust or Zig
  → TCP, HTTP, routing, parsing, validation, native operations

QuickJS
  → JavaScript/TypeScript business logic
```

The design used AWS LLRT as an existence proof for embedding QuickJS behind native Rust capabilities while deliberately avoiding a full Node compatibility project.

# Product priority correction

The owner clarified the primary outcome:

1. fast cold start;
2. Elysia best practices;
3. Eden Treaty-style end-to-end client typing;
4. selected best ideas from Elysia 2.

This changed the design from a generic “faster HTTP server” into a cold-start-first compiled contract framework/runtime.

# Agreed working direction

```text
Bun for development/package/test/build workflow
Rust for production host and native infrastructure
QuickJS-family engine for trusted TypeScript business logic
static route/schema/policy compiler
Treaty-style typed client
no full Node/Bun/Express/Elysia compatibility
```

# Elysia ideas selected

- schema and contract before handler;
- schema-derived types and documentation;
- feature-based modules;
- service logic decoupled from framework context;
- typed policy/macro-like context;
- status-specific responses;
- AOT/precomputation;
- modular/tree-shakable capabilities;
- explicit lifecycle responsibilities;
- `defer()`-style after-response work;
- Eden Treaty object navigation and status-aware errors.

# Improvements over direct imitation

- compiler does not dry-run application services;
- Rust routes before JavaScript;
- request values are lazy native-backed handles;
- source and published Treaty contract modes;
- route/security/capability manifest as a first-class artifact;
- explicit JavaScript validation/wrapper fallbacks;
- native JSON strategy decided by measured total conversion cost;
- local unit dispatcher separated from production runtime conformance;
- no broad runtime compatibility promise.

# Critical design challenges recorded

- QuickJS boundary cost;
- JSON parse/object conversion/serialization;
- async promise integration and cancellation races;
- request-handle lifetime safety;
- source maps;
- TypeScript scaling;
- complete cold-start measurement;
- capability creep;
- same-process isolation limitations;
- compiler implementation complexity.

# Owner request for this bundle

The owner asked for:

- a renewed architecture review;
- Google Open Knowledge Format documentation;
- a PRD;
- sufficient implementation detail;
- a handoff package and prompt for an AI agent to develop the framework.

# Trust status

This record documents conversation-derived direction. It is not implementation evidence and does not claim measured performance.
