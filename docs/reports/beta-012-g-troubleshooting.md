# BETA-012-G — Troubleshooting

## Overview

Added `docs/beta/TROUBLESHOOTING.md` (indexed in `docs/beta/INDEX.md`), a symptom-first guide whose every failure message and exit code was reproduced against a real build in this worktree before being documented.

## Reproduced cases (2026-09-04, this worktree — actual messages in the doc)

| case | stage | result |
|---|---|---|
| missing pack file | `pack.load` | `startup.rejected`, exit 2 |
| config without `configVersion` | `config.resolve` | typed rejection naming the field, exit 2 |
| unknown `VELQU_BOOGABOOGA` env var | `config.resolve` | closed-namespace rejection pointing at CONFIGURATION.md, exit 2 |
| `--host 0.0.0.0` in default reverse-proxy mode | `config.resolve` | loopback-bind requirement message, exit 2 |
| pack with one flipped byte | `pack.load` | fail-closed verification rejection, exit 2 |
| success contrast | — | `ready` line + `/health/live` = `{"status":"ok"}` |

(The earlier rehearsal initially printed `exit=0` — that was the exit of `head` in a pipeline; true exit codes were captured without the pipe and are what the doc states.)

## Additional sections

- Build/toolchain errors: the pinned-toolchain mismatch guard (reproduced during BETA-012-B scaffold work) and the workspace-link guidance for scaffolds.
- Runtime behavior: `engine quarantined` 503, drain `Retry-After`, typed problems with redaction, and the intentional dynamic-code-execution `TypeError`.
- Pointers: INSTALL, CONFIGURATION, DEPLOYMENT-REVERSE-PROXY, LIMITS-AND-NON-GOALS (link check OK; the closed-namespace message itself references `docs/beta/CONFIGURATION.md` at `crates/q-runtime/src/config.rs:327`).

## Guardrail compliance

- Every command/sample tested (table above). No performance claims. Non-SLA/no-production-readiness wording in the still-stuck pointers. No inaccurate bytecode/JIT statement (the limits pointer carries the accurate phrasing).

## Gates

- `cargo test -p velqu-runtime` — pass (8 suites ok)
- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `cargo fmt --all --check` — pass
- `cargo clippy --workspace --all-targets -- -D warnings` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

## Disclosures

- Documentation change only; no runtime behavior modified.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
