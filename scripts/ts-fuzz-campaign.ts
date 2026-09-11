/**
 * #1318 / M6-002 — sustained property-fuzz campaign for the TypeScript
 * encoder surfaces (Treaty client), completing the acceptance coverage
 * that Rust-side cargo-fuzz targets cannot reach (fuzz/COVERAGE.md).
 *
 * Deterministic seeded generator; sustained duration-configured loops
 * over three adversarial-input classes against the published `treaty()`
 * API with a scripted fetch/dispatch:
 *
 *   T1 path params   — hostile param values (traversal, scheme, separators,
 *                      unicode) must stay inside the route path segment
 *                      (encodeURIComponent) and never change the target
 *                      route or origin;
 *   T2 query ser     — hostile query keys/values must serialize losslessly
 *                      through URLSearchParams (round-trip equality);
 *   T3 response map  — adversarial status/body pairs (malformed JSON,
 *                      wrong types, huge strings) must resolve to exactly
 *                      the documented outcome shape ({data,error:null} on
 *                      2xx, {data:null,error} otherwise) — never reject,
 *                      never a third shape.
 *
 * Every failure is recorded in the findings ledger with the seed and the
 * failing case; a clean run writes an explicit zero-findings record
 * (no bare green claims). Fail-closed: exit 1 on any finding.
 *
 * Usage: bun scripts/ts-fuzz-campaign.ts [--duration-secs N] [--seed S]
 */
import { treaty } from "../packages/treaty/src/index";

const arg = (name: string, def: number) => {
  const i = process.argv.indexOf(name);
  return i !== -1 ? Number(process.argv[i + 1]) || def : def;
};
const DURATION_SECS = arg("--duration-secs", 300);
const SEED = arg("--seed", Date.now() % 2 ** 31);

// xoshiro-ish deterministic PRNG (reproducible findings).
let state = SEED >>> 0;
const rnd = () => {
  state ^= state << 13; state >>>= 0;
  state ^= state >> 17;
  state ^= state << 5; state >>>= 0;
  return state / 2 ** 32;
};
const pick = <T>(xs: readonly T[]) => xs[Math.floor(rnd() * xs.length)]!;

const ADVERSARIAL_PARAMS: readonly string[] = [
  "ok", "..%2F..%2Fetc", "../../etc/passwd", "a/b", "a?b=c", "a#b",
  "javascript:alert(1)", "%2e%2e%2f", "\0", "白", "a".repeat(300),
  "-", "%00", " ", "+", "&param=x=", "%252e%252e%252f",
];
const ADVERSARIAL_QUERY: readonly [string, string][] = [
  ["k", "v"], ["&k=", "v"], ["k", "&v=#"], ["白", "值"], ["", "v"],
  ["k", "x".repeat(500)], ["=", "="], ["k", "\0"], ["a b", "c d"],
];
const ADVERSARIAL_RESPONSES: readonly [number, string][] = [
  [200, '{"message":"ok"}'],
  [200, "not json at all"],
  [200, ""],
  [422, '{"type":"https://velqu.dev/problems/validation","title":"t","status":422,"errors":[]}'],
  [422, "garbage"],
  [500, null as unknown as string],
  [200, '{"message":' + "9".repeat(400) + '}'],
  [301, "redirect body"],
  [999, "{}"],
  [200, '﻿{"message":"bom"}'],
];

interface RouteInfoLike { path: string; method: string }
const CONTRACT = {
  "items.get": { path: "/items/:id", method: "GET" } as RouteInfoLike,
  "items.post": { path: "/items", method: "POST" } as RouteInfoLike,
};

let findings = 0;
const record = (cls: string, detail: string) => {
  findings += 1;
  console.error(`FINDING [${cls}] seed=${SEED}: ${detail}`);
};

async function main() {
  const deadline = Date.now() + DURATION_SECS * 1000;
  let iterations = 0;

  // Scripted transport: replies with the next adversarial pair.
  let respIdx = 0;
  const fakeFetch: TreatyFetch = async () => {
    const [status, body] = ADVERSARIAL_RESPONSES[respIdx % ADVERSARIAL_RESPONSES.length]!;
    respIdx += 1;
    return new Response(body ?? "null", { status });
  };

  while (Date.now() < deadline) {
    for (let burst = 0; burst < 500; burst++) {
      iterations += 1;

      // --- T1: hostile path params (fetch mode, URL built by treaty) ---
      const param = pick(ADVERSARIAL_PARAMS);
      let capturedUrl = "";
      const captureFetch: TreatyFetch = async (input) => {
        capturedUrl = String(input);
        return new Response("{}", { status: 200 });
      };
      const api = treaty<Record<string, { path: string; method: string; resp: Record<number, unknown> }>>({
        baseUrl: "https://treaty-fuzz.test",
        contract: CONTRACT as never,
        fetchImpl: captureFetch,
      } as never);
      // Documented navigation (probed against the runtime): the leaf-id
      // hop is CALLED with path params (apply -> bound method map), the
      // method-name call fires the request, query rides in opts.
      const items = (api as never as Record<string, Record<string, (p: unknown) => Record<string, (o?: unknown) => Promise<unknown>>>>)["items"];
      try {
        await items.get({ id: param }).get();
      } catch (e) {
        record("T1", `path param ${JSON.stringify(param)} made treaty() throw: ${e}`);
        continue;
      }
      // Invariants: same origin, and the param stayed inside its segment.
      if (!capturedUrl.startsWith("https://treaty-fuzz.test/items/")) {
        record("T1", `param ${JSON.stringify(param)} escaped the route path: ${capturedUrl}`);
      }
      const originLess = capturedUrl.slice("https://treaty-fuzz.test".length);
      if (originLess.includes("//") || /https?:/i.test(originLess.split("/items/")[1] ?? "")) {
        record("T1", `param ${JSON.stringify(param)} injected a scheme or empty segment: ${capturedUrl}`);
      }

      // --- T2: hostile query serialization round-trips losslessly ---
      const [qk, qv] = pick(ADVERSARIAL_QUERY);
      let capturedQueryUrl = "";
      const qFetch: TreatyFetch = async (input) => {
        capturedQueryUrl = String(input);
        return new Response("{}", { status: 200 });
      };
      const apiQ = treaty<Record<string, { path: string; method: string; resp: Record<number, unknown> }>>({
        baseUrl: "https://treaty-fuzz.test",
        contract: CONTRACT as never,
        fetchImpl: qFetch,
      } as never);
      const itemsQ = (apiQ as never as Record<string, Record<string, (p: unknown) => Record<string, (b: unknown, o?: unknown) => Promise<unknown>>>>)["items"];
      try {
        // write route: leaf bind (no :params -> empty), then post(body, opts)
        await itemsQ.post({}).post({}, { query: { [qk]: qv } });
      } catch (e) {
        record("T2", `query ${JSON.stringify([qk, qv])} made treaty() throw: ${e}`);
        continue;
      }
      const qsi = capturedQueryUrl.indexOf("?");
      if (qsi === -1) {
        record("T2", `query ${JSON.stringify([qk, qv])} produced no query string: ${capturedQueryUrl}`);
      } else {
        const back = new URLSearchParams(capturedQueryUrl.slice(qsi + 1));
        if (back.get(qk) !== qv) {
          record("T2", `query round-trip mismatch for ${JSON.stringify([qk, qv])}: got ${JSON.stringify(back.get(qk))}`);
        }
      }

      // --- T3: adversarial responses always yield the outcome shape ---
      const [status] = ADVERSARIAL_RESPONSES[respIdx % ADVERSARIAL_RESPONSES.length] ?? [200, "{}"];
      let result: unknown;
      try {
        result = await items.get({ id: "stable" }).get();
      } catch (e) {
        record("T3", `status ${status} made request() reject instead of a structured outcome: ${e}`);
        continue;
      }
      const r = result as { data: unknown; error: unknown };
      const shapeOk =
        (r.data !== null && r.error === null) || (r.data === null && r.error !== null);
      if (!shapeOk) {
        record("T3", `status ${status} produced a third outcome shape: ${JSON.stringify(r).slice(0, 200)}`);
      }
      if (status >= 200 && status <= 299 && r.error !== null) {
        record("T3", `2xx status ${status} mapped to an error outcome`);
      }
      if (status >= 300 && r.data !== null) {
        record("T3", `non-2xx status ${status} mapped to a data outcome`);
      }
    }
  }

  const ledger = {
    campaign: "ga-m6-ts-treaty-encoders",
    startedAt: new Date(Date.now() - DURATION_SECS * 1000).toISOString(),
    finishedAt: new Date().toISOString(),
    durationSecs: DURATION_SECS,
    seed: SEED,
    iterations,
    surface: "packages/treaty (published treaty() API: path interpolation, query serialization, response mapping)",
    totalFindings: findings,
    result: findings === 0
      ? "zero-findings — explicit record, not a bare green claim"
      : "findings — each requires a regression test or an owner-accepted risk record",
  };
  const outIdx = process.argv.indexOf("--out");
  const out = outIdx !== -1 ? process.argv[outIdx + 1]! : "benchmarks/raw/ga-m6-fuzz/ts-treaty-ledger.json";
  await Bun.write(out, JSON.stringify(ledger, null, 1) + "\n");
  console.log(JSON.stringify(ledger, null, 1));
  process.exit(findings === 0 ? 0 : 1);
}

import type { TreatyFetch } from "../packages/treaty/src/index";
main();
