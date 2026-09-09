---
task_id: M4A-007-D
parent_task: M4A-007
milestone: M4A
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-007-D — Forbid unbounded recursive spawning

## Atomic goal

Forbid unbounded recursive spawning.

## Parent intent

Provide after-response cleanup/best-effort work without pretending it is durable jobs.

## Dependencies

- `M4A-007-C` — `tasks/07_m4a_developer_preview/M4A-007-C-expose-metrics.md`

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

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Forbid unbounded recursive spawning.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Response is not delayed beyond defined handoff.
- Deferred work is bounded.
- Shutdown handles or aborts it deterministically.
- Docs warn against durable-job use.

## Targeted commands

```bash
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- Lifecycle tests.
- Load/cleanup tests.
- Operational docs.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m4a-007-d: forbid unbounded recursive spawning
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

---

## Result (M4A-007-D) — PASS (2026-09-01)

- Branch/PR: m4a-007-d (squash-merged; see git log for final hash)
- Closes: #471

### Changed files
- `crates/q-engine-quickjs/src/prelude.rs`: the deferred queue is now
  **closure-private** — the `globalThis.__velquDeferred` mutable array is
  gone, so `__velquDefer` (phase- and capacity-checked) is the only entry
  point; admission consults the host-configured capacity via `__velquDeferCap`
  instead of a hardcoded 64; queue length observer (`__velquDeferredLen`) and
  host trim helper (`__velquDrainTrim`) live inside the closure.
- `crates/q-engine-quickjs/src/worker.rs`: `install_natives` takes the
  configured `defer_queue_capacity` and exposes `__velquDeferCap`; the
  array-reading length native is removed (the queue has no JS-reachable
  alias); `drain_deferred` truncates via `__velquDrainTrim` instead of eval-ing
  raw JS against a global.
- `crates/q-engine-quickjs/tests/engine.rs`: three new handlers
  (`defer.queue_hidden`, `defer.recursive_spam`, `defer.spam10`) and three
  tests; handler-table load test now pins 68 handlers.
- `docs/specs/defer-api.md`: bounds table extended with the direct-access
  prohibition; recursion vectors documented.

### Required evidence

- **Lifecycle tests**:
  - `defer_queue_is_hidden_from_handlers` — no global queue array exists;
    a direct push attempt fails closed; nothing admitted.
  - `defer_recursive_spawning_is_bounded` — self-recursion through defer
    fills the bounded queue (64 admitted), the 65th admission throws
    (`defer queue capacity reached`), and at the drain all 64 callbacks'
    re-defer attempts are owner-rejected (`defers_rejected 65` = 1 cap +
    64 drain rejections); drains/interrupts stay clean.
  - `defer_admission_enforces_configured_capacity` — with
    `defer_queue_capacity: 4`, exactly 4 of 10 attempts are admitted (the
    old hardcoded JS literal is gone; admission follows the host config).
  - All A/B/C defer tests stay green (112 total in the suite).
- **Load/cleanup tests** — handler-table load test pins 68 registered
  handlers; drain/cleanup budgets untouched.
- **Operational docs** — `docs/specs/defer-api.md` documents the
  structural prohibition and the configured-capacity admission.

### Guardrail mapping (parent M4A-007)

- **Response is not delayed beyond defined handoff**: unchanged; no admit-path
  work moved to handoff.
- **Deferred work is bounded**: admission now enforces the *configured* cap at
  the JS boundary; the queue is unreachable except through the checked API —
  the bypass (direct global push) that would have allowed unbounded recursive
  spawning is structurally forbidden.
- **Shutdown handles or aborts it deterministically**: unchanged (drop
  counting still works via the closure observer).
- **Docs warn against durable-job use**: warning retained and strengthened
  with the structural-prohibition note.

### Command results

- `cargo test -p q-engine-quickjs` → **112 pass / 0 fail** (was 109; +3)
- `cargo clippy --workspace --all-targets -- -D warnings` → clean
- `./scripts/verify` → **ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)**

### Disclosures

- Standing: CI `verify` workflows fail with zero executed steps on every PR
  since ~#714 (infrastructure-side); disclosed per PR. Local
  `./scripts/verify` is the gate evidence.
