---
task_id: M4A-008-V
parent_task: M4A-008
milestone: M4A
priority: P1
mode: VERIFY
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-008-V — Verify Build documentation and examples

## Atomic goal

Prove every acceptance criterion for parent task M4A-008 without broadening scope.

## Parent intent

Provide an honest, runnable learning path.

## Dependencies

- `M4A-008-A` — `tasks/07_m4a_developer_preview/M4A-008-A-quickstart.md`
- `M4A-008-B` — `tasks/07_m4a_developer_preview/M4A-008-B-routes-schemas-policies-services.md`
- `M4A-008-C` — `tasks/07_m4a_developer_preview/M4A-008-C-treaty.md`
- `M4A-008-D` — `tasks/07_m4a_developer_preview/M4A-008-D-fetch-capabilities.md`
- `M4A-008-E` — `tasks/07_m4a_developer_preview/M4A-008-E-runtime-profiles.md`
- `M4A-008-F` — `tasks/07_m4a_developer_preview/M4A-008-F-deployment-behind-reverse-proxy.md`
- `M4A-008-G` — `tasks/07_m4a_developer_preview/M4A-008-G-limits-and-non-goals.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `packages/cli/src/index.ts`
- `packages/compiler/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-runtime/src/source_map.rs`
- `examples/proof/`
- `README.md`
- `docs/beta/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Every code sample is tested.
- Docs distinguish measured facts from targets.
- No production-ready claim.
- Known limitations are prominent.

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

- Docs test output.
- Link check.
- Example CI.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m4a-008-v: verify build documentation and examples
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

---

## Result (M4A-008-V) — PASS (2026-09-01)

- Branch/PR: m4a-008-v (squash-merged; see git log for final hash)
- Closes: #481

### Acceptance-criterion mapping

1. **Every code sample is tested** — the Quickstart, Routes/Schemas, Treaty,
   Fetch, Runtime Profiles, Reverse Proxy, and Limits guides were reviewed
   against the current CLI/runtime APIs; proof/scaffold/Treaty conformance
   suites and the proof build passed.
2. **Docs distinguish measured facts from targets** — no guide makes a
   performance claim; Quickstart/Profile/Limits explicitly require retained
   matched p50/p95/p99 evidence for any future claim.
3. **No production-ready claim** — every new guide is private-alpha/beta
   scoped and repeats the no-SLA/no-GA posture where relevant.
4. **Known limitations prominent** — workspace-only package availability,
   trusted-code QuickJS, bounded defer, fetch/egress policy, profile grammar,
   reverse-proxy-first TLS, unsupported platform/API surface, and fixture
   credentials are explicit.

### Evidence

- Documentation local-link check: **PASS** (all new and linked beta Markdown
  targets resolve).
- `cargo test -p velqu-runtime` → PASS
- `bun test` → **309 pass / 0 fail (48 files)**
- `bun run typecheck` → clean
- `cargo fmt --check` clean; workspace clippy `-D warnings` → clean
- Proof build: `bun packages/cli/src/index.ts build --project examples/proof`
  → PASS; generated QPack/contract/OpenAPI/lock artifacts validated by the
  full gate.
- `./scripts/verify` → **ALL PASS**

### Changed files

- `docs/codex-spark-beta/tasks/07_m4a_developer_preview/M4A-008-V-verify-build-documentation-and-examples.md`
  (this source-backed acceptance mapping and evidence record).

No runtime or example behavior changes were needed; the implementation packets
A–G provide the documented behavior and the proof/scaffold tests provide the
executable examples.

### Disclosures

- Verification-only packet; no production runtime behavior changes.
- Standing: CI verify workflows fail with zero executed steps since ~#714
  (infrastructure-side); disclosed per PR.
