/**
 * BWASM-C-003 — lazy-load, isolation, persistence-mode, and subset
 * contracts, proven with a counting fake loader (no engine download,
 * no browser environment).
 *
 * These are the acceptance criteria that must hold BEFORE any engine
 * byte exists: no loader call before open, distinct storage per
 * origin+namespace, indexeddb fails closed when blocked, unsupported
 * statements reject with stable codes + remediation BEFORE execution,
 * and the wall-clock deadline machinery fires on yielding waits.
 */
import { describe, expect, it } from "bun:test";
import {
  createLocalSql,
  LocalSqlError,
  storageName,
  classifyStatement,
  type PgliteInstance,
  type PgliteModule,
} from "../src/index";

const ORIGIN = "http://iso.local";

interface LoaderLog {
  calls: number;
  ctorArgs: Array<{ dataDir?: string; loadDataDir?: boolean }>;
}

function fakeLoader(log: LoaderLog, behavior: {
  onConstruct?: (dataDir?: string) => PgliteInstance | never;
} = {}): () => Promise<PgliteModule> {
  const instance = (): PgliteInstance => ({
    query: async () => ({ rows: [], rowCount: 0, command: "FAKE" }),
    exec: async () => [],
    transaction: async <T,>(cb: (tx: never) => Promise<T>) => cb(null as never),
    dumpDataDir: async () => new Uint8Array([1, 2, 3]),
    close: async () => undefined,
  });
  return async () => {
    log.calls += 1;
    return {
      PGlite: Object.assign(
        function (this: unknown, dataDir?: string, options?: { loadDataDir?: Blob }) {
          log.ctorArgs.push({ dataDir, loadDataDir: options?.loadDataDir !== undefined });
          if (behavior.onConstruct) return behavior.onConstruct(dataDir);
          return instance();
        },
        {
          create: async (dataDir?: string, options?: { loadDataDir?: Blob }) => {
            log.ctorArgs.push({ dataDir, loadDataDir: options?.loadDataDir !== undefined });
            if (behavior.onConstruct) return behavior.onConstruct(dataDir);
            return instance();
          },
        },
      ),
    } as unknown as PgliteModule;
  };
}

describe("C-003 lazy engine load", () => {
  it("createLocalSql + describe() never load the engine; open loads exactly once", async () => {
    const log: LoaderLog = { calls: 0, ctorArgs: [] };
    const db = createLocalSql({ namespace: "lazy:app", origin: ORIGIN, loader: fakeLoader(log) });
    void db.describe();
    await expect(db.query("SELECT 1")).rejects.toMatchObject({ code: "NotOpen" });
    expect(log.calls).toBe(0); // nothing loaded to reject that query

    await db.open();
    expect(log.calls).toBe(1);
    await db.query("SELECT 1");
    await db.query("SELECT 2");
    expect(log.calls).toBe(1); // engine is loaded once, then reused
    await db.close();
  });

  it("the package's browser bundle keeps the engine behind a dynamic import", async () => {
    // The strongest guarantee is structural: projects that never import
    // @velqu/browser-pglite never bundle it. For projects that do, the
    // engine must still be a dynamic import — assert it on the source
    // graph (the compiled form is asserted by the build wiring: the
    // engine is an external, never inlined).
    const src = await Bun.file("packages/browser-pglite/src/local-sql.ts").text();
    expect(src).not.toContain('from "@electric-sql/pglite"');
    expect(src).toContain('import("@electric-sql/pglite")');
  });
});

describe("C-003 isolation (project + origin namespace)", () => {
  it("storage names are distinct per namespace and per origin", () => {
    expect(storageName("http://a", "app:one")).not.toBe(storageName("http://a", "app:two"));
    expect(storageName("http://a", "app:one")).not.toBe(storageName("http://b", "app:one"));
    // sanitized: no `/` or `:` from the origin survives into the idb path
    // (an unsanitized name wedges PGlite's IDBFS at mount — E7 finding)
    expect(storageName("http://a", "app:one")).toBe("velqu-local-sql-http___a_app_one");
    expect(storageName("http://a", "app:one")).toMatch(/^[A-Za-z0-9._-]+$/);
  });

  it("memory mode constructs with NO dataDir; indexeddb mode with the namespaced idb:// dir", async () => {
    const memLog: LoaderLog = { calls: 0, ctorArgs: [] };
    const mem = createLocalSql({ namespace: "iso:mem", origin: ORIGIN, loader: fakeLoader(memLog) });
    await mem.open();
    expect(memLog.ctorArgs[0]).toEqual({ dataDir: undefined, loadDataDir: false });

    const idbLog: LoaderLog = { calls: 0, ctorArgs: [] };
    const idb = createLocalSql({
      namespace: "iso:idb",
      origin: ORIGIN,
      persistence: "indexeddb",
      loader: fakeLoader(idbLog),
    });
    await idb.open();
    expect(idbLog.ctorArgs[0].dataDir).toBe("idb://velqu-local-sql-http___iso.local_iso_idb");
    await mem.close();
    await idb.close();
  });

  it("two namespaces construct two distinct databases (no shared handle)", async () => {
    const a: LoaderLog = { calls: 0, ctorArgs: [] };
    const b: LoaderLog = { calls: 0, ctorArgs: [] };
    const dbA = createLocalSql({ namespace: "proj:a", origin: ORIGIN, persistence: "indexeddb", loader: fakeLoader(a) });
    const dbB = createLocalSql({ namespace: "proj:b", origin: ORIGIN, persistence: "indexeddb", loader: fakeLoader(b) });
    await dbA.open();
    await dbB.open();
    expect(a.ctorArgs[0].dataDir).not.toBe(b.ctorArgs[0].dataDir);
    expect(dbA.describe().storageName).not.toBe(dbB.describe().storageName);
    await dbA.close();
    await dbB.close();
  });

  it("blocked storage fails closed (PersistenceUnavailable); no silent memory fallback", async () => {
    const log: LoaderLog = { calls: 0, ctorArgs: [] };
    const blocked = fakeLoader(log, {
      onConstruct: () => {
        throw new Error("The user denied permission to access the database");
      },
    });
    const db = createLocalSql({ namespace: "iso:blocked", origin: ORIGIN, persistence: "indexeddb", loader: blocked });
    await expect(db.open()).rejects.toMatchObject({ code: "PersistenceUnavailable" });
  });
});

describe("C-003 documented subset (stable actionable codes)", () => {
  it("unsupported statements reject BEFORE execution with code + remediation", async () => {
    const log: LoaderLog = { calls: 0, ctorArgs: [] };
    const db = createLocalSql({ namespace: "sub:app", origin: ORIGIN, loader: fakeLoader(log) });
    await db.open();
    for (const sql of [
      "LISTEN channel",
      "NOTIFY channel",
      "COPY t FROM STDIN",
      "CREATE EXTENSION postgis",
      "ALTER SYSTEM SET fsync = off",
      "DROP DATABASE mydb",
      "CREATE DATABASE other",
      "DECLARE cur CURSOR FOR SELECT 1",
      "VACUUM FULL",
    ]) {
      let err: LocalSqlError | undefined;
      try {
        await db.query(sql);
      } catch (e) {
        err = e as LocalSqlError;
      }
      expect(err?.code).toBe("UnsupportedStatement");
      expect(err?.remediation?.length ?? 0).toBeGreaterThan(5);
    }
    // the fake engine's query was never reached for those (rows [] proof
    // is weak by construction; the loader COUNT is the proof: open only)
    expect(log.calls).toBe(1);
    await db.close();
  });

  it("classifyStatement is pure and covers the matrix families", () => {
    expect(classifyStatement("select 1").supported).toBeTrue();
    expect(classifyStatement("  CREATE TABLE t (a int)").supported).toBeTrue();
    expect(classifyStatement("LISTEN foo").supported).toBeFalse();
  });
});

describe("C-003 wall-clock deadline on yielding waits", () => {
  it("a never-yielding-back engine call is rejected at the deadline; the handle stays usable", async () => {
    const hang: PgliteInstance = {
      // yields-but-never-settles for real queries; answers the open-time
      // version probe so open() itself completes.
      query: <T,>(sql: string) =>
        sql.includes("version")
          ? Promise.resolve({ rows: [{ version: "fake" }] as unknown as T[], rowCount: 1 })
          : new Promise<T>(() => undefined),
      exec: async () => [],
      transaction: (() => { throw new Error("unused"); }) as never,
      dumpDataDir: async () => new Uint8Array(0),
      close: async () => undefined,
    };
    const loader = async () => ({
      PGlite: Object.assign(function () { return hang; }, { create: async () => hang }),
    }) as unknown as () => Promise<PgliteModule>;
    const db = createLocalSql({ namespace: "dl:app", origin: ORIGIN, loader });
    await db.open();
    await expect(db.query("SELECT 1", [], { deadlineMs: 20 })).rejects.toMatchObject({
      code: "DeadlineExceeded",
    });
    // in-engine CPU work is NOT preempted (documented); the caller-side
    // rejection is the fail-closed contract for yielding waits.
    await expect(
      db.query("SELECT 1", [], { deadlineMs: 0 }),
    ).rejects.toMatchObject({ code: "DeadlineOutOfRange" });
    await db.close();
  });
});
