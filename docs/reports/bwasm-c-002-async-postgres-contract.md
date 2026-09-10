# BWASM-C-002 — Make the Postgres capability contract asynchronous before browser freeze

## Overview

Locked the `runtime:postgres` v1 authoring contract as Promise-based
before any browser capability adapter depends on it. The native runtime
ALREADY settled `sql` through its op table (a Promise in practice); the
TypeScript types claimed a synchronous result — the contract change is
truth-telling plus frozen async semantics, mirrored error classes, a
migration codemod, and a beta API snapshot.

## Changes

| surface | change |
|---|---|
| `@velqu/core` (`ctx.native.postgres`) | `sql(...): Promise<PostgresSqlResult>`; exported `PostgresSqlParam/Row/Result` aliases; doc-pinned semantics |
| `@velqu/capability-postgres` (SDK) | `sql(...): Promise<SqlResult>`; new typed rejections `PostgresDeadlineExceeded` (with parsed `deadlineMs`) and `PostgresQueryError` (verbatim `nativeReason`); binding check stays SYNCHRONOUS so an unlinked capability still throws `PostgresCapabilityUnavailable` from the call |
| Native prelude (`q-engine-quickjs`) | `ctx.native.postgres.sql` rejections classify identically (same names, same reason matching) |
| Codemod | `scripts/migrate-postgres-async.mjs` — finds sync-looking sites, rewrites mechanical patterns (`const x = X.sql(…)`, `return X.sql(…).rows`), reports non-async enclosures for manual fixes; schema-versioned JSON report; per-file reverse-offset batching (no lost multi-edit files) |
| Docs | `docs/beta/POSTGRES_ASYNC_MIGRATION.md` (affected-API list, semantics table, codemod guide); `docs/beta/API_SNAPSHOT.md` (beta capability snapshot incl. timer + host graph) |

## Frozen v1 async semantics

Always-Promise return; synchronous argument validation
(TypeError/RangeError); deadline 1..120_000 ms (default 5_000) cancels
the round trip and releases the connection before the rejection is
observed; per-call deadline is the only cancellation surface (no
AbortSignal in v1); single-statement autocommit — NO transaction
pinning; rows over the bounded JSON-compatible value set;
`affectedRows` for DML, 0 for SELECT-shaped results; typed errors:
unavailable (sync) / deadline-exceeded / query-error.

## Acceptance criteria

- ✅ All official examples compile against the new contract — repo
  examples had NO postgres call sites (`grep` verified; the pack-wiring
  suite builds a granting route end-to-end and stays green).
- ✅ Native Postgres behavior and errors remain covered: engine tests
  (`q-engine-quickjs` postgres block: resolve, fail-closed, deadline
  boundary, cleanup-phase refusal) and `q-capability-postgres` suites
  pass unchanged.
- ✅ Generated handler code cannot accidentally serialize an unresolved
  Promise: sync-looking use is a compile-time error (pinned by tsc
  negative fixtures: `Property 'rows' does not exist on type
  'Promise<SqlResult>'`); `velqu check` surfaces it.
- ✅ Migration guidance identifies every affected public API (two
  surfaces — `ctx.native.postgres.sql` and SDK `postgres.sql` — both
  listed in POSTGRES_ASYNC_MIGRATION.md) with a working codemod
  (transcript in evidence).
- ✅ Beta API snapshot records the async contract
  (`docs/beta/API_SNAPSHOT.md`).

## Test evidence

`packages/capability-postgres/src/async-contract.test.ts` (10 new):
Promise surface + call passthrough; sync fail-fast validation; deadline
classification (parsed `deadlineMs`); query-error classification
(verbatim native reason); fail-closed unavailability; codemod
find/rewrite/leave-awaited + exit codes; tsc negative/positive/HandlerCtx
fixtures. Existing suites updated to the async contract: capability-postgres
20/20. Adjacent: browser-runtime capability/treaty, compiler import-policy,
CLI capability-inventory — 40/40. `tsc -b tsconfig.json` clean.
Native: `cargo test -p q-engine-quickjs postgres` (4/4),
`cargo test -p q-capability-postgres` green.

Evidence: `docs/browser-wasm/evidence/capabilities/c002/`
(`01-api-diff.patch`, `02-async-contract-tests.txt`,
`03-native-integration.txt`, `04-codemod-transcript.txt`). Canonical
verification: `./scripts/verify` ALL PASS (M0–M2 + M2.2.1 + M2.3 +
M23R2-GATE-CLOSE verified) in the prescribed netns; benchmark manifest
refreshed against the fmt'd workspace release binary (matched evidence).
Heavy compiler-fixture tests carry explicit timeouts (tsc fixture spawns
are multi-second under load).

## Regression found and fixed during verification (verify-or-fix within this packet)

The first prelude classifier used a regex literal — the FIRST one in the
prelude. The minimal context profile ships NO RegExp intrinsics, and
quickjs compiles regex literals when the enclosing function is created
at prelude eval: every `--context-profile minimal` startup died with
"engine worker died during load" (caught by
`runtime_conformance::full_profile_retained_for_compatibility_testing`,
reproduced and bisected). Fixed by parsing the deadline marker with
`indexOf`, and pinned by a static guard test
(`prelude::bwasm_c002_tests::prelude_contains_no_regex_literals_or_regexp_use`)
that keeps the prelude free of regex/RegExp use at the source. The
prelude's regex-free constraint is a documented invariant from here.

## Honesty notes

- Runtime behavior is unchanged by design: the native bridge already
  returned Promises; only authoring types, error classes, docs, and
  tooling changed. No PostgreSQL-parity claim is made or implied.
- The codemod rewrites mechanical patterns only; non-async enclosing
  functions are REPORTED, not rewritten (marking them async is always
  safe because the compiler awaits handler results, but the choice of
  where to mark belongs to the author).
- The tsc fixtures filter ambient bun-types/@types noise to the
  fixture-file errors (documented in-test).
- Browser adapters (C-001/C-004/C-005) build on this snapshot; the
  optional PGlite capability (C-003) remains owner-gated.

Standing CI disclosure applies (zero-step verify workflows since ~#714);
local gates are the acceptance basis.
