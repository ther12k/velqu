# notes-service — external Velqu consumer (#1404)

Notes CRUD over the **Postgres capability**, consuming the published
`@velqu/*@0.1.0-beta.1` packages from npm. This is the second external
consumer (after #1400's relay): it exercises the database capability,
SQL pagination, and — its distinguishing surface — the **generated**
Treaty contract: `src/client.ts` imports `dist/contract.json` +
`dist/contract.d.ts` from `velqu build`, never server source.

`scripts/beta-external/notes-service.sh` drives the full journey,
including an ephemeral `postgres:16-alpine` container (digest recorded
in the transcript) and a live database-loss leg.

## Routes

| Route | What it shows |
|---|---|
| `GET /health/live` | liveness |
| `POST /notes` | write policy over `authorization` (typed 401), validated title/body (422), `INSERT … RETURNING` via `ctx.native.postgres.sql` (201), declared 503 on db failure |
| `GET /notes?page&pageSize` | integer query with defaults (out-of-range → 422), `LIMIT/OFFSET` + `COUNT(*)` pagination |
| `GET /notes/:id` | typed 404 problem for unknown ids |
| `DELETE /notes/:id` | write policy, `affectedRows` → typed 404 / 200 |

The capability grant is the handler's `ctx.native.postgres` reference
itself — compiler-detected, exact `runtime:postgres` v1 requirement in
the pack; a runtime without the capability fails closed before serving.
The schema bootstraps lazily and idempotently through the same `sql()`
op (`CREATE TABLE IF NOT EXISTS`). Every statement is positional-only
parameterized SQL — the capability exposes no string-concatenation API,
and the E2E stores/reads back a title containing SQL metacharacters to
prove it.

The token is a **fixture** (`q-notes-demo-token`), like the proof app's
session fixture. `VELQU_DATABASE_URL` supplies the connection
(environment-only; secrets never enter the pack). Builds are
reproducible only on Bun 1.4.0 / TypeScript 5.9.3 exactly.

## Manual run

```bash
bun install
bun run check && bun run build && bun test   # build before test: the client imports generated artifacts
VELQU_DATABASE_URL=postgres://… VELQU_RUNTIME=/path/to/velqu-runtime bun run dev
```
