/**
 * Real-world benchmark result schema (BETA-001-A).
 *
 * Shared contract between the load generator (load.ts), report generator
 * (report.ts), and validators. A summary is valid only when every
 * workload x concurrency cell is present, percentiles are ordered,
 * error/failure counts are consistent with the raw rows, and the protocol
 * block records environment plus config hashes.
 */

export const REAL_WORLD_SUMMARY_FORMAT = "velqu-realworld-summary-v1";

export interface RealWorldWorkload {
  id: string;
  name: string;
  path: string;
  method: string;
  headers?: Record<string, string>;
  body?: unknown;
  expectedStatus: number;
  description?: string;
}

export interface RealWorldEnvironment {
  bunVersion: string;
  os: string;
  arch: string;
  commit: string;
  nodeVersion?: string | null;
}

export interface RealWorldConfigHashes {
  spec: string;
  workloads: string;
  schema: string;
  seed: string;
  versions: string;
  [key: string]: string;
}

export interface RealWorldCell {
  workload: string;
  concurrency: number;
  totalRequests: number;
  errors: number;
  statusMismatches: number;
  rps: number;
  p50Us: number;
  p95Us: number;
  p99Us: number;
  maxUs: number;
}

export interface RealWorldSummary {
  format: string;
  generatedAt: string;
  baseUrl: string;
  durationSec: number;
  concurrencyLevels: number[];
  environment: RealWorldEnvironment;
  configHashes: RealWorldConfigHashes;
  cells: RealWorldCell[];
  raw: string;
}

export interface RealWorldRawRow {
  workload: string;
  concurrency: number;
  startedAtMs: number;
  latencyUs: number;
  status: number | null;
  ok: boolean;
  error: string | null;
}

const HEX64 = /^[0-9a-f]{64}$/;

export function validateRealWorldSummary(
  summary: RealWorldSummary,
  expectedWorkloads: string[],
  expectedConcurrency: number[],
): string[] {
  const errors: string[] = [];
  const err = (m: string) => errors.push(m);

  if (summary.format !== REAL_WORLD_SUMMARY_FORMAT) {
    err(`format must be ${REAL_WORLD_SUMMARY_FORMAT}, got: ${summary.format}`);
  }
  if (!summary.generatedAt) err("generatedAt is required");
  if (!summary.baseUrl) err("baseUrl is required");
  if (!(summary.durationSec > 0)) err("durationSec must be positive");

  for (const w of expectedWorkloads) {
    for (const c of expectedConcurrency) {
      if (!summary.cells.some((cell) => cell.workload === w && cell.concurrency === c)) {
        err(`missing cell: ${w} c=${c}`);
      }
    }
  }

  for (const cell of summary.cells) {
    const tag = `${cell.workload} c=${cell.concurrency}`;
    if (!(cell.totalRequests > 0)) err(`${tag}: totalRequests must be positive`);
    if (cell.errors < 0 || cell.statusMismatches < 0) err(`${tag}: negative error counts`);
    if (cell.errors + cell.statusMismatches > cell.totalRequests) {
      err(`${tag}: errors + statusMismatches exceed totalRequests`);
    }
    if (!(cell.p50Us <= cell.p95Us)) err(`${tag}: p50Us > p95Us`);
    if (!(cell.p95Us <= cell.p99Us)) err(`${tag}: p95Us > p99Us`);
    if (!(cell.p99Us <= cell.maxUs)) err(`${tag}: p99Us > maxUs`);
    if (!(cell.rps >= 0)) err(`${tag}: rps must be non-negative`);
  }

  const env = summary.environment;
  if (!env?.bunVersion) err("environment.bunVersion is required");
  if (!env?.os) err("environment.os is required");
  if (!env?.arch) err("environment.arch is required");
  if (!env?.commit) err("environment.commit is required");

  const hashes = summary.configHashes;
  for (const key of ["spec", "workloads", "schema", "seed", "versions"]) {
    const v = hashes?.[key];
    if (!v || !HEX64.test(v)) err(`configHashes.${key} must be a sha256 hex digest`);
  }

  if (!summary.raw) err("raw JSONL path is required");
  return errors;
}
