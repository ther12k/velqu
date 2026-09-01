/**
 * M4A-004-A evidence: typecheck scale benchmark for the generated Treaty
 * client surface. Generates a synthetic Api with N routes, typechecks a
 * consumer that navigates every route, and reports wall-clock samples for
 * `tsc --noEmit` (raw samples printed; median computed by the reader from
 * the printed samples — nothing is hand-edited).
 *
 * Usage: bun scripts/typecheck-scale.ts [sizes]  (default: 25,100,200)
 */
import { mkdirSync, writeFileSync, rmSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";

const REPS = 3;

function routeDecl(i: number): string {
  const id = `svc${Math.floor(i / 10)}.action${i % 10}`;
  return `  "${id}": {
    path: "/svc${Math.floor(i / 10)}/action${i % 10}/:id";
    method: "POST";
    params: { id: string };
    query: { verbose?: boolean };
    body: { name: string; count: number };
    headers: never;
    responses: { 200: { id: string; ok: boolean }; 404: { code: string } };
  };`;
}

function usageDecl(i: number): string {
  const id = `svc${Math.floor(i / 10)}.action${i % 10}`;
  return `await api.${id}({ id: "x" }).post({ name: "n", count: ${i} });`;
}

function generate(n: number): string {
  const routes = Array.from({ length: n }, (_, i) => routeDecl(i)).join("\n");
  const calls = Array.from({ length: n }, (_, i) => usageDecl(i)).join("\n");
  return `
import { treaty } from "@velqu/treaty";
export type BigApi = {
${routes}
};
const api = treaty<BigApi>({ baseUrl: "http://127.0.0.1:1", contract: {} });
export async function driveAll(): Promise<void> {
${calls}
}
`;
}

async function timeTsc(cwd: string): Promise<number> {
  const t0 = performance.now();
  const proc = Bun.spawn(["bunx", "tsc", "--noEmit", "--strict", "--target", "es2022", "--module", "esnext", "--moduleResolution", "bundler", "consumer.ts"], {
    cwd,
    stdout: "pipe",
    stderr: "pipe",
    env: process.env,
  });
  const code = await proc.exited;
  const ms = performance.now() - t0;
  if (code !== 0) {
    throw new Error(`tsc failed in ${cwd}: ${await new Response(proc.stderr).text()}`);
  }
  return ms;
}

const sizes = (process.argv[2] ?? "25,100,200").split(",").map(Number);
const root = join(tmpdir(), `velqu-tsc-scale-${Date.now()}`);
mkdirSync(join(root, "node_modules", "@velqu"), { recursive: true });
const { symlinkSync } = require("node:fs");
symlinkSync(join(process.cwd(), "packages", "treaty"), join(root, "node_modules", "@velqu", "treaty"), "dir");

const results: Array<{ n: number; samplesMs: number[] }> = [];
try {
  for (const n of sizes) {
    writeFileSync(join(root, "consumer.ts"), generate(n));
    const samplesMs: number[] = [];
    for (let r = 0; r < REPS; r++) {
      samplesMs.push(await timeTsc(root));
    }
    results.push({ n, samplesMs });
  }
} finally {
  rmSync(root, { recursive: true, force: true });
}

console.log(JSON.stringify({ benchmark: "typecheck-scale", reps: REPS, results }, null, 2));
