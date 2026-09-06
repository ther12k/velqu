/**
 * BWASM-C-004 — namespaced IndexedDB KV persistence tests.
 *
 * ONE shared contract suite runs against BOTH adapters (memory +
 * IndexedDB-over-fake); IDB-only blocks cover migration, blocked-db,
 * and cross-namespace store isolation. The fake IDB implements the
 * same request/event surface the real API uses (async success events),
 * so the adapter's transaction-chaining path is exercised; the
 * real-browser lane (genuine IndexedDB in Chromium) is the committed
 * rehearsal script, per the B-006/C-001 precedent.
 */
import { describe, it, expect } from "bun:test";
import {
  createMemoryKv,
  createIndexedDbKv,
  kvHandle,
  KvKeyInvalid,
  KvQuotaExceeded,
  KvSerializationError,
  KvMigrationRequired,
  KvUnavailable,
  KV_META_KEY,
  KV_CAPABILITY_ID,
  type KvCapability,
  type KvIdbFactory,
  type KvIdbDatabase,
} from "../src/kv";

// ---------------------------------------------------------------------------
// Spec-shaped fake IndexedDB (async success events; persistent maps)
// ---------------------------------------------------------------------------

function fakeIdbFactory(): KvIdbFactory & { databases(): Map<string, FakeDb> } {
  const databases = new Map<string, FakeDb>();
  class FakeRequest {
    listeners = new Map<string, ((e: unknown) => void)[]>();
    result: unknown = undefined;
    error: unknown = undefined;
    addEventListener(type: string, listener: (e: unknown) => void) {
      this.listeners.set(type, [...(this.listeners.get(type) ?? []), listener]);
    }
    settle(result?: unknown, error?: unknown) {
      if (error) {
        this.error = error;
        for (const l of this.listeners.get("error") ?? []) queueMicrotask(() => l({ target: this }));
        return;
      }
      if (result !== undefined) this.result = result;
      for (const l of this.listeners.get("success") ?? []) queueMicrotask(() => l({ target: this }));
    }
  }
  class FakeStore {
    constructor(public db: FakeDb) {}
    get(key: string) {
      const req = new FakeRequest();
      queueMicrotask(() => {
        const hit = this.db.stores.get(this.db.currentStore)?.get(key);
        req.settle(hit === undefined ? undefined : structuredClone(hit));
      });
      return req;
    }
    put(value: unknown, key: string) {
      const req = new FakeRequest();
      let store = this.db.stores.get(this.db.currentStore);
      if (!store) {
        store = new Map();
        this.db.stores.set(this.db.currentStore, store);
      }
      queueMicrotask(() => {
        // real IndexedDB structured-clones on put — the fake must too,
        // otherwise stored references alias the caller's objects
        store!.set(key, structuredClone(value));
        req.settle(undefined);
      });
      return req;
    }
    delete(key: string) {
      const req = new FakeRequest();
      queueMicrotask(() => {
        this.db.stores.get(this.db.currentStore)?.delete(key);
        req.settle(undefined);
      });
      return req;
    }
    getAllKeys() {
      const req = new FakeRequest();
      queueMicrotask(() => req.settle([...(this.db.stores.get(this.db.currentStore)?.keys() ?? [])]));
      return req;
    }
    clear() {
      const req = new FakeRequest();
      queueMicrotask(() => {
        this.db.stores.get(this.db.currentStore)?.clear();
        req.settle(undefined);
      });
      return req;
    }
  }
  class FakeDb implements KvIdbDatabase {
    stores = new Map<string, Map<string, unknown>>();
    currentStore = "";
    version = 1;
    objectStoreNames = { contains: (name: string) => this.stores.has(name) };
    onversionchange: ((event: unknown) => void) | null = null;
    createObjectStore(name: string) {
      this.stores.set(name, new Map());
      return {};
    }
    transaction(storeName: string, _mode: "readonly" | "readwrite") {
      this.currentStore = storeName;
      const store = new FakeStore(this);
      return { objectStore: () => store };
    }
    close() {}
  }
  return {
    databases,
    open(name: string, version?: number, upgrade?: (db: KvIdbDatabase, tx: unknown, oldVersion: number) => void) {
      const request = new FakeRequest();
      queueMicrotask(() => {
        let db = databases.get(name);
        if (!db) {
          db = new FakeDb();
          databases.set(name, db);
          // fresh db: upgradeneeded fires (real API parity)
          for (const l of request.listeners.get("upgradeneeded") ?? []) l({ target: { result: db } });
          upgrade?.(db, {}, 0);
        } else if (version !== undefined && version > db.version) {
          // version bump: upgradeneeded fires with the old version
          for (const l of request.listeners.get("upgradeneeded") ?? []) l({ target: { result: db } });
          upgrade?.(db, {}, db.version);
          db.version = version;
        }
        request.result = db;
        for (const l of request.listeners.get("success") ?? []) l({ target: request });
      });
      return request;
    },
  };
}

// ---------------------------------------------------------------------------
// The shared contract suite (runs against BOTH adapters)
// ---------------------------------------------------------------------------

describe("C-004 shared KV contract (memory + indexeddb)", () => {
  function makeAdapters() {
    const idb = fakeIdbFactory();
    const memory = createMemoryKv({ namespace: "app:shared" });
    const indexeddb = createIndexedDbKv({ namespace: "app:shared", idbFactory: idb });
    return { memory, indexeddb };
  }

  for (const backend of ["memory", "indexeddb"] as const) {
    describe(`backend: ${backend}`, () => {
      let kv: KvCapability;
      it("set/get round-trips structured values", async () => {
        kv = makeAdapters()[backend] as KvCapability;
        await kv.set("a", { nested: [1, "two", null], flag: true });
        expect(await kv.get("a")).toEqual({ nested: [1, "two", null], flag: true });
      });

      it("returns null for missing keys and deletes cleanly", async () => {
        expect(await kv.get("missing")).toBeNull();
        await kv.set("gone", 1);
        await kv.delete("gone");
        expect(await kv.get("gone")).toBeNull();
      });

      it("lists sorted with optional prefix filtering", async () => {
        await kv.setMany([
          { key: "user:1", value: "a" },
          { key: "user:2", value: "b" },
          { key: "settings:x", value: "c" },
        ]);
        expect(await kv.list()).toEqual(["a", "settings:x", "user:1", "user:2"].sort());
        expect(await kv.list("user:")).toEqual(["user:1", "user:2"]);
      });

      it("setMany is atomic: a quota failure leaves no partial writes", async () => {
        const small = makeAdapters()[backend] as KvCapability & { describe(): unknown };
        const limited = backend === "memory"
          ? createMemoryKv({ namespace: "app:atomic", maxEntries: 2 })
          : createIndexedDbKv({ namespace: "app:atomic", maxEntries: 2, idbFactory: fakeIdbFactory() });
        await limited.set("seed", 0);
        await expect(
          limited.setMany([
            { key: "a", value: 1 },
            { key: "b", value: 2 },
            { key: "c", value: 3 },
          ]),
        ).rejects.toBeInstanceOf(KvQuotaExceeded);
        expect(await limited.list()).toEqual(["seed"]);
        void small;
      });

      it("clear removes data but the adapter keeps working", async () => {
        await kv.set("x", 1);
        await kv.clear();
        expect(await kv.list()).toEqual([]);
        await kv.set("y", 2);
        expect(await kv.get("y")).toBe(2);
      });

      it("exportAll returns the full namespace snapshot", async () => {
        await kv.clear();
        await kv.set("e1", "v1");
        await kv.set("e2", { deep: true });
        expect(await kv.exportAll()).toEqual({ e1: "v1", e2: { deep: true } });
      });

      it("reset empties everything including metadata state", async () => {
        await kv.set("r", 1);
        await kv.reset();
        expect(await kv.list()).toEqual([]);
        expect(await kv.get("r")).toBeNull();
      });

      it("gc is a no-op within budget and never touches live data", async () => {
        const limited = backend === "memory"
          ? createMemoryKv({ namespace: "app:gc", maxEntries: 2 })
          : createIndexedDbKv({ namespace: "app:gc", maxEntries: 2, idbFactory: fakeIdbFactory() });
        await limited.set("keep1", 1);
        await limited.set("keep2", 2);
        expect(await limited.gc()).toBe(0);
        expect(await limited.list()).toEqual(["keep1", "keep2"]);
      });

      it("rejects invalid keys and non-cloneable values with structured errors", async () => {
        await expect(kv.set("", 1)).rejects.toBeInstanceOf(KvKeyInvalid);
        await expect(kv.set("\u0000meta", 1)).rejects.toBeInstanceOf(KvKeyInvalid);
        const fn = (() => {}) as unknown as string;
        await expect(kv.set("fn", fn)).rejects.toBeInstanceOf(KvSerializationError);
      });

      it("enforces per-value and entry-count quotas with structured errors", async () => {
        const limited = backend === "memory"
          ? createMemoryKv({ namespace: "app:quota", maxEntries: 2, maxValueBytes: 100 })
          : createIndexedDbKv({ namespace: "app:quota", maxEntries: 2, maxValueBytes: 100, idbFactory: fakeIdbFactory() });
        await expect(limited.set("big", "x".repeat(101))).rejects.toBeInstanceOf(KvQuotaExceeded);
        await limited.set("k1", 1);
        await limited.set("k2", 2);
        await expect(limited.set("k3", 3)).rejects.toBeInstanceOf(KvQuotaExceeded);
      });

      it("values are isolated copies (no live-reference aliasing)", async () => {
        const original = { inner: [1] };
        await kv.set("alias", original);
        original.inner.push(2);
        expect(await kv.get("alias")).toEqual({ inner: [1] });
      });
    });
  }
});

// ---------------------------------------------------------------------------
// IndexedDB-only: migration, blocked/unavailable, namespace isolation
// ---------------------------------------------------------------------------

describe("C-004 indexeddb migration + availability + isolation", () => {
  it("version mismatch without a migration fails closed and preserves data", async () => {
    const idb = fakeIdbFactory();
    const v1 = createIndexedDbKv({ namespace: "app:mig", schemaVersion: 1, idbFactory: idb });
    await v1.set("keep", "me");
    const v2 = createIndexedDbKv({ namespace: "app:mig", schemaVersion: 2, idbFactory: idb });
    await expect(v2.get("keep")).rejects.toBeInstanceOf(KvMigrationRequired);
    // data untouched by the failed open
    const v1b = createIndexedDbKv({ namespace: "app:mig", schemaVersion: 1, idbFactory: idb });
    expect(await v1b.get("keep")).toBe("me");
  });

  it("declared migrations run and the version advances", async () => {
    const idb = fakeIdbFactory();
    const v1 = createIndexedDbKv({ namespace: "app:mig2", schemaVersion: 1, idbFactory: idb });
    await v1.set("old-format", "legacy");
    const v2 = createIndexedDbKv({
      namespace: "app:mig2",
      schemaVersion: 2,
      idbFactory: idb,
      migrations: {
        1: async (store) => {
          const value = await store.get("old-format");
          if (value !== null) {
            await store.delete("old-format");
            await store.set("new-format", { migrated: value });
          }
        },
      },
    });
    expect(await v2.get("new-format")).toEqual({ migrated: "legacy" });
    expect(await v2.get("old-format")).toBeNull();
    // reopening at v2 is stable
    const v2b = createIndexedDbKv({ namespace: "app:mig2", schemaVersion: 2, idbFactory: idb });
    expect(await v2b.get("new-format")).toEqual({ migrated: "legacy" });
  });

  it("unavailable IndexedDB fails closed by default; memory fallback is explicit and flagged", async () => {
    const missing = {} as KvIdbFactory;
    expect(() => createIndexedDbKv({ namespace: "app:x", idbFactory: missing })).toThrow(KvUnavailable);
    const fallback = createIndexedDbKv({ namespace: "app:x", idbFactory: missing, onUnavailable: "memory" });
    expect(fallback.backend).toBe("memory");
    expect(fallback.describe().backend).toBe("memory");
  });

  it("gc evicts oldest-written entries after a quota tightening (explicit control)", async () => {
    const idb = fakeIdbFactory();
    const wide = createIndexedDbKv({ namespace: "app:gc2", maxEntries: 5, idbFactory: idb });
    for (let i = 0; i < 4; i++) {
      await wide.set(`k${i}`, i);
      await new Promise((r) => setTimeout(r, 4));
    }
    // the deployment tightens its entry budget and reclaims
    const tight = createIndexedDbKv({ namespace: "app:gc2", maxEntries: 2, idbFactory: idb });
    const removed = await tight.gc();
    expect(removed).toBe(2);
    expect(await tight.get("k0")).toBeNull();
    expect(await tight.get("k1")).toBeNull();
    expect(await tight.get("k3")).toBe(3);
    // subsequent writes respect the tightened budget
    await expect(tight.set("k4", 4)).rejects.toBeInstanceOf(KvQuotaExceeded);
  });

  it("one project cannot enumerate or read another project's keys", async () => {
    const idb = fakeIdbFactory();
    const projectA = createIndexedDbKv({ namespace: "app-a:data", idbFactory: idb });
    const projectB = createIndexedDbKv({ namespace: "app-b:data", idbFactory: idb });
    await projectA.set("secret", "a-data");
    // B's list and get cannot see A's namespace (separate object stores)
    expect(await projectB.list()).toEqual([]);
    expect(await projectB.get("secret")).toBeNull();
    await projectB.set("secret", "b-data");
    expect(await projectA.get("secret")).toBe("a-data");
    expect(await projectB.get("secret")).toBe("b-data");
    // clear is namespace-scoped too
    await projectB.clear();
    expect(await projectA.get("secret")).toBe("a-data");
  });

  it("namespace names are part of the store identity (no prefix collisions)", async () => {
    const idb = fakeIdbFactory();
    const a = createIndexedDbKv({ namespace: "app:users", idbFactory: idb });
    const b = createIndexedDbKv({ namespace: "app:users:admin", idbFactory: idb });
    await a.set("x", "a");
    await b.set("x", "b");
    expect(await a.get("x")).toBe("a");
    expect(await b.get("x")).toBe("b");
    expect(await a.list()).toEqual(["x"]);
  });
});

// ---------------------------------------------------------------------------
// Registry handle + description
// ---------------------------------------------------------------------------

describe("C-004 kv handle + introspection", () => {
  it("exposes the op-table handle under runtime:kv v1", async () => {
    const kv = createMemoryKv({ namespace: "app:handle" });
    const handle = kvHandle(kv);
    expect(handle.id).toBe(KV_CAPABILITY_ID);
    expect(handle.version).toBe(1);
    await handle.call({ op: "set", key: "k", value: { v: 1 } });
    const got = (await handle.call({ op: "get", key: "k" })) as { value: { v: number } };
    expect(got.value).toEqual({ v: 1 });
    const listed = (await handle.call({ op: "list" })) as { keys: string[] };
    expect(listed.keys).toEqual(["k"]);
    await expect(handle.call({ op: "wat" })).rejects.toMatchObject({ code: "kv-op-unknown" });
  });

  it("describe reports namespace, backend, and effective limits", () => {
    const kv = createMemoryKv({ namespace: "app:d", maxEntries: 5, maxValueBytes: 9, schemaVersion: 3 });
    expect(kv.describe()).toEqual({
      namespace: "app:d",
      backend: "memory",
      maxEntries: 5,
      maxValueBytes: 9,
      schemaVersion: 3,
    });
  });
});
