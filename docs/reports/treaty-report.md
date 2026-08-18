---
type: Evidence Report
title: Treaty Client Report
status: complete
milestone: M2
---

# Treaty client report (M2 §12.6)

## Overview

The `@q/treaty` package implements a type-safe client with object-like route
navigation, typed inputs, and status-narrowed failure values (TRT-001..006).

## Evidence & Verifications

| Requirement | Description | Evidence | Status |
|---|---|---|---|
| TRT-001 | Inferred path, query, header, and body inputs | `conformance/treaty/treaty.conformance.test.ts` | PASS |
| TRT-002 | Success and HTTP failure returned separately without throwing | `conformance/treaty/treaty.conformance.test.ts` (401 & 422 returned as values) | PASS |
| TRT-003 | Failure values narrowed by status code | `packages/treaty/src/treaty.test.ts` (type test with expectTypeOf) | PASS |
| TRT-004 | Client bundle small & independent of server code | Zero imports of server/compiler packages verified | PASS |
| TRT-005 | Fast unit mode labeled separately from native runtime | `unitTreaty` returns `__mode: "unit-local"` vs `runtimeTreaty` | PASS |
| TRT-006 | Published contract mode for independent repos | `ProofPublishedApi` driving runtime calls | PASS |

## Usage Examples

```ts
import { treaty } from "@q/treaty";
import type { Api } from "./dist/contract";

const api = treaty<Api>({
  baseUrl: "http://localhost:3000",
  contract: { "hello.get": { path: "/hello/:name", method: "GET" } }
});

// 1. Success call
const res = await api.hello.get({ name: "Rafi" }).get();
if (res.data) {
  console.log(res.data.message); // typed as string
}

// 2. Status-narrowed error handling
const authRes = await api.users.get({ id: "usr_1" }).get();
if (authRes.error) {
  if (authRes.error.status === 401) {
    console.error("Unauthorized:", authRes.error.problem.title);
  }
}
```

## Bundle Footprint

- Source file: `packages/treaty/src/index.ts` is 5.5 KB
- Minified: < 4 KB (target budget ≤ 8 KB — PASS)
- Dependencies: 0 (pure fetch wrapper)
