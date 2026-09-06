# BWASM-B-003 — Browser Import Policy with Source-Located Diagnostics

## Result

**PASS** — the `browser-wasm` target enforces a frozen import policy
(policy v1) across the application module graph before any artifact is
emitted: direct, transitive, aliased, re-exported, and dynamic
forbidden imports are all caught with source-located, actionable
diagnostics. Fail closed; **no escape hatch** (none owner-approved).

## Deliverables

- `packages/compiler/src/import-policy.ts` — graph-walking scanner +
  classification + diagnostics; `IMPORT_POLICY_VERSION = 1` recorded in
  every emitted `browser-manifest.json` (`importPolicyVersion`).
- `packages/compiler/src/browser.ts` — the browser target runs
  `assertImportPolicyOk(scanImportPolicy(entry))` BEFORE emission
  (liveness diagnostic class unchanged).
- `docs/specs/browser-import-policy.md` — the policy document
  (classification table, graph coverage, diagnostic shape, escape-hatch
  posture).
- Fixtures: `packages/compiler/src/import-policy.test.ts` (9 tests).

## Classification summary

| Class | Specifiers | Consequence |
|---|---|---|
| forbidden | `node:*`/`bun:*`, bare Node builtins, `*.node` addons, `eval`/`new Function`, opaque dynamic `import()` | build blocked (`BWASM-POLICY-NODE-BUILTIN`/`-NATIVE-ADDON`/`-DYNAMIC-CODE`/`-DYNAMIC-IMPORT-OPAQUE`) |
| deployment-required | server-only drivers (`pg`, `postgres`, `mysql*`, `mongodb`, `redis*`, `sqlite*`, `knex`, `typeorm`, `sequelize`, `prisma`, `grpc`, `kafkajs`, `amqplib`) | build blocked (`BWASM-POLICY-DEPLOYMENT-REQUIRED`) with capability remediation (ADR-0037 §5) |
| browser-safe | `@velqu/*`, walked relative imports, bare packages without deny-pattern match (recorded review surface) | allowed |

## Fixture results (`evidence/import-policy/`)

- **Negative fixture log** (`negative-fixture-log.txt`, exit 0): direct
  `node:fs`; transitive via relative helper chain (chain captured:
  entry → helper → fs-wrapper); aliased re-export (`writeFileSync as
  wfs`); bare `path`; opaque dynamic `import(name)`; `eval` +
  `new Function`; `pg` → deployment-required with `runtime:postgres`
  remediation.
- **Positive fixtures** (`positive-fixtures.txt`):
  `examples/browser-demo` violations=0; `examples/proof`
  violations=0 — approved browser-safe packages (`@velqu/*`, relative)
  produce no false positives.
- Golden diagnostic snapshot: the B-001 browser.test.ts diagnostics
  case continues to pass (native-liveness source-located error,
  unchanged).

## Acceptance disposition

- ✅ Direct, transitive, aliased, re-exported, and dynamic forbidden
  imports caught (fixture matrix).
- ✅ Browser-safe positives fixture-tested (both sample apps clean).
- ✅ Diagnostics identify the import chain and suggest the capability
  alternative (deployment-required remediation names
  `runtime:postgres`).
- ✅ No silent externalization: opaque dynamic imports and bare
  packages are handled explicitly (opaque = forbidden; the compiled
  bundle post-scan in Q-phase is the binding second audit — documented
  as the known boundary here).
- ✅ Policy version recorded in the build manifest.

## Notes

- Two scanner defects found and fixed during fixture development
  (opaque-import diagnostic code never emitted; `new Function` is a
  NewExpression, not a CallExpression; a policy fixture that never
  imported its helper) — all caught by the fixture matrix before merge.
- The policy walker uses compiler-side node APIs by design (build-time
  tooling); emitted artifacts stay browser-pure (R-001 scan covers the
  runtime package).

Standing CI disclosure applies (zero-step verify workflows since
~#714); local gates are the acceptance basis.
