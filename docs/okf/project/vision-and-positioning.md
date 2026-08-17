---
type: Product Strategy
title: Vision and Positioning
description: Product category, market position, relationship to Bun, Elysia, and LLRT,
  and allowed claims.
tags:
- positioning
- strategy
- elysia
- llrt
- bun
- cold-start
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
stale_after: '2026-11-17'
sources:
- id: bun-docs
  resource: https://bun.sh/docs
  title: Bun documentation
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

# Product category

Project Q is a **compiled TypeScript server framework and micro-runtime**.

It is not merely:

- a QuickJS embedding;
- a Rust web framework with JavaScript callbacks;
- a Bun framework;
- an Elysia fork;
- or an LLRT-compatible runtime.

Its product surface includes the application contract, compiler, native host, JavaScript execution environment, typed client, testing model, and operational manifests.

# Positioning statement

For TypeScript teams deploying cold-start-sensitive APIs, Project Q provides Elysia-inspired schema-first development and an Eden Treaty-style client while compiling production applications into a minimal Rust + QuickJS runtime. Unlike general Node-compatible runtimes or conventional JavaScript routers, Project Q performs route discovery, schema preparation, plugin resolution, and contract generation before production startup.

# Product wedge

The initial wedge is not maximum requests per second. It is the combined experience of:

1. **Fast process-to-first-validated-response**
2. **Low idle memory**
3. **Deterministic static contracts**
4. **Status-aware end-to-end typing**
5. **Operational route and capability visibility**
6. **A deliberately small compatibility surface**

This combination matters most for:

- serverless functions with frequent scale-to-zero;
- short-lived workers and CLI-backed APIs;
- internal services with many routes but modest JavaScript CPU work;
- multi-service platforms where API drift and security coverage are costly;
- trusted plugin or automation execution requiring bounded runtimes.

# Relationship to Bun

Bun remains central to the developer workflow:

```text
bun install
bun test
bun run q dev
bun run q build
```

However, the deployed process is Project Q's runtime. Describing it as “Bun-powered production execution” would be inaccurate because Bun uses JavaScriptCore while Project Q executes application JavaScript in QuickJS.

The preferred wording is:

> Bun-first tooling, Rust-hosted production, QuickJS-executed business logic.

# Relationship to Elysia 2

Project Q intentionally learns from:

- AOT compilation;
- modular and tree-shakable features;
- schema-as-source-of-truth;
- typed status responses;
- feature-based modules;
- local plugin lifecycle;
- lifecycle separation;
- deferred post-response work;
- and Eden Treaty.

It differs by making static compilation mandatory for the optimized release profile, using a smaller JavaScript engine, owning a native route and capability host, and emitting a compact client contract independent of the full server implementation.

# Relationship to LLRT

LLRT proves that a Rust + QuickJS runtime with selected native APIs is practical and useful for serverless workloads. It also demonstrates the maintenance burden of compatibility layers.

Project Q therefore rejects the goal of running arbitrary Node frameworks. It exposes only:

- a small Web-compatible baseline where useful;
- Project Q framework APIs;
- explicitly linked native capabilities.

# Claims policy

Allowed before benchmarks:

- “designed for fast cold start”;
- “uses build-time route and schema metadata”;
- “provides Treaty-style typed clients”;
- “does not target full Node compatibility.”

Not allowed before evidence:

- “faster than Elysia”;
- “starts in one millisecond”;
- “uses less memory than Bun”;
- “zero-copy JSON”;
- “secure sandbox”;
- “supports npm packages.”

# Long-term vision

If the first architecture survives measurement, Project Q can become a portable execution and contract layer with:

- native database and queue capabilities;
- HTTP streaming, SSE, and WebSockets;
- serverless adapters;
- signed application packs;
- stronger process isolation;
- a compatibility-tested package catalog;
- and language-neutral clients generated from the same contract.

These remain roadmap possibilities, not v0.1 promises.
