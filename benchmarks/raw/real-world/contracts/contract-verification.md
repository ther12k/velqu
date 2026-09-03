# Real-World Candidate Contract Verification

Controlled upstream: http://127.0.0.1:8791

## hono (`bun candidates/hono.ts` on :34589)

| Fixture | Status | Detail |
|---|---|---|
| w1.user.200 | PASS | 200 |
| w1.user.404 | PASS | 404 |
| w1.user.401.missing-auth | PASS | 401 |
| w1.user.401.malformed-auth | PASS | 401 |
| w2.order.201 | PASS | 201 |
| w2.order.400.empty-items | PASS | 400 |
| w2.order.400.unknown-product | PASS | 400 |
| w2.order.409.insufficient-stock | PASS | 409 |
| w2.order.401.missing-auth | PASS | 401 |
| w3.products.200 | PASS | 200 |
| w4.io.200 | PASS | 200 |
| w4.io.400.invalid-ms | PASS | 400 |
| w4.mixed.200.success | PASS | 200 |
| w4.mixed.504.timeout | PASS | 504 |
| w4.mixed.502.malformed | PASS | 502 |
| w4.fanout.200 | PASS | 200 |
| w4.fanout.400.invalid-n | PASS | 400 |
| route.404.unknown | PASS | 404 |

## elysia (`bun candidates/elysia.ts` on :38225)

| Fixture | Status | Detail |
|---|---|---|
| w1.user.200 | PASS | 200 |
| w1.user.404 | PASS | 404 |
| w1.user.401.missing-auth | PASS | 401 |
| w1.user.401.malformed-auth | PASS | 401 |
| w2.order.201 | PASS | 201 |
| w2.order.400.empty-items | PASS | 400 |
| w2.order.400.unknown-product | PASS | 400 |
| w2.order.409.insufficient-stock | PASS | 409 |
| w2.order.401.missing-auth | PASS | 401 |
| w3.products.200 | PASS | 200 |
| w4.io.200 | PASS | 200 |
| w4.io.400.invalid-ms | PASS | 400 |
| w4.mixed.200.success | PASS | 200 |
| w4.mixed.504.timeout | PASS | 504 |
| w4.mixed.502.malformed | PASS | 502 |
| w4.fanout.200 | PASS | 200 |
| w4.fanout.400.invalid-n | PASS | 400 |
| route.404.unknown | PASS | 404 |

## bun-fetch (`bun candidates/bun-fetch.ts` on :42881)

| Fixture | Status | Detail |
|---|---|---|
| w1.user.200 | PASS | 200 |
| w1.user.404 | PASS | 404 |
| w1.user.401.missing-auth | PASS | 401 |
| w1.user.401.malformed-auth | PASS | 401 |
| w2.order.201 | PASS | 201 |
| w2.order.400.empty-items | PASS | 400 |
| w2.order.400.unknown-product | PASS | 400 |
| w2.order.409.insufficient-stock | PASS | 409 |
| w2.order.401.missing-auth | PASS | 401 |
| w3.products.200 | PASS | 200 |
| w4.io.200 | PASS | 200 |
| w4.io.400.invalid-ms | PASS | 400 |
| w4.mixed.200.success | PASS | 200 |
| w4.mixed.504.timeout | PASS | 504 |
| w4.mixed.502.malformed | PASS | 502 |
| w4.fanout.200 | PASS | 200 |
| w4.fanout.400.invalid-n | PASS | 400 |
| route.404.unknown | PASS | 404 |

## fastify (`node candidates/fastify.js` on :40839)

| Fixture | Status | Detail |
|---|---|---|
| w1.user.200 | PASS | 200 |
| w1.user.404 | PASS | 404 |
| w1.user.401.missing-auth | PASS | 401 |
| w1.user.401.malformed-auth | PASS | 401 |
| w2.order.201 | PASS | 201 |
| w2.order.400.empty-items | PASS | 400 |
| w2.order.400.unknown-product | PASS | 400 |
| w2.order.409.insufficient-stock | PASS | 409 |
| w2.order.401.missing-auth | PASS | 401 |
| w3.products.200 | PASS | 200 |
| w4.io.200 | PASS | 200 |
| w4.io.400.invalid-ms | PASS | 400 |
| w4.mixed.200.success | PASS | 200 |
| w4.mixed.504.timeout | PASS | 504 |
| w4.mixed.502.malformed | PASS | 502 |
| w4.fanout.200 | PASS | 200 |
| w4.fanout.400.invalid-n | PASS | 400 |
| route.404.unknown | PASS | 404 |

**Contract verification: PASS** — every candidate answered every fixture identically.
