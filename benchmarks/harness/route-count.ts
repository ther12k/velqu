/**
 * Route-count cold-start scaling (PERF-005): 25 vs 1000 equivalent routes.
 * Measured request: GET /res7/item/7 → {"id":7,"n":N}. Fresh process per
 * sample; same protocol as cold-start.ts.
 */
const ROOT = import.meta.dir + "/../..";

const N_VALUES = [25, 100, 1000, 5000, 10000] as const;

interface Cand {
  id: string;
  spawn: (port: number, n: number) => { cmd: string; args: string[]; env: Record<string, string> };
}

const CANDS: Cand[] = [
  {
    id: "velqu (source)",
    spawn: (port, n) => ({
      cmd: `${ROOT}/target/release/velqu-runtime`,
      args: ["--pack", `${ROOT}/benchmarks/raw/packs/app-${n}.qpack`, "--port", String(port)],
      env: {},
    }),
  },
  {
    id: "velqu (bytecode)",
    spawn: (port, n) => ({
      cmd: `${ROOT}/target/release/velqu-runtime`,
      args: ["--pack", `${ROOT}/benchmarks/raw/packs/app-${n}-bc.qpack`, "--port", String(port)],
      env: {},
    }),
  },
  {
    id: "raw-bun",
    spawn: (port, n) => ({ cmd: "bun", args: [`${ROOT}/baselines/raw-bun/server.ts`], env: { PORT: String(port), N_ROUTES: String(n) } }),
  },
  {
    id: "elysia2",
    spawn: (port, n) => ({ cmd: "bun", args: [`${ROOT}/baselines/elysia2/server.ts`], env: { PORT: String(port), N_ROUTES: String(n) } }),
  },
];

function freePort(): number {
  const l = Bun.listen({ hostname: "127.0.0.1", port: 0, socket: { data() {}, open() {} } });
  const port = l.port;
  l.stop(true);
  return port;
}

interface SampleResult {
  totalMs: number;
  rssKb: number | null;
  ok: boolean;
  err?: string;
  // M26-010-D: per-stage startup timings from the runtime's own ready
  // line (velqu candidates only; null otherwise)
  stages?: Record<string, number> | null;
}

async function sample(cand: Cand, n: number): Promise<SampleResult> {
  const port = freePort();
  const { cmd, args, env } = cand.spawn(port, n);
  const t0 = performance.now();
  const proc = Bun.spawn([cmd, ...args], { env: { ...process.env, ...env }, stdout: "pipe", stderr: "ignore" });
  // drain stdout line-by-line, keeping the first ready line's stages
  let stages: Record<string, number> | null = null;
  const drain = (async () => {
    try {
      const text = await new Response(proc.stdout).text();
      for (const line of text.split("\n")) {
        if (stages) break;
        const t = line.trim();
        if (!t.startsWith("{")) continue;
        try {
          const j = JSON.parse(t);
          if (j?.event === "ready" && Array.isArray(j.stages)) {
            stages = Object.fromEntries(j.stages.map((x: { stage: string; ms: number }) => [x.stage, x.ms]));
          }
        } catch {}
      }
    } catch {}
  })();
  try {
    const deadline = performance.now() + 20_000;
    let ready = false;
    while (performance.now() < deadline) {
      try {
        const c = await Bun.connect({ hostname: "127.0.0.1", port, socket: { data() {}, open() {}, close() {}, error() {} } });
        c.end?.();
        c.terminate?.();
        ready = true;
        break;
      } catch {
        await Bun.sleep(0.5);
      }
    }
    if (!ready) throw new Error("not ready in 20s");
    // first valid response on the measured route
    const expected = JSON.stringify({ id: 7, n });
    const fetchDeadline = performance.now() + 10_000;
    while (performance.now() < fetchDeadline) {
      try {
        const res = await fetch(`http://127.0.0.1:${port}/res7/item/7`);
        const body = await res.text();
        const totalMs = performance.now() - t0;
        const ok = res.status === 200 && body === expected;
        let rssKb: number | null = null;
        try {
          const st = await Bun.file(`/proc/${proc.pid}/status`).text();
          rssKb = parseInt(st.match(/VmRSS:\s+(\d+) kB/)?.[1] ?? "0", 10) || null;
        } catch {}
        proc.kill();
        await proc.exited;
        await drain;
        if (ok) return { totalMs, rssKb, ok, stages };
        return { totalMs, rssKb, ok: false, err: `body=${body.slice(0, 60)}`, stages };
      } catch {
        await Bun.sleep(0.5);
      }
    }
    throw new Error("no response");
  } catch (e) {
    proc.kill();
    await proc.exited;
    await drain;
    return { totalMs: performance.now() - t0, rssKb: null, ok: false, err: String(e), stages };
  }
}

const SAMPLES = parseInt(process.argv.find((a) => a.startsWith("--samples="))?.slice(10) ?? "40", 10);
const runId = process.env.ROUTE_COUNT_RUN_ID ?? `route-count-${Date.now()}`;
let seed = Number(process.env.ROUTE_COUNT_SEED ?? Date.now()) >>> 0;
console.log(`route-count: ${CANDS.length} candidates × ${N_VALUES.length} sizes × ${SAMPLES} samples (run=${runId}, seed=${seed})`);

// warm harness
for (let i = 0; i < 3; i++) await sample(CANDS[1], 25);

// M26-010-C: SAMPLE-LEVEL global shuffle. Every (candidate, size,
// sample-index) triple is one job; all are shuffled together so no
// candidate's samples run consecutively — thermal drift, cache state,
// and time-correlated host noise spread across all cells instead of
// biasing whichever cell would otherwise run last.
const jobs = CANDS.flatMap((cand) =>
  N_VALUES.flatMap((n) => Array.from({ length: SAMPLES }, (_, i) => ({ cand, n, i }))),
);
for (let i = jobs.length - 1; i > 0; i--) {
  seed = (seed * 1664525 + 1013904223) >>> 0;
  const j = seed % (i + 1);
  [jobs[i], jobs[j]] = [jobs[j], jobs[i]];
}
const cellRows: Array<Record<string, unknown>> = [];
const cells = new Map<string, { cand: (typeof CANDS)[number]; n: number; rows: Array<{ totalMs: number; rssKb: number | null }>; stages: Array<Record<string, number>>; failures: number }>();
const raw: string[] = [];
let executionOrder = 0;
for (const { cand, n, i } of jobs) {
  const key = `${cand.id}|${n}`;
  let cell = cells.get(key);
  if (!cell) {
    cell = { cand, n, rows: [], stages: [], failures: 0 };
    cells.set(key, cell);
  }
  {
    const r = await sample(cand, n);
    raw.push(JSON.stringify({ runId, executionOrder: executionOrder++, candidate: cand.id, n, i, totalMs: Math.round(r.totalMs * 1000) / 1000, rssKb: r.rssKb, ok: r.ok, err: r.err, ...(r.stages ? { stages: r.stages } : {}) }));
    if (r.ok) {
      cell.rows.push({ totalMs: r.totalMs, rssKb: r.rssKb });
      if (r.stages) cell.stages.push(r.stages);
    } else cell.failures++;
  }
  // a cell's stats are computed and logged exactly once, when its last
  // sample lands (samples arrive interleaved after the global shuffle)
  if (cell.rows.length + cell.failures !== SAMPLES) continue;
  const rows = cell.rows;
  const failures = cell.failures;
  const sorted = rows.map((r) => r.totalMs).sort((a, b) => a - b);
  const p = (q: number) => Math.round((sorted[Math.min(sorted.length - 1, Math.round(q * (sorted.length - 1)))] ?? 0) * 1000) / 1000;
  const rssSorted = rows.map((r) => r.rssKb ?? 0).filter((x) => x > 0).sort((a, b) => a - b);
  const pRss = (q: number) => rssSorted[Math.min(rssSorted.length - 1, Math.round(q * (rssSorted.length - 1)))] ?? 0;
  // M26-010-D: per-stage p50 across the cell's captured ready lines
  const stageNames = [...new Set(cell.stages.flatMap((st) => Object.keys(st)))];
  const stageP50: Record<string, number> = {};
  for (const name of stageNames) {
    const xs = cell.stages.map((st) => st[name]).filter((x) => typeof x === "number").sort((a, b) => a - b);
    if (xs.length) stageP50[name] = Math.round(xs[Math.min(xs.length - 1, Math.round(0.5 * (xs.length - 1)))] * 1000) / 1000;
  }
  const cellResults = { runId, candidate: cand.id, routes: n, samples: rows.length, requestedSamples: SAMPLES, failures, p50Ms: p(0.5), p95Ms: p(0.95), p99Ms: p(0.99), rssP50Kb: pRss(0.5), rssP95Kb: pRss(0.95), ...(cell.stages.length ? { stageP50Ms: stageP50 } : {}) };
  cellRows.push(cellResults);
  console.log(`${cand.id} n=${n}: p50=${p(0.5)}ms p95=${p(0.95)}ms p99=${p(0.99)}ms rss=${pRss(0.5)}kB failures=${failures}${cell.stages.length ? ` stages=${JSON.stringify(stageP50)}` : ""}`);
}
const results = cellRows;
// scaling deltas
for (const cand of CANDS) {
  const r25 = results.find((r) => r.candidate === cand.id && r.routes === 25)!;
  const r1000 = results.find((r) => r.candidate === cand.id && r.routes === 1000)!;
  const delta = ((r1000.p50Ms as number) / (r25.p50Ms as number) - 1) * 100;
  console.log(`${cand.id}: 1000-route vs 25-route p50 delta = ${delta.toFixed(1)}%`);
}
import { mkdirSync, writeFileSync } from "node:fs";
import { readFileSync } from "node:fs";
import { createHash } from "node:crypto";
mkdirSync(`${ROOT}/benchmarks/raw/route-count`, { recursive: true });
const stamp = Date.now();
const rawPath = `${ROOT}/benchmarks/raw/route-count/route-count-${stamp}.jsonl`;
const rawRelative = `benchmarks/raw/route-count/route-count-${stamp}.jsonl`;
function sha256File(path: string): string {
  try {
    return createHash("sha256").update(readFileSync(path)).digest("hex");
  } catch {
    return "missing";
  }
}
writeFileSync(rawPath, raw.join("\n") + "\n");
writeFileSync(
  `${ROOT}/benchmarks/raw/route-count/summary.json`,
  JSON.stringify({
    format: "velqu-route-count-v4-full-metrics",
    runId,
    seed,
    samples: SAMPLES,
    randomizedCandidateOrder: true,
    sampleOrderRandomized: true,
    generatedAt: new Date().toISOString(),
    // M26-010-D: raw evidence self-identifies its binaries/packs
    // (benchmarks/manifest.json remains the canonical release record)
    binaryHashes: { "target/release/velqu-runtime": sha256File(`${ROOT}/target/release/velqu-runtime`) },
    packHashes: Object.fromEntries(
      N_VALUES.flatMap((n) =>
        ([`app-${n}.qpack`, `app-${n}-bc.qpack`] as const).map((f) => [
          f,
          sha256File(`${ROOT}/benchmarks/raw/packs/${f}`),
        ]),
      ),
    ),
    raw: rawRelative,
    results,
  }, null, 2),
);
console.log("wrote benchmarks/raw/route-count/");
