/**
 * C3 matched before/after diagnostic benchmark.
 *
 * Compares the pre-C3 request-store lifecycle with the slotless validated-input
 * path using the same proof pack, endpoint, duration, warmup, and concurrency.
 * This is measurement evidence only; C3 remains a real JS handler workload.
 */
import { mkdirSync, readFileSync, writeFileSync, existsSync } from "node:fs";

const ROOT = import.meta.dir + "/../..";
const OUT_DIR = process.env.C3PROBE_OUT ?? `${ROOT}/benchmarks/raw/c3-probe`;
const PACK = process.env.C3PROBE_PACK ?? `${ROOT}/examples/proof/dist/app.qpack`;
const RUNTIME_OLD = process.env.C3PROBE_OLD ?? `${ROOT}/target/c3-old/release/velqu-runtime`;
const RUNTIME_NEW = process.env.C3PROBE_NEW ?? `${ROOT}/target/c3-new/release/velqu-runtime`;
const DURATION = parseInt(process.env.C3PROBE_DURATION ?? "5", 10);
const REPS = parseInt(process.env.C3PROBE_REPS ?? "3", 10);
const CONC = (process.env.C3PROBE_CONC ?? "1,10,50").split(",").map((x) => parseInt(x.trim(), 10));
const RUN_ID = process.env.C3PROBE_RUN_ID ?? `c3probe-${Date.now()}`;

interface Candidate { id: string; binary: string; commit: string; }
const CANDIDATES: Candidate[] = [
  { id: "pre-c3-request-store", binary: RUNTIME_OLD, commit: process.env.C3PROBE_OLD_COMMIT ?? "unknown" },
  { id: "c3-slotless-prevalidated", binary: RUNTIME_NEW, commit: process.env.C3PROBE_NEW_COMMIT ?? "working-tree" },
];

function freePort(): number {
  const listener = Bun.listen({ hostname: "127.0.0.1", port: 0, socket: { data() {}, open() {} } });
  const port = listener.port;
  listener.stop(true);
  return port;
}

async function waitReady(port: number) {
  const deadline = performance.now() + 15_000;
  while (performance.now() < deadline) {
    try {
      const r = await fetch(`http://127.0.0.1:${port}/health/live`);
      if (r.status === 200) return;
    } catch {}
    await Bun.sleep(10);
  }
  throw new Error("runtime never became ready");
}

function readStage(path: string): Record<string, { ns: number; n: number }> {
  if (!existsSync(path)) return {};
  try { return JSON.parse(readFileSync(path, "utf8")).stages ?? {}; } catch { return {}; }
}

async function measure(candidate: Candidate, concurrency: number, stageDir: string) {
  const port = freePort();
  mkdirSync(stageDir, { recursive: true });
  const proc = Bun.spawn(
    [candidate.binary, "--pack", PACK, "--port", String(port), "--log", "off"],
    { env: { ...process.env, BENCH_STAGE_DIR: stageDir }, stdout: "ignore", stderr: "ignore" },
  );
  try {
    await waitReady(port);
    const url = `http://127.0.0.1:${port}/hello/Rafi`;
    for (let i = 0; i < 300; i++) await fetch(url).catch(() => undefined);
    const latenciesUs: number[] = [];
    let errors = 0;
    let completed = 0;
    const end = performance.now() + DURATION * 1000;
    const t0 = performance.now();
    await Promise.all(Array.from({ length: concurrency }, async () => {
      while (performance.now() < end) {
        const start = performance.now();
        try {
          const response = await fetch(url);
          if (response.status === 200) latenciesUs.push((performance.now() - start) * 1000);
          else errors++;
        } catch { errors++; }
        completed++;
      }
    }));
    const elapsed = (performance.now() - t0) / 1000;
    await Bun.sleep(2300);
    let rssKb = 0;
    try {
      const status = await Bun.file(`/proc/${proc.pid}/status`).text();
      rssKb = parseInt(status.match(/VmRSS:\s+(\d+) kB/)?.[1] ?? "0", 10);
    } catch {}
    latenciesUs.sort((a, b) => a - b);
    const quantile = (q: number) => latenciesUs[Math.min(latenciesUs.length - 1, Math.round(q * (latenciesUs.length - 1)))] ?? 0;
    return {
      rps: Math.round(completed / elapsed),
      p50Us: Math.round(quantile(0.50) * 10) / 10,
      p95Us: Math.round(quantile(0.95) * 10) / 10,
      p99Us: Math.round(quantile(0.99) * 10) / 10,
      errors,
      totalRequests: completed,
      rssKb,
      stages: readStage(`${stageDir}/stage-timing-qengine.json`),
      serveStages: readStage(`${stageDir}/stage-timing-serve.json`),
    };
  } finally {
    proc.kill();
    await proc.exited;
  }
}

const results: Record<string, unknown>[] = [];
// Balanced interleaved pairing (v3): within each repetition the old/new
// pair runs back-to-back per concurrency level, and the ORDER alternates
// per repetition (odd reps old→new, even reps new→old) so ambient drift
// (thermal, background load) cannot systematically favor one candidate.
// Paired ratios are computed from matched (repetition, concurrency) pairs.
for (let repetition = 1; repetition <= REPS; repetition++) {
  const order = repetition % 2 === 1 ? CANDIDATES : [...CANDIDATES].slice().reverse();
  for (const concurrency of CONC) {
    for (let orderInCell = 0; orderInCell < order.length; orderInCell++) {
      const candidate = order[orderInCell];
      const stageDir = `${OUT_DIR}/${RUN_ID}/${candidate.id}-c${concurrency}-r${repetition}`;
      const result = await measure(candidate, concurrency, stageDir);
      const row = { runId: RUN_ID, candidate: candidate.id, commit: candidate.commit, routeId: "C3", path: "/hello/Rafi", concurrency, repetition, orderInCell, durationSec: DURATION, ...result };
      results.push(row);
      console.log(`rep${repetition} pos${orderInCell} ${candidate.id} c=${concurrency}: ${result.rps} req/s p50=${result.p50Us}us p95=${result.p95Us}us errors=${result.errors}`);
    }
  }
}

function median(values: number[]): number {
  const sorted = [...values].sort((a, b) => a - b);
  const mid = Math.floor(sorted.length / 2);
  return sorted.length % 2 === 1 ? sorted[mid] : (sorted[mid - 1] + sorted[mid]) / 2;
}

const aggregate: Record<string, unknown>[] = [];
// Median per-stage µs/op (stage files hold cumulative {ns, n}; ns/n is the
// per-operation mean including the 300-request warmup, identically for both
// candidates) — surfaces which engine stages the candidate removes/shrinks.
function medianStageUs(cells: Record<string, unknown>[], key: "stages" | "serveStages"): Record<string, number> {
  const perStage: Record<string, number[]> = {};
  for (const cell of cells) {
    const stages = (cell[key] ?? {}) as Record<string, { ns: number; n: number }>;
    for (const [name, v] of Object.entries(stages)) {
      if (v && typeof v.ns === "number" && v.n > 0) (perStage[name] ??= []).push(v.ns / v.n / 1000);
    }
  }
  const out: Record<string, number> = {};
  for (const [name, vals] of Object.entries(perStage)) out[name] = Math.round(median(vals) * 100) / 100;
  return out;
}
for (const candidate of CANDIDATES) {
  for (const concurrency of CONC) {
    const cells = results.filter((r) => r.candidate === candidate.id && r.concurrency === concurrency);
    if (cells.length === 0) continue;
    aggregate.push({
      candidate: candidate.id,
      concurrency,
      reps: cells.length,
      medianRps: Math.round(median(cells.map((c) => c.rps as number))),
      medianP50Us: Math.round(median(cells.map((c) => c.p50Us as number)) * 10) / 10,
      medianP95Us: Math.round(median(cells.map((c) => c.p95Us as number)) * 10) / 10,
      medianP99Us: Math.round(median(cells.map((c) => c.p99Us as number)) * 10) / 10,
      totalErrors: cells.reduce((sum, c) => sum + (c.errors as number), 0),
      stageUs: medianStageUs(cells, "stages"),
      serveStageUs: medianStageUs(cells, "serveStages"),
    });
  }
}

// Matched-pair ratios: new/old rps within each (repetition, concurrency)
// pair — order-independent under balanced alternation, and robust to
// between-repetition drift that pooled medians can hide.
const pairedRatios: Record<string, unknown>[] = [];
for (const concurrency of CONC) {
  const ratios: number[] = [];
  for (let repetition = 1; repetition <= REPS; repetition++) {
    const o = results.find((r) => r.candidate === CANDIDATES[0].id && r.concurrency === concurrency && r.repetition === repetition) as { rps: number } | undefined;
    const n = results.find((r) => r.candidate === CANDIDATES[1].id && r.concurrency === concurrency && r.repetition === repetition) as { rps: number } | undefined;
    if (o && n && o.rps > 0) ratios.push(Math.round((n.rps / o.rps) * 10000) / 10000);
  }
  if (ratios.length > 0) {
    pairedRatios.push({
      concurrency,
      pairs: ratios.length,
      medianRatio: Math.round(median(ratios) * 10000) / 10000,
      minRatio: Math.round(Math.min(...ratios) * 10000) / 10000,
      maxRatio: Math.round(Math.max(...ratios) * 10000) / 10000,
      ratios,
    });
  }
}

mkdirSync(OUT_DIR, { recursive: true });
const raw = `${OUT_DIR}/${RUN_ID}.jsonl`;
writeFileSync(raw, results.map((row) => JSON.stringify(row)).join("\n") + "\n");
writeFileSync(`${OUT_DIR}/${RUN_ID}.summary.json`, JSON.stringify({
  format: "velqu-c3-matched-diagnostic-v3-balanced",
  diagnosticOnly: true,
  runId: RUN_ID,
  hostLabel: process.env.C3PROBE_LABEL ?? "unspecified",
  route: { id: "C3", path: "/hello/Rafi", dynamicJsHandler: true },
  pack: PACK,
  durationSec: DURATION,
  repetitions: REPS,
  concurrency: CONC,
  pairing: "balanced interleaved (old/new back-to-back per cell; order alternates per repetition: odd old→new, even new→old)",
  candidates: CANDIDATES,
  raw: `benchmarks/raw/c3-probe/${RUN_ID}.jsonl`,
  aggregate,
  pairedRatios,
  results,
}, null, 2));
console.log(`wrote ${raw}`);
for (const row of aggregate) {
  console.log(`AGG ${row.candidate} c=${row.concurrency}: median ${row.medianRps} req/s p50=${row.medianP50Us}us p95=${row.medianP95Us}us errors=${row.totalErrors}`);
}
for (const row of pairedRatios) {
  console.log(`PAIRED c=${row.concurrency}: median ratio ${row.medianRatio} (n=${row.pairs}, min ${row.minRatio}, max ${row.maxRatio})`);
}
