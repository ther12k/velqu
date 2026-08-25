/**
 * Real-world benchmark report generator (BETA-001-A).
 *
 * Renders benchmarks/real-world/report.md from one summary.json. Deterministic:
 * the same summary always produces the same markdown (generatedAt and other
 * environment fields are echoed, not regenerated). Failures are retained —
 * cells with errors or status mismatches get a dedicated section.
 *
 * Usage:
 *   bun benchmarks/real-world/report.ts --summary path/to/summary.json
 *       [--out path/to/report.md]
 */

import { readFileSync, writeFileSync } from "node:fs";
import type { RealWorldSummary } from "./result-schema";

const DIR = import.meta.dir;

function main() {
  const argv = process.argv.slice(2);
  const get = (name: string): string | null => {
    const i = argv.indexOf(`--${name}`);
    return i >= 0 ? argv[i + 1] : null;
  };
  const summaryPath = get("summary");
  if (!summaryPath) throw new Error("--summary is required");
  const outPath = get("out") ?? `${DIR}/report.md`;

  const summary: RealWorldSummary = JSON.parse(readFileSync(summaryPath, "utf8"));
  writeFileSync(outPath, renderReport(summary));
  console.log(`report.ts: wrote ${outPath}`);
}

export function renderReport(summary: RealWorldSummary): string {
  const lines: string[] = [];
  const workloads = [...new Set(summary.cells.map((c) => c.workload))];

  lines.push("# Real-World Benchmark Report", "");
  lines.push(`Generated from \`${summary.raw}\` at ${summary.generatedAt}.`, "");
  lines.push(
    `Scope: candidate \`${summary.baseUrl}\`, ${summary.durationSec}s cells, concurrency ` +
      `${summary.concurrencyLevels.join("/")}, per ${summary.format}.`,
    "",
  );

  for (const wl of workloads) {
    lines.push(`## ${wl}`, "");
    lines.push("| c | requests | rps | p50 (µs) | p95 (µs) | p99 (µs) | max (µs) | errors | status mismatches |");
    lines.push("|---:|---:|---:|---:|---:|---:|---:|---:|---:|");
    for (const cell of summary.cells.filter((c) => c.workload === wl)) {
      lines.push(
        `| ${cell.concurrency} | ${cell.totalRequests} | ${cell.rps} | ${cell.p50Us} | ${cell.p95Us} | ` +
          `${cell.p99Us} | ${cell.maxUs} | ${cell.errors} | ${cell.statusMismatches} |`,
      );
    }
    lines.push("");
  }

  const failing = summary.cells.filter((c) => c.errors > 0 || c.statusMismatches > 0);
  lines.push("## Retained failures", "");
  if (failing.length === 0) {
    lines.push("No request errors or status mismatches in this run.", "");
  } else {
    for (const cell of failing) {
      lines.push(
        `- ${cell.workload} c=${cell.concurrency}: ${cell.errors} errors, ` +
          `${cell.statusMismatches} status mismatches out of ${cell.totalRequests} requests (raw rows retained).`,
      );
    }
    lines.push("");
  }

  lines.push("## Protocol", "");
  lines.push("```text");
  lines.push(`bun ${summary.environment.bunVersion}`);
  lines.push(`os ${summary.environment.os} / arch ${summary.environment.arch}`);
  lines.push(`commit ${summary.environment.commit}`);
  for (const [name, hash] of Object.entries(summary.configHashes)) {
    lines.push(`${name} sha256 ${hash}`);
  }
  lines.push("```", "");
  return lines.join("\n");
}

if (import.meta.main) {
  main();
}
