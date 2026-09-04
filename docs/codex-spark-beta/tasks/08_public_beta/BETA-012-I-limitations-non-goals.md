---
task_id: BETA-012-I
parent_task: BETA-012
milestone: BETA
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-012-I — Limitations/non-goals

## Atomic goal

Limitations/non-goals.

## Parent intent

Make scope, support, and trade-offs impossible to misunderstand.

## Dependencies

- `BETA-012-H` — `tasks/08_public_beta/BETA-012-H-performance-methodology.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `docs/beta/`
- `scripts/verify`
- `package.json`
- `.github/workflows/verify.yml`
- `packages/cli/src/index.ts`
- `crates/q-runtime/src/source_map.rs`
- `examples/proof/`
- `README.md`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Limitations/non-goals.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Every command/sample is tested.
- No universal performance claim.
- No production-ready/SLA wording.
- QuickJS bytecode versus JIT is explained accurately.

## Targeted commands

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

- Docs CI.
- Link check.
- Example execution.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
beta-012-i: limitations non goals
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-012-I) — PASS (2026-09-04)

- Branch/PR: beta-012-i (squash-merged; see git log for final hash)
- Closes: #582

### Behavior implemented

Updated `docs/beta/LIMITS-AND-NON-GOALS.md` to public beta (`0.1.0-beta.1`):
- Replaced outdated "private alpha" terminology with "public beta (`0.1.0-beta.1`)".
- Added explicit QuickJS bytecode vs JIT compilation section and terminology table entry (“QuickJS bytecode” vs “native JIT compilation”).
- Verified the bounded execution model (queues, bodies, heap, stack, deadlines, fetch, defer).
- Verified non-goals: no full Node/Bun compatibility, no ORM in core, no WebSockets/SSE, no server-side rendering, no native direct public TLS termination (reverse-proxy first), non-SLA.
- Verified links to `docs/beta/02_SCOPE_MATRIX.md` and `docs/beta/governance/POST_BETA_BACKLOG.md`.

### Changed files

- `docs/beta/LIMITS-AND-NON-GOALS.md`
- `docs/reports/beta-012-i-limitations-non-goals.md`
- `docs/codex-spark-beta/tasks/08_public_beta/BETA-012-I-limitations-non-goals.md`
- `docs/codex-spark-beta/STATUS.md`
- `docs/codex-spark-beta/indexes/TASK_INDEX.md`

### Gates

- `cargo test -p velqu-runtime` — pass (8 suites ok)
- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `cargo fmt --all --check` — pass
- `cargo clippy --workspace --all-targets -- -D warnings` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

### Disclosures

- Documentation only; no runtime binary behavior modified.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
