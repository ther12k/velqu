---
task_id: M26-006-V
parent_task: M26-006
milestone: M26
priority: P1
mode: VERIFY
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-006-V — Verify Implement execution integrity and authenticity hooks

## Atomic goal

Prove every acceptance criterion for parent task M26-006 without broadening scope.

## Parent intent

Protect pack corruption now and provide optional publisher signature verification for beta artifacts.

## Dependencies

- `M26-006-A` — `tasks/03_m26_qpack_v2/M26-006-A-hash-required-execution-sections.md`
- `M26-006-B` — `tasks/03_m26_qpack_v2/M26-006-B-provide-ed25519-compatible-signature-slot-hook.md`
- `M26-006-C` — `tasks/03_m26_qpack_v2/M26-006-C-define-key-discovery-configuration.md`
- `M26-006-D` — `tasks/03_m26_qpack_v2/M26-006-D-keep-unsigned-local-development-supported-with-explicit-policy.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M26.md`
- `context/components/qpack-router.md`
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
- `docs/beta/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m26-006-v: verify implement execution integrity and authenticity hooks
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Verification record — M26-006-V (PASS)

Parent: M26-006 "Implement execution integrity and authenticity
hooks". All four implementation dependencies merged on master before
this branch (PRs #805/#806/#807/#808; issues #210–#213 closed).

### Acceptance criterion mapping

1. **Digest detects corruption.**
   `required_sections_digest_detects_corruption_and_ignores_optional`
   — any required-section byte change changes the recomputed aggregate
   digest EVEN WITH the directory hash repaired (self-consistent
   rewrites are caught); optional-section presence leaves it
   unchanged. In-band reader integrity is independent of any policy:
   `tampered_pack_fails_before_ready` and the integration fuzz suites
   (`random_bytes_never_panic_the_pack_parser`,
   `mutated_valid_pack_never_panic_and_tamper_is_detected`) stay green.

2. **Signature verifies publisher when configured.**
   `ed25519_rfc8032_test_vector_verifies` (RFC 8032 §7.1 TEST 1),
   `detached_signature_roundtrip_and_fail_closed` (sign/verify,
   tamper, digest-pipeline, wrong algorithm/hex/lengths, wrong signer),
   `trust_config_discovers_keys_from_all_sources_and_fails_closed`
   (file+inline+env discovery, dedup, trusted pass, VALID signature by
   untrusted key rejects, malformed key fails the whole load, empty
   trust set refuses to verify).

3. **Unsigned production policy is explicit.**
   `unsigned_local_dev_is_explicit_and_production_requires_signatures`
   — `AuthenticityPolicy::default()` is `UnsignedAllowed`; unsigned
   pack passes with in-band integrity unaffected; a PRESENT signature
   is still verified and a tampered one rejects; `RequireSignature`
   rejects unsigned with an explicit reason, passes the trusted
   signer, rejects an untrusted signer; the policy round-trips JSON.

4. **No docs conflate digest and authenticity.**
   `docs/specs/pack-format-v1.md` states the ADR-0026 boundary
   verbatim ("integrity only … authenticity is out-of-band deployment
   policy"); `q_pack::signatures` module docs restate it; the runtime
   has no code path into the signatures module (tooling-only). The
   threat model now reflects delivered state (below).

### Key rotation notes (required evidence)

Rotation is a release-tooling operation on `TrustConfig` sources; no
runtime trust anchors exist (ADR-0026), so there is no runtime-side
rotation path to migrate:

1. add the NEW publisher public key to the trust set (keyring file /
   env / inline config — union, deduped);
2. sign new artifacts with the new key (old-key artifacts still verify
   while both keys are trusted — overlap window is explicit, never an
   "accept anything" state because the trust set is always enumerated);
3. remove the OLD key after the migration window. An empty trust set
   fails closed, so rotation cannot degrade into unsigned acceptance.
Any malformed key in ANY source fails the whole load — a partially
rotated keyring is never usable. (Application-level JWT credential
rotation is separate scope: BETA-005-B.)

### Threat-model update (required evidence)

`docs/okf/engineering/security-model.md`, "Pack/bytecode tampering":
replaced the stale "content hash and future signature" control with
the delivered state — per-section digests + required-sections
execution-integrity binding (integrity only), plus the out-of-band
detached-signature authenticity policy with unsigned local development
as the explicit default (M26-006-A/B/C/D, ADR-0026), restating that
integrity and authenticity are never conflated.

### Changed files

- `docs/codex-spark-beta/tasks/03_m26_qpack_v2/M26-006-V-verify-implement-execution-integrity-and-authenticity-hooks.md` — this record.
- `docs/codex-spark-beta/STATUS.md` — V checkbox marked.
- `docs/okf/engineering/security-model.md` — threat-model update above.
No code defects found requiring fixes; no unrelated findings needing
follow-up tasks.

### Commands and results (branch on master d995321)

- `cargo test -p q-pack` — 85 passed + 2, 0 failed.
- `cargo test -p velqu-runtime` — 28 passed, 0 failed.
- `bun test` — 83 pass / 0 fail / 495 expect().
- `bun run typecheck` — clean.
- `cargo fmt --check` — clean.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `./scripts/verify` — all gates green except the pre-existing
  documented `validate-benchmark-evidence` scoped failure (manifest
  hash mismatch for qRuntimeRelease + proofPack; flagged
  matched-evidence follow-up from M26-002-A, not altered here).

Environment notes for reproducibility: (1) bare `bun test` at repo root
requires `examples/proof/dist` — build it first (`bun packages/cli/src/
index.ts build --project examples/proof`) exactly as `scripts/verify`
does, otherwise projection-parity reports ENOENT. (2)
`cargo test -p velqu-runtime` does not rebuild the sibling
`velqu-bytecode` binary; a stale target/debug copy predating current
workspace sources makes two bytecode tests fail at the embed step —
`cargo build -p q-bytecode-tool` refreshes it. Both were environment
artifacts of this checkout, not product defects.
