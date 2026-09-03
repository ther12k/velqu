/**
 * Crossover counts unit tests (BETA-003-C) — deterministic coverage of the
 * cumulative curve, crossover, and self-amortization math. No servers.
 */
import { describe, expect, test } from "bun:test";
import { crossoverCount, cumulativeCurves, median, selfAmortizationCount } from "./crossover";

describe("cumulative curves (BETA-003-C)", () => {
  test("prefix sums are medians across reps at each index", () => {
    const { horizon, cumMedian } = cumulativeCurves([
      [100, 100],
      [200, 100],
    ]);
    expect(horizon).toBe(2); // shortest rep bounds the horizon
    expect(cumMedian[0]).toBe(150); // median(100, 200)
    expect(cumMedian[1]).toBe(250); // median(200, 300)
  });

  test("horizon is the shortest rep", () => {
    const { horizon } = cumulativeCurves([[1, 2, 3, 4], [1, 2]]);
    expect(horizon).toBe(2);
  });
});

describe("crossover count", () => {
  test("warm-start-heavy candidate crosses the low-start competitor at N=26", () => {
    // A: 1000us first request then 100us; B: 500us then 120us.
    // cumA(N) = 1000 + 100(N-1); cumB(N) = 500 + 120(N-1)
    // A <= B  <=>  500 <= 20(N-1)  <=>  N >= 26
    const repA = [1000, ...Array(59).fill(100)];
    const repB = [500, ...Array(59).fill(120)];
    const { cumMedian: cumA } = cumulativeCurves([repA]);
    const { cumMedian: cumB } = cumulativeCurves([repB]);
    expect(crossoverCount(cumA, cumB)).toBe(26);
    // and the reverse pair crosses immediately (B is ahead from request 1)
    expect(crossoverCount(cumB, cumA)).toBe(1);
  });

  test("identical steady floors with a startup debt mean never crossing", () => {
    // A pays 4000us up front; both then serve at the same rate.
    const repA = [4000, ...Array(99).fill(100)];
    const repB = [1000, ...Array(99).fill(100)];
    const { cumMedian: cumA } = cumulativeCurves([repA]);
    const { cumMedian: cumB } = cumulativeCurves([repB]);
    expect(crossoverCount(cumA, cumB)).toBeNull();
  });

  test("respects the horizon argument", () => {
    // A pays 500us first, then 10us/req; B pays 100us every req.
    // cumA(n) = 500 + 10(n-1) <= 100n  <=>  n >= 6
    const cumA = [500, ...Array.from({ length: 8 }, (_, i) => 510 + i * 10)];
    const cumB = Array.from({ length: 9 }, () => 100 * 1).map((v, i) => v * (i + 1));
    expect(crossoverCount(cumA, cumB, 2)).toBeNull();
    expect(crossoverCount(cumA, cumB)).toBe(6);
  });
});

describe("self amortization", () => {
  test("amortizes once cumulative average reaches 1.25x the steady median", () => {
    // steady median 100; bound 125. cumulative average: 1000 -> ... -> <=125
    const series = [1000, 100, 100, 100, 100];
    // n=1: 1000; n=2: 550; n=3: 400; n=4: 325; n=5: 280 -> never within 125
    expect(selfAmortizationCount(series, 100)).toBeNull();
  });

  test("a mostly-flat series amortizes almost immediately", () => {
    const series = [150, 100, 100, 100, 100, 100];
    // steady median = 100, bound 125; n=1 avg 150, n=2 avg 125 -> 2
    expect(selfAmortizationCount(series, 100)).toBe(2);
  });

  test("degenerate inputs return null", () => {
    expect(selfAmortizationCount([], 100)).toBeNull();
    expect(selfAmortizationCount([1, 2, 3], 0)).toBeNull();
  });
});

describe("median helper parity", () => {
  test("matches ramp.ts conventions", () => {
    expect(median([3, 1, 2])).toBe(2);
    expect(median([4, 1, 2, 3])).toBe(2.5);
  });
});
