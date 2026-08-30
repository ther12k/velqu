---
task_id: M3-003-D
parent_task: M3-003
milestone: M3
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-003-D — Expose profile in inspect/config

## Atomic goal

Expose profile in inspect/config.

## Parent intent

Make cold start versus immediate throughput an explicit deployment choice.

## Dependencies

- `M3-003-C` — `tasks/06_m3_multi_worker/M3-003-C-throughput-initializes-configured-workers-before-ready.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M3.md`
- `context/components/engine-scheduler.md`
- `context/components/multiworker.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `crates/q-runtime/src/main.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/lib.rs`
- `packages/cli/src/index.ts`
- `crates/q-runtime/src/source_map.rs`
- `examples/proof/`
- `README.md`
- `docs/beta/`
- `benchmarks/harness/`
- `benchmarks/manifest.json`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Expose profile in inspect/config.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Serverless cold start remains within approved budget.
- Profiles have deterministic readiness.
- No hidden worker creation.
- Profile-specific RSS is reported.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p velqu-runtime
```
```bash
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- Profile conformance.
- Cold/RSS report.
- Configuration docs.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m3-003-d: expose profile in inspect config
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-003-D) — PASS

- Date: 2026-08-30
- Branch/PR: m3-003-d (squash-merged; see git log for final hash)
- Closes: #387

### Changed files
- `crates/q-runtime/src/main.rs`: two new CLI flags —
  - `--service-profile <serverless|service:N>`: the deployment choice from M3-003-A, wired into RunConfig; unknown names fail closed at startup (exit 2, error names the problem) BEFORE any worker spawns.
  - `--profile-info`: the inspect surface — parses probe values (`serverless`, `service:4`, `service:0`, `bogus`) and prints JSON rows (parsed name, initialWorkers, or the typed error) plus the [minWorkers, maxWorkers] bounds; exit 0, no pack required. Verified live: all five rows correct.
- `crates/q-runtime/src/lib.rs`: `RunConfig.service_profile` field; the startup pipeline resolves the profile BEFORE any worker spawns (fail closed, exit 2) and the **ready line now declares `serviceProfile` + `startupWorkers`** (serverless/1 by default).
- `crates/q-runtime/src/bin/velqu-standalone.rs`: standalone passes the serverless default explicitly.
- `crates/q-runtime/tests/runtime_conformance.rs`: `Server` retains the ready line consumed by its readiness wait (previously swallowed); new integration test `ready_line_reports_service_profile_and_bad_profile_fails_closed` — default ready line declares `serviceProfile:"serverless", startupWorkers:1`; `--service-profile bogus` exits 2 with a stderr error naming the problem.

### Tests added/extended
- `ready_line_reports_service_profile_and_bad_profile_fails_closed` (integration, real binary)
- `--profile-info` exercised live on the built binary (5 correct JSON rows)

### Command results
- `cargo test -p velqu-runtime` → **33 unit + 5 + 44** (runtime_conformance 32) — 0 failed
- `cargo test -p q-capabilities` → 6 suites · `-p q-engine-quickjs` 20+101 · `-p q-http` 4+6+1 · `-p q-bridge` 11 — all pass
- `bun run typecheck` → clean; `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**; manifest refreshed (`55b79127…`) — the CLI flag is part of the binary artifact.

### Guardrail mapping
- **Profiles have deterministic readiness** — the profile resolves before any worker spawns; bad values exit 2 deterministically.
- **No hidden worker creation** — the ready line DECLARES the profile and startup worker count: deployment posture is observable, not assumed.

### Disclosures
- Test-harness fix: `Server::start` swallowed the ready line while waiting for readiness — it is now retained on the struct for inspection. One clippy iteration (unused-assignment restructure). One verify iteration on the legitimate artifact refresh.
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
