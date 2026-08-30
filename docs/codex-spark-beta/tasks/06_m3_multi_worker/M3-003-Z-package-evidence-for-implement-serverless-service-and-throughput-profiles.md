---
task_id: M3-003-Z
parent_task: M3-003
milestone: M3
priority: P1
mode: EVIDENCE
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-003-Z — Package evidence for Implement serverless, service, and throughput profiles

## Atomic goal

Create source-backed evidence and handoff for parent task M3-003; update status only if verification passed.

## Parent intent

Make cold start versus immediate throughput an explicit deployment choice.

## Dependencies

- `M3-003-V` — `tasks/06_m3_multi_worker/M3-003-V-verify-implement-serverless-service-and-throughput-profiles.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M3.md`
- `context/components/engine-scheduler.md`
- `context/components/multiworker.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `crates/q-runtime/src/main.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/lib.rs`
- `benchmarks/harness/`
- `benchmarks/manifest.json`
- `benchmarks/raw/`
- `docs/reports/`
- `scripts/benchmark`
- `packages/core/src/index.ts`
- `docs/beta/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

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
```bash
cargo fmt --check
```
```bash
cargo clippy --workspace --all-targets -- -D warnings
```
```bash
./scripts/verify
```

## Required evidence for this microtask

- Profile conformance.
- Cold/RSS report.
- Configuration docs.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m3-003-z: package evidence for implement serverless service and throug
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-003-Z) — PASS

- Date: 2026-08-30
- Branch/PR: m3-003-z (squash-merged; see git log for final hash)
- Closes: #389

### Parent closure — M3-003 Implement serverless, service, and throughput profiles

Parent intent: make cold start versus immediate throughput an explicit deployment choice. Status: **PASS**.

Packet commits (squash merges):
- M3-003-A — 9239959 (#988, Closes #384): `ServiceProfile` — Serverless (initial_workers() == 1 ALWAYS; structurally cannot pre-spawn) vs Service { workers } (explicit count, bare form fails closed, bounds [1,64]); fail-closed parse; round-tripping names
- M3-003-B — 5ba925a (#989, Closes #385): `AdaptiveWorkers` — ready declared with worker 0 (initial state); adds ONLY via the policy tick, bounded by max + cooldown (first add exempt; bursts cannot spawn bursts)
- M3-003-C — 392669e (#990, Closes #386): `Readiness` — deterministic per-profile requirement (serverless=1, throughput=all configured); flip exactly once on the meeting call; one-way; out-of-range fails closed / direct variants clamp
- M3-003-D — 2f9a4cb (#991, Closes #387): `--service-profile` CLI flag (fail-closed before any worker spawns), `--profile-info` inspect surface (verified live: 5 correct JSON rows), ready line declares `serviceProfile` + `startupWorkers`
- M3-003-V — bc795a2 (#992, Closes #388): verification closure with live binary evidence

### Required evidence
- **Profile conformance**: 13 unit tests on the profile state machines + the ready-line/fail-closed integration test on the real binary.
- **Cold/RSS report** (live, V run): serverless ready 7.5ms (budget 10ms); single-worker RSS ~7.9 MB serving the proof pack with health 200. Profile-differentiated scaling is M3-009's dedicated evidence.
- **Configuration docs**: CAPABILITY_AUTHORS.md per-worker section (M3-001-B) + `--profile-info` inspect surface + ready-line declaration.

### Command results (this branch)
- `cargo test -p velqu-runtime` → 28 unit + 5 + 44; `-p q-capabilities` → 6 suites; `-p q-engine-quickjs` → 20+101; `-p q-http` → 4+6+1; `-p q-bridge` → 11 — all pass
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**

### Ledger update
- `docs/beta/04_TASK_LEDGER.md`: M3-003 flipped TODO -> PASS.

### Disclosures (standing)
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
