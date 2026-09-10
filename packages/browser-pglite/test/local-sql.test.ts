/**
 * BWASM-C-003 — local-SQL contract suite (real engine, memory mode).
 *
 * Covers the acceptance criteria that need a REAL database:
 * SQL fixture corpus per the documented subset, parameters,
 * transactions (commit + rollback), lifecycle errors, deadlines,
 * export/import round-trip, and reset.
 *
 * Persistence-mode + isolation + lazy-load contracts live in their
 * sibling suites (fake-loader based; no browser needed).
 */
import { describe, expect, it } from "bun:test";
import {
  createLocalSql,
  LocalSqlError,
  storageName,
  MAX_OP_DEADLINE_MS,
} from "../src/index";

const ORIGIN = "http://test.local";

async function seeded(namespace = "app:main") {
  const db = createLocalSql({ namespace, origin: ORIGIN });
  await db.open();
  await db.query(`CREATE TABLE items (
    id serial PRIMARY KEY,
    name text NOT NULL,
    price_cents int NOT NULL DEFAULT 0
  )`);
  await db.query(
    "INSERT INTO items (name, price_cents) VALUES ($1,$2), ($3,$4), ($5,$6)",
    ["alpha", 100, "beta", 250, "gamma", 400],
  );
  return db;
}

describe("C-003 SQL fixture corpus (documented subset)", () => {
  it("DDL: CREATE / ALTER / DROP round-trip", async () => {
    const db = await seeded("ddl:app");
    await db.query("CREATE INDEX items_name_idx ON items (name)");
    await db.query("ALTER TABLE items ADD COLUMN note text");
    const withNote = await db.query<{ note: string | null }>(
      "SELECT note FROM items WHERE id = 1",
    );
    expect(withNote.rows[0].note).toBeNull();
    await db.query("DROP INDEX items_name_idx");
    await db.query("ALTER TABLE items DROP COLUMN note");
    const cols = await db.query<{ column_name: string }>(
      "SELECT column_name FROM information_schema.columns WHERE table_name = 'items' ORDER BY ordinal_position",
    );
    expect(cols.rows.map((r) => r.column_name)).toEqual(["id", "name", "price_cents"]);
    await db.close();
  });

  it("DML: INSERT with RETURNING, UPDATE, DELETE with bounded params", async () => {
    const db = await seeded("dml:app");
    const ins = await db.query<{ id: number; name: string }>(
      "INSERT INTO items (name, price_cents) VALUES ($1, $2) RETURNING id, name",
      ["delta", 50],
    );
    expect(ins.command).toBe("INSERT");
    expect(ins.rows[0].name).toBe("delta");

    const upd = await db.query<{}>("UPDATE items SET price_cents = $1 WHERE name = $2", [99, "alpha"]);
    expect(upd.rowCount).toBe(1);
    const alpha = await db.query<{ price_cents: number }>(
      "SELECT price_cents FROM items WHERE name = $1",
      ["alpha"],
    );
    expect(alpha.rows[0].price_cents).toBe(99);

    const del = await db.query<{}>("DELETE FROM items WHERE name = $1", ["gamma"]);
    expect(del.rowCount).toBe(1);
    const count = await db.query<{ n: number }>("SELECT count(*)::int AS n FROM items");
    expect(count.rows[0].n).toBe(3);
    await db.close();
  });

  it("joins, aggregates, CTE, window functions", async () => {
    const db = await seeded("select:app");
    await db.query("CREATE TABLE tags (item_id int REFERENCES items(id), tag text)");
    await db.query("INSERT INTO tags (item_id, tag) VALUES (1,'x'), (1,'y'), (2,'x')");

    const join = await db.query<{ name: string; tag: string }>(
      "SELECT i.name, t.tag FROM items i JOIN tags t ON t.item_id = i.id ORDER BY i.id, t.tag",
    );
    expect(join.rows).toHaveLength(3);

    const agg = await db.query<{ total: number }>(
      "SELECT sum(price_cents)::int AS total FROM items",
    );
    expect(agg.rows[0].total).toBe(750);

    const cte = await db.query<{ name: string }>(
      `WITH pricey AS (SELECT * FROM items WHERE price_cents > $1)
       SELECT name FROM pricey ORDER BY price_cents DESC`,
      [200],
    );
    expect(cte.rows.map((r) => r.name)).toEqual(["gamma", "beta"]);

    const win = await db.query<{ name: string; rank_: number }>(
      "SELECT name, rank() OVER (ORDER BY price_cents DESC) AS rank_ FROM items",
    );
    expect(win.rows[0]).toEqual({ name: "gamma", rank_: 1 });
    await db.close();
  });

  it("transactions: commit persists the unit, throw rolls the unit back", async () => {
    const db = await seeded("tx:app");
    const out = await db.transaction(async (tx) => {
      await tx.query("INSERT INTO items (name, price_cents) VALUES ($1, $2)", ["tx-ins", 1]);
      const c = await tx.query<{ n: number }>("SELECT count(*)::int AS n FROM items");
      return c.rows[0].n;
    });
    expect(out).toBe(4);

    await expect(
      db.transaction(async (tx) => {
        await tx.query("INSERT INTO items (name, price_cents) VALUES ($1, $2)", ["tx-bad", 1]);
        throw new Error("application error");
      }),
    ).rejects.toMatchObject({ code: "TransactionAborted" });

    const after = await db.query<{ n: number }>("SELECT count(*)::int AS n FROM items");
    expect(after.rows[0].n).toBe(4);

    // contract guards inside a transaction
    await expect(
      db.transaction(async (tx) => {
        await tx.exportAll();
      }),
    ).rejects.toMatchObject({ code: "TransactionAborted" });
    await db.close();
  });
});

describe("C-003 lifecycle + bounds (native ABI parity)", () => {
  it("describe() works BEFORE open; lazy lifecycle errors are typed", async () => {
    const db = createLocalSql({ namespace: "life:app", origin: ORIGIN });
    const d = db.describe();
    expect(d.capability).toBe("runtime:local-sql");
    expect(d.version).toBe(1);
    expect(d.storageName).toBe(storageName(ORIGIN, "life:app"));
    expect(d.engine).toBeUndefined();

    await expect(db.query("SELECT 1")).rejects.toMatchObject({ code: "NotOpen" });
    await db.open();
    await expect(db.open()).rejects.toMatchObject({ code: "AlreadyOpen" });
    expect(db.describe().engine).toContain("PGlite");
    await db.close();
    await expect(db.query("SELECT 1")).rejects.toMatchObject({ code: "Closed" });
  });

  it("namespace is required and validated (fail closed)", () => {
    expect(() => createLocalSql({ namespace: "", origin: ORIGIN })).toThrow(LocalSqlError);
    expect(() => createLocalSql({ namespace: "has space", origin: ORIGIN })).toThrow(/InvalidNamespace/);
    expect(() => createLocalSql({ namespace: "../escape", origin: ORIGIN })).toThrow(/InvalidNamespace/);
  });

  it("deadlines are fail-closed on the range (expiry covered by the fake-loader suite)", async () => {
    expect(() => createLocalSql({ namespace: "d:1", origin: ORIGIN, deadlineMs: 0 })).toThrow(/DeadlineOutOfRange/);
    expect(() => createLocalSql({ namespace: "d:2", origin: ORIGIN, deadlineMs: MAX_OP_DEADLINE_MS + 1 })).toThrow(/DeadlineOutOfRange/);
    // Expiry itself is proven with a yielding never-resolving fake query
    // (lazy.test.ts): a real in-engine pg_sleep BLOCKS this thread (the
    // documented no-preemption limit), so the timer cannot fire mid-exec.
    const db = await seeded("d:3");
    const ok = await db.query<{ x: number }>("SELECT 1 AS x");
    expect(ok.rows[0].x).toBe(1);
    await db.close();
  });

  it("engine failures surface as stable QueryFailed with the engine message", async () => {
    const db = await seeded("err:app");
    await expect(db.query("SELECT * FROM nope")).rejects.toMatchObject({
      code: "QueryFailed",
    });
    await db.close();
  });
});

describe("C-003 export / import / reset", () => {
  it("export → import round-trips the data; reset drops and recreates empty", async () => {
    const db = await seeded("io:app");
    const dump = await db.exportAll();
    expect(dump.byteLength).toBeGreaterThan(1000);

    await db.importAll(dump);
    const restored = await db.query<{ n: number }>("SELECT count(*)::int AS n FROM items");
    expect(restored.rows[0].n).toBe(3);

    await db.reset();
    // reset = drop + recreate: the user table is gone...
    await expect(db.query("SELECT count(*) FROM items")).rejects.toMatchObject({ code: "QueryFailed" });
    // ...and the fresh database accepts new schema
    await db.query("CREATE TABLE fresh (a int)");
    const n = await db.query<{ n: number }>("SELECT count(*)::int AS n FROM fresh");
    expect(n.rows[0].n).toBe(0);
    await db.close();
  });
});
