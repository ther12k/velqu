/**
 * #1426 — canary-shadow mask scope. The harness used to mask every
 * `"instance"` member in every response body, so ordinary application
 * payloads differing in an application-level `instance` field compared
 * EQUAL (reproduced: baseline {"instance":"tenant-A"} vs candidate
 * {"instance":"tenant-B"} in application/json reported 0 divergences).
 * The mask must apply only to the top-level RFC 9457 `instance` member of
 * application/problem+json responses.
 */
import { describe, it, expect, afterEach } from "bun:test";
import { writeFileSync, unlinkSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";

const script = join(import.meta.dir, "canary-shadow.sh");

type Stub = ReturnType<typeof Bun.serve>;
const servers: Stub[] = [];
const manifests: string[] = [];

afterEach(() => {
  for (const s of servers.splice(0)) s.stop(true);
  for (const m of manifests.splice(0)) {
    try {
      unlinkSync(m);
    } catch {}
  }
});

function stub(handler: (req: Request) => Response): Stub {
  const s = Bun.serve({ hostname: "127.0.0.1", port: 0, fetch: handler });
  servers.push(s);
  return s;
}

async function run(base: Stub, cand: Stub, manifest: unknown) {
  const path = join(tmpdir(), `velqu-canary-mask-${Date.now()}-${Math.random().toString(36).slice(2)}.json`);
  writeFileSync(path, JSON.stringify(manifest));
  manifests.push(path);
  // async spawn (not spawnSync): the stub servers live in THIS process, so
  // the event loop must stay free to answer the child's fetches
  const proc = Bun.spawn(["bash", script, base.url.origin, cand.url.origin, path, "1"], {
    stdout: "pipe",
    stderr: "pipe",
  });
  const [out, err] = await Promise.all([
    new Response(proc.stdout).text(),
    new Response(proc.stderr).text(),
  ]);
  return { exit: await proc.exited, out, err };
}

const json = (status: number, ctype: string, body: unknown) =>
  new Response(JSON.stringify(body), { status, headers: { "content-type": ctype } });

describe("canary-shadow mask scope (#1426)", () => {
  it("masks the top-level instance member of problem responses (request-scoped)", async () => {
    const base = stub(() =>
      json(404, "application/problem+json", {
        type: "https://velqu.dev/problems/not-found",
        title: "Not Found",
        status: 404,
        instance: "/orders/abc/base-req-1",
      }),
    );
    const cand = stub(() =>
      json(404, "application/problem+json", {
        type: "https://velqu.dev/problems/not-found",
        title: "Not Found",
        status: 404,
        instance: "/orders/abc/cand-req-2",
      }),
    );
    const res = await run(base, cand, [{ method: "GET", path: "/orders/abc" }]);
    expect(res.exit).toBe(0);
    expect(res.out).toContain("0 divergences");
  });

  it("reports differing application-level instance values in ordinary JSON as divergence (was a false pass)", async () => {
    const base = stub(() => json(200, "application/json", { instance: "tenant-A", value: 1 }));
    const cand = stub(() => json(200, "application/json", { instance: "tenant-B", value: 1 }));
    const res = await run(base, cand, [{ method: "GET", path: "/data" }]);
    expect(res.exit).toBe(1);
    expect(res.err).toContain("DIVERGENCE");
    expect(res.out).toContain("1 divergences");
  });

  it("reports differing NESTED instance members in application JSON", async () => {
    const base = stub(() => json(200, "application/json", { payload: { instance: "a" } }));
    const cand = stub(() => json(200, "application/json", { payload: { instance: "b" } }));
    const res = await run(base, cand, [{ method: "GET", path: "/nested" }]);
    expect(res.exit).toBe(1);
    expect(res.err).toContain("DIVERGENCE");
  });

  it("identical application bodies with an instance field stay equivalent", async () => {
    const base = stub(() => json(200, "application/json", { instance: "tenant-A", value: 1 }));
    const cand = stub(() => json(200, "application/json", { instance: "tenant-A", value: 1 }));
    const res = await run(base, cand, [{ method: "GET", path: "/data" }]);
    expect(res.exit).toBe(0);
    expect(res.out).toContain("0 divergences");
  });

  it("still flags any other differing problem-body member", async () => {
    const base = stub(() =>
      json(422, "application/problem+json", {
        title: "Validation",
        status: 422,
        instance: "/x/base",
      }),
    );
    const cand = stub(() =>
      json(422, "application/problem+json", {
        title: "Validation (changed)",
        status: 422,
        instance: "/x/cand",
      }),
    );
    const res = await run(base, cand, [{ method: "GET", path: "/x" }]);
    expect(res.exit).toBe(1);
    expect(res.err).toContain("DIVERGENCE");
  });
});
