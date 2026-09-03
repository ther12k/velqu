/**
 * Cumulative crossover request counts (BETA-003-C).
 *
 * Consumes the per-request series recorded by `ramp.ts` (BETA-003-B) and
 * computes, per route class and ordered candidate pair, the smallest request
 * count N at which candidate A's cumulative served time (sum of per-request
 * latencies for requests 1..N, median across repetitions) drops to or below
 * candidate B's. A pair with no such N within the recorded horizon is
 * reported as `neverWithinCap` — never extrapolated.
 *
 * Also reports each candidate's self-amortization point: the first N where
 * its cumulative average latency first comes within 1.25x of its own
 * steady-phase median (the practical "warmup debt paid off" mark).
 *
 * Startup (process spawn -> ready -> first response) is intentionally
 * EXCLUDED from these counts: it is a separate measured quantity owned by
 * the cold-start harness (benchmarks/raw/cold-start). These counts answer
 * only: given a candidate that is already serving, how many requests until
 * cumulative served time overtakes the comparison candidate?
 *
 * Usage: bun crossover.ts                       # newest ramp run
 *        bun crossover.ts --run-id ramp-<ts>
 * Writes crossover-counts.json + crossover-counts.md into the run's
 * directory (benchmarks/raw/ramp/).
 */

import { readFileSync, writeFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

const ROOT = import.meta.dir + "/../..";
const RAMP_DIR = `${ROOT}/benchmarks/raw/ramp`;
const AMORTIZATION_RATIO = 1.25;

// ---------- pure, testable helpers ----------

export function median(nums: number[]): number {
  if (nums.length === 0) return 0;
  const sorted = [...nums].sort((a, b) => a - b);
  const mid = Math.floor(sorted.length / 2);
  return sorted.length % 2 === 1 ? sorted[mid] : (sorted[mid - 1] + sorted[mid]) / 2;
}

/**
 * Median-across-reps cumulative curve: cumMedian[i] = median over reps of
 * (sum of the first i+1 request latencies). Reps may have different lengths;
 * each contributes its prefix sums up to its own length.
 */
export function cumulativeCurves(reps: number[][]): { horizon: number; cumMedian: number[] } {
  const rows = reps.map((rep) => {
    const cum: number[] = [];
    let acc = 0;
    for (const v of rep) {
      acc += v;
      cum.push(acc);
    }
    return cum;
  });
  const horizon = Math.min(...rows.map((r) => r.length));
  const cumMedian: number[] = [];
  for (let i = 0; i < horizon; i++) {
    cumMedian.push(median(rows.map((r) => r[i])));
  }
  return { horizon, cumMedian };
}

/**
 * Smallest N (1-based request count) where cumA drops to or below cumB, or
 * null when A never catches B within the horizon.
 */
export function crossoverCount(
  cumA: number[],
  cumB: number[],
  horizon = Math.min(cumA.length, cumB.length),
): number | null {
  for (let n = 0; n < horizon; n++) {
    if (cumA[n] <= cumB[n]) return n + 1;
  }
  return null;
}

/**
 * Self-amortization point: first N where the cumulative average latency of
 * the first N requests is <= AMORTIZATION_RATIO x the steady median of the
 * whole series, or null when the series never amortizes within its length.
 */
export function selfAmortizationCount(latencies: number[], steadyMedian: number): number | null {
  if (steadyMedian <= 0 || latencies.length === 0) return null;
  const bound = AMORTIZATION_RATIO * steadyMedian;
  let acc = 0;
  for (let n = 0; n < latencies.length; n++) {
    acc += latencies[n];
    if (acc / (n + 1) <= bound) return n + 1;
  }
  return null;
}

// ---------- analysis driver ----------

interface RampRow {
  runId: string;
  candidate: string;
  class: string;
  rep: number;
  requestIndex: number;
  latencyUs: number;
  valid: boolean;
}

function newestRunId(dir: string, requested: string | null): string {
  if (requested) return requested;
  const files = readdirSync(dir).filter((f) => f.startsWith("ramp-") && f.endsWith(".jsonl"));
  if (files.length === 0) throw new Error(`no ramp-*.jsonl runs in ${dir}`);
  files.sort();
  return files[files.length - 1].slice("ramp-".length, -".jsonl".length);
}

function loadSeries(dir: string, runId: string): Map<string, Map<number, number[]>> {
  // class -> candidate -> reps (ordered by rep index, valid requests only)
  const rows = readFileSync(join(dir, `ramp-${runId}.jsonl`), "utf8")
    .trim()
    .split("\n")
    .map((line) => JSON.parse(line) as RampRow)
    .filter((r) => r.runId.replace(/^ramp-/, "") === runId.replace(/^ramp-/, "") && r.valid);
  const byClass = new Map<string, Map<number, number[]>>();
  for (const r of rows) {
    let byCand = byClass.get(r.class);
    if (!byCand) byCand = byClass.set(r.class, new Map()).get(r.class)!;
    let reps = byCand.get(r.candidate);
    if (!reps) reps = byCand.set(r.candidate, []).get(r.candidate)!;
    while (reps.length <= r.rep) reps.push([]);
    reps[r.rep][r.requestIndex] = r.latencyUs;
  }
  for (const byCand of byClass.values()) {
    for (const [cand, reps] of byCand) {
      byCand.set(cand, reps.map((rep) => rep.filter((v) => typeof v === "number")));
    }
  }
  return byClass;
}

function fmtPair(a: string, b: string): string {
  return `${a} vs ${b}`;
}

function main() {
  const argv = process.argv.slice(2);
  const runId = newestRunId(RAMP_DIR, argv.includes("--run-id") ? argv[argv.indexOf("--run-id") + 1] : null);
  const byClass = loadSeries(RAMP_DIR, runId);
  const out: Record<string, unknown> = {
    format: "velqu-crossover-counts-v1",
    runId,
    generatedAt: new Date().toISOString(),
    amortizationRatio: AMORTIZATION_RATIO,
    startupExcluded: "process spawn->ready->first response is a cold-start-harness quantity; these counts cover serving only",
    classes: {},
  };
  const mdLines: string[] = [
    `# Cumulative Crossover Request Counts (${runId})`,
    "",
    "Source: per-request series from `ramp.ts` (BETA-003-B), valid requests,",
    "median across reps of cumulative served time. Startup time is excluded",
    "(owned by the cold-start harness). `never` = A never reached B within the",
    "recorded horizon — never extrapolated.",
    "",
  ];

  for (const [klass, byCand] of byClass) {
    const candidates = [...byCand.keys()].sort();
    const classOut: Record<string, unknown> = { pairs: {}, selfAmortization: {} };
    const md = [
      `## Class ${klass}`,
      "",
      "| pair (A vs B) | N* (requests) | horizon |",
      "|---|---:|---:|",
    ];

    // self amortization per candidate
    const selfOut: Record<string, unknown> = {};
    for (const cand of candidates) {
      const reps = byCand.get(cand)!;
      const all = reps.flat();
      const steadyMedian = median(reps.map((rep) => median(rep.slice(Math.floor(rep.length / 2)))));
      const n = selfAmortizationCount(all, steadyMedian);
      selfOut[cand] = { steadyMedianUs: Math.round(steadyMedian * 100) / 100, selfAmortizationRequest: n };
      md.push(`Self-amortization (${cand}): steady median ${Math.round(steadyMedian)}µs -> request ${n ?? "never"}`);
    }
    classOut.selfAmortization = selfOut;

    // pairwise crossover
    for (const a of candidates) {
      for (const b of candidates) {
        if (a === b) continue;
        const curveA = cumulativeCurves(byCand.get(a)!);
        const curveB = cumulativeCurves(byCand.get(b)!);
        const n = crossoverCount(curveA.cumMedian, curveB.cumMedian);
        classOut.pairs[fmtPair(a, b)] = {
          crossoverRequest: n,
          horizon: Math.min(curveA.horizon, curveB.horizon),
        };
        md.push(`| ${a} vs ${b} | ${n ?? "never"} | ${Math.min(curveA.horizon, curveB.horizon)} |`);
      }
    }
    md.push("");
    (out.classes as Record<string, unknown>)[klass] = classOut;
    mdLines.push(...md);
  }

  writeFileSync(join(RAMP_DIR, "crossover-counts.json"), JSON.stringify(out, null, 2) + "\n");
  writeFileSync(join(RAMP_DIR, "crossover-counts.md"), mdLines.join("\n") + "\n");
  console.log(mdLines.join("\n"));
  console.log(`crossover: wrote crossover-counts.json + crossover-counts.md (run ${runId})`);
}

if (import.meta.main) {
  main();
}
