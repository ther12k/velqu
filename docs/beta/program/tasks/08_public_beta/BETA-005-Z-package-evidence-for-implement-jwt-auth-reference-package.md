---
task_id: BETA-005-Z
parent_task: BETA-005
milestone: BETA
priority: P0
mode: EVIDENCE
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-005-Z — Package evidence for Implement JWT/auth reference package

## Atomic goal

Create source-backed evidence and handoff for parent task BETA-005; update status only if verification passed.

## Parent intent

Provide a secure documented policy example and typed errors.

## Dependencies

- `BETA-005-V` — `tasks/08_public_beta/BETA-005-V-verify-implement-jwt-auth-reference-package.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/qpack-router.md`
- `context/components/devex-beta.md`
- `context/components/evidence.md`

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
- `packages/auth-jwt/ (create if absent)`
- `packages/core/src/index.ts`
- `packages/treaty/src/index.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

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
```bash
./scripts/validate-okf
```

## Required evidence for this microtask

- Security tests.
- Reference docs.
- W1/W2/W3 integration.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
beta-005-z: package evidence for implement jwt auth reference package
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-005-Z) — PASS (2026-09-04)

- Branch/PR: beta-005-z (squash-merged; see git log for final hash)
- Closes: #530
- Parent verification: BETA-005-V PASS (PR #1129); this packet packages
  the source-backed evidence across all child packets (A through E + V)
  and flips parent task BETA-005 to PASS in
  `docs/beta/04_TASK_LEDGER.md`.

### Evidence package

- **Implementation packets (squash-merged):**
  - BETA-005-A (PR #1124): one approved algorithm profile — HS256-only
    five-gate fail-closed verification (`@velqu/capability-auth-jwt`),
    algorithm-confusion structurally impossible; RFC 4231 vector pinned.
  - BETA-005-B (PR #1125): key loading/rotation hooks — `JwtKeyring`
    with validated loading, overlap rotation, atomic refresh, ids-only
    snapshots.
  - BETA-005-C (PR #1126): expiry/audience/issuer checks — required
    numeric exp, bounded skew, configured-expectation enforcement, 401
    injectable clock.
  - BETA-005-D (PR #1127): typed 401/403 RFC 9457 problems — total
    mapping, WWW-Authenticate headers, 403-vs-401 distinction.
  - BETA-005-E (PR #1128): no-secret-logging — enforcement sweep +
    redaction/fingerprint affordances.
  - BETA-005-V (PR #1129): verification closure; fresh full-gate run
    reproduces.

### Required evidence

- **Security tests**: package total 50 pass across 5 files (profile
  gates, keyring rotation, claims validation, problem mapping, no-
  secret sweep) — re-run fresh on this branch.
- **Reference docs**: package README (profile/rotation/claims/problems/
  redaction sections) + `docs/reports/beta-005-*.md` per packet.
- **W1/W2/W3 integration**: W1's JWT policy consumer uses the same
  pinned HMAC primitives; policy `declares` 401 is the
  contract-visible surface; full load runs are BETA-013/014 scope —
  stated without overclaim.

### Parent guardrail proofs

1. **Invalid tokens fail closed** — every gate typed; no best-effort
   decode; configured expectations cannot be bypassed by omission.
2. **Algorithm confusion impossible** — single approved algorithm,
   pre-signature gate, no key-type dispatch (tested across none/
   variants/asymmetric/missing).
3. **Auth policy error appears in Treaty contract** — declared 401 via
   policy `declares`; RFC 9457 typed problem URIs documented.
4. **Performance/caching documented** — O(token length) verification;
   no implicit cache; bounded typed configuration everywhere.

### Gate results (fresh on this branch)

- `bun test packages/capability-auth-jwt` -> 50 pass / 0 fail
- `cargo test -p velqu-runtime` -> 8 suites ok
- fmt / clippy (`-D warnings`) / typecheck -> clean
- `bun test` -> 434 pass / 0 fail (67 files)
- `./scripts/verify` -> ALL PASS (M0-M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)
- `./scripts/validate-okf` -> PASS
  (verify run inside an isolated netns; standing port-3000 environment
  note, BETA-002-C record. No test weakened.)

### Ledger

- `docs/beta/04_TASK_LEDGER.md`: BETA-005 flipped TODO -> **PASS**.
- STATUS.md and TASK_INDEX.md updated to PASS (BETA-005-Z row).
