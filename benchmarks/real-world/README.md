# Real-World Benchmark Harness

Deterministic infrastructure for the real-world backend benchmark (BETA-001).
Workloads and scope are defined in [`SPEC.md`](SPEC.md) and
[`workloads.json`](workloads.json); Postgres DDL lives in
[`postgres/schema.sql`](postgres/schema.sql).

## One command

```bash
./run.sh
```

Runs four phases:

1. **prepare** — starts the pinned Postgres (`compose.yaml`,
   `postgres:17.5-alpine3.22`, tmpfs data) and applies the deterministic
   dataset reset (`postgres/reset.sql` + `schema.sql` + `postgres/seed.sql`
   via `reset.sh`).
2. **smoke** — starts the controlled upstream (`upstream.ts`), runs a 2-second
   load-generation smoke against it (`load.ts`), validates the result schema,
   and generates the report (`report.ts`).
3. **contracts** — boots every candidate server and drives the full fixture
   matrix from `contract-fixtures.ts` against it (`verify-contract.ts`): each
   candidate must return the expected status and an exactly equal JSON body
   for every fixture (success and 401/404/400/409/504/502 paths). Any mismatch
   is written to `contract-verification.md` and fails the phase — candidates
   that are not semantically equivalent may not be timed together. Requires
   candidate deps (`cd candidates && bun install --frozen-lockfile`) and
   `node` on PATH for the Fastify candidate.
4. **report** — folded into smoke; `benchmarks/raw/real-world/smoke/` retains
   `summary.json` and `report.md` as evidence (the bulky per-request
   `raw.jsonl` and upstream log stay local; BETA-001-D owns the full
   raw-sample retention policy).

Phases can be run individually: `./run.sh prepare`, `./run.sh smoke`,
`./run.sh contracts`.

## Raw-sample retention

Every run keeps its complete per-request rows: `retain.ts` writes a
deterministic `raw.jsonl.gz` (gzip mtime pinned to 0 — byte-reproducible)
beside `summary.json`, plus a `RETENTION.md` manifest with row counts and
sha256 hashes for both forms. Uncompressed rows and logs stay local-only;
archives are the retained evidence.

## Fairness audit

`fairness.ts` compares two or more candidate summaries and fails loudly on any
contract drift (workloads, dataset, version pins), protocol drift (duration,
concurrency), host-class mismatch, cell-grid mismatch, or retained failures
(errors/status mismatches — SPEC requires 0%):

```bash
RUNS=../raw/real-world/velqu,../raw/real-world/elysia ./run.sh audit
```

Writes `fairness.md` with the verdict. This is the audit BETA-001 requires
before any cross-candidate comparison may be claimed.

## Determinism

- The dataset is a pure function of `schema.sql` + `seed.sql` (generate_series
  with modular arithmetic, no random functions); `reset.sh` reproduces
  identical contents on every invocation, and `docker compose down` discards
  all state (tmpfs).
- The controlled upstream (`upstream.ts`) delays by an explicit `ms` query
  parameter (0–1000, validated) — no jitter, no randomness.
- Every summary records the environment (Bun/OS/arch/commit) and sha256 hashes
  of `SPEC.md`, `workloads.json`, `schema.sql`, and `seed.sql`
  (`result-schema.ts` enforces their presence).

## Candidate failure is retained

`load.ts` writes one raw JSONL row per request; transport errors and status
mismatches carry an `error`/status field and are counted per cell — they never
abort the run and always surface in the report's retained-failures section.

## Driving a real candidate

```bash
bun load.ts --base-url http://127.0.0.1:3000 --out-dir ../raw/real-world/run1
bun report.ts --summary ../raw/real-world/run1/summary.json
```

`--duration` and `--concurrency` (comma list) override `workloads.json`.

## CI scope

CI and `bun test` cover the deterministic pieces only (result schema, workload
config, report rendering). The docker/Postgres phases are operator-run on a
quiet host, like the rest of the benchmark suite.

## Out of scope here

Candidate version pinning, fairness checks, and raw-sample retention policy
are BETA-001-B/C/D.
