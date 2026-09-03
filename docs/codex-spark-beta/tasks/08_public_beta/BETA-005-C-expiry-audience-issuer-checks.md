---
task_id: BETA-005-C
parent_task: BETA-005
milestone: BETA
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-005-C — Expiry/audience/issuer checks

## Atomic goal

Expiry/audience/issuer checks.

## Parent intent

Provide a secure documented policy example and typed errors.

## Dependencies

- `BETA-005-B` — `tasks/08_public_beta/BETA-005-B-key-loading-rotation-hooks.md`

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
- `packages/cli/src/index.ts`
- `crates/q-runtime/src/source_map.rs`
- `examples/proof/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Expiry/audience/issuer checks.
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
cargo test -p velqu-runtime
```
```bash
bun test
```
```bash
bun run typecheck
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
beta-005-c: expiry audience issuer checks
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-005-C) — PASS (2026-09-04)

- Branch/PR: beta-005-c (squash-merged; see git log for final hash)
- Closes: #526

### Changed files
- `packages/capability-auth-jwt/src/claims.ts` (new):
  `validateClaims` (exp required numeric seconds, skew-tolerant expiry,
  optional nbf, iss/aud enforced only when expected — and claim
  omission then fails typed), `verifyJwtWithClaims` composition,
  injectable clock, bounded skew (default 5s / ceiling 60s).
- `packages/capability-auth-jwt/src/claims.test.ts` (new): 8
  deterministic tests (expired/missing/non-numeric exp, skew tolerance
  + ceiling, nbf future/malformed, iss/aud match/mismatch/missing/
  unconfigured, composition with profile gates, forged-signature
  precedence).
- `packages/capability-auth-jwt/README.md`: claims-validation section.
- `docs/reports/beta-005-c-expiry-audience-issuer-checks.md` (new).

### Required evidence

- **Security tests**: 8 new deterministic tests; package total 35 pass.
- **Reference docs**: README claims section + report.
- **W1/W2/W3 integration**: the policy pattern (proof users.get, W1)
  layers these checks via options; no load-run claims.

### Commands

- `bun test packages/capability-auth-jwt` -> 35 pass / 0 fail
- `bun test` -> 419 pass / 0 fail (65 files)
- typecheck / fmt / clippy -> clean
- `./scripts/verify` -> ALL PASS (M0-M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)
  (isolated netns; standing port-3000 environment note, BETA-002-C record)

### Guardrail mapping

- **Invalid tokens fail closed**: missing/invalid exp, future nbf,
  issuer/audience mismatches all typed rejections; configured
  expectations cannot be bypassed by omission.
- **Algorithm confusion impossible**: unchanged profile gates run
  before claims validation.
- **Auth policy error appears in Treaty contract**: typed reasons map
  to the declared 401 (unchanged mechanism).
- **Performance/caching documented**: claims validation is linear over
  a bounded claim set; no cache.

### Standing CI disclosure

CI `verify` workflows stall/fail with zero executed steps on PR creation
across all branches (infrastructure-side, tracked since ~#714); the local
`./scripts/verify` run above is the real gate evidence for this packet.
