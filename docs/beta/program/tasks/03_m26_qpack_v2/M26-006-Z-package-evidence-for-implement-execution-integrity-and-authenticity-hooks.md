---
task_id: M26-006-Z
parent_task: M26-006
milestone: M26
priority: P1
mode: EVIDENCE
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-006-Z — Package evidence for Implement execution integrity and authenticity hooks

## Atomic goal

Create source-backed evidence and handoff for parent task M26-006; update status only if verification passed.

## Parent intent

Protect pack corruption now and provide optional publisher signature verification for beta artifacts.

## Dependencies

- `M26-006-V` — `tasks/03_m26_qpack_v2/M26-006-V-verify-implement-execution-integrity-and-authenticity-hooks.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M26.md`
- `context/components/qpack-router.md`
- `context/components/ingress-bridge.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `crates/q-runtime/src/main.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `docs/specs/pack-format-v1.md`
- `packages/auth-jwt/ (create if absent)`
- `packages/core/src/index.ts`
- `packages/treaty/src/index.ts`
- `conformance/security/security.conformance.test.ts`
- `crates/q-http/tests/fuzz_parsers.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- Digest detects corruption.
- Signature verifies publisher when configured.
- Unsigned production policy is explicit.
- No docs conflate digest and authenticity.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-http
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

- Integrity/signature tests.
- Key rotation notes.
- Threat-model update.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m26-006-z: package evidence for implement execution integrity and authe
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Evidence package

- Status: **PASS**. Parent verification M26-006-V merged in PR #809 at
  commit `66a28be`; issue #214 is closed. The evidence package is based
  on clean parent HEAD `31d7e4f` before this commit.
- Parent acceptance matrix: `M26-006-V` maps all four guardrails to
  source and named tests (digest detects corruption — recomputed
  aggregate over required sections catches self-consistent rewrites;
  signature verifies publisher — RFC 8032 vector + fail-closed
  roundtrip + trust-set authorization; unsigned production policy
  explicit — default UnsignedAllowed, RequireSignature deliberate with
  JSON round-trip; no digest/authenticity conflation — ADR-0026
  boundary in spec, module docs, and the threat model).
- Source-backed implementation records:
  - `M26-006-A` (PR #805, #210 closed): `required_sections_digest` —
    sha256 recomputed per required section, (id, hash) pairs hashed;
    layout artifacts excluded; test-fix honesty note on the +24
    integrity-field offset.
  - `M26-006-B` (PR #806, #211 closed): `DetachedSignature`
    out-of-band slot — `sign_pack`, `verify_over` (fails closed on
    algorithm/hex/length/key/mismatch), `verify_digest`.
  - `M26-006-C` (PR #807, #212 closed): `TrustSource`
    Inline/File/Environment + `TrustConfig::load` (union, dedup,
    any-malformed-key fails the whole load) and `verify_signature`
    (empty trust set refuses).
  - `M26-006-D` (PR #808, #213 closed): `AuthenticityPolicy`
    (`#[default] UnsignedAllowed` / `RequireSignature { config }`) +
    `enforce(pack_bytes, Option<&DetachedSignature>)`.
  - `M26-006-V` (PR #809, #214 closed): verification closure; key
    rotation notes (trust-set rotation with explicit overlap windows,
    empty-set fail-closed, no runtime trust anchors); threat-model
    update in `docs/okf/engineering/security-model.md`.
- Required microtask evidence: integrity/signature tests mapped above;
  key rotation notes and threat-model update delivered in M26-006-V's
  verification record and `docs/okf/engineering/security-model.md`.
- Exact verification (fresh on this branch): q-pack 85+2, q-http 11
  (4+6+1), velqu-runtime 28 passed; bun 83 pass / 0 fail / 495
  expect(); typecheck, fmt --check, clippy `-D warnings` clean.
  `./scripts/verify` completes every stage except the documented
  pre-existing benchmark-manifest mismatch (qRuntimeRelease +
  proofPack; flagged matched-evidence follow-up from M26-002-A).
- Evidence-generation fix in this packet: editing
  `engineering/security-model.md` in M26-006-V invalidated the OKF
  bundle tamper manifest (`validate-okf`: size/hash mismatch) because
  the full `./scripts/verify` gate had last run before that edit.
  Refreshed the file's `bytes`+`sha256` entry in
  `docs/okf/MANIFEST.json` following the 102a580 precedent
  (per-entry refresh only). `validate-okf` now PASS (180 links
  checked).
- Status bookkeeping: `docs/beta/04_TASK_LEDGER.md` marks M26-006
  PASS; TASK_INDEX marks M26-006-V PASS (bookkeeping missed by the V
  packet) and M26-006-Z PASS. STATUS.md marks the Z checkbox. The
  generated Spark queues expose M26-007-A next.
- Remaining scope: `M26-007`+ remain TODO until implemented and
  evidenced.
