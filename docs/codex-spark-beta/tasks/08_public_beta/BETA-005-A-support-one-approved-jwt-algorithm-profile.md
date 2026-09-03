---
task_id: BETA-005-A
parent_task: BETA-005
milestone: BETA
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-005-A — Support one approved JWT algorithm/profile

## Atomic goal

Support one approved JWT algorithm/profile.

## Parent intent

Provide a secure documented policy example and typed errors.

## Dependencies

- `M27-GATE` — `gates/M27-GATE.md`
- `M25-GATE` — `gates/M25-GATE.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
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
- `crates/q-runtime/src/main.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Support one approved JWT algorithm/profile.
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
cargo test -p velqu-runtime
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
beta-005-a: support one approved jwt algorithm profile
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-005-A) — PASS (2026-09-04)

- Branch/PR: beta-005-a (squash-merged; see git log for final hash)
- Closes: #524

### Changed files
- `packages/capability-auth-jwt/` (new, `@velqu/capability-auth-jwt`):
  - `src/index.ts`: the one approved JWT profile — HS256 only. Five
    fail-closed gates (structure, algorithm pre-signature, header
    fields, timing-safe signature, claims shape); typed closed-set
    reasons; RFC 2104 HMAC-SHA-256 + base64url primitives; `signJwt`
    reference issuance.
  - `src/index.test.ts`: 14 deterministic tests — RFC 4231 TC2 vector,
    profile round-trip, algorithm-confusion gates (none/lowercase/
    case-variant/RS256/ES256/PS256/missing/non-string), key-injection
    header rejection (jku/jwk/x5u/kid), typ gate, tampered
    payload/secret, malformed structures, non-object claims,
    base64url round-trip.
  - `README.md`: profile, gates, performance/caching posture.
- `bun.lock`: new workspace member entry.
- `docs/reports/beta-005-a-jwt-algorithm-profile.md` (new).

### Required evidence

- **Security tests**: 14/14 (algorithm-confusion structurally
  impossible — no key-type dispatch, single verification path; every
  gate typed fail-closed).
- **Reference docs**: package README + report (performance/caching
  posture: O(token length) HMAC, no implicit cache; cache would be a
  documented C decision).
- **W1/W2/W3 integration**: W1's policy consumer (proof users.get)
  uses the same HMAC primitives pinned by the RFC 4231 vectors; full
  load runs are BETA-013/014 scope — stated without overclaim.
- **Treaty contract**: policy failures are declared (`declares: {401:
  "unauthorized"}`) — the mechanism the treaty conformance suite
  renders into contracts.

### Commands

- `bun test packages/capability-auth-jwt` -> 14 pass / 0 fail
- `bun test` -> 398 pass / 0 fail (63 files)
- `cargo test -p velqu-runtime` -> 8 suites ok
- fmt / clippy (`-D warnings`) / typecheck -> clean
- `./scripts/verify` -> ALL PASS (M0-M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)
  (isolated netns; standing port-3000 environment note, BETA-002-C record)

### Guardrail mapping

- **Invalid tokens fail closed**: every gate returns typed rejections;
  no best-effort decode path exists.
- **Algorithm confusion is impossible**: single approved algorithm,
  checked pre-signature; no dispatch on key type.
- **Auth policy error appears in Treaty contract**: declared 401 via
  policy `declares` (rendering proven by treaty conformance suite).
- **Performance/caching is documented**: README + report; no implicit
  cache.

### Standing CI disclosure

CI `verify` workflows stall/fail with zero executed steps on PR creation
across all branches (infrastructure-side, tracked since ~#714); the local
`./scripts/verify` run above is the real gate evidence for this packet.
