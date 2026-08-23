---
task_id: M26-006-B
parent_task: M26-006
milestone: M26
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-006-B — Provide Ed25519-compatible signature slot/hook

## Atomic goal

Provide Ed25519-compatible signature slot/hook.

## Parent intent

Protect pack corruption now and provide optional publisher signature verification for beta artifacts.

## Dependencies

- `M26-006-A` — `tasks/03_m26_qpack_v2/M26-006-A-hash-required-execution-sections.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M26.md`
- `context/components/qpack-router.md`
- `context/components/ingress-bridge.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `crates/q-runtime/src/main.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `docs/specs/pack-format-v1.md`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`
- `packages/auth-jwt/ (create if absent)`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Provide Ed25519-compatible signature slot/hook.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

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
cargo test -p q-bridge
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

- Integrity/signature tests.
- Key rotation notes.
- Threat-model update.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m26-006-b: provide ed25519 compatible signature slot hook
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M26-006-B (PASS)

Deliverable: Ed25519-compatible detached signature slot/hook, exactly
as ADR-0026 frames it — OUT-OF-BAND authenticity for release tooling;
the pack carries no signature fields and NO runtime code path reads
this module.

Changed files:

- `Cargo.toml` + `crates/q-pack/Cargo.toml` + `Cargo.lock` —
  `ed25519-dalek = "2"` (q-pack only).
- `crates/q-pack/src/lib.rs` — `q_pack::signatures`:
  - `DetachedSignature` serde record (the "slot" release pipelines
    publish beside the artifact): algorithm tag, publicKeyHex,
    signatureHex.
  - `sign_pack(&SigningKey, pack_bytes)` — tool side.
  - `verify_over(pack_bytes)` — fails closed on wrong algorithm tag,
    malformed hex, wrong key/signature lengths, invalid key, or
    signature mismatch.
  - `verify_digest(&[u8;32])` — for pipelines committing to the
    M26-006-A `required_sections_digest` instead of raw bytes.

Tests (83 total):

- `ed25519_rfc8032_test_vector_verifies` — RFC 8032 §7.1 TEST 1
  (public constants): verifies on the empty message, rejects any other
  message.
- `detached_signature_roundtrip_and_fail_closed` — sign/verify over a
  bound pack; tampered bytes reject; digest-pipeline sign/verify with
  digest-tamper rejection; wrong algorithm, truncated hex, wrong
  lengths, and a different signer's key all fail closed.

Commands and results:

- `cargo test -p q-pack` — 83 passed + 2, 0 failed.
- `cargo test -p velqu-runtime` — 28 passed.
- `cargo test -p q-engine-quickjs` — 1 + 97 passed.
- `bun test` — 83 pass / 0 fail / 487 expect().
- `bun run typecheck` — clean.
- `cargo fmt`, `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `./scripts/verify` — green except the pre-existing documented
  `validate-benchmark-evidence` scoped failure (flagged follow-up from
  M26-002-A).

Guardrails: signature verifies publisher when configured (roundtrip +
RFC vector); no digest/authenticity conflation (module docs state the
boundary; per-section digests remain the in-band corruption control);
key discovery configuration is M26-006-C; the unsigned-local-dev policy
is M26-006-D.
