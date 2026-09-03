# BETA-004-F — No ORM (runtime:postgres posture)

Status: **ENFORCED** (surface-freeze test) + documented.

## Guarantee

The Postgres capability's entire application-facing API is one
parameterized method: `native.postgres.sql(text, params, deadlineMs)`.
There is no query builder, no model/entity mapping, no repository or
migration DSL, no relation graph — and no path for one to appear
silently:

- `packages/capability-postgres/src/index.test.ts` gains a **surface
  freeze test**: the exported function surface must be exactly
  `["sql"]`; a list of banned builder/model names (`select`, `where`,
  `table`, `model`, `repository`, `migrate`, `createQueryBuilder`, ...)
  must remain absent; `sql` stays positional-parameters-only. Adding a
  builder method later fails the suite.
- Parameters are scalars only (`null | boolean | number | string`);
  nested object/array params are rejected typed (BETA-004-C wire
  behavior) — model-shaped values have no carrier.
- SQL is always visible in application code; the executed statement is
  what the developer wrote.

## Documentation

- `packages/capability-postgres/README.md` — the frozen surface,
  identity/linking, limits/observability, and the no-ORM statement.
- `docs/beta/POSTGRES-CAPABILITY.md` (new, indexed) — the normative
  capability guide: identity, fail-closed linking, lifecycle/safety,
  no-ORM posture.

## Evidence

- Surface-freeze + parameterized-only tests: 9/9 pass
  (`bun test packages/capability-postgres`).
- Full gates: `bun test` 384 pass / 0 fail; typecheck clean; fmt/clippy
  clean; `./scripts/verify` ALL PASS.
- Cold/RSS and real-world posture unchanged from BETA-004-A..E (this
  packet adds no runtime code — only tests and documentation).
