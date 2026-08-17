---
type: Architecture Specification
title: Lifecycle, Policies, Modules, and Plugins
description: Compiled lifecycle stages, typed policies, interceptors, module scope,
  plugin identity, and defer semantics.
tags:
- lifecycle
- policy
- plugin
- module
- defer
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
sources:
- id: elysia-lifecycle
  resource: https://elysiajs.com/essential/life-cycle
  title: Elysia lifecycle
- id: elysia-plugin
  resource: https://elysiajs.com/essential/plugin
  title: Elysia plugin and scope model
- id: elysia-2
  resource: https://elysiajs.com/blog/elysia-20
  title: Elysia 2 beta announcement and AOT design
---

# Purpose

Project Q takes the separation of concerns found in mature typed frameworks, but compiles unused lifecycle stages away and prevents plugins from mutating a live application unpredictably.

# Normalized lifecycle

```text
admit
→ decode
→ validate
→ resolve policies
→ before handler
→ handler
→ after handler
→ encode/map response
→ write response
→ settle/defer
```

Error handling can intercept failures at defined boundaries without becoming an unbounded alternate pipeline.

# Developer-facing primitives

The initial public model has four concepts:

1. **policy** — typed precondition that may add handler context or return a declared response;
2. **interceptor** — wraps execution without adding typed context;
3. **module** — groups routes, prefix, policies, metadata, and capabilities;
4. **defer** — schedules bounded after-response work.

The framework should resist adding many near-duplicate hook names until a real use case proves the need.

# Policy example

```ts
export const sessionRequired = definePolicy({
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

  async resolve({ headers, services }) {
    const session = await services.sessions.verify(
      headers.authorization
    );

    if (!session) {
      return Unauthorized({
        detail: "Authentication is required"
      });
    }

    return allow({ session });
  }
});
```

The compiler verifies that:

- provided names are unique or intentionally compatible;
- policy dependencies are acyclic;
- possible response statuses flow into routes;
- required capabilities exist;
- ordering is deterministic;
- an applied policy cannot be lost through grouping.

# Policy graph

Policies may require capabilities or other policies:

```ts
definePolicy({
  id: "auth.admin",
  requires: [sessionRequired],
  provides: {
    administrator: Administrator
  },
  ...
});
```

The compiler topologically sorts the graph and reports a cycle with the complete dependency path.

# Native lowering

A policy MAY provide a native lowering for known operations such as:

- CORS decision;
- static bearer-token extraction;
- request-size override;
- fixed rate-limit rule;
- API key hash verification;
- cache precondition.

Native lowering is versioned and conformance-tested against the policy semantics. JavaScript policy implementation remains the fallback.

Inspection output makes the execution location explicit.

# Interceptor example

```ts
const tracing = defineInterceptor({
  id: "observability.trace",

  async around(invocation, next) {
    const span = startSpan(invocation);

    try {
      const result = await next();
      span.recordStatus(result.status);
      return result;
    } finally {
      span.end();
    }
  }
});
```

Interceptors should be rare on the hot path because each JavaScript wrapper can add calls and promise work. Build reports list interceptor depth.

# Modules

Feature module:

```ts
export const usersModule = defineModule({
  id: "users",
  prefix: "/users",
  use: [sessionRequired],
  routes: [getUser, createUser]
});
```

Modules can add:

- path prefix;
- route tags/ownership;
- policies;
- interceptors;
- capability requirements;
- nested modules.

Modules cannot silently override route-local request or response schemas.

# Scope rules

Default:

- route hooks/policies are local;
- module policies apply to contained routes;
- global application policies require an explicit global declaration;
- imported modules do not leak decorators/context into siblings;
- plugin identity and configuration are deterministic.

No behavior depends merely on import order.

# Plugin categories

## Compiler plugin

Receives stable compiler extension APIs to:

- recognize additional static declarations;
- normalize an external schema;
- emit generated artifacts;
- contribute diagnostics.

It does not receive arbitrary access to execute the application.

## Runtime capability plugin

Implemented in Rust and exposed through `runtime:*` or `services.*`.

It declares:

```text
name
version
ABI version
configuration schema
required native features
provided methods
permissions
shutdown behavior
```

## Framework module package

Ordinary TypeScript package containing routes, policies, contracts, and services. It compiles through the same public authoring model.

# Plugin identity and deduplication

Identity includes:

```text
plugin ID
semantic version
normalized configuration hash
runtime/compiler ABI
```

Conflicting instances fail instead of using “last registration wins.”

# `defer()`

```ts
async handle({ defer, services }) {
  const result = await services.orders.create(...);

  defer({
    name: "audit-order-created",
    timeoutMs: 250,
    run: () => services.audit.record(result.id)
  });

  return result;
}
```

Semantics:

- begins only after the response is committed or designated ready;
- bounded by count, time, and memory;
- receives cancellation/shutdown semantics;
- errors are logged/observed but do not rewrite the sent response;
- is not guaranteed across process crash;
- must not be advertised as durable queueing.

Cleanup obligations such as releasing a lease are distinct from best-effort deferred work.

# Error handling

Declared problems are values. Unexpected errors pass through:

```text
route-local mapper, if declared
→ module mapper
→ application mapper
→ redacted internal problem
```

A mapper cannot convert arbitrary internal failures to success silently without an explicit policy and diagnostic.

# Acceptance criteria

- policy context and errors appear in handler/Treaty types;
- policy cycles and conflicting providers fail compilation;
- module scope fixtures prove no leakage;
- route ownership/security inventory includes inherited policies;
- native and JavaScript policy implementations share fixtures;
- interceptor depth and JS boundary count appear in reports;
- `defer` executes after response, is bounded, and does not claim durability;
- plugin identity conflicts fail deterministically.
