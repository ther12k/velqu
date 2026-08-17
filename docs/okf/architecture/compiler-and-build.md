---
type: Architecture Specification
title: Compiler and Build Architecture
description: Static extraction, normalized IR, deterministic application packs, diagnostics,
  and build outputs.
tags:
- compiler
- aot
- build
- manifest
- determinism
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
sources:
- id: elysia-2
  resource: https://elysiajs.com/blog/elysia-20
  title: Elysia 2 beta announcement and AOT design
- id: bun-docs
  resource: https://bun.sh/docs
  title: Bun documentation
---

# Purpose

The compiler turns static TypeScript framework declarations into a deterministic application pack. It is both a performance mechanism and a contract-governance mechanism.

# Inputs

```text
q.config.ts
src/app.ts
src/modules/**/*
package lockfile
compiler/runtime target
environment-independent build options
```

Configuration may select target profile, capabilities, schema adapters, diagnostics level, and output layout. Secrets and live service credentials are not build inputs.

# Outputs

```text
dist/
├── app.qpack
├── app.js or app.qbc
├── route-manifest.json
├── schema-manifest.json
├── capability-manifest.json
├── contract.json
├── contract.d.ts
├── openapi.json
├── contract.lock.json
├── source-map.json
├── build-report.json
└── build-report.md
```

The pack format MAY combine these resources physically. Their logical identities and hashes remain inspectable.

# Compilation stages

```text
1. configuration validation
2. module graph resolution
3. recognized authoring-form extraction
4. route/module/policy normalization
5. duplicate and shadow analysis
6. schema normalization into IR
7. policy/capability graph resolution
8. handler/module bundle emission
9. route pipeline lowering
10. Treaty/OpenAPI/contract-lock generation
11. artifact hashing and signing metadata
12. reproducibility and unsupported-API report
```

# Static authoring subset

Release compilation accepts constructs that can be interpreted without executing user code:

```ts
const User = s.object({ ... });

export const getUser = route({
  id: "users.get",
  method: "GET",
  path: "/users/:id",
  params: ...,
  use: [sessionRequired],
  response: { ... },
  handle
});

export const users = module({
  prefix: "/users",
  routes: [getUser]
});
```

Allowed metadata SHOULD initially be limited to:

- string, number, boolean, `null`;
- arrays and object literals;
- imported or local references to recognized declarations;
- documented schema combinators;
- simple static composition.

Initially rejected:

```ts
route({
  path: process.env.PREFIX + "/users",
  ...
});

for (const model of await database.models()) {
  registerGeneratedRoute(model);
}
```

A future generated-source mechanism may cover legitimate dynamic inventories while retaining deterministic release output.

# No application dry-run

The compiler SHALL NOT import and execute the application to discover its route tree.

This avoids:

- build-time database or Redis connections;
- timers or workers started during build;
- environment-sensitive route differences;
- accidental file/network side effects;
- handler-source heuristics;
- registration-order surprises.

Plugins that contribute compiler behavior run through an explicit compiler plugin API and receive normalized data, not unrestricted mutation of a live application.

# Canonical route identity

Canonicalization includes:

- uppercase HTTP method;
- normalized slash rules;
- normalized parameter names for collision analysis;
- wildcard position;
- optional trailing-slash policy;
- host or version scope when supported.

The compiler MUST reject:

```text
GET /users/:id
GET /users/:userId
```

as the same canonical route unless an explicit, well-defined disambiguation exists—which is not planned for v0.1.

It MUST diagnose static/wildcard shadowing such as:

```text
GET /assets/*
GET /assets/health
```

according to router precedence rather than silently depending on declaration order.

# Route pipeline IR

Each normalized route becomes an ordered instruction graph:

```text
route_id: users.get
method: GET
path: /users/:id
stages:
  - admit_request
  - read_param id
  - validate uuid
  - run_policy auth.session
  - call_handler 17
  - encode_response response_schema_4
  - run_deferred
```

The initial implementation can represent this as data. Later versions may generate specialized Rust or bridge code where measurement justifies it.

# Handler bundle and references

Handlers are exported through a generated stable table:

```js
export const __q_handlers = [
  health,
  getUser,
  createUser
];
```

The runtime resolves and caches these references once. It does not repeatedly perform module property lookup by path.

# Unsupported API analysis

The compiler maintains a versioned compatibility registry.

Examples of build errors:

```text
QCOMP1201 Unsupported runtime import "node:http".
QCOMP1204 Bun.serve is not available in Project Q production execution.
QCOMP2207 Dynamic import specifier cannot be resolved statically.
QCOMP3102 Route path depends on process.env and is not deterministic.
```

A documented raw/native capability may replace an unsupported API. Diagnostics should link the exact alternative where one exists.

# Reproducibility

For identical source, lockfile, target, compiler/runtime version, and normalized build options, output SHOULD be reproducible.

The report records:

- compiler and runtime versions;
- Bun version used for build;
- QuickJS engine and ABI version;
- Rust target triple;
- lockfile digest;
- application source digest;
- route/schema/capability hashes;
- non-reproducible inputs, if any.

Timestamps intended for humans must not contaminate deterministic binary content unless excluded from the reproducibility comparison.

# Development mode

Development MAY use a less expensive path:

```text
watch source
→ incremental contract extraction
→ bundle source JavaScript with source maps
→ restart or reload development runtime
```

Development convenience must not alter release semantics. A conformance suite verifies that development and release manifests contain the same route contracts.

# Compiler acceptance criteria

- no application service is executed during compilation;
- 25 and 1,000 route fixtures generate deterministic manifests;
- duplicate and shadow fixtures fail with source-located diagnostics;
- unsupported runtime imports fail before packaging;
- route, schema, policy, capability, Treaty, and OpenAPI outputs share stable IDs;
- compiler code is absent from the runtime artifact;
- rebuild of an unchanged fixture produces the same content hashes;
- every JavaScript fallback is listed in the build report.
