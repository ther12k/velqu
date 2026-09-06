/**
 * BWASM-C-004 — namespaced IndexedDB KV persistence capability
 * (`runtime:kv` v1).
 *
 * The small mandatory local-persistence primitive for browser
 * deployments — no SQL engine. One async, versioned contract with two
 * interchangeable adapters (memory + namespaced IndexedDB) proven by a
 * single shared suite:
 *
 * - **Namespacing**: every adapter instance is bound to exactly one
 *   namespace (`appId:name`). IndexedDB isolates namespaces as separate
 *   object stores, so one project cannot enumerate or read another
 *   project's keys — not through get/list/clear, and not through the
 *   raw key space.
 * - **Versioned data**: each namespace carries a schema version. An
 *   upgrade without a declared migration fails closed with
 *   `KvMigrationRequired` — opening never silently erases data.
 * - **Quotas + serialization**: bounded entry count / value bytes with
 *   structured `KvQuotaExceeded`; non-cloneable values produce
 *   `KvSerializationError`. Blocked / unavailable IndexedDB (private
 *   mode, disabled storage) produces `KvUnavailable` with an explicit,
 *   documented policy: `"error"` (fail closed, default) or `"memory"`
 *   (opt-in EPHEMERAL fallback — flagged non-durable).
 * - **Explicit controls**: `exportAll()`, `reset()`, `gc()`. No
 *   implicit garbage collection; no cross-device sync; nothing here is
 *   production-durable or multi-user (browser-local preview data).
 *
 * Purity: browser-only APIs behind injectable facades — no node:* /
 * Bun:* in the module (the R-001 purity scan and C-001 precedent).
 */

import type { CapabilityHandle } from "./capability-treaty";

export const KV_CAPABILITY_ID = "runtime:kv";
export const KV_CAPABILITY_VERSION = 1;

/** Native-parity-ish bounds (tuned for preview-scale persistence). */
export const MAX_KV_KEY_BYTES = 512;
export const MAX_KV_VALUE_BYTES = 1024 * 1024; // 1 MiB per value
export const MAX_KV_ENTRIES = 10_000;
/** Reserved meta key: namespace schema version + gc watermark. */
export const KV_META_KEY = "\u0000velqu:meta";

/** Structured-clone-safe stored values (IndexedDB parity). */
export type KvValue = string | number | boolean | null | KvValue[] | { [key: string]: KvValue };

export interface KvEntry {
  readonly key: string;
  readonly value: KvValue;
}

/** Typed errors (structured, machine-readable). */
export class KvError extends Error {
  readonly code: string;
  constructor(code: string, message: string) {
    super(`[@velqu/browser-runtime:kv:${code}] ${message}`);
    this.name = "KvError";
    this.code = code;
  }
}
export class KvKeyInvalid extends KvError {
  constructor(detail: string) {
    super("kv-key-invalid", detail);
    this.name = "KvKeyInvalid";
  }
}
export class KvQuotaExceeded extends KvError {
  constructor(detail: string) {
    super("kv-quota-exceeded", detail);
    this.name = "KvQuotaExceeded";
  }
}
export class KvSerializationError extends KvError {
  constructor(detail: string) {
    super("kv-serialization", detail);
    this.name = "KvSerializationError";
  }
}
export class KvMigrationRequired extends KvError {
  readonly fromVersion: number;
  readonly toVersion: number;
  constructor(fromVersion: number, toVersion: number) {
    super(
      "kv-migration-required",
      `namespace schema v${fromVersion} cannot open as v${toVersion}: declare a migration or match the existing version (data is preserved, never silently erased)`,
    );
    this.name = "KvMigrationRequired";
    this.fromVersion = fromVersion;
    this.toVersion = toVersion;
  }
}
export class KvUnavailable extends KvError {
  constructor(detail: string) {
    super("kv-unavailable", detail);
    this.name = "KvUnavailable";
  }
}

/** The versioned async KV contract — BOTH adapters implement exactly this. */
export interface KvCapability {
  get(key: string): Promise<KvValue | null>;
  set(key: string, value: KvValue): Promise<void>;
  delete(key: string): Promise<void>;
  /** Keys under `prefix` (all keys when omitted), sorted. */
  list(prefix?: string): Promise<string[]>;
  /** Atomic batch set (adapter-atomic; no intermediate states visible). */
  setMany(entries: ReadonlyArray<KvEntry>): Promise<void>;
  clear(): Promise<void>;
  /** Explicit full export (namespace-scoped). */
  exportAll(): Promise<Record<string, KvValue>>;
  /** Explicit reset: clears data AND resets namespace metadata. */
  reset(): Promise<void>;
  /**
   * Explicit garbage collection: enforces maxEntries by oldest-written
   * order and drops tombstoned records. Returns removed key count.
   */
  gc(): Promise<number>;
  /** Introspection: the bound namespace + effective limits. */
  describe(): { namespace: string; backend: "memory" | "indexeddb"; maxEntries: number; maxValueBytes: number; schemaVersion: number };
}

export interface KvOptions {
  /** Namespace: `${appId}:${name}` — isolation boundary (required). */
  readonly namespace: string;
  /** Namespace schema version (default 1). */
  readonly schemaVersion?: number;
  /** Declared migration from older versions (default: fail closed). */
  readonly migrations?: Readonly<Record<number, (store: KvMigrationStore) => Promise<void>>>;
  readonly maxEntries?: number;
  readonly maxValueBytes?: number;
}

/** Minimal store surface a migration hook may use (its own namespace only). */
export interface KvMigrationStore {
  get(key: string): Promise<KvValue | null>;
  set(key: string, value: KvValue): Promise<void>;
  delete(key: string): Promise<void>;
  list(): Promise<string[]>;
}

function validateKey(key: string): void {
  if (typeof key !== "string" || key.length === 0) {
    throw new KvKeyInvalid("key must be a non-empty string");
  }
  if (key.startsWith("\u0000")) {
    throw new KvKeyInvalid("keys must not start with the reserved meta prefix");
  }
  if (new TextEncoder().encode(key).byteLength > MAX_KV_KEY_BYTES) {
    throw new KvKeyInvalid(`key exceeds ${MAX_KV_KEY_BYTES} bytes`);
  }
}

function checkCloneSafe(value: KvValue): void {
  try {
    structuredClone(value);
  } catch {
    throw new KvSerializationError("value is not structured-clone-safe");
  }
}

const valueBytes = (value: KvValue): number => new TextEncoder().encode(JSON.stringify(value)).byteLength;

// ---------------------------------------------------------------------------
// Memory adapter
// ---------------------------------------------------------------------------

export interface MemoryKv extends KvCapability {
  readonly backend: "memory";
}

/**
 * Volatile adapter — same contract, deep-cloned values (IDB parity).
 * Memory never persists, so the schema-version gate cannot fire: every
 * instance starts empty at its declared version (migration semantics
 * are exercised on the IndexedDB adapter, the durable one).
 */
export function createMemoryKv(options: KvOptions): MemoryKv {
  const maxEntries = options.maxEntries ?? MAX_KV_ENTRIES;
  const maxValueBytes = options.maxValueBytes ?? MAX_KV_VALUE_BYTES;
  const schemaVersion = options.schemaVersion ?? 1;
  const store = new Map<string, { value: KvValue; writtenAt: number }>();

  const get = async (key: string): Promise<KvValue | null> => {
    validateKey(key);
    const hit = store.get(key);
    return hit ? (structuredClone(hit.value) as KvValue) : null;
  };
  const setRaw = async (key: string, value: KvValue, writtenAt = Date.now()): Promise<void> => {
    validateKey(key);
    checkCloneSafe(value);
    const bytes = valueBytes(value);
    if (bytes > maxValueBytes) {
      throw new KvQuotaExceeded(`value ${bytes}B exceeds ${maxValueBytes}B`);
    }
    if (!store.has(key) && store.size >= maxEntries) {
      throw new KvQuotaExceeded(`entry budget exhausted (${maxEntries} entries)`);
    }
    store.set(key, { value: structuredClone(value) as KvValue, writtenAt });
  };
  const del = async (key: string): Promise<void> => {
    validateKey(key);
    store.delete(key);
  };

  return {
    backend: "memory",
    get,
    set: (key, value) => setRaw(key, value),
    delete: del,
    list: async (prefix?: string) =>
      [...store.keys()].filter((k) => k !== KV_META_KEY && (prefix === undefined || k.startsWith(prefix))).sort(),
    setMany: async (entries) => {
      // quota-check everything first (atomic: no partial application)
      for (const e of entries) {
        validateKey(e.key);
        checkCloneSafe(e.value);
        if (valueBytes(e.value) > maxValueBytes) {
          throw new KvQuotaExceeded(`value for "${e.key}" exceeds ${maxValueBytes}B`);
        }
      }
      const projected = new Set([...store.keys(), ...entries.map((e) => e.key)]).size;
      if (projected > maxEntries) {
        throw new KvQuotaExceeded(`batch would exceed ${maxEntries} entries`);
      }
      const now = Date.now();
      for (const e of entries) store.set(e.key, { value: structuredClone(e.value) as KvValue, writtenAt: now });
    },
    clear: async () => {
      for (const k of [...store.keys()]) if (k !== KV_META_KEY) store.delete(k);
    },
    exportAll: async () => {
      const out: Record<string, KvValue> = {};
      for (const [k, hit] of store) if (k !== KV_META_KEY) out[k] = structuredClone(hit.value) as KvValue;
      return out;
    },
    reset: async () => {
      store.clear();
    },
    gc: async () => {
      // oldest-written eviction down to maxEntries; meta key never touched
      const entries = [...store.entries()]
        .filter(([k]) => k !== KV_META_KEY)
        .sort((a, b) => a[1].writtenAt - b[1].writtenAt);
      let removed = 0;
      for (const [k] of entries) {
        if (store.size - removed <= maxEntries) break;
        store.delete(k);
        removed++;
      }
      return removed;
    },
    describe: () => ({
      namespace: options.namespace,
      backend: "memory",
      maxEntries,
      maxValueBytes,
      schemaVersion,
    }),
  };
}

// ---------------------------------------------------------------------------
// IndexedDB facade (injectable; real globalThis.indexedDB in browsers)
// ---------------------------------------------------------------------------

/** Minimal IDBDatabase surface the adapter needs. */
export interface KvIdbDatabase {
  createObjectStore(name: string, options?: { keyPath?: string }): unknown;
  transaction(storeName: string, mode: "readonly" | "readwrite"): {
    objectStore(name: string): {
      get(key: string): unknown;
      put(value: KvValue, key: string): unknown;
      delete(key: string): unknown;
      getAllKeys(): unknown;
      clear(): unknown;
    };
  };
  close(): void;
  onversionchange?: ((event: unknown) => void) | null;
  version: number;
  objectStoreNames: { contains(name: string): boolean };
}

export interface KvIdbFactory {
  open(name: string, version?: number, upgrade?: (db: KvIdbDatabase, tx: unknown, oldVersion: number) => void): {
    addEventListener(type: "success" | "error" | "blocked" | "upgradeneeded", listener: (event: unknown) => void): void;
    result?: KvIdbDatabase;
    error?: unknown;
  };
}

function awaitRequest<T>(requestLike: unknown): Promise<T> {
  return new Promise((resolve, reject) => {
    const req = requestLike as { addEventListener?: (t: string, l: (e: unknown) => void) => void; result: T; error?: unknown };
    if (typeof req.addEventListener === "function") {
      req.addEventListener("success", () => resolve((req as { result: T }).result));
      req.addEventListener("error", () => reject(new KvSerializationError(String((req as { error?: { message?: string } }).error?.message ?? "indexeddb request failed"))));
    } else {
      resolve((req as { result: T }).result);
    }
  });
}

export interface IndexedDbKv extends KvCapability {
  readonly backend: "indexeddb";
}

/**
 * Namespaced IndexedDB adapter: one object store per namespace
 * (`kv:${namespace}`) inside the single `velqu:kv` database — store
 * isolation is the project-isolation boundary.
 */
export function createIndexedDbKv(options: KvOptions & {
  /** Injectable factory (tests); defaults to globalThis.indexedDB. */
  readonly idbFactory?: KvIdbFactory;
  /** Unavailable-IDB policy: "error" (fail closed, default) or opt-in ephemeral memory. */
  readonly onUnavailable?: "error" | "memory";
}): IndexedDbKv | MemoryKv {
  const maxEntries = options.maxEntries ?? MAX_KV_ENTRIES;
  const maxValueBytes = options.maxValueBytes ?? MAX_KV_VALUE_BYTES;
  const schemaVersion = options.schemaVersion ?? 1;
  const storeName = `kv:${options.namespace}`;
  const factory = options.idbFactory ?? (globalThis as { indexedDB?: KvIdbFactory }).indexedDB;

  if (!factory || typeof factory.open !== "function") {
    const detail = "indexedDB is not available (private mode, disabled storage, or non-browser runtime)";
    if (options.onUnavailable === "memory") {
      const memory = createMemoryKv(options);
      return memory; // EPHEMERAL opt-in fallback — never durable
    }
    throw new KvUnavailable(detail);
  }

  // The async open path mirrors the memory adapter's synchronous API;
  // construction succeeds and the FIRST operation performs the open +
  // version gate (fail closed before data access).
  let dbPromise: Promise<KvIdbDatabase> | null = null;
  let metaVersion: number | null = null;

  /**
   * Open the shared `velqu:kv` database ensuring THIS namespace's store
   * exists. Two-phase (real-IDB requirement): a versionless open first
   * (creating the db + store when fresh), then — only when the db
   * exists WITHOUT our store — a version-bumping reopen whose upgrade
   * creates it. Another tab holding the old version surfaces as
   * `blocked` → typed KvUnavailable.
   */
  const openDb = (): Promise<KvIdbDatabase> => {
    if (dbPromise) return dbPromise;
    dbPromise = (async () => {
      const first = await new Promise<KvIdbDatabase>((resolve, reject) => {
        // versionless open: no version argument (never triggers a
        // version bump; upgrades only when the db itself is fresh)
        const request = factory.open("velqu:kv");
        wire(request, resolve, reject, (db) => {
          if (!db.objectStoreNames.contains(storeName)) {
            db.createObjectStore(storeName);
          }
        });
      }).catch((cause) => {
        dbPromise = null;
        throw cause;
      });
      if (first.objectStoreNames.contains(storeName)) return first;
      // the db exists without our namespace store: bump the version
      first.close();
      const upgraded = await new Promise<KvIdbDatabase>((resolve, reject) => {
        const request = factory.open("velqu:kv", first.version + 1);
        wire(request, resolve, reject, (db) => {
          if (!db.objectStoreNames.contains(storeName)) {
            db.createObjectStore(storeName);
          }
        });
      }).catch((cause) => {
        dbPromise = null;
        throw cause;
      });
      return upgraded;
    })();
    return dbPromise;
  };

  /** Wire the standard success/error/blocked/upgradeneeded listeners. */
  function wire(
    request: ReturnType<KvIdbFactory["open"]>,
    resolve: (db: KvIdbDatabase) => void,
    reject: (e: unknown) => void,
    upgrade: (db: KvIdbDatabase) => void,
  ): void {
    request.addEventListener("upgradeneeded", (event: unknown) => {
      upgrade((event as { target: { result: KvIdbDatabase } }).target.result);
    });
    request.addEventListener("blocked", () => {
      reject(new KvUnavailable("indexedDB open blocked by another version-holding tab (close other tabs and retry)"));
    });
    request.addEventListener("error", () => {
      reject(new KvUnavailable(`indexedDB open failed: ${String((request as { error?: { message?: string } }).error?.message ?? "unknown")}`));
    });
    request.addEventListener("success", () => {
      const db = request.result!;
      db.onversionchange = () => db.close();
      resolve(db);
    });
  }

  const withStore = async <T>(mode: "readonly" | "readwrite", fn: (store: ReturnType<NonNullable<ReturnType<KvIdbDatabase["transaction"]>>["objectStore"]>) => Promise<T>): Promise<T> => {
    try {
      return await run(mode, fn);
    } catch (cause) {
      // A version bump from ANOTHER namespace's first open fires
      // versionchange here and the connection self-closes; reopen once
      // transparently so existing namespaces keep working (BWASM-C-004,
      // found in the real-browser rehearsal).
      const message = cause instanceof Error ? cause.message : String(cause);
      if (/closing|InvalidStateError/i.test(message)) {
        dbPromise = null;
        return await run(mode, fn);
      }
      throw cause;
    }
  };

  const run = async <T>(mode: "readonly" | "readwrite", fn: (store: ReturnType<NonNullable<ReturnType<KvIdbDatabase["transaction"]>>["objectStore"]>) => Promise<T>): Promise<T> => {
    const db = await openDb();
    const tx = db.transaction(storeName, mode);
    const store = tx.objectStore(storeName);
    return fn(store);
  };

  const readMeta = async (store: ReturnType<NonNullable<ReturnType<KvIdbDatabase["transaction"]>>["objectStore"]>): Promise<number> => {
    if (metaVersion !== null) return metaVersion;
    const raw = await awaitRequest<{ v?: number } | undefined>(store.get(KV_META_KEY));
    metaVersion = raw?.v ?? 0;
    return metaVersion;
  };

  const versionGate = async (store: ReturnType<NonNullable<ReturnType<KvIdbDatabase["transaction"]>>["objectStore"]>): Promise<void> => {
    const existing = await readMeta(store);
    if (existing === 0 || existing === schemaVersion) {
      if (existing === 0) {
        await awaitRequest(store.put({ v: schemaVersion }, KV_META_KEY));
        metaVersion = schemaVersion;
      }
      return;
    }
    const migration = options.migrations?.[existing];
    if (!migration) {
      throw new KvMigrationRequired(existing, schemaVersion);
    }
    await migration(wrapStore(store));
    await awaitRequest(store.put({ v: schemaVersion }, KV_META_KEY));
    metaVersion = schemaVersion;
  };

  const wrapStore = (store: ReturnType<NonNullable<ReturnType<KvIdbDatabase["transaction"]>>["objectStore"]>): KvMigrationStore => ({
    get: async (key) => {
      const raw = await awaitRequest<{ v: KvValue } | undefined>(store.get(key));
      return raw ? raw.v : null;
    },
    set: async (key, value) => {
      await awaitRequest(store.put({ v: value, writtenAt: Date.now() }, key));
    },
    delete: async (key) => {
      await awaitRequest(store.delete(key));
    },
    list: async () => {
      const keys = await awaitRequest<string[]>(store.getAllKeys());
      return keys.filter((k) => k !== KV_META_KEY).sort();
    },
  });

  const getValue = async (key: string): Promise<KvValue | null> => {
    validateKey(key);
    return withStore("readwrite", async (store) => {
      await versionGate(store);
      const raw = await awaitRequest<{ v: KvValue } | undefined>(store.get(key));
      return raw ? raw.v : null;
    });
  };

  const setValue = async (key: string, value: KvValue, writtenAt?: number): Promise<void> => {
    validateKey(key);
    checkCloneSafe(value);
    const bytes = valueBytes(value);
    if (bytes > maxValueBytes) throw new KvQuotaExceeded(`value ${bytes}B exceeds ${maxValueBytes}B`);
    return withStore("readwrite", async (store) => {
      await versionGate(store);
      const keys = await awaitRequest<string[]>(store.getAllKeys());
      if (!keys.includes(key) && keys.filter((k) => k !== KV_META_KEY).length >= maxEntries) {
        throw new KvQuotaExceeded(`entry budget exhausted (${maxEntries} entries)`);
      }
      await awaitRequest(store.put({ v: value, writtenAt: writtenAt ?? Date.now() }, key));
    });
  };

  return {
    backend: "indexeddb",
    get: getValue,
    set: (key, value) => setValue(key, value),
    delete: async (key) => {
      validateKey(key);
      return withStore("readwrite", async (store) => {
        await versionGate(store);
        await awaitRequest(store.delete(key));
      });
    },
    list: async (prefix?: string) =>
      withStore("readwrite", async (store) => {
        await versionGate(store);
        const keys = await awaitRequest<string[]>(store.getAllKeys());
        return keys.filter((k) => k !== KV_META_KEY && (prefix === undefined || k.startsWith(prefix))).sort();
      }),
    setMany: async (entries) => {
      for (const e of entries) {
        validateKey(e.key);
        checkCloneSafe(e.value);
        if (valueBytes(e.value) > maxValueBytes) {
          throw new KvQuotaExceeded(`value for "${e.key}" exceeds ${maxValueBytes}B`);
        }
      }
      return withStore("readwrite", async (store) => {
        await versionGate(store);
        const keys = await awaitRequest<string[]>(store.getAllKeys());
        const projected = new Set([...keys.filter((k) => k !== KV_META_KEY), ...entries.map((e) => e.key)]).size;
        if (projected > maxEntries) throw new KvQuotaExceeded(`batch would exceed ${maxEntries} entries`);
        const now = Date.now();
        for (const e of entries) await awaitRequest(store.put({ v: e.value, writtenAt: now }, e.key));
      });
    },
    clear: async () =>
      withStore("readwrite", async (store) => {
        await versionGate(store);
        const keys = await awaitRequest<string[]>(store.getAllKeys());
        for (const k of keys) if (k !== KV_META_KEY) await awaitRequest(store.delete(k));
      }),
    exportAll: async () =>
      withStore("readwrite", async (store) => {
        await versionGate(store);
        const keys = await awaitRequest<string[]>(store.getAllKeys());
        const out: Record<string, KvValue> = {};
        for (const k of keys.filter((k) => k !== KV_META_KEY)) {
          const raw = await awaitRequest<{ v: KvValue } | undefined>(store.get(k));
          if (raw) out[k] = raw.v;
        }
        return out;
      }),
    reset: async () =>
      withStore("readwrite", async (store) => {
        await awaitRequest(store.clear());
        metaVersion = null;
      }),
    gc: async () =>
      withStore("readwrite", async (store) => {
        await versionGate(store);
        const keys = await awaitRequest<string[]>(store.getAllKeys());
        const live = keys.filter((k) => k !== KV_META_KEY);
        if (live.length <= maxEntries) return 0;
        const withTime: Array<{ k: string; writtenAt: number }> = [];
        for (const k of live) {
          const raw = await awaitRequest<{ writtenAt?: number } | undefined>(store.get(k));
          withTime.push({ k, writtenAt: raw?.writtenAt ?? 0 });
        }
        withTime.sort((a, b) => a.writtenAt - b.writtenAt);
        const excess = live.length - maxEntries;
        for (let i = 0; i < excess; i++) await awaitRequest(store.delete(withTime[i]!.k));
        return excess;
      }),
    describe: () => ({
      namespace: options.namespace,
      backend: "indexeddb",
      maxEntries,
      maxValueBytes,
      schemaVersion,
    }),
  };
}

/** Registry-installable handle for the KV capability. */
export function kvHandle(kv: KvCapability): CapabilityHandle {
  return {
    id: KV_CAPABILITY_ID,
    version: KV_CAPABILITY_VERSION,
    call: async (input: unknown) => {
      const op = input as {
        op: "get" | "set" | "delete" | "list" | "setMany" | "clear" | "exportAll" | "gc";
        key?: string;
        prefix?: string;
        value?: KvValue;
        entries?: ReadonlyArray<KvEntry>;
      };
      switch (op.op) {
        case "get":
          return { value: await kv.get(op.key!) };
        case "set":
          await kv.set(op.key!, op.value!);
          return { ok: true };
        case "delete":
          await kv.delete(op.key!);
          return { ok: true };
        case "list":
          return { keys: await kv.list(op.prefix) };
        case "setMany":
          await kv.setMany(op.entries ?? []);
          return { ok: true };
        case "clear":
          await kv.clear();
          return { ok: true };
        case "exportAll":
          return { data: await kv.exportAll() };
        case "gc":
          return { removed: await kv.gc() };
        default:
          throw new KvError("kv-op-unknown", `unknown kv op "${String((op as { op?: string }).op)}"`);
      }
    },
  };
}
