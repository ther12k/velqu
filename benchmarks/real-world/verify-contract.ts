/**
 * Contract-response verification for real-world benchmark candidates
 * (BETA-002-C).
 *
 * Boots every candidate server against the controlled upstream and drives the
 * full fixture matrix from `contract-fixtures.ts`. A candidate passes only
 * when every fixture returns the expected status and an exactly equal JSON
 * body — before any timing is allowed to count. Any mismatch is a fairness
 * finding: candidates that do not answer identical contracts identically
 * cannot be compared (SPEC: candidates are semantically equivalent, 0%
 * error requirement).
 *
 * Usage:
 *   bun verify-contract.ts [--upstream-url URL] [--out report.md]
 *                          [--only hono,elysia]
 *
 * With no --upstream-url the verifier starts its own controlled upstream
 * (`upstream.ts`) and stops it on exit. Candidate dependencies must be
 * installed first: `cd candidates && bun install --frozen-lockfile`.
 */

import { readFileSync, writeFileSync, existsSync, mkdirSync } from "node:fs";
import { MATCHED_CONFIG } from "./candidates/matched";
import { buildContractFixtures, matchesExpected, type ContractFixture } from "./contract-fixtures";

const DIR = import.meta.dir;

interface CandidateSpec {
  name: string;
  command: string[];
}

const CANDIDATES: CandidateSpec[] = [
  { name: "hono", command: ["bun", "candidates/hono.ts"] },
  { name: "elysia", command: ["bun", "candidates/elysia.ts"] },
  { name: "bun-fetch", command: ["bun", "candidates/bun-fetch.ts"] },
  { name: "fastify", command: ["node", "candidates/fastify.js"] },
];

interface FixtureResult {
  fixture: string;
  status: "PASS" | "FAIL";
  detail: string;
}

interface CandidateResult {
  name: string;
  command: string;
  port: number;
  results: FixtureResult[];
  passed: number;
  failed: number;
}

function fail(message: string): never {
  console.error(`verify-contract: ${message}`);
  process.exit(1);
}

function parseArgs(argv: string[]): { upstreamUrl: string | null; out: string; only: string[] } {
  const get = (name: string): string | null => {
    const i = argv.indexOf(`--${name}`);
    return i >= 0 ? argv[i + 1] : null;
  };
  return {
    upstreamUrl: get("upstream-url"),
    out: get("out") ?? `${DIR}/contract-verification.md`,
    only: (get("only") ?? "")
      .split(",")
      .map((s) => s.trim())
      .filter(Boolean),
  };
}

function checkPrerequisites(only: string[]): void {
  for (const dep of ["hono", "elysia", "fastify"]) {
    if (!existsSync(`${DIR}/candidates/node_modules/${dep}`)) {
      fail(
        `candidates/node_modules/${dep} is missing — run: cd candidates && bun install --frozen-lockfile`,
      );
    }
  }
  if (CANDIDATES.some((c) => c.command[0] === "node") && !Bun.which("node")) {
    fail("node is required for the fastify candidate but was not found on PATH");
  }
  const unknown = only.filter((name) => !CANDIDATES.some((c) => c.name === name));
  if (unknown.length > 0) fail(`unknown candidate name(s): ${unknown.join(", ")}`);
}

interface ReadyChild {
  proc: Bun.Subprocess<"ignore", "pipe", "pipe">;
  port: number;
}

async function waitForReadyLine(
  proc: Bun.Subprocess<"ignore", "pipe", "pipe">,
  event: string,
  timeoutMs: number,
): Promise<number> {
  const reader = (proc.stdout as ReadableStream<Uint8Array>).getReader();
  const decoder = new TextDecoder();
  let buffered = "";
  const deadline = Date.now() + timeoutMs;
  try {
    while (Date.now() < deadline) {
      const timer = new Promise<"timeout">((resolve) => setTimeout(() => resolve("timeout"), deadline - Date.now()));
      const chunk = await Promise.race([reader.read(), timer]);
      if (chunk === "timeout") break;
      if (chunk.done) break;
      buffered += decoder.decode(chunk.value);
      let idx: number;
      while ((idx = buffered.indexOf("\n")) >= 0) {
        const line = buffered.slice(0, idx).trim();
        buffered = buffered.slice(idx + 1);
        if (!line) continue;
        try {
          const parsed = JSON.parse(line) as { event?: string; port?: number };
          if (parsed.event === event && typeof parsed.port === "number") {
            return parsed.port;
          }
        } catch {
          // non-JSON stdout line; keep scanning for the ready event
        }
      }
    }
  } finally {
    reader.releaseLock();
  }
  throw new Error(`no ${event} line within ${timeoutMs}ms`);
}

async function startCandidate(spec: CandidateSpec, upstreamUrl: string): Promise<ReadyChild> {
  const proc = Bun.spawn({
    cmd: spec.command,
    cwd: DIR,
    env: { ...process.env, PORT: "0", UPSTREAM_URL: upstreamUrl },
    stdout: "pipe",
    stderr: "pipe",
    stdin: "ignore",
  });
  try {
    const port = await waitForReadyLine(proc, "candidate.ready", 15000);
    return { proc, port };
  } catch (err) {
    proc.kill();
    const stderr = await new Response(proc.stderr as ReadableStream).text();
    throw new Error(
      `candidate ${spec.name} failed to start (${String(err)}): ${stderr.trim().slice(0, 500)}`,
    );
  }
}

async function startUpstream(): Promise<ReadyChild> {
  const proc = Bun.spawn({
    cmd: ["bun", "upstream.ts"],
    cwd: DIR,
    env: { ...process.env, PORT: "0" },
    stdout: "pipe",
    stderr: "pipe",
    stdin: "ignore",
  });
  try {
    const port = await waitForReadyLine(proc, "upstream.ready", 10000);
    return { proc, port };
  } catch (err) {
    proc.kill();
    throw new Error(`controlled upstream failed to start: ${String(err)}`);
  }
}

async function runFixtures(port: number, fixtures: ContractFixture[]): Promise<FixtureResult[]> {
  const results: FixtureResult[] = [];
  for (const fixture of fixtures) {
    try {
      const res = await fetch(`http://127.0.0.1:${port}${fixture.path}`, {
        method: fixture.method,
        headers: {
          ...(fixture.headers ?? {}),
          ...(fixture.body !== undefined ? { "content-type": "application/json" } : {}),
        },
        body: fixture.body !== undefined ? JSON.stringify(fixture.body) : undefined,
        signal: AbortSignal.timeout(MATCHED_CONFIG.timeouts.requestDeadlineMs),
      });
      const text = await res.text();
      let json: unknown;
      try {
        json = JSON.parse(text);
      } catch {
        results.push({
          fixture: fixture.name,
          status: "FAIL",
          detail: `status ${res.status} body is not JSON: ${text.slice(0, 200)}`,
        });
        continue;
      }
      if (res.status !== fixture.expectStatus) {
        results.push({
          fixture: fixture.name,
          status: "FAIL",
          detail: `expected status ${fixture.expectStatus}, got ${res.status} with ${JSON.stringify(json).slice(0, 200)}`,
        });
        continue;
      }
      if (!matchesExpected(fixture.expectJson, json)) {
        results.push({
          fixture: fixture.name,
          status: "FAIL",
          detail: `body mismatch: expected ${JSON.stringify(fixture.expectJson).slice(0, 200)}, got ${JSON.stringify(json).slice(0, 200)}`,
        });
        continue;
      }
      results.push({ fixture: fixture.name, status: "PASS", detail: `${res.status}` });
    } catch (err) {
      results.push({ fixture: fixture.name, status: "FAIL", detail: `request failed: ${String(err)}` });
    }
  }
  return results;
}

function renderReport(candidateResults: CandidateResult[], upstreamUrl: string): string {
  const lines: string[] = [];
  lines.push("# Real-World Candidate Contract Verification", "");
  lines.push(`Controlled upstream: ${upstreamUrl}`, "");
  for (const candidate of candidateResults) {
    lines.push(`## ${candidate.name} (\`${candidate.command}\` on :${candidate.port})`, "");
    lines.push("| Fixture | Status | Detail |");
    lines.push("|---|---|---|");
    for (const r of candidate.results) {
      lines.push(`| ${r.fixture} | ${r.status} | ${r.detail} |`);
    }
    lines.push("");
  }
  const totalFailed = candidateResults.reduce((acc, c) => acc + c.failed, 0);
  lines.push(
    totalFailed === 0
      ? "**Contract verification: PASS** — every candidate answered every fixture identically."
      : `**Contract verification: FAIL (${totalFailed} mismatch(es))** — candidates are not semantically equivalent; timing comparisons are invalid until resolved.`,
    "",
  );
  return lines.join("\n");
}

async function main(): Promise<void> {
  const args = parseArgs(process.argv.slice(2));
  checkPrerequisites(args.only);
  const specs = args.only.length > 0 ? CANDIDATES.filter((c) => args.only.includes(c.name)) : CANDIDATES;

  let upstream: ReadyChild | null = null;
  const upstreamUrl = args.upstreamUrl ?? "";
  if (!upstreamUrl) {
    upstream = await startUpstream();
    const upstreamUrlBuilt = `http://127.0.0.1:${upstream.port}`;
    console.log(`verify-contract: spawned controlled upstream on :${upstream.port}`);
    await runVerification(specs, upstreamUrlBuilt, args.out, upstream);
    return;
  }
  await runVerification(specs, upstreamUrl, args.out, null);
}

async function runVerification(
  specs: CandidateSpec[],
  upstreamUrl: string,
  out: string,
  upstream: ReadyChild | null,
): Promise<void> {
  const fixtures = buildContractFixtures();
  const candidateResults: CandidateResult[] = [];
  try {
    for (const spec of specs) {
      console.log(`verify-contract: starting ${spec.name} (${spec.command.join(" ")})`);
      const child = await startCandidate(spec, upstreamUrl);
      const results = await runFixtures(child.port, fixtures);
      child.proc.kill();
      await child.proc.exited;
      const passed = results.filter((r) => r.status === "PASS").length;
      const failed = results.length - passed;
      candidateResults.push({ name: spec.name, command: spec.command.join(" "), port: child.port, results, passed, failed });
      console.log(`verify-contract: ${spec.name} ${passed}/${results.length} fixtures passed`);
    }
  } finally {
    if (upstream) upstream.proc.kill();
  }

  mkdirSync(out.substring(0, out.lastIndexOf("/")) || ".", { recursive: true });
  const report = renderReport(candidateResults, upstreamUrl);
  writeFileSync(out, report);
  console.log(report);
  const totalFailed = candidateResults.reduce((acc, c) => acc + c.failed, 0);
  process.exit(totalFailed === 0 ? 0 : 1);
}

if (import.meta.main) {
  main().catch((err) => {
    console.error(`verify-contract: ${String(err)}`);
    process.exit(1);
  });
}
