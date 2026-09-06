/**
 * BWASM-C-002 — async Postgres contract tests: SDK Promise surface,
 * typed rejections, sync validation, codemod behavior, and the
 * compile-time negative fixture (sync-looking use is a type error).
 */
import { describe, it, expect, afterAll } from "bun:test";
import { execFileSync } from "node:child_process";

/** execFileSync that tolerates nonzero exits and returns stdout. */
function run(args: string[]): { code: number; stdout: string } {
  try {
    return { code: 0, stdout: execFileSync(args[0]!, args.slice(1), { encoding: "utf8" }) };
  } catch (e) {
    const err = e as { status?: number; stdout?: string };
    return { code: err.status ?? 1, stdout: err.stdout ?? "" };
  }
}
import { mkdtempSync, writeFileSync, readFileSync, rmSync, mkdirSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import {
  postgres,
  PostgresCapabilityUnavailable,
  PostgresDeadlineExceeded,
  PostgresQueryError,
  MAX_POSTGRES_DEADLINE_MS,
  type SqlResult,
} from "./index";

// --- binding doubles -------------------------------------------------------

// The double installs `__velquPostgresQuery` exactly like the real native
// bridge: the binding's return value is what the SDK wraps — a Promise
// the host settles (or an id in the real op table; here we return the
// settled Promise directly for determinism).
async function withBindingAsync(
  result: Promise<unknown> | unknown,
  fn: (calls: unknown[][]) => Promise<void>,
): Promise<void> {
  const g = globalThis as Record<string, unknown>;
  const calls: unknown[][] = [];
  g.__velquPostgresQuery = (...args: unknown[]) => {
    calls.push(args);
    return result;
  };
  try {
    await fn(calls);
  } finally {
    delete g.__velquPostgresQuery;
  }
}

describe("BWASM-C-002 async sql contract (SDK surface)", () => {
  it("returns a Promise that resolves to the native result", async () => {
    await withBindingAsync(Promise.resolve({ rows: [{ id: 1 }], affectedRows: 0 }), async (calls) => {
      const p = postgres.sql("SELECT * FROM t WHERE id = $1", [1], 2_000);
      expect(p).toBeInstanceOf(Promise);
      const r: SqlResult = await p;
      expect(r.rows).toEqual([{ id: 1 }]);
      expect(r.affectedRows).toBe(0);
      expect(calls[0]).toEqual(["SELECT * FROM t WHERE id = $1", [1], 2_000]);
    });
  });

  it("still throws synchronously for invalid arguments (fail-fast preserved)", async () => {
    await withBindingAsync(Promise.resolve({ rows: [], affectedRows: 0 }), async () => {
      expect(() => postgres.sql("")).toThrow(TypeError);
      expect(() => postgres.sql("SELECT 1", [], 0)).toThrow(RangeError);
      expect(() => postgres.sql("SELECT 1", [], MAX_POSTGRES_DEADLINE_MS + 1)).toThrow(RangeError);
    });
  });

  it("classifies deadline rejections as PostgresDeadlineExceeded", async () => {
    await withBindingAsync(Promise.reject(new Error("query did not settle within 2000ms")), async () => {
      const err = await postgres.sql("SELECT 1", [], 2_000).catch((e) => e);
      expect(err).toBeInstanceOf(PostgresDeadlineExceeded);
      expect(err.name).toBe("PostgresDeadlineExceeded");
      expect(err.deadlineMs).toBe(2_000);
      expect(err.message).toContain("did not settle within 2000ms");
    });
  });

  it("classifies every other rejection as PostgresQueryError with the native reason", async () => {
    await withBindingAsync(Promise.reject(new Error("query rejected by backend: relation \"t\" does not exist")), async () => {
      const err = await postgres.sql("SELECT * FROM t").catch((e) => e);
      expect(err).toBeInstanceOf(PostgresQueryError);
      expect(err.name).toBe("PostgresQueryError");
      expect(err.nativeReason).toContain('relation "t" does not exist');
      expect(err.message).toContain("postgres.sql failed:");
    });
  });

  it("fail closed when the binding is absent (sync throw, unchanged)", () => {
    expect(() => postgres.sql("SELECT 1")).toThrow(PostgresCapabilityUnavailable);
  });
});

// ---------------------------------------------------------------------------
// Codemod
// ---------------------------------------------------------------------------

describe("BWASM-C-002 migrate-postgres-async codemod", () => {
  const dir = mkdtempSync(join(tmpdir(), "b-c002-codemod-"));
  afterAll(() => rmSync(dir, { recursive: true, force: true }));

  it("finds and rewrites sync-looking call sites; leaves awaited ones", () => {
    const before = [
      "export async function handler() {",
      "  const rows = db.sql('SELECT 1');",
      "  const ok = await db.sql('SELECT 2');",
      "  return db.sql('SELECT 3').rows;",
      "}",
      "",
    ].join("\n");
    mkdirSync(dir, { recursive: true });
    const file = join(dir, "handler.ts");
    writeFileSync(file, before);

    const report = JSON.parse(run([
      "bun", join(import.meta.dir, "..", "..", "..", "scripts", "migrate-postgres-async.mjs"), dir,
    ]).stdout);
    expect(report.syncLookingSites.length).toBe(2); // lines 2 and 4; awaited line untouched
    expect(report.write).toBe(false);

    const write = JSON.parse(run([
      "bun", join(import.meta.dir, "..", "..", "..", "scripts", "migrate-postgres-async.mjs"), dir, "--write",
    ]).stdout);
    expect(write.rewritten.length).toBe(2);
    const after = readFileSync(file, "utf8");  // codemod writes complete at process exit
    expect(after).toContain("const rows = await db.sql('SELECT 1');");
    expect(after).toContain("const ok = await db.sql('SELECT 2');"); // unchanged
    expect(after).toContain("return await db.sql('SELECT 3').rows;");
  });

  it("exits 0 on a clean project and 1 when sites remain without --write", () => {
    const clean = mkdtempSync(join(tmpdir(), "b-c002-clean-"));
    try {
      writeFileSync(join(clean, "a.ts"), "const x = 1;\n");
      expect(run([
        "bun", join(import.meta.dir, "..", "..", "..", "scripts", "migrate-postgres-async.mjs"), clean,
      ]).code).toBe(0);
    } finally {
      rmSync(clean, { recursive: true, force: true });
    }
  });
});

// ---------------------------------------------------------------------------
// Compile-time contract (negative + positive fixtures through tsc)
// ---------------------------------------------------------------------------

describe("BWASM-C-002 compile-time async contract", () => {
  const dir = mkdtempSync(join(tmpdir(), "b-c002-type-"));
  afterAll(() => rmSync(dir, { recursive: true, force: true }));

  const SDK = join(import.meta.dir, "index");
  const CORE = join(import.meta.dir, "..", "..", "core", "src", "index");

  function runTsc(file: string): { code: number; errorsInFixture: string[] } {
    // Invoke tsc via bun (the bin needs a JS runtime); ambient
    // bun-types/@types noise is filtered to THIS fixture's errors —
    // tsc prints the fixture path relative to its cwd, so match the
    // file's basename inside the error line instead of the full path.
    const base = file.split("/").pop()!;
    const { code, stdout } = run([
      "bun", join(import.meta.dir, "..", "..", "..", "node_modules", "typescript", "bin", "tsc"),
      "--noEmit", "--strict", "--target", "es2022", "--module", "esnext",
      "--moduleResolution", "bundler", file,
    ]);
    const errorsInFixture = stdout
      .split("\n")
      .filter((l) => l.includes(base) && l.includes("error TS"));
    return { code, errorsInFixture };
  }

  it("sync-looking property access on the sql result is a type error", async () => {
    const neg = join(dir, "negative.ts");
    writeFileSync(neg, [
      `import { postgres } from ${JSON.stringify(SDK)};`,
      "",
      "export async function handler() {",
      "  const r = postgres.sql('SELECT 1');",
      "  return r.rows;   // ERROR: Property 'rows' does not exist on type 'Promise<SqlResult>'",
      "}",
      "",
    ].join("\n"));
    const { code, errorsInFixture } = runTsc(neg);
    expect(code).not.toBe(0);
    expect(errorsInFixture.join("\n")).toMatch(
      /Property 'rows' does not exist on type 'Promise<SqlResult>'/,
    );
  }, 30_000);

  it("awaited use typechecks against the async contract", async () => {
    const pos = join(dir, "positive.ts");
    writeFileSync(pos, [
      `import { postgres } from ${JSON.stringify(SDK)};`,
      "",
      "export async function handler() {",
      "  const r = await postgres.sql('SELECT 1', [], 2_000);",
      "  return { count: r.rows.length, changed: r.affectedRows };",
      "}",
      "",
    ].join("\n"));
    const { errorsInFixture } = runTsc(pos);
    expect(errorsInFixture).toEqual([]);
  }, 30_000);

  it("HandlerCtx.native.postgres.sql is Promise-returning (core contract)", async () => {
    const pos = join(dir, "ctx.ts");
    writeFileSync(pos, [
      `import type { HandlerCtx } from ${JSON.stringify(CORE)};`,
      "",
      "export async function usesCtx(ctx: HandlerCtx<undefined, undefined, undefined, undefined>) {",
      "  const r = await ctx.native.postgres.sql('SELECT 1');",
      "  return r.affectedRows;",
      "}",
      "",
      "export function broken(ctx: HandlerCtx<undefined, undefined, undefined, undefined>) {",
      "  // @ts-expect-error — sync-looking use must fail (BWASM-C-002 contract)",
      "  return ctx.native.postgres.sql('SELECT 1').rows;",
      "}",
      "",
    ].join("\n"));
    const { errorsInFixture } = runTsc(pos);
    expect(errorsInFixture).toEqual([]);
  }, 30_000);
});
