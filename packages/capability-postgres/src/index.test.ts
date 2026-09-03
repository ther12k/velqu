/**
 * Postgres capability SDK tests (BETA-004-A).
 *
 * Deterministic coverage of identity pinning (must match the Rust ABI
 * model), the zero-construction cost posture, fail-closed behavior
 * without the native binding, and the parameterized-only API shape.
 * Live pool/query behavior arrives with BETA-004-B/C.
 */
import { describe, expect, test } from "bun:test";
import {
  postgres,
  MAX_POSTGRES_DEADLINE_MS,
  POSTGRES_CAPABILITY_ID,
  POSTGRES_CAPABILITY_VERSION,
  POSTGRES_GRANT,
  PostgresCapabilityUnavailable,
} from "./index";

describe("postgres capability identity (BETA-004-A)", () => {
  test("identity pins match the Rust ABI model exactly", () => {
    expect(POSTGRES_CAPABILITY_ID).toBe("runtime:postgres");
    expect(POSTGRES_CAPABILITY_VERSION).toBe(1);
    expect(POSTGRES_GRANT).toBe("postgres");
    expect(MAX_POSTGRES_DEADLINE_MS).toBe(120_000);
  });
});

describe("zero-cost posture", () => {
  test("import constructs no native bindings and no pool state", () => {
    expect((globalThis as Record<string, unknown>).__velquPostgresQuery).toBeUndefined();
    expect((globalThis as Record<string, unknown>).__velquPostgresPool).toBeUndefined();
    // the capability object itself carries only frozen identity metadata
    expect(Object.keys(postgres).sort()).toEqual([
      "capabilityId",
      "capabilityVersion",
      "sql",
    ]);
  });

  test("BETA-004-F: no-ORM surface freeze — the entire API is one parameterized method", () => {
    // The capability exposes exactly one operation: sql(text, params).
    // No query builder, no model/repository/migration DSL, no chaining —
    // a builder method added later would break this freeze.
    const methodNames = Object.getOwnPropertyNames(postgres).filter(
      (k) => typeof (postgres as Record<string, unknown>)[k] === "function",
    );
    expect(methodNames).toEqual(["sql"]);
    const banned = [
      "select", "insert", "update", "delete", "where", "from", "table",
      "join", "model", "define", "entity", "repository", "migrate",
      "migration", "schema", "relations", "createQueryBuilder", "builder",
    ];
    for (const name of banned) {
      expect((postgres as Record<string, unknown>)[name]).toBeUndefined();
    }
    // sql() is positional-parameters-only: no overload that takes a
    // template/config object (those are builder shapes)
    expect(postgres.sql.length).toBeLessThanOrEqual(3);
  });
});

describe("fail-closed without the native binding", () => {
  test("sql throws the typed unavailable error, never a silent fallback", () => {
    expect(() => postgres.sql("SELECT 1")).toThrow(PostgresCapabilityUnavailable);
    try {
      postgres.sql("SELECT 1");
    } catch (e) {
      expect((e as Error).name).toBe("PostgresCapabilityUnavailable");
      expect((e as Error).message).toContain("runtime:postgres");
    }
  });

  test("binding present but non-function still fails closed", () => {
    (globalThis as Record<string, unknown>).__velquPostgresQuery = 42;
    try {
      expect(() => postgres.sql("SELECT 1")).toThrow(PostgresCapabilityUnavailable);
    } finally {
      delete (globalThis as Record<string, unknown>).__velquPostgresQuery;
    }
  });
});

describe("parameterized-only API shape", () => {
  test("statement text is required", () => {
    const g = (globalThis as Record<string, unknown>).__velquPostgresQuery = (
      ..._args: unknown[]
    ) => ({ rows: [], affectedRows: 0 });
    try {
      expect(() => postgres.sql("")).toThrow(TypeError);
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      expect(() => (postgres as any).sql(undefined)).toThrow(TypeError);
    } finally {
      delete (globalThis as Record<string, unknown>).__velquPostgresQuery;
    }
  });

  test("deadline above the ceiling is a RangeError before any native call", () => {
    let called = false;
    (globalThis as Record<string, unknown>).__velquPostgresQuery = (..._args: unknown[]) => {
      called = true;
      return { rows: [], affectedRows: 0 };
    };
    try {
      expect(() => postgres.sql("SELECT 1", [], MAX_POSTGRES_DEADLINE_MS + 1)).toThrow(RangeError);
      expect(() => postgres.sql("SELECT 1", [], 0)).toThrow(RangeError);
      expect(called).toBe(false);
    } finally {
      delete (globalThis as Record<string, unknown>).__velquPostgresQuery;
    }
  });

  test("with the binding linked, sql passes text, copied params, and deadline", () => {
    let seen: unknown[] = [];
    (globalThis as Record<string, unknown>).__velquPostgresQuery = (...args: unknown[]) => {
      seen = args;
      return { rows: [{ id: "usr_1" }], affectedRows: 0 };
    };
    try {
      const res = postgres.sql("SELECT * FROM users WHERE id = $1", ["usr_1"], 2_500);
      expect(seen).toEqual(["SELECT * FROM users WHERE id = $1", ["usr_1"], 2_500]);
      expect(res.rows[0]?.id).toBe("usr_1");
    } finally {
      delete (globalThis as Record<string, unknown>).__velquPostgresQuery;
    }
  });
});
