# BWASM-C-003 — persistence / isolation evidence

Proven by `packages/browser-pglite/test/lazy.test.ts` (isolation,
persistence modes, fail-closed storage) and `test/local-sql.test.ts`
(export/import/reset), at the C-003 packet commit.

## Isolation by project AND origin (acceptance criterion 3)

- Storage name = `velqu-local-sql:<origin>:<namespace>` (function
  `storageName`, unit-tested for distinctness per origin and per
  namespace).
- Constructor-argument proof (counting fake loader): `"memory"` mode
  constructs with NO dataDir; `"indexeddb"` mode constructs with
  `idb://velqu-local-sql:<origin>:<namespace>` — one IndexedDB database
  per origin+namespace, so one project can neither address nor
  enumerate another project's database through the storage key space.
- Two namespaces on one origin construct two distinct engine databases
  (asserted on the actual ctor args + `describe().storageName`).
- `namespace` is validated (`/^[a-zA-Z0-9][a-zA-Z0-9._:-]{0,127}$/`)
  and rejects path-escape attempts (`../escape` → `InvalidNamespace`).

## Persistence modes (acceptance criterion 2)

- `"memory"` (default): page/handle lifetime; engine constructor
  receives no dataDir.
- `"indexeddb"` (explicit opt-in): engine persists to the namespaced
  IDB data dir; the mode never activates implicitly.

## Fail-closed storage (no silent fallback)

When the engine constructor throws under `"indexeddb"` (blocked
storage, private mode), `open()` rejects with `PersistenceUnavailable`
and an explicit note that there is no silent memory fallback (KV C-004
default-deny posture). Test: blocked loader → typed rejection.

## Export / import / reset (acceptance criterion 5)

Real-engine round-trip (`local-sql.test.ts` "export → import →
reset"):

1. seeded database → `exportAll()` (gzip tar data-dir, ~4.6 MB for the
   fixture corpus + system catalog);
2. `importAll(dump)` on the same handle: engine re-instantiated via the
   async factory (`PGlite.create` with `loadDataDir`) — user data
   restored (count asserted);
3. `reset()`: database dropped and recreated EMPTY — prior user tables
   are gone (asserted by rejection), new schema is accepted.

Known limits (documented, not hidden): `importAll` is a full replace;
`exportAll`/`importAll` inside a transaction are contract-forbidden
(`TransactionAborted`).

## Real-browser durability (RESOLVED — was a disclosed note)

The rehearsal's E7 lane proves IndexedDB durability in a real chromium:
the adapter (`persistence: "indexeddb"`) writes a row, the page does a
full reload, and the row is read back from the persisted database
(`chromium-e7.json`: `written: 1, readAfterReload: 1`), followed by a
verified `reset()`. Two defects this E2E surfaced and fixed in the
adapter:

1. **Storage-name sanitization**: the storage name embeds the origin,
   and an origin's `http://` wedges PGlite's IDBFS at mount
   (`ErrnoError` — path characters). `storageName()` now sanitizes to
   `[A-Za-z0-9._-]` (`velqu-local-sql-http___host…`), unit-tested.
2. **Cross-origin isolation is a real runtime requirement** of the
   engine (pthreads WASM build; it wedges without SharedArrayBuffer).
   The rehearsal serves the isolation headers for the SQL evidence
   document only, keeping the main shell's deployment contract (and
   the required service-worker lane, which refuses COEP) unchanged.
   This requirement is documented for deployers in the package README.

The engine ships as its own dist layout (import-mapped, always
external): a single-file re-bundle breaks emscripten's pthread worker
in isolated pages — the supported configuration is the engine's own
files, matching the package's lazy-load design.
