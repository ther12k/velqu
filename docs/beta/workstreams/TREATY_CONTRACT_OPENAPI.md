---
type: Workstream
title: Treaty, Contract, OpenAPI, and Semantic Diff
status: draft
tags:
- treaty
- contract
- openapi

---

# Treaty, Contract, OpenAPI, and Semantic Diff

## Product invariant

One canonical public contract graph drives:

```text
server input/output types
runtime validation/encoding
Treaty clients
OpenAPI
published contract package
contract lock
semantic API diff
```

## Beta requirements

- Only declared HTTP methods exist at type and runtime level.
- Params/query/headers/body are exact; no caller-selected body generic.
- Every 2xx response narrows to `data`; every declared non-2xx narrows to typed `error`.
- Network/abort/unexpected-status errors are distinct.
- Policy-provided 401/403/problems flow into route unions.
- Unit-local, runtime-local, and remote clients share one contract.
- Published client does not import server implementation.
- Public contract hash excludes internal numeric layout.
- Semantic diff recursively handles objects, arrays, unions, enums, formats, constraints, requiredness, statuses, and security direction.

## Negative type tests

```ts
// unsupported method
api.users({ id }).post(...)

// missing parameter
api.users({}).get()

// wrong query/body
api.users({ id }).get({ query: { unknown: true } })

// success status cannot appear as error
result.error.status satisfies 401 | 404
```

## Release evidence

- Typecheck scale at 25/1,000/10,000 routes.
- Golden OpenAPI and contract fixtures.
- Cross-mode Treaty parity.
- Semantic diff compatibility matrix.
