/**
 * Contract fixture matrix tests (BETA-002-C).
 *
 * Covers the deterministic pieces of candidate contract-response
 * verification: the fixture matrix shape and coverage, expected bodies
 * derived from the shared matched contract, the sentinel-aware comparison,
 * and the candidate runner definitions. The live HTTP verification itself
 * runs via `bun verify-contract.ts` (needs installed candidate deps + node).
 */
import { describe, it, expect } from "bun:test";
import {
  buildContractFixtures,
  matchesExpected,
  typeofSentinel,
} from "./contract-fixtures";
import { DeterministicStore, MATCHED_CONFIG } from "./candidates/matched";

describe("Contract fixture matrix (BETA-002-C)", () => {
  it("covers the full matched surface with unique names and expected statuses", () => {
    const fixtures = buildContractFixtures();
    const names = fixtures.map((f) => f.name);
    expect(new Set(names).size).toBe(names.length);

    // Every fixture pins a status and a body; W1..W4 and the auth/route
    // error paths are all represented.
    for (const f of fixtures) {
      expect(Number.isInteger(f.expectStatus)).toBe(true);
      expect(f.expectJson).toBeDefined();
      expect(["GET", "POST"]).toContain(f.method);
      expect(f.path.startsWith("/")).toBe(true);
    }
    for (const prefix of ["w1.", "w2.", "w3.", "w4.", "cpu.", "route."]) {
      expect(names.some((n) => n.startsWith(prefix))).toBe(true);
    }
    // Rejection statuses required by the matched contract
    expect(fixtures.some((f) => f.expectStatus === 401)).toBe(true);
    expect(fixtures.some((f) => f.expectStatus === 404)).toBe(true);
    expect(fixtures.some((f) => f.expectStatus === 400)).toBe(true);
    expect(fixtures.some((f) => f.expectStatus === 409)).toBe(true);
    expect(fixtures.some((f) => f.expectStatus === 504)).toBe(true);
    expect(fixtures.some((f) => f.expectStatus === 502)).toBe(true);
    expect(fixtures.some((f) => f.expectStatus === 201)).toBe(true);
  });

  it("derives the W1 expected body from the shared store, not hand-pinned data", () => {
    const store = new DeterministicStore();
    const user = store.getUser("usr_1")!;
    const fixture = buildContractFixtures().find((f) => f.name === "w1.user.200")!;
    expect(fixture.expectJson).toEqual({
      id: user.id,
      name: user.name,
      email: user.email,
      role: user.role,
      createdAt: user.created_at,
    });
    expect(fixture.headers?.authorization).toBe(`Bearer ${MATCHED_CONFIG.jwt.benchmarkToken}`);
  });

  it("derives the W3 expected body from an identical paginated store query", () => {
    const fixture = buildContractFixtures().find((f) => f.name === "w3.products.200")!;
    expect(fixture.path).toBe("/api/products?category=electronics&page=1&limit=2");
    const reference = new DeterministicStore().getProducts("electronics", 1, 2);
    expect(fixture.expectJson).toEqual(reference);
    expect(reference.products.length).toBe(2);
  });

  it("derives the W2 201 expected receipt from a fresh first-order reference", () => {
    const fixture = buildContractFixtures().find((f) => f.name === "w2.order.201")!;
    const reference = new DeterministicStore().createOrder("usr_1", [
      { productId: "prod_1", qty: 2 },
    ]);
    expect(reference.ok).toBe(true);
    if (reference.ok) {
      expect(fixture.expectJson).toEqual(reference.order);
      expect((fixture.expectJson as { id: string }).id).toBe("ord_1");
    }
  });

  it("sends the benchmark bearer on protected fixtures; 401 fixtures vary auth deliberately", () => {
    const fixtures = buildContractFixtures();
    for (const f of fixtures) {
      if (f.name.endsWith("missing-auth")) {
        expect(f.headers).toBeUndefined();
      } else if (f.name.endsWith("malformed-auth")) {
        expect(f.headers?.authorization).toBe("Basic 12345");
      } else if (f.name.startsWith("w1.") || f.name.startsWith("w2.")) {
        expect(f.headers?.authorization).toBe(`Bearer ${MATCHED_CONFIG.jwt.benchmarkToken}`);
      }
    }
  });
});

describe("matchesExpected sentinel comparison", () => {
  it("matches primitives, nested objects, and arrays regardless of key order", () => {
    expect(matchesExpected({ a: 1, b: { c: "x" } }, { b: { c: "x" }, a: 1 })).toBe(true);
    expect(matchesExpected([1, 2, 3], [1, 2, 3])).toBe(true);
    expect(matchesExpected([1, 2, 3], [3, 2, 1])).toBe(false);
    expect(matchesExpected({ a: 1 }, { a: 1, extra: 2 })).toBe(false);
    expect(matchesExpected({ a: 1 }, {})).toBe(false);
    expect(matchesExpected("x", "x")).toBe(true);
    expect(matchesExpected(null, null)).toBe(true);
    expect(matchesExpected({ a: 1 }, [1])).toBe(false);
  });

  it("resolves __typeof__ sentinels as type checks (wall-clock fields)", () => {
    expect(matchesExpected({ actualMs: typeofSentinel("number") }, { actualMs: 1234.5 })).toBe(true);
    expect(matchesExpected({ actualMs: typeofSentinel("number") }, { actualMs: "fast" })).toBe(false);
    expect(matchesExpected(typeofSentinel("string"), "ok")).toBe(true);
    expect(matchesExpected(typeofSentinel("boolean"), true)).toBe(true);
    expect(matchesExpected(typeofSentinel("number"), null)).toBe(false);
  });

  it("matches the W4 io fixture shape against an upstream-style body", () => {
    const fixture = buildContractFixtures().find((f) => f.name === "w4.io.200")!;
    expect(matchesExpected(fixture.expectJson, { status: "ok", ms: 1, actualMs: 1.234 })).toBe(true);
    expect(matchesExpected(fixture.expectJson, { status: "ok", ms: 2, actualMs: 1.234 })).toBe(false);
    expect(matchesExpected(fixture.expectJson, { status: "ok", ms: 1 })).toBe(false);
  });
});
