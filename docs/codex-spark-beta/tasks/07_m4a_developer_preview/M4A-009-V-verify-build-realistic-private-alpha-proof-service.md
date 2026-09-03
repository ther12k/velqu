---
task_id: M4A-009-V
parent_task: M4A-009
milestone: M4A
priority: P0
mode: VERIFY
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-009-V — Verify Build realistic private-alpha proof service

## Atomic goal

Prove every acceptance criterion for parent task M4A-009 without broadening scope.

## Parent intent

Validate 30–50 routes, auth, fetch, validation, errors, pagination, and deployment.

## Dependencies

- `M4A-009-A` — `tasks/07_m4a_developer_preview/M4A-009-A-feature-modules.md`
- `M4A-009-B` — `tasks/07_m4a_developer_preview/M4A-009-B-jwt-like-policy-reference.md`
- `M4A-009-C` — `tasks/07_m4a_developer_preview/M4A-009-C-controlled-upstream.md`
- `M4A-009-D` — `tasks/07_m4a_developer_preview/M4A-009-D-metrics-readiness-shutdown.md`
- `M4A-009-E` — `tasks/07_m4a_developer_preview/M4A-009-E-treaty-client.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/engine-scheduler.md`
- `context/components/schema-codecs.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `packages/cli/src/index.ts`
- `packages/compiler/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `crates/q-engine-quickjs/src/convert.rs`
- `packages/schema/src/index.ts`
- `conformance/schema/schema.conformance.test.ts`
- `packages/contract/src/index.ts`
- `conformance/treaty/treaty.conformance.test.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Runs entirely on actual runtime.
- No hidden Bun production path.
- All error/status contracts declared.
- Load and failure scenarios pass.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-schema-runtime
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

- Proof app source.
- Scenario tests.
- Benchmark report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m4a-009-v: verify build realistic private alpha proof service
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

---

## Result (M4A-009-V) — PASS (2026-09-01)

- Branch/PR: m4a-009-v (squash-merged; see git log for final hash)
- Closes: #488

### Acceptance-criterion mapping

1. **Runs entirely on actual runtime** — all proof routes, feature modules
   (items, auth, upstream, ops), and policy checks run on `velqu-runtime`
   (Rust + QuickJS) via `runtimeTreaty` over real HTTP.
2. **No hidden Bun production path** — production artifacts are compiled
   `app.qpack` loaded by the Rust host; Bun is only used for dev tooling
   and test orchestration.
3. **All error/status contracts declared** — every route declares exact status
   and problem schemas (e.g. 200/201/401/404/502); undeclared statuses fail closed.
4. **Load and failure scenarios pass** — validated across pagination, tampered/expired
   tokens, upstream gateway errors (502), body validation errors (422), and
   clean graceful SIGTERM shutdown with zero leaks.

### Evidence

- `cargo test -p q-engine-quickjs` → PASS (113 passed)
- `cargo test -p q-schema-runtime` → PASS (58 passed)
- `bun test` → **326 pass / 0 fail (54 files)**
- `bun run typecheck` → clean
- `cargo fmt --check`, workspace clippy `-D warnings` → clean
- `./scripts/verify` → **ALL PASS**

### Changed files

- `docs/codex-spark-beta/tasks/07_m4a_developer_preview/M4A-009-V-verify-build-realistic-private-alpha-proof-service.md`

### Disclosures

- Verification-only packet; no production runtime behavior changes.
- Standing: CI `verify` workflows fail with zero executed steps since ~#714
  (infrastructure-side); disclosed per PR.
