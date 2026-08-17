---
type: Protocol Specification
title: Routing and HTTP Semantics
description: Native route grammar, precedence, method behavior, body admission, static
  bypass, and HTTP security rules.
tags:
- routing
- http
- protocol
- native
- manifest
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
sources:
- id: hyper
  resource: https://docs.rs/hyper
  title: hyper Rust HTTP library
---

# Purpose

Routing and HTTP semantics remain native, deterministic, and intentionally smaller than a general Node.js server API.

# Protocol scope

M1 supports:

- HTTP/1.1;
- keep-alive;
- request-target parsing;
- methods `GET`, `HEAD`, `POST`, `PUT`, `PATCH`, `DELETE`, and `OPTIONS`;
- static paths;
- named path parameters;
- one terminal wildcard form;
- content-length and chunked request bodies as supported by the selected HTTP library;
- graceful connection shutdown;
- ordinary text, JSON, bytes, and empty responses.

HTTP/2, HTTP/3, WebSocket, SSE, trailers, automatic compression, and proxy-protocol support are deferred unless needed for the proof.

# Route declaration

A route has at least:

```ts
route({
  id: "users.get",
  method: "GET",
  path: "/users/:id",
  handle
});
```

The route ID is stable product identity. The method/path pair is transport identity.

# Canonical path grammar

Initial grammar:

```text
/                    root
/users               static segment
/users/:id           named parameter
/assets/*path        terminal wildcard
```

Rules:

- paths begin with `/`;
- empty interior segments are rejected;
- parameter and wildcard names use a documented ASCII identifier grammar;
- a wildcard is terminal;
- duplicate parameter names in one route are rejected;
- percent-decoding behavior is defined once and tested;
- malformed encodings fail rather than being normalized inconsistently;
- dot-segment handling follows the chosen URI policy and is fixture-backed;
- matching is performed against the normalized path, never the query string.

# Precedence

Expected precedence:

```text
static segment
> parameter segment
> terminal wildcard
```

Precedence does not excuse ambiguous route sets. The compiler reports shadowing and requires the developer to resolve it where a more general route can make intent unclear.

# Method handling

- `HEAD` MAY use a declared `HEAD` handler or a documented `GET` fallback with the body suppressed.
- A matched path with an unsupported method returns native `405` and an `Allow` header.
- An unmatched path returns native `404` without entering QuickJS, unless an explicit JavaScript fallback route is configured.
- Automatic `OPTIONS` is optional and generated only from a clear policy.
- Method override headers or form fields are not supported in the core.

# Route manifest

Example:

```json
{
  "id": "users.get",
  "method": "GET",
  "path": "/users/:id",
  "canonicalPath": "/users/:_",
  "handlerId": 17,
  "pipelineId": 9,
  "input": {
    "paramsSchemaId": 3
  },
  "responses": {
    "200": 11,
    "401": 20,
    "404": 21
  },
  "policies": ["auth.session"],
  "capabilities": ["service.users"],
  "fallbacks": []
}
```

# Request body admission

The route manifest declares whether the route accepts a body and its maximum bytes. The host should reject oversized input before constructing a JavaScript body value.

Content type dispatch is explicit:

```text
application/json
text/plain
application/octet-stream
multipart/form-data — later adapter
application/x-www-form-urlencoded — P1 if required
```

Unexpected content type returns a typed `415` when declared by the framework contract.

# Query and headers

Query parsing and header lookup are lazy. The route contract may request a known subset so the runtime can avoid materializing unrelated values.

Headers are case-insensitive by HTTP semantics while preserving a documented representation. Duplicate-header behavior is specified per header category; unsafe ambiguous cases fail or remain raw rather than being joined blindly.

# Cookies

Cookie parsing is optional route capability. Signed/encrypted cookies are separate capabilities. The core does not make cookies globally available if no route uses them.

# Static responses

The compiler may lower a route with a build-time constant response into a native static response:

```ts
route({
  id: "health.live",
  method: "GET",
  path: "/health/live",
  static: {
    status: 200,
    body: "ok"
  }
});
```

Static bypass must preserve headers, observability, limits, and contract reporting. It may not be used to inflate comparisons that otherwise require JavaScript business logic.

# Raw fallback

An explicit raw route can receive a Web-compatible request or native-backed raw context:

```ts
rawRoute({
  id: "legacy.proxy",
  method: "ANY",
  path: "/legacy/*path",
  handle(request) { ... }
});
```

Raw routes:

- are clearly labeled in inspection output;
- may have weaker generated Treaty/OpenAPI support;
- pay wrapper/materialization costs;
- do not silently weaken normal route contracts.

# Security invariants

- response header values are validated before write;
- request smuggling behavior follows the underlying HTTP implementation and deployment guidance;
- body, header, URI, route, and queue limits are bounded;
- path parameters do not become filesystem paths without separate validation;
- host/proxy trust is explicit;
- client IP extraction is disabled unless trusted proxy configuration is supplied;
- malformed requests never reach application JavaScript as partially parsed values.

# Acceptance fixtures

At minimum:

- static, parameter, wildcard, root;
- 404 and 405;
- `HEAD` behavior;
- duplicate canonical parameter routes;
- static-over-parameter precedence;
- parameter-over-wildcard precedence;
- malformed percent encoding;
- query does not affect route match;
- oversized headers/body;
- wrong content type;
- keep-alive sequence;
- cancellation during body read;
- graceful shutdown with in-flight request.
