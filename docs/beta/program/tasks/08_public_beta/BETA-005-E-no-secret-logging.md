---
task_id: BETA-005-E
parent_task: BETA-005
milestone: BETA
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-005-E — No secret logging

## Atomic goal

No secret logging.

## Parent intent

Provide a secure documented policy example and typed errors.

## Dependencies

- `BETA-005-D` — `tasks/08_public_beta/BETA-005-D-typed-401-403-problems.md`

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
5. Implement exactly this deliverable: No secret logging.
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
beta-005-e: no secret logging
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-005-E) — PASS (2026-09-04)

- Branch/PR: beta-005-e (squash-merged; see git log for final hash)
- Closes: #528

### Changed files
- `packages/capability-auth-jwt/src/redaction.ts` (new): logging
  affordances — `redactToken` / `redactAuthorizationHeader`
  (constant-shape markers: segment counts and byte lengths only, no
  prefixes/suffixes/partial material), `scrub(text, secrets)`
  (defense-in-depth scrubbing), `secretFingerprint` (keyed, stable,
  non-reversible 12-hex id).
- `packages/capability-auth-jwt/src/redaction.test.ts` (new): the
  enforcement sweep — every typed failure (profile gates, problems,
  keyring rejections/snapshots) constructed with a distinctive secret
  and asserted free of secret/token material; marker shapes; scrub
  completeness; fingerprint properties. 6 tests.
- `packages/capability-auth-jwt/README.md`: no-secret-logging section.
- `docs/reports/beta-005-e-no-secret-logging.md` (new).

### Required evidence

- **Security tests**: enforcement sweep + helper tests; package total
  50 pass.
- **Reference docs**: README section + report.
- **W1/W2/W3 integration**: logging affordances are consumer-facing;
  no load-run claims.

### Commands

- `bun test packages/capability-auth-jwt` -> 50 pass / 0 fail
- `bun test` -> 434 pass / 0 fail (67 files)
- typecheck / fmt / clippy -> clean
- `./scripts/verify` -> ALL PASS (M0-M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)
  (isolated netns; standing port-3000 environment note, BETA-002-C record)

### Guardrail mapping

- **Invalid tokens fail closed**: unchanged; sweep proves failures
  carry no material.
- **Algorithm confusion impossible**: unchanged gates.
- **Auth policy error appears in Treaty contract**: problem documents
  remain the contract-visible surface, now swept for material.
- **Performance/caching documented**: redaction helpers are linear
  over the line; no cache.

### Standing CI disclosure

CI `verify` workflows stall/fail with zero executed steps on PR creation
across all branches (infrastructure-side, tracked since ~#714); the local
`./scripts/verify` run above is the real gate evidence for this packet.
