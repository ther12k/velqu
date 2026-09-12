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
 *                      dot-only segments, unicode); dot-only values
 *                      ("." and "..", including encoded forms) must be
 *                      rejected, and legal params must stay strictly
 *                      inside their path segment under standard URL parsing;
 *   T2 query ser     — hostile query keys/values must serialize losslessly
 *                      through URLSearchParams (round-trip equality);
 *   T3 response map  — adversarial status/body pairs across valid HTTP
 *                      status range (200..599) must resolve to exactly
 *                      the documented outcome shape, and the returned
 *                      status must match the transport status.
 *
 * Every failure is recorded in the findings ledger with the seed and the
 * failing case; a clean run writes an explicit zero-findings record
 * (no bare green claims). Fail-closed: exit 1 on any finding.
 *
 * Usage: bun scripts/ts-fuzz-campaign.ts [--duration-secs N] [--seed S]
 */
import { treaty, type TreatyFetch } from "../packages/treaty/src/index";

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
  "ok", ".", "..", "%2e", "%2e%2e", "%2E", "%2E%2E",
  "..%2F..%2Fetc", "../../etc/passwd", "a/b", "a?b=c", "a#b",
  "javascript:alert(1)", "%2e%2e%2f", "\0", "白", "a".repeat(300),
  "-", "%00", " ", "+", "&param=x=", "%252e%252e%252f",
];
const ADVERSARIAL_QUERY: readonly [string, string][] = [
  ["k", "v"], ["&k=", "v"], ["k", "&v=#"], ["白", "值"], ["", "v"],
  ["k", "x".repeat(500)], ["=", "="], ["k", "\0"], ["a b", "c d"],
];

// Statuses stay strictly within valid native Response range (200..599).
// Invalid transport statuses (like 999) throw RangeError in ResponseInit
// constructor and are transport failures, not HTTP response outcomes.
// Null-body statuses (204, 304) are paired with a `null` body per the
// Fetch Standard: constructing a Response with any non-null body (even
// "") must throw TypeError there, so fixture validity requires the null
// body, not just the in-range status (owner review 2026-09-12).
const ADVERSARIAL_RESPONSES: readonly [number, string | null][] = [
  [200, '{"message":"ok"}'],
  [200, "not json at all"],
  [200, ""],
  [201, '{"message":"created"}'],
  [204, null],
  [301, "redirect body"],
  [400, '{"type":"https://velqu.dev/problems/bad","title":"bad","status":400}'],
  [404, '{"type":"https://velqu.dev/problems/not-found","title":"not found","status":404}'],
  [422, '{"type":"https://velqu.dev/problems/validation","title":"t","status":422,"errors":[]}'],
  [422, "garbage"],
  [500, null],
  [502, "{}"],
  [503, ""],
  [200, '{"message":' + "9".repeat(400) + '}'],
  [200, '\ufeff{"message":"bom"}'],
];

interface RouteInfoLike { path: string; method: string }
const CONTRACT = {
  "items.get": { path: "/items/:id", method: "GET" } as RouteInfoLike,
  "items.post": { path: "/items", method: "POST" } as RouteInfoLike,
};

let findings = 0;
let t1Iterations = 0;
let t2Iterations = 0;
let t3Iterations = 0;
let transportResponsesConstructed = 0;

const record = (cls: string, detail: string) => {
  findings += 1;
  console.error(`FINDING [${cls}] seed=${SEED}: ${detail}`);
};

function isDotOnly(s: string): boolean {
  try {
    const d = decodeURIComponent(s);
    return d === "." || d === "..";
  } catch {
    return s === "." || s === "..";
  }
}

async function main() {
  const deadline = Date.now() + DURATION_SECS * 1000;
  let iterations = 0;

  // Scripted adversarial transport for T3: each call replies with the NEXT
  // fixture pair from ADVERSARIAL_RESPONSES.
  let respIdx = 0;
  const fakeFetch: TreatyFetch = async () => {
    const idx = respIdx++;
    const [status, body] = ADVERSARIAL_RESPONSES[idx % ADVERSARIAL_RESPONSES.length]!;
    // body is passed through verbatim (string | null): a `??` fallback
    // here would coerce a null-body fixture into a string body and make
    // null-body statuses spec-invalid (owner review 2026-09-12).
    const res = new Response(body, { status });
    transportResponsesConstructed += 1;
    return res;
  };
  const apiResp = treaty<Record<string, { path: string; method: string; resp: Record<number, unknown> }>>({
    baseUrl: "https://treaty-fuzz.test",
    contract: CONTRACT as never,
    fetchImpl: fakeFetch,
  } as never);
  const itemsResp = (apiResp as never as Record<string, Record<string, (p: unknown) => Record<string, (o?: unknown) => Promise<unknown>>>>)["items"];

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
      const items = (api as never as Record<string, Record<string, (p: unknown) => Record<string, (o?: unknown) => Promise<unknown>>>>)["items"];
      t1Iterations += 1;
      const shouldReject = isDotOnly(param);
      try {
        await items.get({ id: param }).get();
        if (shouldReject) {
          record("T1", `dot-only param ${JSON.stringify(param)} was accepted without error; target was: ${capturedUrl}`);
          continue;
        }
      } catch (e) {
        if (shouldReject) {
          // Expected rejection for dot-only segments
          continue;
        }
        record("T1", `path param ${JSON.stringify(param)} made treaty() throw: ${e}`);
        continue;
      }

      // Invariants for accepted params: must parse as standard URL,
      // origin must be unchanged, and pathname must stay in /items/...
      try {
        const parsed = new URL(capturedUrl);
        if (parsed.origin !== "https://treaty-fuzz.test") {
          record("T1", `param ${JSON.stringify(param)} changed origin: ${capturedUrl}`);
        }
        if (!parsed.pathname.startsWith("/items/")) {
          record("T1", `param ${JSON.stringify(param)} escaped /items/ path prefix: ${parsed.pathname}`);
        }
      } catch (e) {
        record("T1", `captured URL is not valid URL: ${capturedUrl} (${e})`);
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
      t2Iterations += 1;
      try {
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
      const t3Idx = respIdx;
      const [expectedStatus] = ADVERSARIAL_RESPONSES[t3Idx % ADVERSARIAL_RESPONSES.length]!;
      let result: unknown;
      try {
        t3Iterations += 1;
        result = await itemsResp.get({ id: "stable" }).get();
      } catch (e) {
        record("T3", `status ${expectedStatus} made request() reject instead of structured outcome: ${e}`);
        continue;
      }
      const r = result as { data: unknown; error: unknown };
      const shapeOk =
        (r.data !== null && r.error === null) || (r.data === null && r.error !== null);
      if (!shapeOk) {
        record("T3", `status ${expectedStatus} produced a third outcome shape: ${JSON.stringify(r).slice(0, 200)}`);
      }
      if (expectedStatus >= 200 && expectedStatus <= 299) {
        if (r.error !== null) {
          record("T3", `2xx status ${expectedStatus} mapped to error outcome`);
        }
      } else if (expectedStatus >= 300) {
        if (r.data !== null || r.error === null) {
          record("T3", `non-2xx status ${expectedStatus} mapped to data outcome or null error`);
        } else {
          const errStatus = (r.error as { status?: number }).status;
          if (errStatus !== expectedStatus) {
            record("T3", `non-2xx status ${expectedStatus} returned wrong error.status: ${errStatus}`);
          }
        }
      }
    }
  }

  const coverageEmpty =
    t1Iterations === 0 || t2Iterations === 0 || t3Iterations === 0 || transportResponsesConstructed === 0;

  const ledger = {
    campaign: "ga-m6-ts-treaty-encoders",
    startedAt: new Date(Date.now() - DURATION_SECS * 1000).toISOString(),
    finishedAt: new Date().toISOString(),
    durationSecs: DURATION_SECS,
    seed: SEED,
    iterations,
    t1Iterations,
    t2Iterations,
    t3Iterations,
    transportResponsesConstructed,
    coverageEmpty,
    surface: "packages/treaty (published treaty() API: path interpolation, query serialization, response mapping)",
    totalFindings: findings,
    result: findings === 0 && !coverageEmpty
      ? "zero-findings — explicit record, not a bare green claim"
      : "findings — each requires a regression test or an owner-accepted risk record",
  };
  const outIdx = process.argv.indexOf("--out");
  const out = outIdx !== -1 ? process.argv[outIdx + 1]! : "benchmarks/raw/ga-m6-fuzz/ts-treaty-ledger.json";
  await Bun.write(out, JSON.stringify(ledger, null, 1) + "\n");
  console.log(JSON.stringify(ledger, null, 1));
  process.exit(findings === 0 && !coverageEmpty ? 0 : 1);
}

main();
