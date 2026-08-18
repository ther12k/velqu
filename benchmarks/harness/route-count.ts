/**
 * Route-count cold-start scaling (PERF-005): 25 vs 1000 equivalent routes.
 * Measured request: GET /res7/item/7 → {"id":7,"n":N}. Fresh process per
 * sample; same protocol as cold-start.ts.
 */
const ROOT = import.meta.dir + "/../..";

const N_VALUES = [25, 1000] as const;

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

async function sample(cand: Cand, n: number): Promise<{ totalMs: number; rssKb: number | null; ok: boolean; err?: string }> {
  const port = freePort();
  const { cmd, args, env } = cand.spawn(port, n);
  const t0 = performance.now();
  const proc = Bun.spawn([cmd, ...args], { env: { ...process.env, ...env }, stdout: "ignore", stderr: "ignore" });
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
        if (ok) return { totalMs, rssKb, ok };
        return { totalMs, rssKb, ok: false, err: `body=${body.slice(0, 60)}` };
      } catch {
        await Bun.sleep(0.5);
      }
    }
    throw new Error("no response");
  } catch (e) {
    proc.kill();
    await proc.exited;
    return { totalMs: performance.now() - t0, rssKb: null, ok: false, err: String(e) };
  }
}

const SAMPLES = parseInt(process.argv.find((a) => a.startsWith("--samples="))?.slice(10) ?? "40", 10);
console.log(`route-count: ${CANDS.length} candidates × ${N_VALUES.length} sizes × ${SAMPLES} samples`);

// warm harness
for (let i = 0; i < 3; i++) await sample(CANDS[1], 25);

const results: Array<Record<string, unknown>> = [];
const raw: string[] = [];
for (const cand of CANDS) {
  for (const n of N_VALUES) {
    const rows: Array<{ totalMs: number; rssKb: number | null }> = [];
    let failures = 0;
    for (let i = 0; i < SAMPLES; i++) {
      const r = await sample(cand, n);
      raw.push(JSON.stringify({ candidate: cand.id, n, i, totalMs: Math.round(r.totalMs * 1000) / 1000, rssKb: r.rssKb, ok: r.ok, err: r.err }));
      if (r.ok) rows.push({ totalMs: r.totalMs, rssKb: r.rssKb });
      else failures++;
    }
    const sorted = rows.map((r) => r.totalMs).sort((a, b) => a - b);
    const p = (q: number) => Math.round(sorted[Math.min(sorted.length - 1, Math.round(q * (sorted.length - 1)))] * 1000) / 1000;
    const rssSorted = rows.map((r) => r.rssKb ?? 0).filter((x) => x > 0).sort((a, b) => a - b);
    const pRss = (q: number) => rssSorted[Math.min(rssSorted.length - 1, Math.round(q * (rssSorted.length - 1)))] ?? 0;
    results.push({
      candidate: cand.id,
      routes: n,
      samples: rows.length,
      failures,
      p50Ms: p(0.5),
      p95Ms: p(0.95),
      rssP50Kb: pRss(0.5),
    });
    console.log(`${cand.id} n=${n}: p50=${p(0.5)}ms p95=${p(0.95)}ms rss=${pRss(0.5)}kB failures=${failures}`);
  }
}
// scaling deltas
for (const cand of CANDS) {
  const r25 = results.find((r) => r.candidate === cand.id && r.routes === 25)!;
  const r1000 = results.find((r) => r.candidate === cand.id && r.routes === 1000)!;
  const delta = ((r1000.p50Ms as number) / (r25.p50Ms as number) - 1) * 100;
  console.log(`${cand.id}: 1000-route vs 25-route p50 delta = ${delta.toFixed(1)}%`);
}
import { mkdirSync, writeFileSync } from "node:fs";
mkdirSync(`${ROOT}/benchmarks/raw/route-count`, { recursive: true });
const stamp = Date.now();
writeFileSync(`${ROOT}/benchmarks/raw/route-count/route-count-${stamp}.jsonl`, raw.join("\n") + "\n");
writeFileSync(
  `${ROOT}/benchmarks/raw/route-count/summary.json`,
  JSON.stringify({ format: "velqu-route-count-v1", samples: SAMPLES, generatedAt: new Date().toISOString(), results }, null, 2),
);
console.log("wrote benchmarks/raw/route-count/");
