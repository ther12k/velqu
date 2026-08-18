# Velqu Application Pack Format — v1 (frozen for M1/M2)

Status: frozen working decision (see `docs/open-decisions.md`). Changing this
format after M1 measurements requires a version bump and an ADR.

The production artifact `app.qpack` is a single UTF-8 JSON file with this shape.
Field order is fixed for deterministic hashing (serialize with sorted keys
where noted). The runtime never compiles routes, schemas, or OpenAPI at
startup; everything below is consumed as-is.

```jsonc
{
  // identity & versions (checked before ready)
  "formatVersion": 1,                 // velqu.qpack format
  "kind": "velqu.qpack",
  "runtimeAbi": 1,                    // q-runtime ABI this pack targets
  "engine": { "name": "quickjs-ng", "version": "0.15.1", "binding": "rquickjs-0.12.2" },
  "schemaIrVersion": 1,
  "contractVersion": 1,
  "contractHash": "sha256:<hex of contract.json canonicalization>",
  "builtBy": { "compiler": "<semver>", "typescript": "<semver>", "bun": "<semver>" },

  // application
  "appId": "proof",
  "modules": ["health", "hello", "users", "async"],
  "entry": "app.js",                  // single bundled application source (below)
  "bundle": "<one JavaScript source string, no CommonJS, no node:/bun: imports>",
  "sourceMap": "<optional source map JSON string for bundle>",

  // routing table (pre-compiled; consumed by the native router)
  "routes": [
    {
      "id": "users.get",              // stable route ID
      "moduleId": "users",
      "method": "GET",                // canonical uppercase
      "path": "/users/:id",           // canonical path; :name = param, *rest = terminal wildcard
      "pathSegments": [               // pre-compiled segments (additive v1 field): the runtime
        { "kind": "static", "value": "users" },   // consumes this with ZERO parsing/compilation
        { "kind": "param", "value": "id" }
      ],
      "handler": "users.get",         // key into handlerTable
      "policy": "auth.session",       // policy id or null
      "params":  { "schema": "sch:users.get.params",  "coerce": "path" },
      "query":   { "schema": "sch:users.get.query",   "coerce": "query" },
      "body":    { "schema": "sch:users.create.body", "contentType": "application/json", "limitBytes": 65536 },
      "headers": { "schema": null, "select": [] },
      "responses": {
        "200": { "schema": "sch:users.get.200", "strategy": "js" },
        "401": { "problem": "unauthorized" },
        "404": { "problem": "not-found" }
      },
      "validationStrategy": "native", // native | js (js = explicit fallback, shown in build report)
      "nativeLiveness": null,         // or { "status": 200, "contentType": "...", "body": "..." } for C0 routes
      "security": [ { "scheme": "bearer", "header": "authorization", "problemStatus": 401 } ],
      "capabilities": [ "timer" ],
      "deadlineMs": 5000
    }
  ],

  // schema IR registry (schemaIrVersion 1)
  "schemas": {
    "sch:users.create.body": {
      "kind": "object",
      "properties": {
        "name":  { "kind": "string", "minLength": 1, "maxLength": 60 },
        "email": { "kind": "string", "format": "email" }
      },
      "required": ["name", "email"]
    }
  },

  // policies
  "policies": {
    "auth.session": {
      "id": "auth.session",
      "handler": "auth.session",
      "declaredStatuses": [401],
      "provides": "session"
    }
  },

  // declared capabilities (only these are linked/allowed)
  "capabilities": ["timer"],

  // handler table: key -> exported binding name in the bundle's registration call
  "handlerTable": { "users.get": "users_get" },

  // integrity (verified before ready)
  "integrity": {
    "algorithm": "sha256",
    "bundleSha256": "<hex>",
    "routesSha256": "<hex over canonical JSON of routes+schemas+policies>"
  }
}
```

## Schema IR v1 subset

```jsonc
{ "kind": "string", "minLength"?: 1, "maxLength"?: 60, "pattern"?: "^usr_[0-9]+$", "format"?: "email" | "uuid" }
{ "kind": "integer", "minimum"?: 0, "maximum"?: 1000 }
{ "kind": "number", "minimum"?: 0, "maximum"?: 0 }
{ "kind": "boolean" }
{ "kind": "literal", "value": "a" }                 // single enum member
{ "kind": "enum", "values": ["a", "b"] }
{ "kind": "optional", "inner": { ... }, "default"?: 10 }  // optional with explicit default
{ "kind": "nullable", "inner": { ... } }
{ "kind": "array", "items": { ... }, "minItems"?: 0, "maxItems"?: 100 }
{ "kind": "object", "properties": { ... }, "required": ["name"], "additionalProperties": false }
{ "kind": "union", "members": [ { ... }, { ... } ] }     // bounded: at most 4 members
```

Coercion is source-aware: `path`/`query` values arrive as strings and coerce
per IR (`integer` parses base-10; failure = validation problem, never a 500).
`body` JSON types must match exactly (no string→number coercion in JSON bodies).

## Problem registry (RFC 9457-compatible, type URNs fixed)

```jsonc
{
  "validation":   { "type": "https://velqu.dev/problems/validation",   "title": "Validation failed",        "status": 422 },
  "unauthorized": { "type": "https://velqu.dev/problems/unauthorized", "title": "Unauthorized",            "status": 401 },
  "not-found":    { "type": "https://velqu.dev/problems/not-found",    "title": "Not Found",               "status": 404 },
  "method":       { "type": "https://velqu.dev/problems/method",       "title": "Method Not Allowed",      "status": 405 },
  "body":         { "type": "https://velqu.dev/problems/body",         "title": "Unsupported body",        "status": 415 },
  "limit":        { "type": "https://velqu.dev/problems/limit",        "title": "Payload too large",       "status": 413 },
  "timeout":      { "type": "https://velqu.dev/problems/timeout",      "title": "Handler deadline",        "status": 504 },
  "internal":     { "type": "https://velqu.dev/problems/internal",     "title": "Internal Server Error",   "status": 500 }
}
```

Bodies always include `type`, `title`, `status`, and optionally `detail`,
`errors: [{path, code, message}]`, `instance` (request ID).

## Bundle protocol

The bundle is evaluated once at startup with `globalThis.__velquRegister`
already installed by the host. The bundle must end with one registration
statement per handler-table entry:

```js
__velquRegister("users.get", users_get);
```

The host verifies: every manifest handlerTable key was registered, and no
unknown IDs appeared. Handler references are cached; application source is
never re-evaluated.

Handler calling convention (host -> JS):

```js
// policy: async (req) => ({ session }) | { __problem: true, ... }
// route:  async (ctx) => value | { status, value?, headers? } | { __problem: true, ... }
// ctx = { params, query, headers, session?, json(), text(), bytes(), native: { timer: { delay(ms) } } }
```

`json()/text()/bytes()` are lazy: the first access materializes the request
body through the native bridge (generation-checked); unread bodies are never
copied into the engine.

## Runtime CLI contract

```text
velqu-runtime --pack <app.qpack> --port <p> [--host 127.0.0.1] [--config <json>]
```

Limits (defaults, overridable by config): body 1 MiB, header 32 KiB, URI 8 KiB,
queue 256 concurrent, heap 32 MiB, stack 512 KiB, handler deadline 5 s,
pending ops 1024. Failures before ready exit non-zero with a structured
diagnostic on stderr.
