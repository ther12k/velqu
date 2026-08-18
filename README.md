# VelquJS

VelquJS, or simply **Velqu**, is a cold-start-first TypeScript framework and
runtime powered by Rust and QuickJS. This repository is the M0–M2
feasibility implementation authorized by the OKF design bundle in
[`docs/okf/`](docs/okf/README.md).

> Naming (owner-decided 2026-08-18, ADR-0016): brand **Velqu**, descriptive
> **VelquJS**, CLI `velqu`, packages `@velqu/*`, runtime binary
> `velqu-runtime`. Remaining open owner decisions: repository, license,
> governance (see [docs/open-decisions.md](docs/open-decisions.md)).

## What is here

| Path | Purpose |
|---|---|
| `crates/` | Rust workspace: HTTP host, router, bridge, pack reader, QuickJS engine adapter, capabilities |
| `packages/` | TypeScript: authoring core, schema, Treaty client, compiler, CLI, testing helpers |
| `examples/proof/` | Proof application (health, hello, users, auth policy fixture) |
| `baselines/` | Matched raw Rust, raw Bun, and Elysia 2 benchmark applications |
| `conformance/` | Routing/schema/bridge/treaty/lifecycle/security conformance fixtures |
| `benchmarks/` | Cold-start/warm/bridge harnesses, frozen fixtures, raw + summarized evidence |
| `docs/okf/` | Canonical evolving OKF design bundle (read its README first) |
| `docs/reports/` | Implementation evidence reports (M0–M2) |
| `scripts/verify` | One command that verifies the authorized scope |

## Quick start

Requirements pinned in `rust-toolchain.toml` and `package.json`:

- Rust stable toolchain (see `rust-toolchain.toml`)
- Bun (development/package/test tooling only — never the production engine)

```bash
bun install                # TypeScript dependencies
bun run verify             # full verification (Rust + TypeScript + conformance)
```

Build the proof application and run it under the Rust/QuickJS runtime:

```bash
bun run velqu build --project examples/proof
cargo build --release -p q-runtime
./target/release/velqu-runtime --pack examples/proof/dist/app.qpack --port 3000
```

Run the benchmark suite (matched candidates, fresh-process cold starts):

```bash
bun run benchmark:all
```

## Status and claims

All performance characteristics are **hypotheses until measured**. Comparative
results are published only as exact tested-workload statements with raw data in
`benchmarks/raw/` and `docs/reports/`. Nothing here is production-ready, a
Node.js replacement, or a hostile-code sandbox.

See [AGENTS.md](AGENTS.md) for non-negotiable architectural constraints.
