---
type: User Experience Specification
title: Personas and User Journeys
description: Target users, needs, critical workflows, and developer experience requirements.
tags:
- personas
- developer-experience
- platform
- serverless
- client
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
---

# Personas

## Persona A — TypeScript API developer

**Context:** Builds small and medium APIs, knows Bun and Elysia, wants strong typing without managing Rust.

**Needs:**

- fast feedback and clear route syntax;
- inferred request and response types;
- predictable validation and errors;
- easy service tests;
- minimal deployment artifact.

**Critical journey:**

```text
create project
→ define schema
→ define route and handler
→ run bun tests
→ use Treaty client
→ inspect build report
→ build release artifact
→ run native integration tests
→ deploy
```

## Persona B — Platform engineer

**Context:** Owns many internal services and CI policy.

**Needs:**

- complete route inventory;
- unauthenticated-route detection;
- semantic API diff;
- capability and compatibility manifests;
- reproducible benchmarks and builds;
- explicit startup and memory budgets.

**Critical journey:**

```text
q inspect
→ q contract diff origin/main
→ q verify
→ review native/JS stage report
→ approve artifact metadata
→ deploy canary
```

## Persona C — Serverless and scale-to-zero developer

**Context:** Pays latency and cost on frequent cold starts.

**Needs:**

- one-worker minimum profile;
- no route/schema compilation at process start;
- lazy nonessential services;
- low idle memory;
- reliable first-request metrics.

**Critical journey:**

```text
select serverless profile
→ mark services lazy/eager explicitly
→ build engine-pinned artifact
→ run cold-start suite
→ deploy with measured readiness semantics
```

## Persona D — Client application developer

**Context:** Consumes an API from a browser, mobile TypeScript app, worker, or another service.

**Needs:**

- route autocomplete;
- typed body, params, query, and headers;
- status-narrowed errors;
- small client runtime;
- contract package independent of server internals.

**Critical journey:**

```text
install published contract
→ create Treaty client
→ call typed route
→ switch on error.status
→ upgrade contract package
→ resolve semantic diff warnings
```

## Persona E — Runtime contributor

**Context:** Works in Rust, QuickJS bindings, compilers, or protocol internals.

**Needs:**

- isolated crates and ownership boundaries;
- exact performance harnesses;
- no public API pressure during prototypes;
- fuzz targets and bridge invariants;
- engine abstraction and pinned fixtures.

# Journey requirements

## First route

The default template must demonstrate:

- a static plaintext route;
- a path-param route;
- a validated JSON POST;
- one status-specific problem;
- one policy-provided context value;
- a Treaty client test.

## Debugging a contract error

A developer who creates a dynamic path or unsupported schema transform must receive:

```text
error code
source file and range
what cannot be statically represented
why optimized release mode requires it
a supported rewrite
an explicit fallback only when one exists
```

## Debugging runtime performance

A route inspection should show:

```text
route id
native route match
request fields materialized
validation strategy
policy stages
QuickJS calls
response serializer
linked capabilities
expected fallback costs
```

## Publishing a client contract

The producer must be able to emit a package containing only:

- flattened route contract declarations;
- client runtime or factory;
- contract metadata and hash;
- optional OpenAPI;
- no application service or handler implementation.

# Accessibility and usability

This is primarily a developer tool, but usability still includes:

- machine-readable errors alongside human output;
- stable error codes;
- terminal output that remains understandable without color;
- deterministic commands for AI agents and CI;
- documentation examples that compile;
- no hidden network access during build.
