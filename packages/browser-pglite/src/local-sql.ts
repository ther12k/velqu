/**
 * BWASM-C-003 — optional PGlite-backed local SQL adapter
 * (`runtime:local-sql` v1, library surface).
 *
 * Maps the frozen async Postgres operation subset (BWASM-C-002's
 * contract: async ops with an owner, a bounded fail-closed deadline,
 * typed errors, lazy lifecycle) onto PGlite — a real PostgreSQL build
 * compiled to WASM — running entirely inside the browser.
 *
 * Design lines (mirrors the C-004 KV capability):
 *
 * - **Opt-in package, lazy engine.** The PGlite engine is loaded by a
 *   dynamic `import()` on FIRST OPEN, never statically. A project that
 *   never imports `@velqu/browser-pglite` never bundles it; a project
 *   that imports it but never opens a database never instantiates the
 *   engine. (Deployment artifacts stay content-addressed by the B-002
 *   manifest, which covers the database bytes whenever they ship.)
 * - **One namespace = one database.** `namespace` is required and is
 *   folded with the origin into the storage name
 *   (`velqu-local-sql:<origin>:<namespace>`), so projects cannot
 *   enumerate or read each other's databases — not by name and not
 *   through the raw IndexedDB key space.
 * - **Persistence is explicit.** `"memory"` (default) lives exactly as
 *   long as the handle. `"indexeddb"` persists via PGlite's IndexedDB
 *   filesystem and fails closed (`PersistenceUnavailable`) when storage
 *   is blocked — there is no silent memory fallback, matching the KV
 *   capability's default-deny posture.
 * - **Bounded.** Every operation carries a fail-closed deadline
 *   (default 30 s, ceiling 120 s — the native postgres ceiling). The
 *   deadline rejects the CALL; it cannot interrupt in-engine work (the
 *   WASM engine shares the caller's thread) — cancellation class is
 *   best-effort, documented, never claimed as preemption.
 * - **Honest claims.** Single connection, browser-local prototype
 *   data. Not multi-user, not production-durable, not native
 *   PostgreSQL performance — and the native `postgres` grant remains
 *   `deployment-required` (C-005): this adapter never satisfies it.
 *
 * Purity: browser-only APIs behind injectable facades — no node:* or
 * Bun.* in this module (C-001/R-001 purity precedent).
 */

import { classifyStatement } from "./subset";

export const LOCAL_SQL_CAPABILITY_ID = "runtime:local-sql";
export const LOCAL_SQL_CAPABILITY_VERSION = 1;

/** Fail-closed default/ceiling for one operation (native parity). */
export const DEFAULT_OP_DEADLINE_MS = 30_000;
export const MAX_OP_DEADLINE_MS = 120_000;

export type LocalSqlPersistence = "memory" | "indexeddb";

/** Typed, machine-readable errors (closed set). */
export type LocalSqlErrorCode =
  | "NamespaceRequired"
  | "InvalidNamespace"
  | "PersistenceUnavailable"
  | "UnsupportedStatement"
  | "DeadlineOutOfRange"
  | "DeadlineExceeded"
  | "NotOpen"
  | "AlreadyOpen"
  | "Closed"
  | "TransactionAborted"
  | "QueryFailed";

export class LocalSqlError extends Error {
  readonly code: LocalSqlErrorCode;
  readonly remediation?: string;
  constructor(code: LocalSqlErrorCode, message: string, remediation?: string) {
    super(`[@velqu/browser-pglite:${code}] ${message}`);
    this.name = "LocalSqlError";
    this.code = code;
    this.remediation = remediation;
  }
}

/** Structural engine types (the real import stays behind the loader). */
export interface PgliteResults<T> {
  readonly rows: readonly T[];
  readonly affectedRows?: number;
  readonly command?: string;
  readonly rowCount?: number;
}
export interface PgliteTx {
  query<T>(sql: string, params?: unknown[]): Promise<PgliteResults<T>>;
  rollback(): Promise<void>;
}
export interface PgliteInstance {
  query<T>(sql: string, params?: unknown[]): Promise<PgliteResults<T>>;
  exec(sql: string): Promise<unknown>;
  transaction<T>(cb: (tx: PgliteTx) => Promise<T>): Promise<T>;
  dumpDataDir(): Promise<Uint8Array | Blob>;
  close(): Promise<void>;
}
export interface PgliteStatic {
  new (dataDir?: string, options?: { readonly loadDataDir?: Blob }): PgliteInstance;
  /** Async factory; awaits engine init (required for loadDataDir restores). */
  create?(dataDir?: string, options?: { readonly loadDataDir?: Blob }): Promise<PgliteInstance>;
}
export interface PgliteModule {
  readonly PGlite: PgliteStatic;
}

/** Injectable engine loader — the ONLY seam where the engine loads. */
export type PgliteLoader = () => Promise<PgliteModule>;

const defaultLoader: PgliteLoader = () => import("@electric-sql/pglite");

export interface CreateLocalSqlOptions {
  /** `appId:name` — one database per namespace, isolated by project and origin. */
  readonly namespace: string;
  /** Origin folded into the storage name. Defaults to `globalThis.location.origin`. */
  readonly origin?: string;
  /** `"memory"` (default, handle-lifetime) or `"indexeddb"` (explicit persistence). */
  readonly persistence?: LocalSqlPersistence;
  /** Fail-closed per-operation deadline. Default 30 s, ceiling 120 s. */
  readonly deadlineMs?: number;
  /** Test seam: replace the dynamic engine import. */
  readonly loader?: PgliteLoader;
  /** Test seam: storage-name derivation (isolation assertions). */
  readonly storageNameFor?: (origin: string, namespace: string) => string;
}

export interface QueryOptions {
  /** Correlation owner for the bounded op (native ABI parity). */
  readonly owner?: string;
  /** Per-call deadline override (same fail-closed ceiling). */
  readonly deadlineMs?: number;
}

export interface LocalSqlRow {
  readonly [column: string]: unknown;
}

export interface QueryResult<T extends LocalSqlRow = LocalSqlRow> {
  readonly rows: readonly T[];
  readonly rowCount: number;
  readonly command?: string;
}

export interface LocalSqlDescribe {
  readonly capability: typeof LOCAL_SQL_CAPABILITY_ID;
  readonly version: typeof LOCAL_SQL_CAPABILITY_VERSION;
  readonly namespace: string;
  readonly origin: string;
  readonly persistence: LocalSqlPersistence;
  /** `SELECT version()` from the opened engine (undefined before open). */
  readonly engine?: string;
  /** Distinct per origin+namespace — the isolation proof surface. */
  readonly storageName: string;
}

export interface LocalSql {
  /** Open: lazily load + instantiate the engine and initialize the database. */
  open(): Promise<void>;
  /** One classified, bounded statement. */
  query<T extends LocalSqlRow = LocalSqlRow>(
    sql: string,
    params?: readonly unknown[],
    opts?: QueryOptions,
  ): Promise<QueryResult<T>>;
  /** Scoped unit on the single connection; rollback on any throw. */
  transaction<T>(fn: (tx: LocalSql) => Promise<T>, opts?: QueryOptions): Promise<T>;
  /** Introspect BEFORE handler execution (C-001 availability precedent). */
  describe(): LocalSqlDescribe;
  /** Serialize the database (data-dir dump) for tests/preview UX. */
  exportAll(): Promise<Uint8Array>;
  /** Replace the database contents from a previous exportAll(). */
  importAll(bytes: Uint8Array): Promise<void>;
  /** Drop and recreate the database (explicit destroy; replaces DROP DATABASE). */
  reset(): Promise<void>;
  /** Close the single connection. Terminal for this handle. */
  close(): Promise<void>;
}

function defaultOrigin(): string {
  const loc = (globalThis as { location?: { origin?: string } }).location;
  return loc?.origin ?? "no-origin";
}

export function storageName(origin: string, namespace: string): string {
  return `velqu-local-sql:${origin}:${namespace}`;
}

function validateNamespace(namespace: string): void {
  if (!namespace || !namespace.trim()) {
    throw new LocalSqlError(
      "NamespaceRequired",
      "a namespace (appId:name) is required — one database per namespace",
    );
  }
  if (!/^[a-zA-Z0-9][a-zA-Z0-9._:-]{0,127}$/.test(namespace)) {
    throw new LocalSqlError(
      "InvalidNamespace",
      `namespace ${JSON.stringify(namespace)} must match /^[a-zA-Z0-9][a-zA-Z0-9._:-]{0,127}$/`,
    );
  }
}

function checkDeadline(ms: number): void {
  if (!Number.isFinite(ms) || ms <= 0 || ms > MAX_OP_DEADLINE_MS) {
    throw new LocalSqlError(
      "DeadlineOutOfRange",
      `deadline must be in (0, ${MAX_OP_DEADLINE_MS}] (fail-closed ceiling, native parity), got ${ms}`,
    );
  }
}

async function blobBytes(dump: Uint8Array | Blob): Promise<Uint8Array> {
  if (dump instanceof Uint8Array) return dump;
  return new Uint8Array(await dump.arrayBuffer());
}

/**
 * Create the adapter. Nothing loads, instantiates, or stores until
 * {@link LocalSql.open | open()} is called.
 */
export function createLocalSql(options: CreateLocalSqlOptions): LocalSql {
  validateNamespace(options.namespace);
  const namespace = options.namespace;
  const origin = options.origin ?? defaultOrigin();
  const persistence: LocalSqlPersistence = options.persistence ?? "memory";
  const baseDeadline = options.deadlineMs ?? DEFAULT_OP_DEADLINE_MS;
  checkDeadline(baseDeadline);
  const loader = options.loader ?? defaultLoader;
  const nameFor = options.storageNameFor ?? storageName;
  const storage = nameFor(origin, namespace);
  const dataDir = persistence === "indexeddb" ? `idb://${storage}` : undefined;

  let instance: PgliteInstance | null = null;
  let closed = false;
  let engineVersion: string | undefined;

  function requireOpen(): PgliteInstance {
    if (closed) {
      throw new LocalSqlError("Closed", "handle is closed; create a new LocalSql for another session");
    }
    if (!instance) {
      throw new LocalSqlError("NotOpen", "call open() first (lazy lifecycle, native parity)");
    }
    return instance;
  }

  function deadlineOf(opts?: QueryOptions): number {
    const ms = opts?.deadlineMs ?? baseDeadline;
    checkDeadline(ms);
    return ms;
  }

  function withDeadline<T>(p: Promise<T>, ms: number): Promise<T> {
    return new Promise<T>((resolve, reject) => {
      const timer = setTimeout(() => {
        // Rejection is fail-closed for the caller; the engine shares this
        // thread, so in-engine work is never preempted (documented).
        reject(new LocalSqlError("DeadlineExceeded", `operation exceeded ${ms}ms (fail-closed deadline)`));
      }, ms);
      p.then(
        (v) => { clearTimeout(timer); resolve(v); },
        (e) => { clearTimeout(timer); reject(e); },
      );
    });
  }

  function assertSubset(sql: string): void {
    const verdict = classifyStatement(sql);
    if (!verdict.supported) {
      throw new LocalSqlError(
        "UnsupportedStatement",
        `${verdict.reason}: ${sql.trim().slice(0, 80)}`,
        verdict.remediation,
      );
    }
  }

  async function runQuery<T extends LocalSqlRow>(
    exec: (sql: string, params?: unknown[]) => Promise<PgliteResults<T>>,
    sql: string,
    params: readonly unknown[] | undefined,
    opts: QueryOptions | undefined,
  ): Promise<QueryResult<T>> {
    assertSubset(sql);
    const ms = deadlineOf(opts);
    try {
      const r = await withDeadline(exec(sql, params === undefined ? undefined : [...params]), ms);
      return {
        rows: r.rows ?? [],
        rowCount: r.rowCount ?? r.affectedRows ?? r.rows?.length ?? 0,
        command: r.command,
      };
    } catch (e) {
      if (e instanceof LocalSqlError) throw e;
      throw new LocalSqlError(
        "QueryFailed",
        `engine rejected the statement: ${String((e as Error)?.message ?? e)}`,
      );
    }
  }

  async function instantiate(loadDataDir?: Blob): Promise<PgliteInstance> {
    const mod = await loader();
    const options = loadDataDir ? { loadDataDir } : undefined;
    try {
      // create() awaits engine init — required for loadDataDir restores;
      // the bare constructor resolves before the data dir is loaded.
      if (typeof mod.PGlite.create === "function") {
        return await mod.PGlite.create(dataDir, options);
      }
      return new mod.PGlite(dataDir, options);
    } catch (cause) {
      if (persistence === "indexeddb") {
        throw new LocalSqlError(
          "PersistenceUnavailable",
          `IndexedDB persistence is unavailable for namespace ${JSON.stringify(namespace)}: ${String(cause)} — storage blocked or disabled; there is no silent memory fallback`,
        );
      }
      throw cause;
    }
  }

  const handle: LocalSql = {
    async open(): Promise<void> {
      if (closed) throw new LocalSqlError("Closed", "handle is closed; create a new LocalSql for another session");
      if (instance) throw new LocalSqlError("AlreadyOpen", "database is already open");
      instance = await instantiate();
      // Cache engine identity once (one bounded local roundtrip at init).
      try {
        const v = await instance.query<{ version: string }>("SELECT version() as version");
        engineVersion = v.rows[0]?.version;
      } catch {
        engineVersion = undefined;
      }
    },

    async query<T extends LocalSqlRow = LocalSqlRow>(
      sql: string,
      params?: readonly unknown[],
      opts?: QueryOptions,
    ): Promise<QueryResult<T>> {
      const db = requireOpen();
      return runQuery<T>((s, p) => db.query<T>(s, p), sql, params, opts);
    },

    async transaction<T>(fn: (tx: LocalSql) => Promise<T>, opts?: QueryOptions): Promise<T> {
      const db = requireOpen();
      deadlineOf(opts); // validate before entering the unit
      try {
        return await withDeadline(
          db.transaction(async (t) => {
            // Runs INSIDE the engine's transaction: every fn() query uses
            // the same tx, so BEGIN/COMMIT/ROLLBACK wrap the whole unit.
            const txHandle: LocalSql = {
              open: async () => {
                throw new LocalSqlError("NotOpen", "open inside a transaction is not part of the contract");
              },
              query: <R extends LocalSqlRow>(sql: string, params?: readonly unknown[], o?: QueryOptions) =>
                runQuery<R>((s, p) => t.query<R>(s, p), sql, params, o),
              transaction: () => {
                throw new LocalSqlError("TransactionAborted", "nested transactions are not part of the contract (single connection)");
              },
              describe: () => handle.describe(),
              exportAll: () => {
                throw new LocalSqlError("TransactionAborted", "exportAll inside a transaction is not part of the contract");
              },
              importAll: () => {
                throw new LocalSqlError("TransactionAborted", "importAll inside a transaction is not part of the contract");
              },
              reset: () => {
                throw new LocalSqlError("TransactionAborted", "reset inside a transaction is not part of the contract");
              },
              close: () => {
                throw new LocalSqlError("TransactionAborted", "close inside a transaction is not part of the contract");
              },
            };
            try {
              return await fn(txHandle);
            } catch (e) {
              await t.rollback().catch(() => undefined);
              throw e;
            }
          }),
          deadlineOf(opts),
        );
      } catch (e) {
        if (e instanceof LocalSqlError) throw e;
        throw new LocalSqlError(
          "TransactionAborted",
          `transaction rolled back: ${String((e as Error)?.message ?? e)}`,
        );
      }
    },

    describe(): LocalSqlDescribe {
      return {
        capability: LOCAL_SQL_CAPABILITY_ID,
        version: LOCAL_SQL_CAPABILITY_VERSION,
        namespace,
        origin,
        persistence,
        engine: engineVersion,
        storageName: storage,
      };
    },

    async exportAll(): Promise<Uint8Array> {
      const db = requireOpen();
      try {
        return await withDeadline(blobBytes(await db.dumpDataDir()), deadlineOf());
      } catch (e) {
        if (e instanceof LocalSqlError) throw e;
        throw new LocalSqlError("QueryFailed", `export failed: ${String((e as Error)?.message ?? e)}`);
      }
    },

    async importAll(bytes: Uint8Array): Promise<void> {
      requireOpen();
      const old = instance;
      instance = null;
      await old?.close();
      // Byte-level restore: the engine initializes from the dump.
      // (Copy into a fresh ArrayBuffer-backed view — Blob parts reject
      // SharedArrayBuffer-typed views under TS 5.9's Uint8Array generics.)
      instance = await instantiate(new Blob([new Uint8Array(bytes)]));
      try {
        const v = await instance.query<{ version: string }>("SELECT version() as version");
        engineVersion = v.rows[0]?.version;
      } catch {
        engineVersion = undefined;
      }
    },

    async reset(): Promise<void> {
      requireOpen();
      const old = instance;
      instance = null;
      await old?.close();
      instance = await instantiate();
      engineVersion = undefined;
      try {
        const v = await instance.query<{ version: string }>("SELECT version() as version");
        engineVersion = v.rows[0]?.version;
      } catch {
        engineVersion = undefined;
      }
    },

    async close(): Promise<void> {
      const old = instance;
      instance = null;
      closed = true;
      if (old) await old.close();
    },
  };

  return handle;
}
