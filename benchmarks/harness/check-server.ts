/**
 * Canonical black-box fixture checker (benchmarks/fixtures/fixture-contract.json).
 * Validates ANY candidate implementing the frozen behavior.
 *
 * Usage: bun check-server.ts <port> [--candidate velqu|bun|elysia|rust]
 *   --candidate velqu enables byte-exact problem-body checks; others are
 *   checked semantically (status + JSON body + failing-field identification).
 */
import fixture from "../fixtures/fixture-contract.json";

const args = process.argv.slice(2);
const port = parseInt(args[0] ?? "3000", 10);
const candidateIdx = args.indexOf("--candidate");
const candidate = candidateIdx >= 0 ? args[candidateIdx + 1] : "other";

const base = `http://127.0.0.1:${port}`;
let pass = 0;
let fail = 0;
const failures: string[] = [];

async function req(
  method: string,
  path: string,
  opts: { body?: unknown; headers?: Record<string, string>; raw?: boolean } = {},
): Promise<{ status: number; headers: Record<string, string>; body: string }> {
  const res = await fetch(base + path, {
    method,
    headers: {
      ...(opts.body !== undefined ? { "content-type": "application/json" } : {}),
      ...(opts.headers ?? {}),
    },
    body: opts.body !== undefined ? (opts.raw ? (opts.body as string) : JSON.stringify(opts.body)) : undefined,
  });
  const headers: Record<string, string> = {};
  res.headers.forEach((v, k) => (headers[k] = v));
  return { status: res.status, headers, body: await res.text() };
}

function check(name: string, cond: boolean, detail = "") {
  if (cond) {
    pass++;
  } else {
    fail++;
    failures.push(`${name}${detail ? `: ${detail}` : ""}`);
  }
}

/** Single-request validation used by the cold harness for first-valid-response. */
export async function checkFirstResponse(portArg: number, routeId: string): Promise<{ ok: boolean; detail: string }> {
  const b = `http://127.0.0.1:${portArg}`;
  const doFetch = (p: string, init?: RequestInit) => fetch(b + p, init).then((r) => r.text().then((t) => ({ status: r.status, body: t })));
  switch (routeId) {
    case "health.live": {
      const r = await doFetch("/health/live");
      return { ok: r.status === 200 && r.body === '{"status":"ok"}', detail: r.body };
    }
    case "js.text": {
      const r = await doFetch("/js-text");
      return { ok: r.status === 200 && r.body === "plain", detail: r.body };
    }
    case "js.json": {
      const r = await doFetch("/js-json");
      return { ok: r.status === 200 && r.body === '{"ok":true}', detail: r.body };
    }
    case "hello.get": {
      const r = await doFetch("/hello/Rafi");
      return { ok: r.status === 200 && r.body === '{"message":"Hello Rafi"}', detail: r.body };
    }
    case "users.create": {
      const r = await doFetch("/users", {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ name: "Ada", email: "ada@example.org" }),
      });
      return { ok: r.status === 201 && r.body === '{"id":"usr_1","name":"Ada","email":"ada@example.org"}', detail: r.body };
    }
    case "users.get": {
      const r = await doFetch("/users/usr_1", { headers: { authorization: "Bearer q-demo-token" } });
      return { ok: r.status === 200 && r.body === '{"id":"usr_1","name":"Ada","email":"ada@example.org"}', detail: r.body };
    }
    case "async.timer": {
      const r = await doFetch("/async?ms=10");
      return { ok: r.status === 200 && r.body === '{"waited":10}', detail: r.body };
    }
    case "throw.redacted": {
      const r = await doFetch("/throw");
      return { ok: r.status === 500 && !r.body.includes("secret"), detail: r.body };
    }
    default:
      return { ok: false, detail: `unknown route ${routeId}` };
  }
}

// ---------------------------------------------------------------- full suite

async function main() {
  // C0
  {
    const r = await req("GET", "/health/live");
    check("C0 status", r.status === 200, `got ${r.status}`);
    check("C0 exact bytes", r.body === fixture.routes[0].response.body, r.body);
  }
  // C1
  {
    const r = await req("GET", "/js-text");
    check("C1 status+bytes", r.status === 200 && r.body === "plain", `${r.status} ${r.body}`);
    check("C1 content-type", (r.headers["content-type"] ?? "").includes("text/plain"), r.headers["content-type"]);
  }
  // C2
  {
    const r = await req("GET", "/js-json");
    check("C2 status+bytes", r.status === 200 && r.body === '{"ok":true}', `${r.status} ${r.body}`);
  }
  // C3 hello
  {
    const r = await req("GET", "/hello/Rafi");
    check("C3 hello 200", r.status === 200, `got ${r.status}`);
    check("C3 hello bytes", r.body === '{"message":"Hello Rafi"}', r.body);
    const long = "x".repeat(61);
    const e = await req("GET", `/hello/${long}`);
    check("C3 hello 422 on long name", e.status === 422, `got ${e.status}`);
    check("C3 hello identifies field", e.body.includes("name"), e.body);
  }
  // C3 users.create
  {
    const r = await req("POST", "/users", { body: { name: "Ada", email: "ada@example.org" } });
    check("POST /users 201", r.status === 201, `got ${r.status}`);
    check("POST /users bytes", r.body === '{"id":"usr_1","name":"Ada","email":"ada@example.org"}', r.body);
    const malformed = await req("POST", "/users", { body: '{"name":', raw: true });
    check("malformed JSON 422", malformed.status === 422, `got ${malformed.status}`);
    const bad = await req("POST", "/users", { body: { name: "Ada", email: "not-an-email" } });
    check("bad email 422", bad.status === 422, `got ${bad.status}`);
    check("bad email identifies field", bad.body.includes("email"), bad.body);
  }
  // C4 users.get
  {
    const unauth = await req("GET", "/users/usr_1");
    check("C4 401 without token", unauth.status === 401, `got ${unauth.status}`);
    check("C4 401 is JSON", unauth.body.trim().startsWith("{"), unauth.body);
    const ok = await req("GET", "/users/usr_1", { headers: { authorization: "Bearer q-demo-token" } });
    check("C4 200 with token", ok.status === 200, `got ${ok.status}`);
    check("C4 bytes", ok.body === '{"id":"usr_1","name":"Ada","email":"ada@example.org"}', ok.body);
    const nf = await req("GET", "/users/usr_999", { headers: { authorization: "Bearer q-demo-token" } });
    check("C4 404 unknown user", nf.status === 404, `got ${nf.status}`);
  }
  // async
  {
    const r = await req("GET", "/async?ms=10");
    check("async waited 10", r.status === 200 && r.body === '{"waited":10}', `${r.status} ${r.body}`);
  }
  // throw redaction
  {
    const r = await req("GET", "/throw");
    check("throw 500", r.status === 500, `got ${r.status}`);
    check("throw redacted", !r.body.includes("secret") && !r.body.includes("at "), r.body);
  }
  // 404 + 405
  {
    const nf = await req("GET", "/definitely/not/here");
    check("unknown path 404", nf.status === 404, `got ${nf.status}`);
    const mna = await req("POST", "/js-text", { body: "" });
    check("405 on wrong method", mna.status === 405, `got ${mna.status}`);
    const allow = mna.headers["allow"] ?? "";
    check("405 Allow header", allow.includes("GET") && allow.includes("HEAD"), allow);
  }
  // HEAD
  {
    const res = await fetch(base + "/health/live", { method: "HEAD" });
    const body = await res.text();
    check("HEAD 200 empty body", res.status === 200 && body === "", `${res.status} '${body}'`);
  }
  // cancel abort (server health)
  {
    const controller = new AbortController();
    fetch(base + "/cancel?ms=1000", { signal: controller.signal }).catch(() => {});
    await new Promise((r) => setTimeout(r, 20));
    controller.abort();
    await new Promise((r) => setTimeout(r, 30));
    const live = await req("GET", "/health/live");
    check("server healthy after abort", live.status === 200, `got ${live.status}`);
  }

  // velqu-specific exact problem bodies
  if (candidate === "velqu") {
    const e = await req("GET", "/hello/" + "x".repeat(61));
    let problem: { type?: string; title?: string; status?: number; errors?: { path: string; code: string }[] } = {};
    try {
      problem = JSON.parse(e.body);
    } catch {
      check("velqu problem JSON", false, e.body);
    }
    check("velqu validation problem type", problem.type === "https://velqu.dev/problems/validation", problem.type);
    check("velqu validation problem status", problem.status === 422, String(problem.status));
    check(
      "velqu validation field errors",
      Array.isArray(problem.errors) && problem.errors[0]?.path === "name" && problem.errors[0]?.code === "maxLength",
      JSON.stringify(problem.errors),
    );
    const unauth = await req("GET", "/users/usr_1");
    const p = JSON.parse(unauth.body);
    check("velqu unauthorized problem", p.type === "https://velqu.dev/problems/unauthorized" && p.status === 401, unauth.body);
  }

  console.log(JSON.stringify({ candidate, pass, fail, failures }, null, 2));
  if (fail > 0) process.exit(1);
}

if (import.meta.main) {
  await main();
}
