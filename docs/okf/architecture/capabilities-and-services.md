---
type: Architecture Specification
title: Native Capabilities and Application Services
description: Minimal native API surface, capability manifests, lazy services, permissions,
  and framework-independent business logic.
tags:
- capabilities
- services
- fetch
- crypto
- dependency-management
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
sources:
- id: aws-llrt
  resource: https://github.com/awslabs/llrt
  title: AWS LLRT
- id: elysia-best-practice
  resource: https://elysiajs.com/essential/best-practice
  title: Elysia best-practice guide
---

# Purpose

Capabilities define what the production runtime exposes. Services define application-owned dependencies. Keeping them separate prevents a small framework from becoming a hidden operating system or dependency-injection container.

# Capability principle

```text
no ambient Node compatibility
no accidental filesystem/network authority
link only declared capability modules
report every capability used by every route
```

# Initial native capability set

P0:

```text
runtime:console
runtime:timers
runtime:text
runtime:url
runtime:abort
```

P1 for the first alpha:

```text
runtime:fetch
runtime:crypto
```

Possible later capabilities:

```text
runtime:streams
runtime:filesystem
runtime:tcp
runtime:websocket
runtime:telemetry
runtime:process-info
```

A capability is not included merely because a Node or browser API exists.

# Capability module example

```ts
import { fetch } from "runtime:fetch";
import { randomUUID } from "runtime:crypto";
```

The compiler resolves these imports to manifest requirements. The runtime refuses to start a pack whose required capabilities are unavailable or incompatible.

# Capability manifest

```json
{
  "runtimeAbi": 1,
  "capabilities": [
    {
      "id": "runtime:fetch",
      "version": 1,
      "permissions": {
        "network": ["https"]
      }
    }
  ]
}
```

Future policies may restrict destinations, DNS/IP classes, methods, payloads, or TLS behavior.

# Native `fetch`

The initial outbound fetch capability requires:

- HTTPS and HTTP according to configuration;
- redirects with bounded policy;
- DNS/connect/TLS/read deadlines;
- cancellation via `AbortSignal`;
- response body limit or streaming mode;
- header validation;
- no automatic access to inbound secrets;
- future SSRF allow/deny hooks;
- metrics split by DNS/connect/TLS/first-byte/read.

It does not imply complete browser fetch conformance in v0.1.

# Crypto

Initial subset:

- cryptographically secure random bytes;
- UUID generation;
- digest/hash primitives selected through review;
- constant-time comparison helper where appropriate.

A broader Web Crypto API is added feature-by-feature with conformance and security review.

# Timers

Timers are owned by the worker scheduler and bounded per invocation/application. A timer does not keep a serverless invocation alive indefinitely unless profile policy permits it.

# Console

Structured console output includes:

```text
timestamp
level
application/version
route ID
request/trace ID, when in invocation
source map location, development
message/fields
```

Formatting never reveals sensitive request objects automatically.

# Application services

A service is a typed lifecycle-managed application dependency:

```ts
export const usersService = defineService({
  id: "service.users",

  async create(config, lifecycle) {
    const repository = await createRepository(config);

    lifecycle.onShutdown(() => repository.close());

    return {
      findById: repository.findById,
      create: repository.create
    };
  }
});
```

# Lazy initialization

Default service mode:

```text
declared at build
not created during compile
not created during runtime startup
created on first route demand
cached according to scope
closed during shutdown
```

An application MAY mark a service eager in the `service` profile. The build/runtime report then shows its cold-start impact.

# Service scopes

Initial scopes:

- application singleton;
- invocation/request value;
- policy-provided context.

Per-route or transient service container graphs are deferred.

# Business service best practice

Business logic should remain framework-independent:

```ts
export async function renameUser(
  repository: UserRepository,
  input: RenameUserInput
): Promise<User> {
  return repository.rename(input.userId, input.name);
}
```

Avoid:

```ts
export async function renameUser(ctx: FullFrameworkContext) {
  // HTTP, auth, database, response all coupled.
}
```

Routes adapt transport to ordinary values. Policies adapt request-dependent identity/authorization. Services own domain/infrastructure behavior.

# Database support

No database is part of core. Adapters may expose:

- native Rust driver service;
- JavaScript protocol client over `runtime:tcp` in a later release;
- HTTP database service through `runtime:fetch`;
- application-specific FFI/service.

Oracle, PostgreSQL, Redis, and other systems are optional packages with separate cold-start, pooling, cancellation, and security characteristics.

# Permission model

The application pack lists capabilities. Deployment configuration may further restrict them.

Example:

```text
build requires runtime:fetch
deployment permits:
  DNS: api.internal.example
  ports: 443
  redirects: same-origin only
```

This is a future policy direction; the initial implementation must preserve the seam and avoid hardcoding unrestricted ambient authority.

# Acceptance criteria

- unused capability is absent from the minimal pack/runtime link set where architecture permits;
- unsupported capability fails build or startup clearly;
- service factories never run during compilation;
- lazy service test proves health route does not initialize database-like service;
- shutdown closes initialized services once;
- capability calls respect deadlines/cancellation;
- route inspection shows transitive capabilities;
- secrets are not logged by console/fetch errors.
