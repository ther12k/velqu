---
task_id: BETA-012-V
parent_task: BETA-012
milestone: BETA
priority: P1
mode: VERIFY
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-012-V — Verify Complete beta documentation and limitations

## Atomic goal

Prove every acceptance criterion for parent task BETA-012 without broadening scope.

## Parent intent

Make scope, support, and trade-offs impossible to misunderstand.

## Dependencies

- `BETA-012-A` — `tasks/08_public_beta/BETA-012-A-installation.md`
- `BETA-012-B` — `tasks/08_public_beta/BETA-012-B-quickstart.md`
- `BETA-012-C` — `tasks/08_public_beta/BETA-012-C-architecture.md`
- `BETA-012-D` — `tasks/08_public_beta/BETA-012-D-contracts-treaty.md`
- `BETA-012-E` — `tasks/08_public_beta/BETA-012-E-fetch-postgres-auth.md`
- `BETA-012-F` — `tasks/08_public_beta/BETA-012-F-deployment.md`
- `BETA-012-G` — `tasks/08_public_beta/BETA-012-G-troubleshooting.md`
- `BETA-012-H` — `tasks/08_public_beta/BETA-012-H-performance-methodology.md`
- `BETA-012-I` — `tasks/08_public_beta/BETA-012-I-limitations-non-goals.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `docs/beta/`
- `scripts/verify`
- `package.json`
- `.github/workflows/verify.yml`
- `crates/q-pack/src/lib.rs`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `docs/specs/pack-format-v1.md`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Every command/sample is tested.
- No universal performance claim.
- No production-ready/SLA wording.
- QuickJS bytecode versus JIT is explained accurately.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-engine-quickjs
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

- Docs CI.
- Link check.
- Example execution.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
beta-012-v: verify complete beta documentation and limitations
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-012-V) — PASS (2026-09-04)

- Branch/PR: beta-012-v (squash-merged; see git log for final hash)
- Closes: #583

### Behavior verified

Verification closure for parent task BETA-012 ("Complete beta documentation and limitations"):
- Every command/sample across all beta docs tested and verified against actual builds.
- Universal performance claims explicitly rejected; QuickJS bytecode vs JIT compilation trade-offs accurately documented.
- Public beta (`0.1.0-beta.1`) non-SLA status and evaluation/internal-service framing enforced across all documentation.
- Link check and Docs CI: `./scripts/validate-okf` checked all 189 internal markdown links with 0 errors.
- Example execution: `examples/proof` pack built, runtime served on loopback, `/health/live` returned `{"status":"ok"}`.

### Changed files

- `docs/reports/beta-012-v-verify-beta-documentation-limitations.md`
- `docs/codex-spark-beta/tasks/08_public_beta/BETA-012-V-verify-complete-beta-documentation-and-limitations.md`
- `docs/codex-spark-beta/STATUS.md`
- `docs/codex-spark-beta/indexes/TASK_INDEX.md`

### Gates

- `cargo test -p q-pack` — pass (100+2)
- `cargo test -p q-engine-quickjs` — pass (24+117+1)
- `cargo fmt --all --check` — pass
- `cargo clippy --workspace --all-targets -- -D warnings` — pass
- `bun run typecheck` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

### Disclosures

- Documentation verification only; no production runtime behavior changed.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
