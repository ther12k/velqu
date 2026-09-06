# Browser Import Policy (BWASM-B-003)

Status: **Frozen policy v1** (`IMPORT_POLICY_VERSION = 1`,
`packages/compiler/src/import-policy.ts`; enforced by the
`browser-wasm` compiler target before any artifact is emitted).

## Purpose

Browser handlers are trusted code, but the runtime and the user's
browser bundle must not carry server-only or unverifiable machinery
(ADR-0038 §5: the compiler import policy is the trusted-handler
enforcement mechanism; the Q-phase bundle post-scan re-checks the
compiled form).

## Classification

| Class | Specifiers | Consequence |
|---|---|---|
| **forbidden** | `node:*`, `bun:*`, bare Node builtins (`fs`, `path`, `crypto`, `http`, `net`, `child_process`, `worker_threads`, …), native addons (`*.node`), `eval(...)`, `new Function(...)`, opaque dynamic `import()` (non-static specifier — cannot be verified ⇒ fail closed) | build blocked, source-located diagnostic |
| **deployment-required** | known server-only drivers: `pg`, `postgres`, `mysql`, `mysql2`, `mongodb`, `redis`, `ioredis`, `better-sqlite3`, `sqlite3`, `knex`, `typeorm`, `sequelize`, `prisma`, `@prisma/client`, `grpc`, `@grpc/grpc-js`, `kafkajs`, `amqplib` | build blocked with capability remediation (e.g. `runtime:postgres` is deployment-required; ADR-0037 §5) |
| **browser-safe** | `@velqu/*` (paths-resolved), relative imports (walked transitively), bare packages matching no deny pattern (recorded as the explicit review surface) | allowed |

## Graph coverage

- Walk starts at the app entry and follows **relative imports
  transitively** (`.ts`/`index.ts`/`.js`→`.ts` resolution).
- **Re-exports** (`export … from`) and `require()` interop are followed
  identically — forbidden imports cannot hide behind a local wrapper.
- **Dynamic `import()`** with a static specifier is classified like a
  static import; an **opaque** specifier is itself a violation
  (`BWASM-POLICY-DYNAMIC-IMPORT-OPAQUE` — fail closed, not skipped).
- `eval` and `new Function` are forbidden dynamic code loading.

## Diagnostics

Each violation carries: code (`BWASM-POLICY-NODE-BUILTIN`,
`-NATIVE-ADDON`, `-DYNAMIC-CODE`, `-DYNAMIC-IMPORT-OPAQUE`,
`-DEPLOYMENT-REQUIRED`), classification, file + source range,
specifier, the full import chain (entry → … → violating file),
remediation text (including the capability alternative for
deployment-required), and the policy doc link.

## Escape hatch

**None.** Default is fail closed; adding an auditable escape hatch
requires an owner decision and a policy-version bump.

## Policy version

`IMPORT_POLICY_VERSION` is recorded in every emitted
`browser-manifest.json` (`importPolicyVersion`), so an artifact's
reviewed-under-policy identity travels with it.
