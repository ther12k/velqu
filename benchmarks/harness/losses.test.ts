/**
 * Honest-loss ledger unit tests (BETA-003-D) — deterministic coverage of
 * the extraction rules over synthetic evidence. No measured files needed.
 */
import { describe, expect, test } from "bun:test";
import { extractLosses, renderLossesMd, type CrossoverCounts, type RampResult } from "./losses";

const counts: CrossoverCounts = {
  classes: {
    C0: {
      pairs: {
        "velqu vs jit": { crossoverRequest: 76, horizon: 100 },
        "jit vs velqu": { crossoverRequest: null, horizon: 100 },
        "a vs b": { crossoverRequest: 1, horizon: 100 },
      },
    },
  },
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
} as any;

const rampResults: RampResult[] = [
  { candidate: "velqu", class: "C0", steady: { p50: 34 }, steadyOnsetRequest: 100, errors: 0 },
  { candidate: "jit", class: "C0", steady: { p50: 68 }, steadyOnsetRequest: null, errors: 0 },
];

describe("loss extraction rules (BETA-003-D)", () => {
  test("steady-floor loss is reported with the multiplier against the class best", () => {
    const rows = extractLosses(counts, rampResults);
    const floor = rows.find((r) => r.kind === "steady-floor");
    expect(floor).toBeDefined();
    expect(floor!.candidate).toBe("jit");
    expect(floor!.value).toBe("68µs vs best 34µs (2x)");
  });

  test("no-onset cells are reported as their own kind", () => {
    const rows = extractLosses(counts, rampResults);
    const onset = rows.find((r) => r.kind === "no-onset");
    expect(onset).toBeDefined();
    expect(onset!.candidate).toBe("jit");
  });

  test("crossover-never and crossover-lag rows carry pair and horizon", () => {
    const rows = extractLosses(counts, rampResults);
    const never = rows.find((r) => r.kind === "crossover-never");
    expect(never).toBeDefined();
    expect(never!.candidate).toBe("jit");
    expect(never!.detail).toContain("never overtook velqu within the 100-request horizon");
    const lag = rows.find((r) => r.kind === "crossover-lag");
    expect(lag).toBeDefined();
    expect(lag!.candidate).toBe("velqu");
    expect(lag!.value).toBe("lag 75 requests");
  });

  test("an immediate crossover (N=1) is not a loss row", () => {
    const rows = extractLosses(counts, rampResults);
    expect(rows.some((r) => r.kind === "crossover-lag" && r.candidate === "a")).toBe(false);
  });

  test("candidates at the class best get no floor row", () => {
    const rows = extractLosses(counts, rampResults);
    expect(rows.some((r) => r.kind === "steady-floor" && r.candidate === "velqu")).toBe(false);
  });
});

describe("rendering", () => {
  test("every extracted row appears in the markdown ledger", () => {
    const rows = extractLosses(counts, rampResults);
    const md = renderLossesMd(rows, { runId: "ramp-test", generatedAt: "2026-09-03T00:00:00Z" });
    for (const r of rows) {
      expect(md).toContain(r.kind);
      expect(md).toContain(r.detail);
    }
    expect(md).toContain("4 substantiated loss row(s)");
  });
});
