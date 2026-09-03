# BETA-004-A — Capability ABI Costs (Postgres capability)

Status: **MEASURED** (zero-cost posture) — this packet wires the
`runtime:postgres` capability into the ABI; it deliberately delivers **no
database functionality yet** (pool = BETA-004-B, parameterized wire
behavior = C, deadlines = D, pool limits = E, W1/W2/W3 = parent exit).

## What was built

- `crates/q-capabilities/src/postgres.rs` — the `runtime:postgres` v1
  ABI model: identity (`POSTGRES_CAPABILITY_ID`/`_VERSION`), lazy
  lifecycle (Declared → Installed → Ready → Draining → Quiesced, terminal
  Failed), and the bounded query-op surface (owner-tagged, cancellable,
  deadline ceiling 120s — stricter than the ABI-wide 300s). 7 Rust tests.
- `packages/capability-postgres/` — the `@velqu/capability-postgres` SDK:
  frozen identity constants mirroring the Rust model, a parameterized-only
  `sql()` surface, and a typed `PostgresCapabilityUnavailable` fail-closed
  error when the host has not linked the capability. Importing constructs
  nothing. 5 tests + 2 end-to-end pack-wiring tests.
- Compiler: `postgres` added to `KNOWN_GRANTS` / `GRANT_MODULES` — a
  handler touching `native.postgres` puts an exact `runtime:postgres` v1
  requirement into the pack; unknown grants still fail the build.
- `packages/core`: type-only `PostgresCapability` member on the handler
  `native` context (erased at emit — zero pack bytes).

## Cold / RSS cost report (zero-cost guardrail)

Measured on this worktree (release build, remapped; i5-13420H, linux):

- **Pack-level**: the proof app grants no postgres — its capability
  manifest declares only `timer`, and `resolveLinkedModules` yields zero
  `runtime:postgres` requirement entries for it (unit-tested). The
  inventory of an unrelated app is byte-identical to before this packet.
- **Cold start** (fresh process → first valid response, `cold-start.ts
  --samples=10 --only=velqu`, C0 health.live): p50 **11.697ms** total
  (11.338ms to ready + 0.358ms ready→first), 0 failures.
- **RSS after ready**: p50 **9,676 kB** (~9.4 MB) — the same single-worker
  QuickJS runtime shape as before this packet; no pool, no sockets, no
  Postgres client memory exists because nothing Postgres is linked.
- A Postgres-free app therefore pays **zero dependency, init, or RSS
  cost** — the capability never leaves `Declared`, and nothing in the new
  code paths runs at build, pack-load, or serve time.

## Real-world results (scope note)

W1/W2/W3 real-world database results are the parent's exit criteria and
land with BETA-004-B..E once the pool and wire protocol exist. This packet
adds no benchmark claims and changes no numbers.

## Fail-closed behavior (tested)

- Runtime side (ABI): a pack requiring `runtime:postgres` against a linked
  set without it is `ResolveError::Missing` — fails before `Ready`
  (`validate_compatibility_per_worker`, existing ABI test covers the exact
  `runtime:postgres` requirement vector).
- SDK side: every SDK call without the host binding throws the typed
  `PostgresCapabilityUnavailable` (never a silent fallback, never a JS
  reimplementation) — tested, including the non-function-binding case.
