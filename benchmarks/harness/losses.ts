/**
 * Honest-loss ledger generator (BETA-003-D).
 *
 * Reads the committed measured evidence — the ramp summary
 * (`benchmarks/raw/ramp/summary.json`, BETA-003-B) and the crossover counts
 * (`benchmarks/raw/ramp/crossover-counts.json`, BETA-003-C) — and
 * mechanically extracts every loss or non-win it can substantiate:
 *
 *   steady-floor    candidate's steady p50 exceeds the best in its class
 *   crossover-never A never overtook B within the recorded horizon
 *   crossover-lag   A only overtook B after N-1 requests (N > 1)
 *   no-onset        a ramp cell never reached steady state (worst case)
 *
 * Nothing here is hand-written per candidate: the same rules run over
 * whatever the evidence contains. The ledger exists so that the public
 * wording cannot cherry-pick wins while omitting losses.
 *
 * Usage: bun losses.ts   -> benchmarks/raw/ramp/losses.json + losses.md
 */

import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const ROOT = import.meta.dir + "/../..";
const RAMP_DIR = `${ROOT}/benchmarks/raw/ramp`;

// ---------- pure, testable extraction ----------

export interface LossRow {
  kind: "steady-floor" | "crossover-never" | "crossover-lag" | "no-onset";
  candidate?: string;
  class?: string;
  detail: string;
  value?: string;
}

interface RampResult {
  candidate: string;
  class: string;
  steady?: { p50?: number };
  steadyOnsetRequest?: number | null;
  errors?: number;
}

interface CrossoverCounts {
  classes: Record<
    string,
    {
      pairs?: Record<string, { crossoverRequest: number | null; horizon: number }>;
      selfAmortization?: Record<string, { steadyMedianUs?: number; selfAmortizationRequest?: number | null }>;
    }
  >;
}

export function extractLosses(counts: CrossoverCounts, rampResults: RampResult[]): LossRow[] {
  const rows: LossRow[] = [];

  // steady-floor losses: per class, any candidate whose steady p50 exceeds
  // the class best (when the ramp summary carries steady floors)
  const floorsByClass = new Map<string, Map<string, number>>();
  for (const r of rampResults) {
    const p50 = r.steady?.p50;
    if (typeof p50 === "number" && p50 > 0) {
      let byCand = floorsByClass.get(r.class);
      if (!byCand) floorsByClass.set(r.class, (byCand = new Map()));
      const prev = byCand.get(r.candidate);
      if (prev === undefined || p50 > prev) byCand.set(r.candidate, p50);
    }
  }
  for (const [klass, byCand] of floorsByClass) {
    const best = Math.min(...byCand.values());
    for (const [cand, p50] of byCand) {
      if (p50 > best) {
        rows.push({
          kind: "steady-floor",
          candidate: cand,
          class: klass,
          value: `${p50}µs vs best ${best}µs (${Math.round((p50 / best) * 100) / 100}x)`,
          detail: `steady p50 is ${Math.round((p50 / best) * 100) / 100}x the class best`,
        });
      }
    }
  }

  // no-onset rows: ramp cells that never reached steady state
  for (const r of rampResults) {
    if (r.steadyOnsetRequest === null || r.steadyOnsetRequest === undefined) {
      rows.push({
        kind: "no-onset",
        candidate: r.candidate,
        class: r.class,
        detail: "no steady onset within the request cap",
      });
    }
  }

  // crossover rows: every ordered pair
  for (const [klass, classOut] of Object.entries(counts.classes ?? {})) {
    for (const [pair, res] of Object.entries(classOut.pairs ?? {})) {
      const [a, b] = pair.split(" vs ");
      if (res.crossoverRequest === null) {
        rows.push({
          kind: "crossover-never",
          candidate: a,
          class: klass,
          detail: `never overtook ${b} within the ${res.horizon}-request horizon`,
          value: `never (${res.horizon} requests)`,
        });
      } else if (res.crossoverRequest > 1) {
        rows.push({
          kind: "crossover-lag",
          candidate: a,
          class: klass,
          detail: `behind ${b} for the first ${res.crossoverRequest - 1} requests`,
          value: `lag ${res.crossoverRequest - 1} requests`,
        });
      }
    }
  }

  return rows;
}

// ---------- rendering ----------

export function renderLossesMd(rows: LossRow[], source: { runId: string; generatedAt: string }): string {
  const lines: string[] = [
    "# Honest-Loss Ledger (BETA-003-D)",
    "",
    "Every loss or non-win substantiated by the committed measured evidence",
    "(ramp summary + crossover counts). Mechanically extracted — the same",
    "rules run over whatever the evidence contains; nothing is omitted or",
    "hand-selected.",
    "",
    `Source run: ${source.runId} (generated ${source.generatedAt})`,
    "",
    "| kind | candidate | class | value | detail |",
    "|---|---|---|---|---|",
  ];
  for (const r of rows) {
    lines.push(`| ${r.kind} | ${r.candidate ?? "-"} | ${r.class ?? "-"} | ${r.value ?? "-"} | ${r.detail} |`);
  }
  lines.push("", `${rows.length} substantiated loss row(s).`, "");
  return lines.join("\n");
}

// ---------- driver ----------

function main() {
  const counts: CrossoverCounts = JSON.parse(readFileSync(join(RAMP_DIR, "crossover-counts.json"), "utf8"));
  const rampSummary = JSON.parse(readFileSync(join(RAMP_DIR, "summary.json"), "utf8"));
  const rows = extractLosses(counts, rampSummary.results ?? []);
  const source = { runId: counts.runId ?? "unknown", generatedAt: new Date().toISOString() };
  const json = {
    format: "velqu-losses-v1",
    source,
    rules: [
      "steady-floor: steady p50 exceeds the class best",
      "crossover-never: A never overtook B within the recorded horizon",
      "crossover-lag: A overtook B only after N-1 requests",
      "no-onset: ramp cell never reached steady state",
    ],
    rows,
  };
  writeFileSync(join(RAMP_DIR, "losses.json"), JSON.stringify(json, null, 2) + "\n");
  const md = renderLossesMd(rows, source);
  writeFileSync(join(RAMP_DIR, "losses.md"), md);
  console.log(md);
  console.log("losses: wrote losses.json + losses.md");
}

if (import.meta.main) {
  main();
}
