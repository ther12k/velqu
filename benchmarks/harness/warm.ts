/**
 * Warm-load benchmark harness (benchmark-methodology.md §Warm-load protocol).
 *
 * Runs throughput & latency measurements across all 4 candidates with fixed
 * concurrency (c=1, c=10, c=50) after a warm-up phase.
 * Produces benchmarks/raw/warm/summary.json and raw JSONL records.
 */

import { mkdirSync, writeFileSync } from "node:fs";

const ROOT = import.meta.dir + "/../..";

interface Candidate {
  id: string;
  spawn: (port: number) => { cmd: string; args: string[]; env: Record<string, string>; cwd?: string };
}

const CANDIDATES: Candidate[] = [
  {
    id: "velqu",
    spawn: (port) => ({
      cmd: `${ROOT}/target/release/velqu-runtime`,
      args: ["--pack", `${ROOT}/examples/proof/dist/app.qpack`, "--port", String(port)],
      env: {},
    }),
  },
  {
    id: "raw-rust",
    spawn: (port) => ({
      cmd: `${ROOT}/baselines/raw-rust/target/release/velqu-baseline-raw-rust`,
      args: [],
      env: { PORT: String(port) },
    }),
  },
  {
    id: "raw-bun",
    spawn: (port) => ({
      cmd: "bun",
      args: [`${ROOT}/baselines/raw-bun/server.ts`],
      env: { PORT: String(port) },
    }),
  },
  {
    id: "elysia2",
    spawn: (port) => ({
      cmd: "bun",
      args: [`server.ts`],
      cwd: `${ROOT}/baselines/elysia2`,
      env: { PORT: String(port) },
    }),
  },
];

const ROUTES = [
  { id: "C0", path: "/health/live" },
  { id: "C1", path: "/js-text" },
  { id: "C2", path: "/js-json" },
  { id: "C3", path: "/hello/Rafi" },
];

function freePort(): number {
  const l = Bun.listen({ hostname: "127.0.0.1", port: 0, socket: { data() {}, open() {} } });
  const port = l.port;
  l.stop(true);
  return port;
}

async function runWarmLoad(
  cand: Candidate,
  route: { id: string; path: string },
  concurrency: number,
  totalRequests: number,
): Promise<{ rps: number; p50Us: number; p95Us: number; p99Us: number; errors: number; rssKb: number }> {
  const port = freePort();
  const { cmd, args, env, cwd } = cand.spawn(port);
  const proc = Bun.spawn([cmd, ...args], { env: { ...process.env, ...env }, cwd, stdout: "ignore", stderr: "ignore" });

  try {
    // Wait ready
    const deadline = performance.now() + 10_000;
    while (performance.now() < deadline) {
      try {
        const c = await Bun.connect({ hostname: "127.0.0.1", port, socket: { data() {}, open() {} } });
        c.end?.();
        c.terminate?.();
        break;
      } catch {
        await Bun.sleep(5);
      }
    }

    const url = `http://127.0.0.1:${port}${route.path}`;

    // Warmup (100 requests)
    for (let i = 0; i < 100; i++) {
      try { await fetch(url); } catch {}
    }

    // Measured load with worker pool
    const latenciesUs: number[] = [];
    let errors = 0;
    let completed = 0;
    const reqsPerWorker = Math.ceil(totalRequests / concurrency);

    const t0 = performance.now();
    const workers = Array.from({ length: concurrency }, async () => {
      for (let i = 0; i < reqsPerWorker; i++) {
        const start = performance.now();
        try {
          const res = await fetch(url);
          if (res.status === 200) {
            latenciesUs.push((performance.now() - start) * 1000);
          } else {
            errors++;
          }
        } catch {
          errors++;
        }
        completed++;
      }
    });

    await Promise.all(workers);
    const totalElapsedSec = (performance.now() - t0) / 1000;
    const rps = Math.round(completed / totalElapsedSec);

    latenciesUs.sort((a, b) => a - b);
    const p = (q: number) => Math.round((latenciesUs[Math.min(latenciesUs.length - 1, Math.round(q * (latenciesUs.length - 1)))] ?? 0) * 10) / 10;

    let rssKb = 0;
    try {
      const st = await Bun.file(`/proc/${proc.pid}/status`).text();
      rssKb = parseInt(st.match(/VmRSS:\s+(\d+) kB/)?.[1] ?? "0", 10);
    } catch {}

    proc.kill();
    await proc.exited;

    return {
      rps,
      p50Us: p(0.5),
      p95Us: p(0.95),
      p99Us: p(0.99),
      errors,
      rssKb,
    };
  } catch (e) {
    proc.kill();
    await proc.exited;
    throw e;
  }
}

async function main() {
  const outDir = `${ROOT}/benchmarks/raw/warm`;
  mkdirSync(outDir, { recursive: true });

  const totalReqs = parseInt(process.env.WARM_REQS ?? "1000", 10);
  const concurrency = parseInt(process.env.WARM_CONCURRENCY ?? "10", 10);

  console.log(`Warm-load benchmark (requests=${totalReqs}, concurrency=${concurrency})`);
  const results: Array<Record<string, unknown>> = [];

  for (const cand of CANDIDATES) {
    for (const route of ROUTES) {
      console.log(`  measuring ${cand.id} on ${route.id}...`);
      const res = await runWarmLoad(cand, route, concurrency, totalReqs);
      results.push({
        candidate: cand.id,
        routeId: route.id,
        path: route.path,
        concurrency,
        totalRequests: totalReqs,
        ...res,
      });
      console.log(`    ${cand.id.padEnd(9)} ${route.id}: ${res.rps} req/s, p50=${res.p50Us}us, p95=${res.p95Us}us, errors=${res.errors}`);
    }
  }

  const summary = {
    format: "velqu-warm-load-v1",
    generatedAt: new Date().toISOString(),
    concurrency,
    requestsPerCell: totalReqs,
    results,
  };

  writeFileSync(`${outDir}/summary.json`, JSON.stringify(summary, null, 2));
  console.log(`wrote ${outDir}/summary.json`);
}

await main();
