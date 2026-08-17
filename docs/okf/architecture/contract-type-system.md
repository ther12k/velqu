---
type: Architecture Specification
title: Contract Type System
description: Canonical route contracts, policy composition, status result typing,
  compact declarations, OpenAPI, and contract locks.
tags:
- contracts
- typescript
- types
- policy
- semantic-diff
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
sources:
- id: eden-treaty
  resource: https://elysiajs.com/eden/treaty/overview
  title: Eden Treaty overview
- id: elysia-2
  resource: https://elysiajs.com/blog/elysia-20
  title: Elysia 2 beta announcement and AOT design
- id: elysia-best-practice
  resource: https://elysiajs.com/essential/best-practice
  title: Elysia best-practice guide
---

# Purpose

The contract type system preserves Elysia/Eden-style end-to-end inference without making the entire production implementation or one enormous fluent generic chain the client API.

# Canonical route contract

Conceptual normalized TypeScript type:

```ts
type ApiContract = {
  "users.get": {
    method: "GET";
    path: "/users/:id";
    input: {
      params: {
        id: string;
      };
      query: {};
      headers: {
        authorization?: string;
      };
      body: never;
    };
    response: {
      200: User;
      401: UnauthorizedProblem;
      404: UserNotFoundProblem;
    };
  };
};
```

The contract is keyed internally by stable route ID. Treaty exposes object-like path navigation derived from path segments.

# Type derivation

The authoring schema provides:

```text
Static<typeof Schema>
Input<typeof Schema>
Output<typeof Schema>
```

These may differ where explicit coercion exists. For example, a query integer accepts a string transport representation but gives the handler a number.

# Policy composition

A policy declares:

```ts
definePolicy({
  id: "auth.session",
  needs: {
    headers: s.object({
      authorization: s.optional(s.string())
    })
  },
  provides: {
    session: Session
  },
  response: {
    401: UnauthorizedProblem
  },
  resolve
});
```

Applying the policy to a route:

- merges required transport input;
- adds `session` to handler context;
- adds `401` to the route response union;
- adds capability dependencies;
- preserves the policy in route/security manifests.

Conflicts in provided context names or incompatible input schemas fail compilation.

# Result typing

Handler result:

```ts
type RouteResult<R> =
  | SuccessResult<R>
  | DeclaredProblemResult<R>
  | AllowedRawResult<R>;
```

Examples:

```ts
return user;                    // default declared 200
return status(201, created);    // declared 201
return UserNotFound({ ... });   // declared 404
```

Returning `status(418, ...)` on a route without `418` is a TypeScript and compiler error where statically visible, plus a runtime contract error as defense in depth.

# TypeScript scaling

The design avoids requiring clients to import:

- the Rust runtime;
- handler implementations;
- service implementations;
- compiler internals;
- a whole framework instance's accumulated generic type.

Two outputs support scale:

1. source contract mode from static authoring declarations;
2. generated compact `contract.d.ts`.

Benchmark fixtures measure TypeScript check time and declaration size at 100, 500, and 1,000 routes.

# Contract serialization

`contract.json` contains transport-relevant, language-neutral metadata:

```json
{
  "version": 1,
  "hash": "sha256:...",
  "routes": [
    {
      "id": "users.get",
      "method": "GET",
      "path": "/users/:id",
      "inputs": {
        "params": "schema:3"
      },
      "responses": {
        "200": "schema:11",
        "401": "problem:2",
        "404": "problem:3"
      }
    }
  ]
}
```

It does not contain executable handler code.

# OpenAPI relationship

OpenAPI is a projection of the normalized contract, not a separate source of truth. Features without an exact OpenAPI representation receive documented extensions or warnings; the runtime contract remains authoritative.

# Contract lock

`contract.lock.json` stores the previous accepted public contract with:

- route identities;
- schema identities;
- policy/security metadata;
- deprecation;
- ownership;
- compatibility policy version.

CI can run:

```text
q contract diff --against contract.lock.json
q contract verify
```

# Security inventory

The same contract enables questions such as:

```text
Which routes have no authentication policy?
Which routes can return 500 only?
Which routes accept bodies above 1 MiB?
Which routes use raw Request/Response fallbacks?
Which routes link filesystem or outbound-network capabilities?
Which routes introduced a new 401/403/404 shape?
```

This is a key operational advantage, not an optional documentation feature.

# Limits

The type system does not attempt to prove arbitrary business logic correct. It guarantees the declared boundary:

- what transport input is accepted;
- what normalized values reach the handler;
- what typed context policies provide;
- what status/body pairs may leave the route;
- what Treaty clients can send and receive.

Runtime/database domain invariants remain application responsibilities.
