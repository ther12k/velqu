---
task_id: BETA-008-V
parent_task: BETA-008
milestone: BETA
priority: P0
mode: VERIFY
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-008-V — Verify Implement reverse-proxy, drain, and deployment semantics

## Atomic goal

Prove every acceptance criterion for parent task BETA-008 without broadening scope.

## Parent intent

Make the beta deployable behind common cloud/reverse-proxy setups.

## Dependencies

- `BETA-008-A` — `tasks/08_public_beta/BETA-008-A-trusted-proxy-configuration.md`
- `BETA-008-B` — `tasks/08_public_beta/BETA-008-B-forwarded-header-policy.md`
- `BETA-008-C` — `tasks/08_public_beta/BETA-008-C-liveness-readiness-startup-endpoints.md`
- `BETA-008-D` — `tasks/08_public_beta/BETA-008-D-graceful-drain-and-termination.md`
- `BETA-008-E` — `tasks/08_public_beta/BETA-008-E-container-example.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `docs/beta/`
- `scripts/verify`
- `package.json`
- `.github/workflows/verify.yml`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/src/serve.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Spoofed forwarding headers are ignored unless trusted.
- Readiness drops before drain.
- In-flight requests honor deadline.
- Container shutdown exits deterministically.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-http
```
```bash
cargo test -p q-bridge
```
```bash
cargo test -p velqu-runtime
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

- Proxy tests.
- Container smoke test.
- Runbook.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
beta-008-v: verify implement reverse proxy drain and deployment semantic
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
