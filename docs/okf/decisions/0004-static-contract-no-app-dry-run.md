---
type: Architecture Decision
title: 'ADR-0004: Static Contract Compilation Without App Dry-Run'
description: Requires static route extraction and prohibits executing services or
  app setup during release discovery.
tags:
- adr
- compiler
- aot
- determinism
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
sources:
- id: elysia-2
  resource: https://elysiajs.com/blog/elysia-20
  title: Elysia 2 beta announcement and AOT design
---

# ADR-0004: Static Contract Compilation Without Application Dry-Run

## Decision state

Proposed baseline.

## Context

AOT compilation needs a route and policy graph. Executing application setup at build time can trigger database connections, timers, environment-sensitive behavior, or differences between build and runtime.

## Decision

The release compiler analyzes a constrained static authoring form and does not execute handlers, service factories, or application setup to discover structure.

Dynamic route metadata is rejected in release mode.

## Consequences

Positive:

- deterministic manifests;
- no build-time service side effects;
- source-located diagnostics;
- reproducible route/security inventory.

Negative:

- arbitrary loops and dynamic registrations are not accepted;
- the authoring API must remain compiler-friendly;
- compiler implementation is a significant project.

## Generated-route alternative

A developer may run a separate explicit generator before `q build` to create static source. The generated source then follows ordinary compilation and is tracked as an input.

## Rejected alternatives

- import and execute the app in a build sandbox;
- inspect handler source strings;
- let runtime registration remain the source of truth;
- silently evaluate environment-dependent paths.

## Validation

Fixtures prove that service factories and top-level network helpers are never invoked during compilation, while 1,000 static routes compile deterministically.
