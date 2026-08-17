---
type: Architecture Specification
title: Project Q Architecture Overview
description: System context, layers, request flow, deployment profiles, and architectural
  invariants.
tags:
- architecture
- overview
- rust
- quickjs
- typescript
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
- id: quickjs
  resource: https://bellard.org/quickjs/quickjs.html
  title: QuickJS documentation
---

# System definition

Project Q is a **compiled TypeScript server framework and purpose-built runtime**.

It has four cooperating products:

```text
@q/core        authoring contracts and framework primitives
q compiler     static analysis, schema IR, manifests, client/docs output
q runtime      Rust HTTP host plus QuickJS-family JavaScript execution
@q/treaty      typed local and remote client
```

The package names are placeholders.

# Context diagram

```text
Developer
   │
   ├── Bun package manager, tests, scripts, editor
   │
   └── TypeScript source
            │
            ▼
      Project Q compiler
            │
            ├── bundled ESM/source or engine bytecode
            ├── route manifest
            ├── schema/serializer instructions
            ├── policy/capability graph
            ├── compact Treaty contract
            ├── OpenAPI
            ├── semantic contract lock
            └── source maps/build report
                         │
                         ▼
                 Project Q application pack
                         │
                         ▼
               Rust production runtime
                         │
                         ├── HTTP transport
                         ├── native route dispatch
                         ├── request limits
                         ├── native/lazy bridge
                         ├── QuickJS handler execution
                         ├── selected capabilities
                         └── response encoding
```

# Production request flow

```text
socket accepted
  → request head/body limits
  → canonical route match
  → route pipeline lookup
  → required decode/materialization
  → validation and native policies, when compiled
  → JavaScript policies, when required
  → cached JavaScript handler
  → typed result mapping
  → response serialization/write
  → after-response cleanup/defer
```

A route pays only for stages present in its compiled pipeline.

# Authoring model

The public API is intentionally declarative:

```ts
import { defineApp, route, s, status } from "@q/core";

const User = s.object({
  id: s.uuid(),
  name: s.string({ minLength: 1, maxLength: 100 })
});

export const app = defineApp({
  modules: [
    usersModule
  ]
});

export const getUser = route({
  id: "users.get",
  method: "GET",
  path: "/users/:id",

  params: s.object({
    id: s.uuid()
  }),

  response: {
    200: User,
    404: UserNotFound
  },

  async handle({ params, services }) {
    const user = await services.users.findById(params.id);

    if (!user) {
      return status(404, {
        type: "urn:q:problem:user-not-found",
        title: "User not found",
        status: 404,
        userId: params.id
      });
    }

    return user;
  }
});
```

The exact syntax is provisional. The normative requirements are:

- contract metadata precedes the handler;
- method, path, schemas, policies, and statuses are explicit;
- handler bodies remain normal TypeScript;
- route definitions are statically discoverable;
- services are not initialized during compilation;
- handler results are status-aware and type checked.

# Layer boundaries

## Compiler boundary

The compiler owns application structure but does not own business execution.

It MAY parse:

- imports;
- calls to recognized static authoring primitives;
- literal object/array metadata;
- schema declarations representable in the IR;
- route/module composition;
- policy and capability references.

It MUST NOT execute:

- handlers;
- service factories;
- top-level application side effects;
- arbitrary plugin code to discover routes;
- environment-dependent registration.

## Runtime boundary

Rust owns:

- network acceptance and HTTP state;
- route matching;
- resource admission;
- request body limits;
- native handles and lifetimes;
- worker queues;
- selected native capabilities;
- response write and shutdown.

QuickJS owns:

- trusted application business logic;
- JavaScript policy logic not compiled natively;
- ordinary object/array/string computation;
- promises and application-level composition.

## Contract boundary

One normalized route contract drives:

- server input/output types;
- compiler diagnostics;
- runtime manifest;
- validation and serialization strategy;
- Treaty client types;
- OpenAPI;
- semantic API diff;
- route/security inventory.

# Deployment profiles

## `serverless`

- one initial JavaScript worker;
- smallest linked capability set;
- no eager external service connections;
- strict package and initialization budgets;
- process-to-first-response prioritized.

## `service`

- one initial worker;
- optional adaptive additional workers;
- persistent service lifecycle;
- steady-state throughput and queueing controls;
- optional explicit warm-up.

## `isolate` — future

- process or stronger operating-system isolation;
- intended for untrusted or multi-tenant scripts;
- excluded from initial implementation.

# Public compatibility promise

The first product is not a Node.js runtime. Its compatibility layers are:

1. ECMAScript supported by the selected QuickJS engine;
2. a documented minimal Web-compatible API subset;
3. Project Q-native runtime capabilities;
4. explicit build-time errors for unsupported imports/APIs.

Compatibility is versioned and machine-readable.

# Architecture invariants

1. Production startup performs no route or schema compilation.
2. Rust routes before JavaScript.
3. Handler references are cached.
4. Request materialization is demand-driven.
5. Every route has a stable ID and canonical path.
6. Every status/body pair is declared or rejected.
7. Policy dependencies are explicit.
8. Compiler and runtime versions are recorded in the application pack.
9. Bytecode is version-matched and trusted.
10. Build reports expose fallbacks and JavaScript boundary calls.
11. Local test shortcuts are not mislabeled as runtime conformance.
12. Performance assertions are evidence-linked.
