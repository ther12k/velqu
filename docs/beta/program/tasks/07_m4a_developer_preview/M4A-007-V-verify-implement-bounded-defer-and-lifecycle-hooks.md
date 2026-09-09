---
task_id: M4A-007-V
parent_task: M4A-007
milestone: M4A
priority: P0
mode: VERIFY
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-007-V — Verify Implement bounded `defer` and lifecycle hooks

## Atomic goal

Prove every acceptance criterion for parent task M4A-007 without broadening scope.

## Parent intent

Provide after-response cleanup/best-effort work without pretending it is durable jobs.

## Dependencies

- `M4A-007-A` — `tasks/07_m4a_developer_preview/M4A-007-A-define-deferred-owner-queue-deadline-cancellation-shutdown.md`
- `M4A-007-B` — `tasks/07_m4a_developer_preview/M4A-007-B-separate-cleanup-from-best-effort-work.md`
- `M4A-007-C` — `tasks/07_m4a_developer_preview/M4A-007-C-expose-metrics.md`
- `M4A-007-D` — `tasks/07_m4a_developer_preview/M4A-007-D-forbid-unbounded-recursive-spawning.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/ingress-bridge.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `packages/cli/src/index.ts`
- `packages/compiler/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Response is not delayed beyond defined handoff.
- Deferred work is bounded.
- Shutdown handles or aborts it deterministically.
- Docs warn against durable-job use.

## Targeted commands

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

- Lifecycle tests.
- Load/cleanup tests.
- Operational docs.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m4a-007-v: verify implement bounded defer and lifecycle hooks
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

---

## Result (M4A-007-V) — PASS (2026-09-01)

- Branch/PR: m4a-007-v (squash-merged; see git log for final hash)
- Closes: #472

### Acceptance-criterion mapping

1. **Response is not delayed beyond handoff** — corrected the `Step::Failed`
   path to send the response before `drain_deferred`, matching Immediate and
   resolved-watch paths. `failed_response_is_handed_off_before_defer_drain`
   pins this with a 100 ms spinning deferred callback and a response-latency
   assertion under 90 ms; the worker remains usable afterward.
2. **Deferred work is bounded** — queue capacity, configured-cap admission,
   dedicated `DeferredDrain` phase, deadline interrupt, and closure-private
   queue are covered by the A/B/D tests; recursive bypasses fail closed.
3. **Shutdown handles/aborts deterministically** — A/C/D lifecycle tests pin
   deadline interruption, timeout/cancel behavior, and shutdown drop counters.
4. **Durable-job warning** — `docs/specs/defer-api.md` explicitly says defer
   is best-effort, in-memory, never persisted/retried, and unsuitable for
   durable work; the operational bounds and metric semantics are documented.

### Evidence

- `cargo test -p q-http` → PASS
- `cargo test -p q-bridge` → PASS
- `cargo test -p velqu-runtime` → PASS (55 + 6 + 5 + 2 + 35 + 3 tests)
- `cargo test -p q-engine-quickjs` → **113 pass / 0 fail**
- `bun test` → **308 pass / 0 fail (48 files)**
- `bun run typecheck` → clean
- `cargo fmt --check` clean; workspace clippy `-D warnings` → clean
- `./scripts/verify` → **ALL PASS** (after rebuilding debug proof artifacts;
  the first isolated runtime/Bun attempts correctly reported missing/stale
  generated artifacts rather than weakening tests)

### Changed files

- `crates/q-engine-quickjs/src/worker.rs`: failed-path handoff ordering fix.
- `crates/q-engine-quickjs/tests/engine.rs`:
  `failed_response_is_handed_off_before_defer_drain` (handler + assertion).
- `docs/specs/defer-api.md`: corrected uniform handoff ordering statement.

### Disclosures

- Standing: CI `verify` workflows fail with zero executed steps on every PR
  since ~#714 (infrastructure-side); disclosed per PR. Local `./scripts/verify`
  is the gate evidence.
