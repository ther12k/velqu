/**
 * C1-A diagnostic probe harness (non-canonical, measurement only).
 *
 * Measures the four c1-probe routes (sync/async × text/json) against ONE
 * velqu runtime instance. Deterministic cell order (concurrency outer,
 * route, repetition inner) — single server, no interleaving needed.
 * Canonical warm-load evidence is untouched; output lands in
 * benchmarks/raw/c1-probe/.
 *
 * Optional stage breakdown: pass --stage-out <dir>; the runtime is then
 * expected to be the bench-stages feature build with BENCH_STAGE_DIR set
 * to the same dir; the periodic stage dumps are copied next to the run.
 */

import { mkdirSync, writeFileSync, existsSync, copyFileSync, readdirSync } from "node:fs";

const ROOT = import.meta.dir + "/../..";

const RUNTIME = process.env.C1PROBE_RUNTIME ?? `${ROOT}/target/release/velqu-runtime`;
const PACK = process.env.C1PROBE_PACK ?? `${ROOT}/examples/c1-probe/dist/app.qpack`;
const DURATION = parseInt(process.env.C1PROBE_DURATION ?? "8", 10);
const REPS = parseInt(process.env.C1PROBE_REPS ?? "5", 10);
const CONC = (process.env.C1PROBE_CONC ?? "1,10,50").split(",").map((c) => parseInt(c.trim(), 10));
const OUT_DIR = process.env.C1PROBE_OUT ?? `${ROOT}/benchmarks/raw/c1-probe`;
const RUN_ID = process.env.C1PROBE_RUN_ID ?? `c1probe-${Date.now()}`;
const LABEL = process.env.C1PROBE_LABEL ?? (process.env.C1PROBE_STAGE_OUT ? "staged" : "stock");
const STAGE_OUT = process.env.C1PROBE_STAGE_OUT ?? "";

const ROUTES = [
  { id: "text-sync", path: "/diag/text-sync" },
  { id: "text-async", path: "/diag/text-async" },
  { id: "json-sync", path: "/diag/json-sync" },
  { id: "json-async", path: "/diag/json-async" },
];

function freePort(): number {
  const l = Bun.listen({ hostname: "127.0.0.1", port: 0, socket: { data() {}, open() {} } });
  const port = l.port;
  l.stop(true);
  return port;
}

async function measure(path: string, concurrency: number, durationSec: number) {
  const port = freePort();
  const proc = Bun.spawn(
    [RUNTIME, "--pack", PACK, "--port", String(port), "--log", "off"],
    {
      env: {
        ...process.env,
        ...(STAGE_OUT ? { BENCH_STAGE_DIR: STAGE_OUT } : {}),
      },
      stdout: "ignore",
      stderr: "ignore",
    },
  );
  try {
    const deadline = performance.now() + 10_000;
    for (;;) {
      try {
        const c = await Bun.connect({ hostname: "127.0.0.1", port, socket: { data() {}, open() {} } });
        c.end?.();
        c.terminate?.();
        break;
      } catch {
        if (performance.now() > deadline) throw new Error("runtime never became ready");
        await Bun.sleep(5);
      }
    }
    const url = `http://127.0.0.1:${port}${path}`;
    for (let i = 0; i < 200; i++) {
      try { await fetch(url); } catch {}
    }
    const latenciesUs: number[] = [];
    let errors = 0;
    let completed = 0;
    const deadlineMs = performance.now() + durationSec * 1000;
    const t0 = performance.now();
    await Promise.all(
      Array.from({ length: concurrency }, async () => {
        while (performance.now() < deadlineMs) {
          const start = performance.now();
          try {
            const res = await fetch(url);
            if (res.status === 200) latenciesUs.push((performance.now() - start) * 1000);
            else errors++;
          } catch {
            errors++;
          }
          completed++;
        }
      }),
    );
    const elapsed = (performance.now() - t0) / 1000;
    latenciesUs.sort((a, b) => a - b);
    const q = (p: number) =>
      Math.round((latenciesUs[Math.min(latenciesUs.length - 1, Math.round(p * (latenciesUs.length - 1)))] ?? 0) * 10) / 10;
    let rssKb = 0;
    try {
      const st = await Bun.file(`/proc/${proc.pid}/status`).text();
      rssKb = parseInt(st.match(/VmRSS:\s+(\d+) kB/)?.[1] ?? "0", 10);
    } catch {}
    return {
      rps: Math.round(completed / elapsed),
      p50Us: q(0.5),
      p95Us: q(0.95),
      p99Us: q(0.99),
      errors,
      rssKb,
      totalRequests: completed,
    };
  } finally {
    proc.kill();
    await proc.exited;
  }
}

console.log(
  `c1-probe (${LABEL}): duration=${DURATION}s reps=${REPS} conc=[${CONC.join(",")}] routes=${ROUTES.length} runId=${RUN_ID}`,
);
const results: Array<Record<string, unknown>> = [];
for (const concurrency of CONC) {
  for (const route of ROUTES) {
    for (let rep = 1; rep <= REPS; rep++) {
      const res = await measure(route.path, concurrency, DURATION);
      results.push({
        runId: RUN_ID,
        label: LABEL,
        routeId: route.id,
        path: route.path,
        concurrency,
        rep,
        durationSec: DURATION,
        ...res,
      });
      console.log(
        `  ${route.id.padEnd(11)} c=${String(concurrency).padEnd(3)} rep${rep}: ${res.rps} req/s p50=${res.p50Us}us p95=${res.p95Us}us errors=${res.errors}`,
      );
    }
  }
}

mkdirSync(OUT_DIR, { recursive: true });
const jsonl = `${OUT_DIR}/${RUN_ID}.jsonl`;
writeFileSync(jsonl, results.map((r) => JSON.stringify(r)).join("\n") + "\n");
const summary = {
  format: "velqu-c1-probe-diagnostic-v1",
  diagnosticOnly: true,
  generatedAt: new Date().toISOString(),
  runId: RUN_ID,
  label: LABEL,
  durationSec: DURATION,
  repetitions: REPS,
  concurrency: CONC,
  raw: `benchmarks/raw/c1-probe/${RUN_ID}.jsonl`,
  results,
};
writeFileSync(`${OUT_DIR}/${RUN_ID}.summary.json`, JSON.stringify(summary, null, 2));
if (STAGE_OUT && existsSync(STAGE_OUT)) {
  for (const f of readdirSync(STAGE_OUT)) {
    if (f.endsWith(".json")) {
      try {
        copyFileSync(`${STAGE_OUT}/${f}`, `${OUT_DIR}/${RUN_ID}.${f}`);
      } catch {}
    }
  }
}
console.log(`wrote ${jsonl}`);
console.log(`wrote ${OUT_DIR}/${RUN_ID}.summary.json`);
