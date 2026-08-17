---
type: Scope Definition
title: Scope and Non-Goals
description: Authorized M0–M2 scope, deferred capabilities, explicit exclusions, and
  scope-change rules.
tags:
- scope
- mvp
- non-goals
- quickjs
- runtime
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
---

# Scope model

This document separates the full product vision from the first authorized implementation.

# M0–M2 included scope

## Developer contract

- TypeScript route declarations with stable route IDs.
- Static, dynamic-segment, and terminal wildcard paths.
- Params, query, selected headers, JSON body, and typed response schemas.
- Status-specific success and problem response types.
- Feature-based modules and framework-independent services.
- One authentication-style policy in the proof application.

## Compiler

- Deterministic route extraction without executing application setup.
- Route conflict and unsupported dynamic-contract diagnostics.
- Bundled JavaScript suitable for QuickJS.
- Route and schema manifest output.
- Compact TypeScript contract output.
- Treaty client metadata.
- Build report showing native and JavaScript stages.

## Native runtime proof

- Rust HTTP/1.1 server.
- One QuickJS runtime and context.
- Cached handler references.
- Text, primitive JSON, structured JSON, and typed problem responses.
- Timer or equivalent asynchronous native operation.
- Memory, stack, body-size, queue, and execution-time controls.
- Graceful shutdown and structured diagnostics.

## Treaty proof

- Object-like typed client.
- Typed path, query, header, and body inputs.
- Status-aware success/error union.
- Remote HTTP mode.
- Fast unit mode clearly labeled as non-native-runtime conformance.
- Native integration test mode through the built runtime.

## Evidence

- Raw Rust, raw Bun, Elysia 2 AOT, and Project Q matched fixtures.
- Process-to-ready and process-to-first-response measurements.
- Route-count scaling.
- Idle and peak memory.
- Bridge microbenchmarks.
- Type-check and client inference measurements.
- Reproducible commands and machine-readable reports.

# Deferred until the core thesis passes

- Multi-worker and adaptive worker pools.
- HTTP/2 and TLS termination.
- WebSockets, SSE, and full streaming.
- Multipart upload.
- Standard Schema adapters beyond the built-in schema DSL.
- Native database, cache, queue, filesystem, and telemetry packages.
- Published package ecosystem.
- Serverless provider-specific deployment adapters.
- Source-level debugging beyond basic source maps.
- Signed application packs.
- Process-level tenant isolation.

# Explicit non-goals

- Full Node.js compatibility.
- Full Bun runtime compatibility.
- Drop-in Elysia, Express, Fastify, Hono, or Nest compatibility.
- CommonJS support.
- Running arbitrary npm packages without a compatibility declaration.
- Runtime route registration in optimized release mode.
- Runtime plugin discovery.
- ORM, authentication product, job system, template engine, or frontend framework in core.
- Rewriting TLS, cryptography, HTTP parsing, DNS, or database protocols from scratch.
- Claiming a secure sandbox for hostile code in the same process.
- Beating JIT engines on CPU-heavy JavaScript loops.
- Winning every throughput benchmark.
- Publishing a public repository or selecting a final license without owner approval.

# Scope-change rule

Any material expansion before M2 exit requires:

1. a stated proof-app need;
2. an ADR or updated requirement;
3. tests and benchmark implications;
4. explicit evidence that it does not obscure the primary bridge and cold-start questions.

Deletion of speculative scope is preferred over adding an abstraction merely because another framework has it.
