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

## Real-browser note (disclosed)

IndexedDB durability across page reloads in a real browser is an
engine-level property of PGlite's IDB filesystem; this packet proves
the adapter's modes, naming, fail-closed behavior, and data round-trip
locally (Bun + real engine for memory mode; fake loader for IDB-mode
ctor wiring). A chromium-lane E2E (open → write → reload → read under
`"indexeddb"`) is recorded as follow-up evidence in the task record.
