---
task_id: BETA-007-E
parent_task: BETA-007
milestone: BETA
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-007-E — No dynamic code execution

## Atomic goal

No dynamic code execution.

## Parent intent

Provide typed configuration without top-level network I/O or accidental secret disclosure.

## Dependencies

- `BETA-007-D` — `tasks/08_public_beta/BETA-007-D-profile-specific-settings.md`

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
- `crates/q-runtime/src/main.rs`
- `packages/core/src/index.ts`
- `examples/proof/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: No dynamic code execution.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Invalid config fails before ready.
- Secrets never appear in inspect/log/error.
- Defaults are safe.
- Configuration is documented/versioned.

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

- Config tests.
- Redaction tests.
- Examples.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
beta-007-e: no dynamic code execution
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-007-E) — PASS (2026-09-04)

- Branch/PR: beta-007-e (squash-merged; see git log for final hash)
- Closes: #543

### Behavior implemented

Every production runtime context enforces no dynamic code execution
before any application code runs: the `eval` global (direct AND
indirect forms resolve it — pinned by tests), the `Function` global,
and the function/async/generator prototype constructor routes are
replaced with a typed `TypeError` ("velqu: dynamic code execution is
disabled (...)"). Lockdown installs host-side in `create_context`
(single hook, every profile, both deployment modes) and fails closed:
a lock failure rejects startup. Static definitions (classes, closures,
generators) and instance constructor identity are unaffected
(test-pinned). The `Eval` intrinsic itself cannot be excluded (it
gates the host's own script evaluation; exclusion attempted and
reverted) — the global-binding replacement covers the runtime routes.

### Changed files

- `crates/q-engine-quickjs/src/prelude.rs`
  (`NO_DYNAMIC_CODE_LOCKDOWN` script)
- `crates/q-engine-quickjs/src/worker.rs` (lockdown in
  `create_context`; 4 new tests + probe helper)
- `docs/beta/LIMITS-AND-NON-GOALS.md` ("No dynamic code execution"
  bullet)
- `docs/reports/beta-007-e-no-dynamic-code-execution.md` (new)

### Required evidence

- **Config tests** (hardening tests; 4 new, 24 engine lib total):
  `dynamic_code_routes_fail_typed_in_every_profile` (7 routes × 3
  profiles with the typed message),
  `lockdown_is_tamper_resistant_by_construction`,
  `static_code_still_runs_after_lockdown`,
  `lockdown_marker_present_and_instances_keep_identity`.
- **Redaction tests**: unaffected suites still green (typed denial
  carries no secrets by construction).
- **Examples**: LIMITS-AND-NON-GOALS.md bullet documents the
  guarantee, covered routes, and typed error text.

### Guardrail proofs

1. **Invalid config fails before ready** — lockdown failure is a
   context-creation error; runtime never serves with a dynamic route
   live.
2. **Secrets never appear in inspect/log/error** — denial messages are
   static strings; unrelated suites still green.
3. **Defaults are safe** — lockdown is unconditional; no opt-out.
4. **Configuration is documented/versioned** — not a config surface;
   documented as a runtime limit in LIMITS-AND-NON-GOALS.md.

### Gate results (fresh on this branch)

- `cargo test -p q-engine-quickjs` 24 lib + 117 worker + 1 ->
  0 failures; `-p velqu-runtime` 96 lib + 35 conformance + 16;
  `-p q-http` 14; `-p q-bridge` 11
- fmt / clippy (`-D warnings`) / typecheck -> clean
- `bun test` -> 434 pass / 0 fail (67 files)
- `./scripts/verify` -> ALL PASS (M0-M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)
- `./scripts/validate-okf` -> PASS
  (verify run inside an isolated netns; standing port-3000 environment
  note, BETA-002-C record. No test weakened.)
