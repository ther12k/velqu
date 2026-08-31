# CLI Reference — `velqu` Command Surface (M4A-002)

The unified `velqu` command line interface provides developer tooling,
compilation, inspection, contract difference auditing, testing, and pack
migration for Velqu applications.

Production execution uses the native `velqu-runtime` binary with pinned
QuickJS (`quickjs-ng 0.15.1`). The `velqu` CLI is developer and build tooling
running on Bun.

## Command Overview

| Command | Purpose |
|---|---|
| `velqu dev` | Starts live dev server with safe worker reload loop |
| `velqu build` | Compiles application into immutable verified QPack and manifests |
| `velqu inspect` | Inspects routes, capabilities, fallback reasons, or static diagnostics |
| `velqu contract diff` | Compares compiled route contracts against `contract.lock.json` |
| `velqu test` | Runs test runner with optional filter |
| `velqu check` | Fast static validation of routes and toolchain |
| `velqu pack inspect` | Inspects a compiled QPack artifact headers, format, and manifests |
| `velqu pack migrate` | Provides format migration guidance for legacy packs |

---

## 1. `velqu dev`

Starts the local development proxy gateway and spawns the real QuickJS
runtime. Automatically watches source files, rebuilds incremental temporary
QPacks on change, and verifies candidate workers before switching traffic.

```bash
velqu dev [--project <dir|entry>] [--port 3000] [--debounce-ms 50] [--profile serverless]
```

- `--project`: Project directory or entry path (default: `examples/proof`).
- `--port`: Public HTTP gateway port (default: `3000`).
- `--debounce-ms`: File modification debounce window in ms (default: `50`).
- `--profile`: Initial runtime service profile (default: `serverless`).

### Safety & Parity
- **Safe reload swap**: candidate worker must verify readiness before traffic is switched.
- **Fail-safe retention**: syntax or compile errors preserve the prior healthy worker.
- **Graceful drain**: old workers receive `SIGTERM` and drain within a 5 s budget.

---

## 2. `velqu build`

Compiles application declarations into a verified QPack (`app.qpack`),
OpenAPI 3.1 definitions (`openapi.json`), Treaty types (`contract.d.ts`),
compact contract metadata (`contract.meta.json`), route manifests, and
source maps (`app.qpack.sources.json`).

```bash
velqu build [--project <dir|entry>] [--out <dir>] [--update-lock] [--profile serverless]
```

- `--project`: Entry file (e.g. `src/app.ts`) or project directory.
- `--out`: Destination artifact directory (default: `<project>/dist`).
- `--update-lock`: Overwrite `contract.lock.json` if one already exists.

---

## 3. `velqu inspect`

Inspects compiled manifests or static project diagnostics without executing application handlers.

```bash
velqu inspect routes [--dist <dir>]
velqu inspect route <id> [--dist <dir>]
velqu inspect capabilities [--dist <dir>]
velqu inspect fallbacks [--dist <dir>]
velqu inspect diagnostics [--project <dir|entry>]
```

- `routes`: Table of all routes, validation/response strategies, codecs, and capabilities.
- `route <id>`: JSON inspection of a specific route declaration.
- `capabilities`: Linked capability inventory and reduction impact report.
- `fallbacks`: Active strategy fallbacks, reasons, and estimated overheads.
- `diagnostics`: Static AST diagnostics, module counts, and strategy overview.

---

## 4. `velqu contract diff`

Compares compiled route contracts in `<dist>` against a committed `contract.lock.json` base.

```bash
velqu contract diff [--dist <dir>] [--against <path>]
```

- Exit code `0`: No changes or non-breaking additions.
- Exit code `2`: Breaking contract changes detected (deleted routes, changed schemas, removed responses).

---

## 5. `velqu pack`

Inspects or assesses migration paths for `.qpack` artifacts.

```bash
velqu pack inspect <app.qpack>
velqu pack migrate <app.qpack>
```

- `inspect`: Displays formatVersion, runtime engine tuple, route counts, schemas, and bundle SHA-256.
- `migrate`: Checks formatVersion and outputs migration instructions for legacy versions.

---

## 6. `velqu test` & `velqu check`

```bash
velqu test [filter]
velqu check [--project <dir|entry>]
```

- `test`: Runs project test suites via Bun test runner.
- `check`: Fast static contract extraction and toolchain verification.

---

## Standard Exit Codes

- `0`: Success / clean execution.
- `1`: User error, compilation error, or missing file.
- `2`: Breaking contract difference or invalid configuration.
