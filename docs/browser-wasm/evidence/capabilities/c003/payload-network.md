# BWASM-C-003 — payload / network measurements

Measured at the C-003 packet commit (see git log); sizes are exact bytes
unless stated.

## Adapter payload (what a project adds by importing the package)

| Artifact | Bytes |
| --- | --- |
| `@velqu/browser-pglite` dist bundle (`index.js`, engine external) | 11,806 |
| same, gzipped | 3,370 |

## Engine payload (loaded ONLY on first `open()` — dynamic import)

| Artifact | Bytes |
| --- | --- |
| `pglite.wasm` (PostgreSQL engine, wasm32) | 10,088,161 |
| `initdb.wasm` (database initializer) | 395,242 |
| `@electric-sql/pglite` npm package, unpacked | 25,437,263 (all files incl. docs) |
| engine dependency count | 1 direct (`@electric-sql/pglite@0.5.8` exact) |

## No-download-when-unused (acceptance criterion 5)

Two structural layers, both verified:

1. **Package-level (strongest):** the adapter is a separate optional
   package. A project that never imports `@velqu/browser-pglite` never
   has it on its module graph — nothing to download, ever. Build wiring
   keeps `@electric-sql/pglite` external
   (`scripts/build-packages.ts`), so even adapter adopters get the
   engine as a real dependency reference, never an inlined blob.
2. **Import-level:** the engine is behind a dynamic `import()` executed
   only inside `open()`. Verified by `lazy.test.ts`:
   - counting loader: `createLocalSql` + `describe()` + a `NotOpen`
     query rejection load ZERO engine modules; `open()` loads exactly
     once and is reused thereafter;
   - source-graph assertion: no static
     `from "@electric-sql/pglite"` anywhere in `src/`.

## Network-trace caveat (disclosed)

A real-browser network trace (deploy a project WITH the adapter, assert
zero pglite-origin requests before open) was not part of this packet's
local lanes; the CI chromium lane does not exercise this package. The
two structural layers above are the committed proof; a browser-lane
extension that exercises `runtime:local-sql` end-to-end (including
IndexedDB persistence durability) is the natural follow-up evidence and
is listed as a known limitation in the task record.
