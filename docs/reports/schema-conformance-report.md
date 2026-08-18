---
type: Evidence Report
title: Schema Conformance Report
status: complete
milestone: M2
---

# Schema conformance report (M2 §12.3)

## Overview

The `@velqu/schema` package and `q-schema-runtime` crate implement the Schema IR v1
specification (SCHEMA-001..005).

## Verification Matrix

| Schema Type / Feature | TypeScript Inference | Runtime Validator | Native Strategy | Coercion Rule |
|---|---|---|---|---|
| `s.string({ minLength, maxLength, pattern, format })` | `string` | `q-schema-runtime` | PASS | String as-is |
| `s.integer({ minimum, maximum })` | `number` | `q-schema-runtime` | PASS | Path/query: string→i64; Body: exact i64 |
| `s.number({ minimum, maximum })` | `number` | `q-schema-runtime` | PASS | Path/query: string→f64; Body: exact f64 |
| `s.boolean()` | `boolean` | `q-schema-runtime` | PASS | Path/query: "true"/"false"→bool; Body: exact bool |
| `s.literal(value)` | `Literal` | `q-schema-runtime` | PASS | Exact equality |
| `s.enum(values)` | `Union` | `q-schema-runtime` | PASS | Membership check |
| `s.optional(inner, { default })` | `T \| undefined` | `q-schema-runtime` | PASS | Default applied on missing/null query |
| `s.nullable(inner)` | `T \| null` | `q-schema-runtime` | PASS | Null permitted |
| `s.array(items, { minItems, maxItems })` | `T[]` | `q-schema-runtime` | PASS | Element validation |
| `s.object(properties)` | `ObjectShape` | `q-schema-runtime` | PASS | Required by default; additional rejected |
| `s.union([m1, m2])` | `A \| B` | `q-schema-runtime` | PASS | First matching member |

## Source-Aware Coercion Evidence (SCHEMA-002)

- Path & query parameters: arrived as strings, coerced to integer/number/boolean per schema. Coercion failure produces a 422 validation problem (not 500).
- Body payloads: JSON types must match schema types exactly (no string→number coercion in JSON bodies).
- Unknown fields: unknown query keys ignored; unknown body keys rejected (`additionalProperties: false`).

## Validation Problem Format

All validation failures produce RFC 9457-compatible responses:
```json
{
  "type": "https://velqu.dev/problems/validation",
  "title": "Validation failed",
  "status": 422,
  "instance": "req-1786987412402-1",
  "errors": [
    { "path": "name", "code": "maxLength", "message": "must be at most 60 characters" }
  ]
}
```
Tested and verified in `conformance/schema/schema.conformance.test.ts` (6/6 pass).
