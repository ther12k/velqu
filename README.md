# Velqu (VelquJS)

Velqu is a Rust HTTP runtime that embeds quickjs-ng (0.15.1, via
rquickjs 0.12.2) to execute application handlers authored in
TypeScript. Routes are declared with a single schema contract that
drives types, runtime validation, Treaty clients, OpenAPI, and the
contract lock. The Rust host routes by method/path and enforces
bounded queues, bodies, heap, stack, and deadlines before any
JavaScript runs.

## Status

Pre-beta development toward `0.1.0-beta.1` (ADR-0020, `docs/beta/`).
Milestone/task state: `docs/codex-spark-beta/STATUS.md`. Material
decisions live in `docs/okf/decisions/`; open owner decisions in
`docs/open-decisions.md`.

## Development

Bun is dev/package/test tooling only — production execution is the
Rust binary (`target/release/velqu-runtime`) loading a compiled QPack.

```bash
bun run verify        # tests + typecheck + OKF validation for the authorized scope
cargo build --release -p velqu-runtime
```

## Evidence

Performance claims are evidence-bound: matched candidates, retained raw
samples, p50/p95/p99 (`benchmarks/raw/`, index in
`benchmarks/manifest.json`). Recent reports:

- Cold-start delta at 25/1,000 routes:
  `docs/reports/m25-010-c-cold-start-delta.md`
- Baseline cold-start methodology: `docs/reports/cold-start-report.md`

Same-process QuickJS executes trusted application code only; it is not
a hostile-code sandbox.
