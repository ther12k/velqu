---
type: Architecture Specification
title: Treaty-Style Typed Client
description: Object-like remote client, status-aware results, source/published contract
  modes, and local testing modes.
tags:
- treaty
- client
- eden
- typescript
- testing
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
sources:
- id: eden-treaty
  resource: https://elysiajs.com/eden/treaty/overview
  title: Eden Treaty overview
---

# Product objective

Treaty is not a generated REST wrapper with weak strings. It is an object-like, status-aware client that makes the server contract feel local while retaining explicit HTTP semantics.

The package name `@q/treaty` is provisional.

# Remote client example

```ts
import { treaty } from "@q/treaty";
import type { Api } from "@server/contract";

const api = treaty<Api>({
  baseUrl: "https://api.example.com"
});

const result = await api.users({ id }).get({
  query: {
    includeRoles: true
  },
  headers: {
    authorization: token
  }
});

if (result.error) {
  switch (result.error.status) {
    case 401:
      return redirectToLogin();

    case 404:
      return showMissingUser(result.error.value.userId);
  }
} else {
  console.log(result.data.name);
}
```

# Result shape

Default non-throwing result:

```ts
type TreatyResult<Responses> =
  | {
      data: SuccessValue<Responses>;
      error: null;
      status: SuccessStatus<Responses>;
      headers: HeadersLike;
      response?: Response;
    }
  | {
      data: null;
      error: StatusNarrowedError<Responses>;
      status: ErrorStatus<Responses>;
      headers: HeadersLike;
      response?: Response;
    };
```

Network, abort, and client configuration errors are distinct from declared HTTP errors.

Optional throwing behavior MAY be offered as a wrapper, never as the only mode.

# Path navigation

Routes derive an ergonomic tree:

```text
GET /users/:id
→ api.users({ id }).get()

POST /users
→ api.users.post({ body })

GET /organizations/:orgId/users/:userId
→ api.organizations({ orgId }).users({ userId }).get()
```

Route-name conflicts, reserved properties, wildcard paths, and duplicate navigation shapes require deterministic escaping rules and generated tests.

# Two contract modes

## Source mode

```ts
import type { Api } from "../server/src/contract";
```

Benefits:

- rapid monorepo feedback;
- no manual generation command;
- route changes appear in editor types immediately.

Constraints:

- server contract declaration must remain light;
- frontend should not transitively bundle server implementation;
- project references/package boundaries are recommended.

## Published mode

```text
q contract build
```

Output:

```text
@organization/api-contract/
├── contract.json
├── index.d.ts
├── client.js
└── package.json
```

Benefits:

- separate repositories;
- independently versioned API contract;
- smaller type surface;
- no server source import;
- future multi-language generation.

# Local invocation modes

## Unit-local

```ts
const api = treaty.local(appContract, {
  dispatcher: generatedDispatcher
});
```

This bypasses network serialization and MAY run handlers through a JavaScript-local dispatcher.

Use for:

- business route unit tests;
- policy composition tests;
- fast type-safe fixtures.

It does not prove Rust or QuickJS host behavior.

## Runtime-local

```ts
const server = await spawnQRuntime(testPack);
const api = treaty<typeof Api>(server.url);
```

This invokes the real release-like binary over loopback.

Use for:

- bridge conformance;
- request/response serialization;
- cancellation;
- limits;
- native capabilities;
- source-map and error behavior.

# Request encoding

Treaty uses contract metadata to place data correctly:

```ts
api.users({ id }).patch({
  query: { notify: true },
  headers: { "if-match": revision },
  body: { name }
});
```

Rules:

- path parameters are URI-encoded;
- query arrays/null/undefined follow documented schema semantics;
- body content type follows the route contract;
- undeclared input is a TypeScript error and optionally a development diagnostic;
- auth and common headers can be supplied through hooks without erasing route-local types.

# Hooks

Minimal client hooks:

```ts
treaty<Api>({
  baseUrl,
  beforeRequest({ routeId, request }) {},
  afterResponse({ routeId, result }) {},
  onNetworkError(error) {}
});
```

Hooks must not mutate contract types or silently convert declared HTTP errors into success.

# Contract hash

The client MAY send a development or diagnostic header with its contract hash. The server can report mismatch without rejecting compatible traffic by default.

A strict deployment can enforce version compatibility at an API gateway or runtime policy.

# Client runtime budget

The remote client should contain:

- URL/path/query construction;
- fetch adapter;
- request encoding;
- result decoding;
- small hook plumbing.

It does not contain schemas, validators, server handlers, compiler code, or framework runtime by default.

# Client acceptance criteria

- path/query/header/body autocomplete;
- compile-time errors for missing/invalid input;
- status-narrowed declared problems;
- network error distinct from HTTP error;
- source and published mode parity;
- no server implementation imported into published client output;
- wildcard and reserved-segment fixtures;
- unit-local and runtime-local modes clearly labeled;
- browser and server-side fetch adapters tested;
- bundle and TypeScript-check budgets reported.
