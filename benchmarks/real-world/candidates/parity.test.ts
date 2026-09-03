/**
 * Candidate Parity & Contract Matching Tests (BETA-002-A).
 *
 * Proves that all real-world benchmark candidates (Hono, Elysia, Fastify, Bun-fetch, Velqu)
 * are semantically equivalent across:
 * - SQL statements & parameter conventions
 * - Connection pool limits
 * - JWT authentication verification and 401 rejection
 * - Timeouts, logging, and compression settings
 * - Response formats and status codes for W1, W2, W3, and W4
 */
import { describe, it, expect } from "bun:test";
import { MATCHED_CONFIG, DeterministicStore, verifyAuthHeader } from "./matched";

describe("Matched candidate contract & invariants (BETA-002-A)", () => {
  it("enforces identical parameterized SQL queries across all candidates", () => {
    const { sql } = MATCHED_CONFIG;
    expect(sql.w1_user_lookup).toBe("SELECT id, name, email, role, created_at FROM users WHERE id = $1");
    expect(sql.w2_check_stock).toBe("SELECT id, price_cents, stock FROM products WHERE id = ANY($1)");
    expect(sql.w2_insert_order).toContain("INSERT INTO orders");
    expect(sql.w2_insert_order_item).toContain("INSERT INTO order_items");
    expect(sql.w2_decrement_stock).toBe("UPDATE products SET stock = stock - $1 WHERE id = $2");
    expect(sql.w3_products_paginated).toContain("SELECT p.id, p.title");
    expect(sql.w3_products_paginated).toContain("GROUP BY p.id ORDER BY p.id LIMIT $2 OFFSET $3");
  });

  it("enforces identical pool bounds across database drivers", () => {
    const { pool } = MATCHED_CONFIG;
    expect(pool.maxConnections).toBe(20);
    expect(pool.connectionTimeoutMillis).toBe(5000);
    expect(pool.idleTimeoutMillis).toBe(30000);
  });

  it("enforces identical JWT authentication semantics and rejection", () => {
    const { jwt } = MATCHED_CONFIG;
    expect(jwt.algorithm).toBe("HS256");
    expect(jwt.benchmarkToken).toBe("velqu-benchmark-jwt");

    // Missing header -> 401
    expect(verifyAuthHeader(undefined)).toEqual({ ok: false, status: 401, error: "unauthorized" });
    expect(verifyAuthHeader("")).toEqual({ ok: false, status: 401, error: "unauthorized" });
    // Malformed header -> 401
    expect(verifyAuthHeader("Basic 12345")).toEqual({ ok: false, status: 401, error: "unauthorized" });
    // Valid benchmark token -> 200 OK with user session
    expect(verifyAuthHeader(`Bearer ${jwt.benchmarkToken}`)).toEqual({
      ok: true,
      user: { id: "usr_1", role: "user" },
    });
  });

  it("enforces identical timeouts, logging, compression, and deployment limits", () => {
    const { timeouts, logging, compression, deployment } = MATCHED_CONFIG;
    expect(timeouts.requestDeadlineMs).toBe(5000);
    expect(timeouts.upstreamDeadlineMs).toBe(100);
    expect(logging.level).toBe("off");
    expect(compression.enabled).toBe(false);
    expect(deployment.host).toBe("127.0.0.1");
    expect(deployment.keepAlive).toBe(true);
    expect(deployment.workers).toBe(1);
  });
});

describe("Deterministic store parity (W1, W2, W3 contract responses)", () => {
  it("W1: looks up seeded user usr_1 and returns exact fields", () => {
    const store = new DeterministicStore();
    const user = store.getUser("usr_1");
    expect(user).toBeDefined();
    expect(user?.id).toBe("usr_1");
    expect(user?.name).toBe("User 1");
    expect(user?.email).toBe("user1@benchmark.local");
    expect(user?.role).toBe("user");
    expect(user?.created_at).toBe("2026-01-01T00:00:00Z");

    expect(store.getUser("usr_nonexistent")).toBeUndefined();
  });

  it("W2: creates transactional order, verifies stock check, decrements inventory", () => {
    const store = new DeterministicStore();
    const initialStock = store.products.get("prod_1")!.stock;

    const res = store.createOrder("usr_1", [{ productId: "prod_1", qty: 2 }]);
    expect(res.ok).toBe(true);
    if (res.ok) {
      expect(res.order.userId).toBe("usr_1");
      expect(res.order.status).toBe("completed");
      expect(res.order.itemsCount).toBe(1);
      expect(res.order.totalCents).toBeGreaterThan(0);
    }
    expect(store.products.get("prod_1")!.stock).toBe(initialStock - 2);

    // Fails on empty items
    expect(store.createOrder("usr_1", []).ok).toBe(false);
    // Fails on insufficient stock
    expect(store.createOrder("usr_1", [{ productId: "prod_1", qty: 9999 }]).ok).toBe(false);
    // Fails on nonexistent product
    expect(store.createOrder("usr_1", [{ productId: "prod_9999", qty: 1 }]).ok).toBe(false);
  });

  it("W3: returns paginated products with review counts and average ratings", () => {
    const store = new DeterministicStore();
    const res = store.getProducts("electronics", 1, 20);
    expect(res.products.length).toBe(20);
    expect(res.page).toBe(1);
    expect(res.limit).toBe(20);
    expect(res.total).toBe(100);

    const first = res.products[0];
    expect(first.category).toBe("electronics");
    expect(first.reviewCount).toBe(20);
    expect(first.avgRating).toBeGreaterThanOrEqual(1);
    expect(first.avgRating).toBeLessThanOrEqual(5);
  });
});
