---
type: Architecture Specification
title: Schema and Validation Architecture
description: Native schema IR, explicit coercion, fallback visibility, response validation,
  adapters, and evolution.
tags:
- schema
- validation
- type-system
- openapi
- ir
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
sources:
- id: elysia-validation
  resource: https://elysiajs.com/essential/validation
  title: Elysia validation and schema model
- id: elysia-2
  resource: https://elysiajs.com/blog/elysia-20
  title: Elysia 2 beta announcement and AOT design
- id: rfc-9457
  resource: https://www.rfc-editor.org/info/rfc9457/
  title: RFC 9457 Problem Details for HTTP APIs
---

# Purpose

The schema system is the single source of truth for server input, output, runtime validation strategy, TypeScript inference, Treaty types, OpenAPI, and semantic API diffing.

# Design principle

Project Q does not need every possible schema feature in its native fast path. It needs a small, explicit, well-defined schema language whose semantics are identical across all generated artifacts.

# Initial schema API

Illustrative syntax:

```ts
const CreateUser = s.object({
  name: s.string({
    minLength: 1,
    maxLength: 100
  }),
  email: s.email(),
  age: s.optional(
    s.integer({
      minimum: 0,
      maximum: 150
    })
  )
});
```

The syntax may evolve, but one declaration SHALL map to one normalized schema IR.

# Core schema IR

P0 types:

```text
null
boolean
integer
number
string
literal
enum
array
object
optional field
nullable
union with explicit discriminant where possible
```

P0 string formats:

```text
uuid
email
date-time as validated string
uri
```

P0 constraints:

```text
minimum / maximum
exclusive bounds
minLength / maxLength
pattern with a supported regex subset
minItems / maxItems
required fields
additional-properties policy
```

Binary/file/stream forms are represented separately from JSON schemas.

# Explicit coercion

No global “helpful” conversion.

Examples:

```ts
s.integerFromString()
s.booleanFromString({
  true: ["true", "1"],
  false: ["false", "0"]
})
s.dateTimeString()
```

Coercion is allowed only for sources where string transport is expected, such as params and query. JSON body values do not silently convert `"42"` into `42` unless the schema explicitly requests it.

# Unsupported semantics

Initially excluded from native IR:

- arbitrary user callbacks;
- asynchronous refinements;
- transforms that change the output type through code;
- effects with external I/O;
- opaque custom classes;
- non-deterministic defaults;
- unrestricted regular expressions when engine parity is uncertain.

An adapter encountering unsupported semantics must either fail or explicitly select JavaScript fallback.

# Validation strategies

A route's build report declares:

```text
input validation:
  params: native-ir
  query: native-ir
  body: quickjs-fallback
output validation:
  200: development-only native-ir
```

Possible strategies:

- `native-ir`;
- `generated-direct`;
- `quickjs-fallback`;
- `none`, only when explicitly allowed;
- `development-only-output`.

No route may silently downgrade.

# Default values

Defaults must be:

- deterministic;
- serializable in the schema IR;
- precomputed where safe;
- applied with consistent missing-versus-null semantics;
- visible to OpenAPI and Treaty types.

Functions such as `default: () => new Date()` are excluded from native precomputation. Runtime factories require an explicit JavaScript fallback.

# Errors

Validation failure becomes an RFC 9457-compatible problem:

```json
{
  "type": "urn:q:problem:validation",
  "title": "Request validation failed",
  "status": 422,
  "errors": [
    {
      "location": "body.email",
      "code": "format",
      "expected": "email"
    }
  ]
}
```

Production errors avoid echoing secrets or full bodies. Diagnostics can include safe paths, codes, and expected constraints.

# Response validation

Response schemas are primarily contracts and serializers. Modes:

- development: validate all declared structured responses;
- production strict: validate where configured;
- production optimized: trust generated/result constructors and encoder invariants;
- raw response: bypass with an explicit inspection warning.

An undeclared status is always a framework contract error.

# Third-party adapters

Potential packages:

```text
@q/schema-typebox
@q/schema-standard
@q/schema-zod
@q/schema-valibot
```

Adapter contract:

```ts
interface SchemaAdapter<Input> {
  normalize(schema: Input): NormalizationResult;
}
```

The normalization result contains:

- IR, if fully representable;
- TypeScript-facing metadata;
- OpenAPI/JSON Schema capability;
- fallback validator module, if accepted;
- semantic limitations;
- stable schema hash.

The core is not coupled to a third-party schema library.

# Schema identity and cache

Canonical IR is hashed. Equivalent schemas can share generated validators and serializers when semantics, source location requirements, and diagnostics policy permit.

Cache identity includes:

- IR version;
- schema canonical form;
- source context/coercion mode;
- validator options;
- runtime target.

# Schema evolution

Semantic contract diff classifies:

| Change | Default classification |
|---|---|
| remove required response field | breaking |
| change response type | breaking |
| add required request field | breaking |
| make request field optional | compatible |
| add optional response field | compatible for structural clients; configurable |
| narrow numeric/string bounds | breaking |
| widen accepted request bounds | compatible |
| add enum response value | policy-sensitive |
| add possible error status | policy-sensitive or breaking |
| add route | compatible |

Organizations may configure stricter policies.

# Conformance

Every schema feature requires shared fixtures proving:

```text
TypeScript expectation
native validator result
fallback validator result, if applicable
OpenAPI representation
Treaty input/output type snapshot
semantic diff behavior
error path/code
```

A feature is not “supported” because one validator happens to accept it.
