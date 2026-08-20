---
task_id: BETA-002-V
parent_task: BETA-002
milestone: BETA
priority: P1
mode: VERIFY
status: TODO
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-002-V — Verify Implement matched competitor candidates

## Atomic goal

Prove every acceptance criterion for parent task BETA-002 without broadening scope.

## Parent intent

Provide Raw Rust, Elysia 2, Hono/Bun, and Fastify/Node implementations of identical contracts.

## Dependencies

- `BETA-002-A` — `tasks/08_public_beta/BETA-002-A-match-sql-pool-jwt-timeouts-logging-responses-compression-and-deployment-limits.md`
- `BETA-002-B` — `tasks/08_public_beta/BETA-002-B-pin-versions.md`
- `BETA-002-C` — `tasks/08_public_beta/BETA-002-C-add-contract-response-verification.md`
- `BETA-002-D` — `tasks/08_public_beta/BETA-002-D-document-unavoidable-differences.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/qpack-router.md`
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
- `packages/treaty/src/index.ts`
- `packages/contract/src/index.ts`
- `packages/testing/src/index.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Candidates are semantically equivalent.
- No framework receives hidden advantages.
- All outputs pass contract fixtures.
- Version/hash metadata is captured.

## Targeted commands

```bash
cargo test -p q-pack
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

- Candidate source.
- Parity tests.
- Fairness report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
beta-002-v: verify implement matched competitor candidates
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
