---
task_id: M4A-009-B
parent_task: M4A-009
milestone: M4A
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-009-B — JWT-like policy reference

## Atomic goal

JWT-like policy reference.

## Parent intent

Validate 30–50 routes, auth, fetch, validation, errors, pagination, and deployment.

## Dependencies

- `M4A-009-A` — `tasks/07_m4a_developer_preview/M4A-009-A-feature-modules.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/schema-codecs.md`

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
- `crates/q-capabilities/src/lib.rs`
- `crates/q-http/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: JWT-like policy reference.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

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
cargo test -p q-http
```
```bash
cargo test -p q-schema-runtime
```
```bash
cargo test -p q-capabilities
```
```bash
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- Proof app source.
- Scenario tests.
- Benchmark report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m4a-009-b: jwt like policy reference
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

---

## Result (M4A-009-B) — PASS (2026-09-01)

- Branch/PR: m4a-009-b (squash-merged; see git log for final hash)
- Closes: #484

### Changed files
- `examples/proof/src/policy/jwt.ts` (new): JWT-*like* bearer policy
  reference — JWS-compact-shaped three-segment token (header.payload.signature,
  base64url), HMAC-SHA-256 verification, expiry enforcement with a 5 s
  clock-skew allowance, timing-safe MAC comparison, and a typed session
  (`userId`, `scope`). The runtime intentionally has NO SubtleCrypto (M28:
  crypto is getRandomValues/randomUUID only, mocks forbidden — asserted in the
  engine tests), so the packet includes a compact pure-JS SHA-256/HMAC
  reference implementation pinned by RFC 4231 vectors.
- `examples/proof/src/modules/auth/routes.ts` (new): `auth.login` (POST, issues
  a reference token; declared 401 on bad fixture credentials) and
  `auth.profile` (GET, guarded by the JWT-like policy with typed session).
- `examples/proof/src/app.ts`: auth module + jwtPolicy registered (proof now
  16 routes, 2 policies).
- `examples/proof/src/modules/auth/service.test.ts` (new): 6 tests — issuance
  + session, missing/malformed/tampered/expired rejections, fixture-secret
  disclosure, and RFC 4231 HMAC vectors (cases 1 & 2) + a Node-crypto
  cross-check.
- `conformance/treaty/treaty.conformance.test.ts`: runtime-local scenario
  extended with bad-credential 401, unauthenticated 401, real login → token →
  authorized profile (`usr_ada`, scoped), and forged-signature 401 — all over
  HTTP on the actual runtime.
- Pinned inventory tests updated (inspect routeCount 16, current proof
  contract hash); `benchmarks/manifest.json` refreshed.

### Required evidence

- **Scenario tests**: auth unit suite (6) + runtime-local treaty auth flow
  (login/verify/reject paths on the actual binary).
- **Fixture discipline**: the demo secret contains "demo", is published in the
  repository, and the generated route's 401s are declared; docs/notes state it
  is not production authentication.
- **No crypto mocks**: `crypto.subtle` remains undefined in the runtime
  (engine test asserts it); signing is pure JS validated against RFC 4231.

### Guardrail mapping
- **Runs entirely on actual runtime**: auth login/profile/verify/reject flow
  runs through `runtimeTreaty` on the release `velqu-runtime` binary.
- **No hidden Bun production path**: none added; token logic runs inside
  QuickJS.
- **All error/status contracts declared**: login declares 200/401; profile
  declares 200 + policy 401.
- **Load and failure scenarios pass**: malformed/tampered/expired tokens and
  wrong credentials all fail closed with typed 401s; worker stays healthy.

### Command results

- `cargo test -p velqu-runtime` → PASS
- `bun test` → **318 pass / 0 fail (50 files)**
- `bun run typecheck`, fmt, workspace clippy → clean
- `./scripts/verify` → **ALL PASS**

### Disclosures
- Standing: CI `verify` workflows fail with zero executed steps on every PR
  since ~#714 (infrastructure-side); disclosed per PR.
