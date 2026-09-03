/**
 * Contract fixture matrix for candidate response verification (BETA-002-C).
 *
 * Defines the exact HTTP requests every real-world candidate must answer
 * identically and computes the expected status + JSON body from the shared
 * matched contract (`candidates/matched.ts`) — the same reference store the
 * candidates run — instead of hand-pinned data. A candidate "passes contract
 * verification" only when every fixture returns the expected status and an
 * exactly equal JSON body (key order ignored; wall-clock values matched by
 * the `{ __typeof__ }` sentinel).
 *
 * The matrix deliberately includes the error paths the matched contract
 * pins: 401 (missing/malformed bearer), 404 (unknown user/route), 400
 * (empty items, unknown product, invalid ms/n), 409 (insufficient stock),
 * 504 (upstream timeout), 502 (malformed upstream body).
 */

import { DeterministicStore, MATCHED_CONFIG } from "./candidates/matched";

export interface TypeSentinel {
  __typeof__: "string" | "number" | "boolean";
}

/** Sentinel for values that are legitimately wall-clock (e.g. upstream `actualMs`). */
export function typeofSentinel(type: "string" | "number" | "boolean"): TypeSentinel {
  return { __typeof__: type };
}

export interface ContractFixture {
  name: string;
  method: "GET" | "POST";
  path: string;
  headers?: Record<string, string>;
  body?: unknown;
  expectStatus: number;
  expectJson: unknown;
}

export function authHeader(token = MATCHED_CONFIG.jwt.benchmarkToken): Record<string, string> {
  return { authorization: `Bearer ${token}` };
}

/**
 * Builds the full fixture matrix. Expected bodies are derived from a fresh
 * DeterministicStore — exactly what each candidate seeds on boot — so a
 * fixture failure means a candidate response drifted from the shared
 * contract, not from stale expectations.
 */
export function buildContractFixtures(): ContractFixture[] {
  const store = new DeterministicStore();

  // Reference outcomes computed on throwaway stores so the fixture matrix
  // itself stays mutation-free and order-independent.
  const user = store.getUser("usr_1");
  if (!user) throw new Error("contract fixtures: seed is missing usr_1");
  const orderStore = new DeterministicStore();
  const order = orderStore.createOrder("usr_1", [{ productId: "prod_1", qty: 2 }]);
  if (!order.ok) throw new Error("contract fixtures: reference order unexpectedly failed");
  const w3 = store.getProducts("electronics", 1, 2);
  if (w3.products.length !== 2) {
    throw new Error("contract fixtures: expected 2 seeded electronics products on page 1");
  }

  const unauthorized = { error: "unauthorized" };

  return [
    // W1 — authenticated single-record lookup
    {
      name: "w1.user.200",
      method: "GET",
      path: "/api/users/usr_1",
      headers: authHeader(),
      expectStatus: 200,
      expectJson: {
        id: user.id,
        name: user.name,
        email: user.email,
        role: user.role,
        createdAt: user.created_at,
      },
    },
    {
      name: "w1.user.404",
      method: "GET",
      path: "/api/users/usr_missing",
      headers: authHeader(),
      expectStatus: 404,
      expectJson: { error: "not found" },
    },
    {
      name: "w1.user.401.missing-auth",
      method: "GET",
      path: "/api/users/usr_1",
      expectStatus: 401,
      expectJson: unauthorized,
    },
    {
      name: "w1.user.401.malformed-auth",
      method: "GET",
      path: "/api/users/usr_1",
      headers: { authorization: "Basic 12345" },
      expectStatus: 401,
      expectJson: unauthorized,
    },

    // W2 — authenticated write transaction
    {
      name: "w2.order.201",
      method: "POST",
      path: "/api/orders",
      headers: authHeader(),
      body: { items: [{ productId: "prod_1", qty: 2 }] },
      expectStatus: 201,
      expectJson: order.order,
    },
    {
      name: "w2.order.400.empty-items",
      method: "POST",
      path: "/api/orders",
      headers: authHeader(),
      body: { items: [] },
      expectStatus: 400,
      expectJson: { error: "items must not be empty" },
    },
    {
      name: "w2.order.400.unknown-product",
      method: "POST",
      path: "/api/orders",
      headers: authHeader(),
      body: { items: [{ productId: "prod_9999", qty: 1 }] },
      expectStatus: 400,
      expectJson: { error: "product not found: prod_9999" },
    },
    {
      name: "w2.order.409.insufficient-stock",
      method: "POST",
      path: "/api/orders",
      headers: authHeader(),
      body: { items: [{ productId: "prod_1", qty: 9999 }] },
      expectStatus: 409,
      expectJson: { error: "insufficient stock for product: prod_1" },
    },
    {
      name: "w2.order.401.missing-auth",
      method: "POST",
      path: "/api/orders",
      body: { items: [{ productId: "prod_1", qty: 1 }] },
      expectStatus: 401,
      expectJson: unauthorized,
    },

    // W3 — paginated list with aggregation
    {
      name: "w3.products.200",
      method: "GET",
      path: "/api/products?category=electronics&page=1&limit=2",
      expectStatus: 200,
      expectJson: w3,
    },

    // W4 — controlled I/O matrix (upstream relays; actualMs is wall-clock)
    {
      name: "w4.io.200",
      method: "GET",
      path: "/api/bench/io?ms=1",
      expectStatus: 200,
      expectJson: { status: "ok", ms: 1, actualMs: typeofSentinel("number") },
    },
    {
      name: "w4.io.400.invalid-ms",
      method: "GET",
      path: "/api/bench/io?ms=abc",
      expectStatus: 400,
      expectJson: { error: "invalid ms" },
    },
    {
      name: "w4.mixed.200.success",
      method: "GET",
      path: "/api/bench/mixed?mode=success",
      expectStatus: 200,
      expectJson: { status: "ok", ms: 5, actualMs: typeofSentinel("number") },
    },
    {
      name: "w4.mixed.504.timeout",
      method: "GET",
      path: "/api/bench/mixed?mode=timeout",
      expectStatus: 504,
      expectJson: { mode: "timeout", handled: "timeout" },
    },
    {
      name: "w4.mixed.502.malformed",
      method: "GET",
      path: "/api/bench/mixed?mode=malformed",
      expectStatus: 502,
      expectJson: {
        mode: "malformed",
        handled: "malformed",
        problem: "upstream response was not valid JSON",
      },
    },
    {
      name: "w4.fanout.200",
      method: "GET",
      path: "/api/bench/fanout?n=2&ms=1",
      expectStatus: 200,
      expectJson: { n: 2, ms: 1, ok: true },
    },
    {
      name: "w4.fanout.400.invalid-n",
      method: "GET",
      path: "/api/bench/fanout?n=3&ms=1",
      expectStatus: 400,
      expectJson: { error: "invalid n or ms" },
    },

    // Unknown routes share one contract shape across candidates
    {
      name: "route.404.unknown",
      method: "GET",
      path: "/api/definitely-not-a-route",
      expectStatus: 404,
      expectJson: { error: "not found" },
    },
  ];
}

function isSentinel(value: unknown): value is TypeSentinel {
  return (
    typeof value === "object" &&
    value !== null &&
    typeof (value as TypeSentinel).__typeof__ === "string"
  );
}

/**
 * Deep equality between the expected value (which may contain
 * `{ __typeof__ }` sentinels) and the candidate's parsed JSON response.
 * Key order is irrelevant; arrays are order-sensitive.
 */
export function matchesExpected(expected: unknown, actual: unknown): boolean {
  if (isSentinel(expected)) {
    return typeof actual === expected.__typeof__;
  }
  if (expected === null || actual === null) return expected === actual;
  if (Array.isArray(expected)) {
    if (!Array.isArray(actual) || actual.length !== expected.length) return false;
    return expected.every((item, i) => matchesExpected(item, actual[i]));
  }
  if (typeof expected === "object") {
    if (typeof actual !== "object" || Array.isArray(actual)) return false;
    const expectedKeys = Object.keys(expected).sort();
    const actualKeys = Object.keys(actual).sort();
    if (expectedKeys.length !== actualKeys.length) return false;
    if (expectedKeys.some((k, i) => k !== actualKeys[i])) return false;
    return expectedKeys.every((k) =>
      matchesExpected((expected as Record<string, unknown>)[k], (actual as Record<string, unknown>)[k]),
    );
  }
  return expected === actual;
}
