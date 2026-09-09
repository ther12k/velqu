# Velqu (VelquJS) — Wiki

Velqu is a Rust HTTP runtime that runs TypeScript handlers on an embedded
quickjs-ng engine. One schema contract drives types, runtime validation,
Treaty clients, OpenAPI, and the contract lock.

> **Honesty line:** same-process QuickJS executes trusted application code
> only — Velqu is not a hostile-code sandbox. Performance claims are
> evidence-bound; no PostgreSQL-parity or native-performance-parity claims
> are made without matched, reproducible evidence.

## Pages

| Page | Contents |
| --- | --- |
| [[Getting Started]] | Build from source, 30-second example, deployment |
| [[Architecture]] | Rust host + QuickJS engine, schema contract, artifact flow |
| [[Browser-WASM]] | Static browser deployment: Rust/WASM kernel + Worker handlers |
| [[Evidence and Verification]] | Gates, verify command, evidence-bound performance |
| [[Limitations and Honesty]] | Recorded limitations and claim policy |

## Canonical sources

The wiki is a navigator; the canonical documents live in the repository:

- Beta plan: `docs/beta/` (start at [INDEX](https://github.com/ther12k/velqu/blob/master/docs/beta/INDEX.md))
- Browser-WASM guide: [docs/beta/BROWSER_WASM.md](https://github.com/ther12k/velqu/blob/master/docs/beta/BROWSER_WASM.md)
- Architecture decisions: [docs/okf/decisions/](https://github.com/ther12k/velqu/tree/master/docs/okf/decisions)
- Open owner decisions: [docs/open-decisions.md](https://github.com/ther12k/velqu/blob/master/docs/open-decisions.md)

**Status:** pre-beta toward `0.1.0-beta.1` (ADR-0020). License: MIT (OD-004).
