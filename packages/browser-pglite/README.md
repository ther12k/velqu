# @velqu/browser-pglite

Optional browser-local SQL for Velqu browser deployments (BWASM-C-003):
the frozen async Postgres operation subset (C-002's contract) mapped onto
[PGlite](https://pglite.dev) — a real PostgreSQL build compiled to WASM —
running entirely inside the browser.

**What this is:** an opt-in adapter for prototype applications that want
local SQL (`runtime:local-sql` v1) with explicit persistence, project
isolation, bounded operations, and typed fail-closed errors.

**What this is not:** the native `postgres` capability. That grant stays
`deployment-required` (C-005) and is never satisfied here. This adapter
is single-connection, browser-local prototype data — not multi-user, not
production-durable, not native PostgreSQL performance, and never claimed
as any of those.

## Install

```bash
bun add @velqu/browser-pglite
```

## Use

```ts
import { createLocalSql } from "@velqu/browser-pglite";

const db = createLocalSql({
  namespace: "my-app:main",       // one database per origin+namespace
  persistence: "memory",          // or "indexeddb" (explicit persistence)
});

await db.open();                  // first open lazily loads the engine
await db.query("CREATE TABLE items (id serial PRIMARY KEY, name text)");
await db.query("INSERT INTO items (name) VALUES ($1)", ["alpha"]);

const { rows } = await db.query<{ id: number; name: string }>(
  "SELECT id, name FROM items ORDER BY id",
);

await db.transaction(async (tx) => {
  await tx.query("UPDATE items SET name = $1 WHERE id = $2", ["beta", 1]);
});

const dump = await db.exportAll(); // data-dir bytes
await db.reset();                  // drop + recreate (empty)
await db.importAll(dump);          // restore from a dump

await db.close();
```

## Contract

- **Lazy engine**: the PGlite WASM engine loads via dynamic `import()` on
  first `open()`. A project that never imports this package never bundles
  it; one that imports it but never opens a database never instantiates
  the engine.
- **Isolation**: storage name is `velqu-local-sql:<origin>:<namespace>`;
  projects cannot enumerate or read each other's databases.
- **Persistence**: `"memory"` (default, handle-lifetime) or `"indexeddb"`
  (explicit opt-in). Blocked storage fails closed with
  `PersistenceUnavailable` — no silent memory fallback.
- **Bounds**: every operation carries a fail-closed deadline (default
  30 s, ceiling 120 s). Deadlines reject the caller; they do not preempt
  in-engine work (the engine shares the caller's thread — documented,
  never claimed as cancellation).
- **Subset**: statements outside the documented SQL subset (LISTEN/NOTIFY,
  COPY FROM STDIN, CREATE EXTENSION, ALTER SYSTEM, CREATE/DROP DATABASE,
  cursors, advisory locks, VACUUM/REINDEX) reject with
  `UnsupportedStatement` + remediation BEFORE execution.

## Error codes

`NamespaceRequired`, `InvalidNamespace`, `PersistenceUnavailable`,
`UnsupportedStatement`, `DeadlineOutOfRange`, `DeadlineExceeded`,
`NotOpen`, `AlreadyOpen`, `Closed`, `TransactionAborted`, `QueryFailed`.

## Browser-only

No `node:*`/`Bun.*` in the package sources (purity-audited like the core
browser runtime). Engine: `@electric-sql/pglite` 0.5.8, an exact pin.

## Deployer note: cross-origin isolation

PGlite is a pthreads WebAssembly build: a page that opens the database
must be **cross-origin isolated** (serve `Cross-Origin-Opener-Policy:
same-origin` + `Cross-Origin-Embedder-Policy: require-corp` for the
document) or engine initialization wedges. The storage name is
sanitized (`velqu-local-sql-<origin>_<namespace>` with
`[^A-Za-z0-9._-]` → `_`) so origins with `://` are safe for the
IndexedDB path. Verified end-to-end in the chromium E7 rehearsal lane
(`docs/browser-wasm/evidence/browser-lanes/chromium-e7.json`).
