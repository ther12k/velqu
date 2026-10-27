# Startup codec-table compilation cost — honest reconciliation of the zero-compilation invariant (#1428)

- Date: 2026-10-27 (capture host, release profile)
- Issue: #1428 — production startup compiles schema decoder/encoder programs,
  contrary to AGENTS.md constraint 5 ("production startup performs zero
  route/schema/OpenAPI/plugin compilation")
- Raw evidence: `benchmarks/raw/codec-table-startup/` (40 samples per size,
  JSONL rows retained; `summary.json` with n/mean/p50/p95/p99)
- Instrument: `crates/q-runtime/examples/codec_table_cost.rs` — measures
  exactly the construction the runtime performs at startup
  (`crates/q-runtime/src/lib.rs`: `DecoderTable::from_schemas` +
  `EncoderTable::from_schemas` over the dense schema vector built from the
  verified pack manifest), across the route-count ladder packs on the
  **default legacy-v1 JSON load path** (the only pack mode the runtime
  loads today, ADR-0024).

## What actually happens at startup

On the default path, every startup:

1. loads and verifies the v1 JSON pack (recorded as the `pack.load` stage —
   1216.71ms p50 at 10k routes; this dominates startup);
2. prints the ready line (`startupMs` covers the recorded stages only);
3. **then** compiles the decoder and encoder programs from the pack's
   `SchemaIr` (previously commented as "precompiled programs" — that comment
   was wrong and is corrected by this packet). This work happens **after the
   ready line**: it sits between `startupMs` and the first accepted request
   and is invisible in the recorded startup stages;
4. derives per-route context plans (ADR-0045) — a separate startup-time
   derivation, out of scope here but in the same post-ready-line window.

So constraint 5's zero-schema-compilation clause is **not met** by the
shipped default load path. The programs are compiled at startup and then
cached for the request path (constraint 6's resolve-once discipline is met);
what is violated is the *zero compilation at startup* clause itself.

## Measured cost (default legacy-v1 path, release, 40 samples per size)

| routes | schemas | warm first build | total p50 | total p95 | total p99 |
|---:|---:|---:|---:|---:|---:|
| 25 | 25 | 13.2µs | 8.5µs | 11.2µs | 12.5µs |
| 100 | 100 | 24.3µs | 21.9µs | 23.9µs | 29.0µs |
| 1,000 | 1,000 | 172.9µs | 139.6µs | 156.9µs | 177.3µs |
| 5,000 | 5,000 | 889.7µs | 753.4µs | 807.8µs | 821.2µs |
| 10,000 | 10,000 | 1806.0µs | 1765.4µs | 1862.0µs | 1892.7µs |

Scaling is linear in schema count (~0.18µs per schema for the pair).
For context, the recorded route-count stages at 10k routes on the same
packs: `pack.load` 1216.71ms p50, `bundle.load` 68.85ms p50 — the codec
compilation is ~0.15% of the dominant JSON parse cost on the default path
today. The magnitude is modest; the violation is structural, not currently
a performance problem.

## Route-count scaling evidence and the default path

The existing route-count scaling evidence (`benchmarks/raw/route-count/`)
already measures the **shipped default load path** — the ladder packs are
legacy-v1 JSON (`formatVersion: 1`; the runtime's only loadable mode), so
its stage timings reflect what production startup does. What was missing
from that evidence is precisely the stage measured here: the codec-table
compilation was neither recorded nor measurable before. Acceptance
criterion 2 of #1428 is satisfied by this completion: the default-path
scaling picture now includes the previously invisible stage.

## Forward options (not decided here)

Carrying precompiled codec tables **in the pack** — loaded, not compiled, at
startup — restores the invariant. Two routes exist:

1. **M26-003 (QPack v2 binary encoder)** carries the compiled tables as a
   dense section; loading them is a bounded, zero-compile read. This is the
   natural home: v2 also removes the dominant 1.2s JSON parse, after which
   the ~1.8ms compile would become proportionally significant (~2.5% of
   pack load at 10k routes).
2. **v1 carries the tables** — possible (the programs are
   deterministic from the SchemaIr), but adds a second producer concern to
   a frozen format adapter kept for compatibility (ADR-0024).

Either is a material format decision requiring an ADR and owner sign-off;
this packet records the deviation honestly with measured cost and leaves
the format decision open. When a precompiled-table path lands, the startup
window should also be made honest: move the table load (and the ADR-0045
context-plan derivation) **before** the ready line or record them as
stages, so `startupMs` covers all startup work.

## Artifacts in this packet

- `crates/q-runtime/examples/codec_table_cost.rs` — the measurement
  instrument (compiled by every `cargo test`/clippy `--all-targets` run).
- `benchmarks/raw/codec-table-startup/codec-table.jsonl` — 200 raw rows.
- `benchmarks/raw/codec-table-startup/summary.json` — percentiles per size.
- `crates/q-runtime/src/lib.rs` — the false "zero compilation / precompiled"
  comment replaced with the honest description and a pointer to this report.
