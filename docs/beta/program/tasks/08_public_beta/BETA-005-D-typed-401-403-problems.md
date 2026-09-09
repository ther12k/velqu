---
task_id: BETA-005-D
parent_task: BETA-005
milestone: BETA
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-005-D — Typed 401/403 problems

## Atomic goal

Typed 401/403 problems.

## Parent intent

Provide a secure documented policy example and typed errors.

## Dependencies

- `BETA-005-C` — `tasks/08_public_beta/BETA-005-C-expiry-audience-issuer-checks.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/schema-codecs.md`

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
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `crates/q-engine-quickjs/src/convert.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Typed 401/403 problems.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Invalid tokens fail closed.
- Algorithm confusion is impossible.
- Auth policy error appears in Treaty contract.
- Performance/caching is documented.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-schema-runtime
```

## Required evidence for this microtask

- Security tests.
- Reference docs.
- W1/W2/W3 integration.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
beta-005-d: typed 401 403 problems
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-005-D) — PASS (2026-09-04)

- Branch/PR: beta-005-d (squash-merged; see git log for final hash)
- Closes: #527

### Changed files
- `packages/capability-auth-jwt/src/problems.ts` (new): total failure
  mapping to RFC 9457 problems — 401 for authentication failures with
  `WWW-Authenticate: Bearer error="invalid_token"`, 403
  `insufficient-scope` for authorization failures with
  `error="insufficient_scope"`; closed-set reasons (unknown collapses
  to generic invalid-token 401); `authenticateBearer` whole-flow
  helper; `requireScope` explicit authorization step (403 vs 401
  distinction).
- `packages/capability-auth-jwt/src/problems.test.ts` (new): 9
  deterministic tests.
- `packages/capability-auth-jwt/README.md`: typed-problems section.
- `docs/reports/beta-005-d-typed-401-403-problems.md` (new).

### Required evidence

- **Security tests**: 9 new tests; package total 44 pass (mapping
  totality, WWW-Authenticate headers, 403-vs-401 distinction,
  closed-set collapse, algorithm-confused token -> typed 401).
- **Reference docs**: README section + report.
- **W1/W2/W3 integration**: the declared-401 policy pattern is the
  W1 consumer surface; typed problems layer onto it without changing
  routes; no load-run claims.

### Commands

- `bun test packages/capability-auth-jwt` -> 44 pass / 0 fail
- `bun test` -> 428 pass / 0 fail (66 files)
- typecheck / fmt / clippy -> clean
- `./scripts/verify` -> ALL PASS (M0-M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)
  (isolated netns; standing port-3000 environment note, BETA-002-C record)

### Guardrail mapping

- **Invalid tokens fail closed**: every failure is a typed problem;
  unknown reasons collapse to generic invalid-token 401, never a pass.
- **Algorithm confusion impossible**: unchanged profile gates surface
  as typed 401s.
- **Auth policy error appears in Treaty contract**: problem `type`
  URIs are closed-set and documented; declared statuses (401/403) are
  the contract-visible surface.
- **Performance/caching documented**: mapping is a constant-time table
  lookup; no cache.

### Standing CI disclosure

CI `verify` workflows stall/fail with zero executed steps on PR creation
across all branches (infrastructure-side, tracked since ~#714); the local
`./scripts/verify` run above is the real gate evidence for this packet.
