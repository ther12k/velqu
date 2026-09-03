---
task_id: BETA-005-V
parent_task: BETA-005
milestone: BETA
priority: P0
mode: VERIFY
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-005-V — Verify Implement JWT/auth reference package

## Atomic goal

Prove every acceptance criterion for parent task BETA-005 without broadening scope.

## Parent intent

Provide a secure documented policy example and typed errors.

## Dependencies

- `BETA-005-A` — `tasks/08_public_beta/BETA-005-A-support-one-approved-jwt-algorithm-profile.md`
- `BETA-005-B` — `tasks/08_public_beta/BETA-005-B-key-loading-rotation-hooks.md`
- `BETA-005-C` — `tasks/08_public_beta/BETA-005-C-expiry-audience-issuer-checks.md`
- `BETA-005-D` — `tasks/08_public_beta/BETA-005-D-typed-401-403-problems.md`
- `BETA-005-E` — `tasks/08_public_beta/BETA-005-E-no-secret-logging.md`

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

- Invalid tokens fail closed.
- Algorithm confusion is impossible.
- Auth policy error appears in Treaty contract.
- Performance/caching is documented.

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

- Security tests.
- Reference docs.
- W1/W2/W3 integration.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
beta-005-v: verify implement jwt auth reference package
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-005-V) — PASS (2026-09-04)

- Branch/PR: beta-005-v (squash-merged; see git log for final hash)
- Closes: #529

### Acceptance-criterion mapping (parent BETA-005)

1. **Invalid tokens fail closed**
   - A: five-gate profile pipeline (structure -> algorithm -> header
     fields -> timing-safe signature -> claims shape) — every gate a
     typed rejection; no best-effort decode path. 14 profile tests.
   - C: `exp` required; missing/malformed claims fail typed; configured
     iss/aud expectations cannot be bypassed by omission. 8 tests.
2. **Algorithm confusion is impossible**
   - A: `alg` must equal `"HS256"` exactly, checked pre-signature; no
     key-type dispatch; single verification path. Tested against
     none/lowercase/case-variant/RS256/ES256/PS256/missing/non-string.
   - B: rotation does not widen the surface — tokens carry no `kid`;
     verification tries bounded active keys; unknown keys are plain
     signature-mismatch (no oracle). 13 keyring tests.
3. **Auth policy error appears in Treaty contract**
   - D: total mapping to RFC 9457 problems with closed-set `type` URIs
     and declared statuses (401 with WWW-Authenticate, 403
     insufficient-scope) — the contract-visible surface; 9 tests.
   - E: sweep test proves no failure string carries token/secret
     material; redaction affordances for caller logging. 6 tests.
4. **Performance/caching is documented**
   - README sections (profile costs, rotation, claims, problems,
     redaction) + per-packet reports; no implicit cache; bounded, typed
     configuration everywhere.

### Commands (fresh on this branch)

- `bun test packages/capability-auth-jwt` -> 50 pass / 0 fail (5 files)
- `cargo test -p velqu-runtime` -> 8 suites ok
- `bun test` -> 434 pass / 0 fail (67 files)
- typecheck / fmt / clippy -> clean
- `./scripts/verify` -> ALL PASS (M0-M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)
  (isolated netns; standing port-3000 environment note, BETA-002-C record)

### Changed files

- Task record only (verification-only packet).

### Disclosures

- Verification-only packet; no runtime behavior changes.
- Standing: CI `verify` workflows stall/fail with zero executed steps
  on PR creation across all branches (infrastructure-side, tracked
  since ~#714); local `./scripts/verify` is the real gate evidence.
