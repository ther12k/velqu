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

## Real-browser network trace (RESOLVED — was a disclosed caveat)

The rehearsal now carries an E7 lane (`chromium-e7.json`): the bundled
adapter imports the engine only at first `open()`, and the network
trace confirms ZERO pglite-origin requests before that point (11 engine
asset fetches after; the deployment's own `kernel.wasm` is filtered out
of the assertion). Cross-origin isolation status is asserted alongside
(`crossOriginIsolated: true` — the engine is a pthreads WASM build).
