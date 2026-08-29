/**
 * Real-world benchmark load generator (BETA-001-A).
 *
 * Drives every workload in workloads.json against one candidate base URL at
 * each configured concurrency for a fixed duration per cell. Each request row
 * (including errors and status mismatches) is retained in raw JSONL; candidate
 * failures never abort the run — they are recorded and counted.
 *
 * Usage:
 *   bun benchmarks/real-world/load.ts --base-url http://127.0.0.1:3000 \
 *       [--out-dir benchmarks/raw/real-world/run1] [--duration 10] \
 *       [--concurrency 1,10,50,200] [--upstream-url http://127.0.0.1:8791]
 *
 * --upstream-url only replaces the authority of W4 paths (/api/bench/io?ms=N)
 * when the candidate is the upstream itself (used by run.sh smoke).
 */

import { createHash } from "node:crypto";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import {
  REAL_WORLD_SUMMARY_FORMAT,
  type RealWorldCell,
  type RealWorldRawRow,
  type RealWorldSummary,
  type RealWorldWorkload,
  validateRealWorldSummary,
} from "./result-schema";

const DIR = import.meta.dir;

interface WorkloadsFile {
  workloads: RealWorldWorkload[];
  concurrencyLevels: number[];
  durationSec: number;
}

interface Args {
  baseUrl: string;
  outDir: string;
  duration: number;
  concurrency: number[];
  upstreamUrl: string | null;
  workloads: string[] | null;
}

function parseArgs(argv: string[]): Args {
  const get = (name: string): string | null => {
    const i = argv.indexOf(`--${name}`);
    return i >= 0 ? argv[i + 1] : null;
  };
  const baseUrl = get("base-url");
  if (!baseUrl) throw new Error("--base-url is required");
  const durationRaw = Number(get("duration"));
  const concurrency = (get("concurrency") ?? "")
    .split(",")
    .map((s) => Number(s.trim()))
    .filter((n) => n > 0);
  const workloadsRaw = get("workloads");
  return {
    baseUrl: baseUrl.replace(/\/$/, ""),
    outDir: get("out-dir") ?? `${DIR}/raw/smoke`,
    duration: Number.isFinite(durationRaw) && durationRaw > 0 ? durationRaw : 0,
    concurrency,
    upstreamUrl: get("upstream-url"),
    workloads: workloadsRaw
      ? workloadsRaw.split(",").map((s) => s.trim()).filter((s) => s.length > 0)
      : null,
  };
}

function sha256File(path: string): string {
  return createHash("sha256").update(readFileSync(path)).digest("hex");
}

async function gitCommit(): Promise<string> {
  const proc = Bun.spawn(["git", "rev-parse", "HEAD"], { cwd: DIR + "/../..", stdout: "pipe", stderr: "ignore" });
  if ((await proc.exited) !== 0) return "unknown";
  return (await new Response(proc.stdout).text()).trim();
}

async function nodeVersion(): Promise<string | null> {
  const proc = Bun.spawn(["node", "--version"], { stdout: "pipe", stderr: "ignore" });
  if ((await proc.exited) !== 0) return null;
  return (await new Response(proc.stdout).text()).trim() || null;
}

function percentileUs(sortedUs: number[], p: number): number {
  if (sortedUs.length === 0) return 0;
  const idx = Math.min(sortedUs.length - 1, Math.floor((p / 100) * sortedUs.length));
  return Math.round(sortedUs[idx] * 100) / 100;
}

async function runCell(
  workload: RealWorldWorkload,
  concurrency: number,
  durationSec: number,
  baseUrl: string,
  upstreamUrl: string | null,
): Promise<RealWorldRawRow[]> {
  let url = baseUrl + workload.path;
  if (upstreamUrl && workload.path.startsWith("/api/bench/io")) {
    url = upstreamUrl.replace(/\/$/, "") + workload.path.replace("/api/bench/io", "/io");
  }
  const body = workload.method === "POST" ? JSON.stringify(workload.body) : undefined;
  const rows: RealWorldRawRow[] = [];
  const deadline = performance.now() + durationSec * 1000;

  async function worker() {
    while (performance.now() < deadline) {
      const startedAtMs = performance.now();
      let status: number | null = null;
      let error: string | null = null;
      try {
        const res = await fetch(url, {
          method: workload.method,
          headers: workload.headers as Record<string, string>,
          body,
        });
        status = res.status;
        await res.arrayBuffer();
      } catch (e) {
        error = e instanceof Error ? e.message : String(e);
      }
      const latencyUs = Math.round((performance.now() - startedAtMs) * 1000);
      rows.push({
        workload: workload.id,
        concurrency,
        startedAtMs: Math.round(startedAtMs * 1000) / 1000,
        latencyUs,
        status,
        ok: error === null && status === workload.expectedStatus,
        error,
      });
    }
  }

  await Promise.all(Array.from({ length: concurrency }, () => worker()));
  return rows;
}

async function main() {
  const args = parseArgs(process.argv.slice(2));
  const config: WorkloadsFile = JSON.parse(readFileSync(`${DIR}/workloads.json`, "utf8"));
  const durationSec = args.duration || config.durationSec;
  const concurrencyLevels = args.concurrency.length ? args.concurrency : config.concurrencyLevels;

  mkdirSync(args.outDir, { recursive: true });
  const rawPath = `${args.outDir}/raw.jsonl`;
  const allRows: RealWorldRawRow[] = [];
  const cells: RealWorldCell[] = [];

  // Optional workload filter (--workloads W4_1ms,W4_5ms): unknown IDs fail
  // loudly instead of silently running nothing.
  let selected = config.workloads;
  if (args.workloads) {
    const known = new Set(config.workloads.map((w) => w.id));
    for (const id of args.workloads) {
      if (!known.has(id)) throw new Error(`unknown workload id: ${id}`);
    }
    selected = config.workloads.filter((w) => args.workloads!.includes(w.id));
    if (selected.length === 0) throw new Error("workload filter selected nothing");
  }

  for (const workload of selected) {
    for (const c of concurrencyLevels) {
      const rows = await runCell(workload, c, durationSec, args.baseUrl, args.upstreamUrl);
      allRows.push(...rows);
      const sorted = rows.map((r) => r.latencyUs).sort((a, b) => a - b);
      const elapsedSec = durationSec;
      cells.push({
        workload: workload.id,
        concurrency: c,
        totalRequests: rows.length,
        errors: rows.filter((r) => r.error !== null).length,
        statusMismatches: rows.filter((r) => r.error === null && r.status !== workload.expectedStatus).length,
        rps: Math.round((rows.length / elapsedSec) * 100) / 100,
        p50Us: percentileUs(sorted, 50),
        p95Us: percentileUs(sorted, 95),
        p99Us: percentileUs(sorted, 99),
        maxUs: sorted.length ? sorted[sorted.length - 1] : 0,
      });
      console.log(
        `${workload.id} c=${c}: ${rows.length} req, ${cells[cells.length - 1].errors} errors, ` +
          `${cells[cells.length - 1].statusMismatches} mismatches, p50=${cells[cells.length - 1].p50Us}us`,
      );
    }
  }

  writeFileSync(rawPath, allRows.map((r) => JSON.stringify(r)).join("\n") + "\n");

  const summary: RealWorldSummary = {
    format: REAL_WORLD_SUMMARY_FORMAT,
    generatedAt: new Date().toISOString(),
    baseUrl: args.baseUrl,
    durationSec,
    concurrencyLevels,
    environment: {
      bunVersion: Bun.version,
      os: process.platform,
      arch: process.arch,
      commit: await gitCommit(),
      nodeVersion: await nodeVersion(),
    },
    configHashes: {
      spec: sha256File(`${DIR}/SPEC.md`),
      workloads: sha256File(`${DIR}/workloads.json`),
      schema: sha256File(`${DIR}/postgres/schema.sql`),
      seed: sha256File(`${DIR}/postgres/seed.sql`),
      versions: sha256File(`${DIR}/versions.json`),
    },
    cells,
    raw: rawPath,
  };

  const schemaErrors = validateRealWorldSummary(summary, config.workloads.map((w) => w.id), concurrencyLevels);
  if (schemaErrors.length > 0) {
    console.error("load.ts: summary failed result-schema validation:");
    for (const e of schemaErrors) console.error(`  - ${e}`);
  }
  writeFileSync(`${args.outDir}/summary.json`, JSON.stringify(summary, null, 2) + "\n");
  console.log(`load.ts: wrote ${args.outDir}/summary.json (${schemaErrors.length} schema errors)`);
}

await main();
