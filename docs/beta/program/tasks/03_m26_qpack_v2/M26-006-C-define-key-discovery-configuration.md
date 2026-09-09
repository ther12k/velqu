---
task_id: M26-006-C
parent_task: M26-006
milestone: M26
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-006-C — Define key discovery/configuration

## Atomic goal

Define key discovery/configuration.

## Parent intent

Protect pack corruption now and provide optional publisher signature verification for beta artifacts.

## Dependencies

- `M26-006-B` — `tasks/03_m26_qpack_v2/M26-006-B-provide-ed25519-compatible-signature-slot-hook.md`

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

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Define key discovery/configuration.
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
m26-006-c: define key discovery configuration
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M26-006-C (PASS)

Deliverable: key discovery/configuration for the OUT-OF-BAND
verification tooling (ADR-0026: the runtime never loads keys — this
configuration belongs to release pipelines and operators).

Change (`crates/q-pack/src/lib.rs`, `q_pack::signatures`):

- `TrustSource` (serde-tagged enum): `Inline { keys }`,
  `File { path }` (one hex key per line; blank lines and `#` comments
  ignored), `Environment { var }` (newline-separated).
- `TrustConfig { sources }`: `load()` unions all sources, validates
  every key (32 bytes, valid hex), dedups, and FAILS CLOSED on any
  malformed key in any source — a partially trusted keyring is never
  usable. `verify_signature(&DetachedSignature, pack_bytes)` verifies
  the signature AND that its signer key is in the trust set; an empty
  trust set refuses to verify anything.

Test (`trust_config_discovers_keys_from_all_sources_and_fails_closed`,
84 total):

- keys discovered from file (with comments/blank lines) + inline + env;
  duplicates dedup;
- signature by a trusted publisher verifies; tampered bytes reject;
- a VALID ed25519 signature by an untrusted key rejects
  ("not in the trust set");
- malformed/short keys fail the whole load; empty trust set fails
  closed;
- the config round-trips through JSON (release-pipeline config file).

Commands and results:

- `cargo test -p q-pack` — 84 passed + 2, 0 failed.
- `cargo test -p velqu-runtime` — 28 passed.
- `cargo test -p q-engine-quickjs` — 1 + 97 passed.
- `bun test` — 83 pass / 0 fail / 487 expect().
- `bun run typecheck` — clean.
- `cargo fmt`, `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `./scripts/verify` — green except the pre-existing documented
  `validate-benchmark-evidence` scoped failure (flagged follow-up from
  M26-002-A).

Guardrails: signature verifies publisher when configured (trust-set
authorization on top of M26-006-B verification); no runtime trust
anchors (tooling-only module, untouched by any runtime path); the
unsigned-local-dev policy is M26-006-D.
