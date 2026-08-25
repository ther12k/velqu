# Velqu Application Pack Format — v1 (frozen for M1/M2)

Status: frozen working decision (see `docs/open-decisions.md`). Changing this
format after M1 measurements requires a version bump and an ADR.

M2.6 note (ADR-0024): this format is the **legacy v1 adapter** mode
(`formatVersion: 1`, `PackFormatMode::LegacyV1`). Binary QPack v2 is a
separate numeric mode with its normative layout in
`docs/specs/pack-format-v2.md` (ADR-0025); unknown versions fail closed
at pack verify. The `integrity` block below is **integrity only**
(ADR-0026): it detects corruption and naive tampering; authenticity is
out-of-band deployment policy (detached signatures / build provenance).
The optional `sourceMap` field is debug material (ADR-0027): production
producers should omit it; the runtime never requires or consults it.

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
  "schemaIrVersion": 2,
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

Canonical JSON (ADR-0023, M25-001-C): object keys recursively sorted
(byte order), arrays keep order, integral floats ≤ 2^53-1 normalize to
integers (`0.0` → `0`). Both hash surfaces — `integrity.routesSha256`
(execution graph) and `contractHash` (public contract) — canonicalize
their whole view through this form; the compiler and the runtime share
the committed canonical corpus in `conformance/schema/golden/canonical/`.

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
  "overload":     { "type": "https://velqu.dev/problems/overload",     "title": "Overloaded",              "status": 503 },
  "internal":     { "type": "https://velqu.dev/problems/internal",     "title": "Internal Server Error",   "status": 500 }
}
```

Bodies always include `type`, `title`, `status`, and optionally `detail`,
`errors: [{path, code, message}]`, `instance` (request ID). Problem
responses carry `Content-Type: application/problem+json` (RFC 9457 §3);
success bodies stay `application/json`.

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

## Deprecation and migration (ADR-0024, M26-008)

Status: **deprecated, supported.** v1 remains loadable through the
separate legacy adapter (`q_pack::legacy_v1`) for the whole M2.6 window;
removal requires an explicit owner-track decision.

- Loader entry point: `q_pack::legacy_v1::read_and_verify` (disk) /
  `read_and_verify_bytes` (bytes). Mode dispatch happens before the
  adapter; legacy structures are built only behind it and never on v2
  hot paths.
- Unknown/unsupported `formatVersion` values fail closed with an
  actionable message naming the rebuild/migrate options.
- Migration paths: (1) **assess a pack** with
  `velqu pack migrate <app.qpack>` — reports the pack's mode and prints
  the recommended path; (2) **rebuild from source** with the current
  compiler (`velqu build --project <dir>`) — deterministic output per
  the M26-007 reproducibility work, so a rebuild is byte-stable and
  behavior-neutral; (3) binary mode-2 migration guidance will be
  reported by the same command once producers emit mode 2.
- **Mixed-mode packs are rejected by name (M26-008-C).** Modes are
  exclusive: a binary `VELQUQPK` container presented where a JSON pack
  is parsed, or a JSON pack carrying mode-2-reserved top-level fields
  (`sections`, `sectionDirectory`, `qpack2`), is rejected before any
  adapter interprets it. Unknown JSON keys were previously dropped
  silently by serde; the reserved-key gate closes that hole so a hybrid
  artifact can never load as v1 while tooling reads different semantics.
  Unsupported legacy features fail deterministically (M26-008-D).

### Deterministic failure matrix (M26-008-D)

Every rejection below uses a static, environment-independent message
(no addresses, counters, or host state); the same input always produces
the same error text and exit path, with no fallback. Committed negative
fixtures under `tests/fixtures/v1/unsupported/` pin each case
(`unsupported_legacy_features_fail_deterministically`):

| unsupported feature | fixture | deterministic rejection contains |
|---|---|---|
| Schema IR v1 (pre-M25 producer) | `schema-ir-v1.json` | `schema IR version 1 not supported` |
| wrong embedded engine fingerprint | `engine-mismatch.json` | `engine mismatch` |
| `bundlePrelude: "embedded"` without bytecode | `prelude-without-bytecode.json` | `requires bundleBytecode` |
| unknown/future runtime ABI | `runtime-abi.json` | `runtime ABI` |
| unknown `formatVersion` | (mutation test) | `not supported … fail closed — rebuild … migrate` |
| mode-2 keys inside JSON pack | `../mixed-mode-sections.json` | `mixed-mode pack: 'sections'` |
| binary container presented as JSON | (synthetic bytes) | `mixed-mode pack … VELQUQPK` |

Recovery for every row is the same two options: rebuild from source with
the current compiler, or follow the `velqu pack migrate` guidance.
