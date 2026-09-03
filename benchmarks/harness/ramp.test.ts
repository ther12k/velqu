/**
 * Ramp harness unit tests (BETA-003-B) — deterministic coverage of the
 * steady-state onset detector, phase labeling, and aggregation. No servers;
 * the live ramp runs via `bun ramp.ts`.
 */
import { describe, expect, test } from "bun:test";
import {
  aggregateRamp,
  median,
  percentile,
  phaseLabel,
  steadyOnsetIndex,
} from "./ramp";

describe("steady-state onset detection (BETA-003-B)", () => {
  test("a flat series is steady after two consecutive flat transitions", () => {
    const flat = Array.from({ length: 100 }, () => 100);
    expect(steadyOnsetIndex(flat)).toBe(50);
  });

  test("a decaying series reaches steady where the decay flattens", () => {
    // window medians: 800, 400, 200, 200, 200, 200 -> transitions
    // 0.5, 0.5, 1.0, 1.0, 1.0; the first two consecutive flat transitions
    // are windows (3,4) -> onset at window 4 x 25 = 100
    const vals: number[] = [];
    for (const v of [800, 400, 200, 200, 200, 200]) {
      for (let i = 0; i < 25; i++) vals.push(v + (i % 5) - 2);
    }
    expect(steadyOnsetIndex(vals)).toBe(100);
  });

  test("a steep drop that then flattens delays onset until flatness holds", () => {
    // window medians: 1000, 100, 10, 10, 10 -> transitions 0.1, 0.1, 1.0, 1.0
    // -> first two consecutive flat transitions are windows (3,4) -> 100
    const vals: number[] = [];
    for (const v of [1000, 100, 10, 10, 10]) {
      for (let i = 0; i < 25; i++) vals.push(v);
    }
    expect(steadyOnsetIndex(vals)).toBe(100);
  });

  test("a series still improving at the cap reports no onset (never faked)", () => {
    const vals: number[] = [];
    let v = 10000;
    for (let w = 0; w < 8; w++) {
      for (let i = 0; i < 25; i++) vals.push(v);
      v = Math.floor(v / 3); // every window improves by ~3x: never flat
    }
    expect(steadyOnsetIndex(vals)).toBeNull();
  });

  test("a series still regressing at the cap reports no onset", () => {
    const vals: number[] = [];
    let v = 10;
    for (let w = 0; w < 8; w++) {
      for (let i = 0; i < 25; i++) vals.push(v);
      v *= 4; // every window regresses 4x: never flat
    }
    expect(steadyOnsetIndex(vals)).toBeNull();
  });

  test("short series (under three windows) cannot reach steady", () => {
    expect(steadyOnsetIndex(Array.from({ length: 60 }, () => 5))).toBeNull();
  });
});

describe("phase labeling", () => {
  test("index 0 is 'first'; pre-onset warming; post-onset steady", () => {
    expect(phaseLabel(0, 50)).toBe("first");
    expect(phaseLabel(49, 50)).toBe("warming");
    expect(phaseLabel(50, 50)).toBe("steady");
    expect(phaseLabel(10, null)).toBe("warming");
  });
});

describe("aggregation over repetitions", () => {
  test("first-request stats, steady stats, and onset median combine across reps", () => {
    const reps = [
      { latenciesUs: [900, ...Array(149).fill(100)], errors: 0 },
      { latenciesUs: [1100, ...Array(149).fill(120)], errors: 1 },
    ];
    const agg = aggregateRamp(reps);
    expect(agg.firstRequest.n).toBe(2);
    expect(agg.firstRequest.p50).toBe(1100); // round(0.5*1) -> upper sample
    expect(agg.steadyOnsetRequest).toBe(50);
    expect(agg.steady.n).toBe(200); // onset 50 of 150 -> 100 per rep x 2
    expect(agg.errors).toBe(1);
  });

  test("reps with no onset contribute neither onset nor steady samples", () => {
    const noOnset: number[] = [];
    let v = 10000;
    for (let w = 0; w < 8; w++) {
      for (let i = 0; i < 25; i++) noOnset.push(v);
      v = Math.floor(v / 3);
    }
    const reps = [
      { latenciesUs: [500, ...Array(149).fill(90)], errors: 0 },
      { latenciesUs: noOnset, errors: 0 },
    ];
    const agg = aggregateRamp(reps);
    expect(agg.steadyOnsetRequest).toBe(50);
    expect(agg.steady.n).toBe(100); // only from rep 0
    expect(agg.firstRequest.n).toBe(2);
    expect(agg.firstRequest.p50).toBe(10000); // noOnset[0] = 10000 (upper sample)
  });
});

describe("stats helpers", () => {
  test("median and percentile follow the cold-start conventions", () => {
    expect(median([3, 1, 2])).toBe(2);
    expect(median([4, 1, 2, 3])).toBe(2.5);
    expect(percentile([], 0.99)).toBe(0);
    const sorted = Array.from({ length: 100 }, (_, i) => i + 1);
    expect(percentile(sorted, 0.5)).toBe(51); // round(0.5 * 99) = 50
    expect(percentile(sorted, 0.99)).toBe(99); // round(0.99 * 99) = 98
  });
});
