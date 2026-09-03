# Real-World Backend Benchmark Specification

This suite measures real-world backend API performance across frameworks under realistic conditions (database queries, authentication, complex JSON serialization, concurrent I/O), complementing loopback microbenchmarks.

## Workloads

1. **W1 — Authenticated Single-Record Lookup (I/O + Auth)**:
   - Request: `GET /api/users/:id` with `Authorization: Bearer <jwt>`
   - Policy: JWT verification $\to$ inject user session
   - DB: `SELECT id, name, email, role, created_at FROM users WHERE id = $1` (indexed PK query)
   - Response: JSON user profile (200 OK)

2. **W2 — Authenticated Write Transaction (ACID write + validation)**:
   - Request: `POST /api/orders` with body `{ "items": [{ "productId": "p1", "qty": 2 }] }`
   - Policy: JWT verification $\to$ inject user session
   - Schema validation: check item bounds, product IDs, UUIDs
   - DB: Transaction (verify stock $\to$ insert order $\to$ insert order items $\to$ decrement stock)
   - Response: JSON order receipt (201 Created)

3. **W3 — Paginated List with Aggregation (Heavy JSON + DB scan)**:
   - Request: `GET /api/products?category=electronics&page=1&limit=20`
   - DB: `SELECT p.*, COUNT(r.id) as review_count, AVG(r.rating) as avg_rating FROM products p LEFT JOIN reviews r ON r.product_id = p.id WHERE p.category = $1 GROUP BY p.id LIMIT 20`
   - Response: 16 KB JSON array with nested aggregation objects

4. **W4 — Controlled I/O Matrix (Latency & Concurrency Isolation)**:
   - Simulated deterministic database / upstream microservice latency: 1ms, 5ms, 10ms, 25ms.
   - Measures connection multiplexing, queue pressure, and p95/p99 tail latency under load.

## Evaluated Candidates

1. **Velqu**: Rust HTTP transport + QuickJS bytecode handler + async connection pooling
2. **Elysia 2 (Bun)**: Elysia on Bun runtime with bun:sqlite / postgres.js
3. **Hono (Bun)**: Hono on Bun runtime with native fetch / connection pooling
4. **Fastify (Node.js)**: Fastify on Node.js 22 LTS with pg connection pool

## Contract-Response Verification (gate before timing)

Before any load run, every candidate must pass `verify-contract.ts`
(`./run.sh contracts`): the verifier boots each candidate against the
controlled upstream and checks an 18-fixture matrix (W1 lookup + 404/401s,
W2 order + 400/409/401s, W3 exact paginated body, W4 io/mixed/fanout +
400/504/502, unknown-route 404) against expected responses computed from the
shared matched contract. Only candidates that answer every fixture with the
same status and byte-equivalent JSON are semantically equivalent and may be
compared.

## Metrics Tracked

- **Throughput**: Requests per second (RPS) at concurrency 1, 10, 50, 200
- **Latency Profile**: p50, p95, p99, max latency in microseconds
- **Resource Usage**: Process RSS memory (idle vs under peak load), CPU utilization
- **Reliability**: Error rate (0% required), connection timeout rate, pool exhaustion resilience
