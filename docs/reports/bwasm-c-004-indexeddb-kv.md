# BWASM-C-004 — Namespaced IndexedDB KV persistence capability

## Overview

Adds the mandatory browser local-persistence primitive as `runtime:kv` v1
(`packages/browser-runtime/src/kv.ts`): one versioned async KV contract
with two interchangeable adapters — volatile memory and namespaced
IndexedDB — proven by a single shared contract suite plus a real-Chromium
rehearsal over the genuine IndexedDB path. Opt-in per deployment via
`velqu build --target browser-wasm --kv` (declared `runtime:kv@1` in the
artifact manifest; the generated page installs the adapter with namespace
`<appId>:kv`).

## Contract

`get / set / delete / list(prefix) / setMany / clear / exportAll / reset /
gc / describe` — all Promise-based (C-002 precedent). Values are
structured-clone-safe, deep-isolated on every write and read. Typed,
structured failures: `KvKeyInvalid`, `KvQuotaExceeded` (per-value 1 MiB,
entry budget 10k, both tunable), `KvSerializationError`,
`KvMigrationRequired` (with `fromVersion`/`toVersion`), `KvUnavailable`.

## Isolation

One IndexedDB object store per namespace (`kv:<appId>:<name>`) inside the
single `velqu:kv` database: **store isolation is the project boundary** —
one project cannot enumerate, read, or clear another project's keys
(pinned in unit tests and proven in Chromium). Key collision between
similar namespaces (`app:users` vs `app:users:admin`) is covered.

## Versioning and migration

Each namespace stores its schema version. Opening at a different version
without a declared migration fails closed (`KvMigrationRequired`) and
preserves data; a declared `migrations: { [from]: async (store) => … }`
hook runs and advances the version. Upgrades never silently erase.

## Availability policy (documented, not hidden)

- IndexedDB unavailable (private mode, disabled storage, non-browser):
  default **fail closed** with `KvUnavailable`; opt-in
  `onUnavailable: "memory"` yields an EPHEMERAL fallback flagged
  `backend: "memory"` — never represented as durable.
- Open blocked by another version-holding tab: typed `KvUnavailable`.
- **Version-bump coexistence** (found in the real-browser rehearsal and
  fixed): a new namespace's first open bumps the db version; existing
  connections self-close on `versionchange` and the adapter reopens once
  transparently, so previously-open namespaces keep working.

## Real-browser rehearsal (Chromium 151.0.7922.34, genuine IndexedDB)

`scripts/browser-kv-rehearsal.py` (committed, rerunnable; Playwright is
evidence tooling only) — all seven checks pass
(`evidence/capabilities/c004/kv-browser-rehearsal.json`):

| scenario | result |
|---|---|
| k1 set/get/list round trip (real IDB) | PASS |
| k2 persistence across page reload | PASS |
| k3 namespace isolation (no leakage; version-bump coexistence) | PASS |
| k4 quota → structured `kv-quota-exceeded` | PASS |
| k5 v1→v2 migration with declared hook | PASS |
| k5b version mismatch fails closed, data preserved | PASS |
| k6 export / reset controls | PASS |

## Acceptance criteria

- ✅ Memory and IndexedDB adapters pass ONE shared contract suite (both
  backends run every block, including atomic setMany quota behavior).
- ✅ One project cannot enumerate or read another project's keys
  (store-per-namespace; unit + real-browser evidence).
- ✅ Quota, serialization, migration, blocked-database failures are
  structured (typed codes; blocked → `KvUnavailable` with guidance).
- ✅ Upgrading never silently erases: mismatch fails closed; migrations
  are explicit and tested.
- ✅ Private/unavailable conditions: documented fail-closed default plus
  an explicit, flagged ephemeral fallback.
- ✅ Nothing here is production-durable or multi-user: browser-local
  preview data; export/reset/gc are explicit controls.

## Gates

Browser-runtime + CLI suites: 187/187 (30 new KV tests + 1 new compose
test). `tsc -b` clean. Purity: browser-only APIs behind injectable
facades (no node:*/Bun:* in the module). Canonical verification run in
this packet's handoff (see PR body for the exact-commit gate summary).

## Honest notes

- Bun has no `indexedDB` — unit tests use a spec-shaped fake (async
  success events, structured-clone-on-put, upgradeneeded on version
  bumps); the genuine IndexedDB path is covered by the committed
  real-browser rehearsal (both run in CI's bun lane + manual rehearsal,
  respectively). Real-browser CI lanes remain Q-002.
- `setMany` is adapter-atomic (all-or-nothing quota/validation), but
  IndexedDB multi-store transactions are NOT exposed — there is no
  cross-namespace or SQL-style transaction API (out of scope; C-003 is
  the SQL lane and stays owner-gated).
- Nothing here is PostgreSQL parity, production-durable, or
  multi-user — it is a browser-local preview primitive.

Standing CI disclosure applies (zero-step verify workflows since ~#714);
local gates are the acceptance basis.
