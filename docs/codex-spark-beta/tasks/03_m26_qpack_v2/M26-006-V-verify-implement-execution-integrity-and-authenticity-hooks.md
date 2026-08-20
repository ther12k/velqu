---
task_id: M26-006-V
parent_task: M26-006
milestone: M26
priority: P1
mode: VERIFY
status: TODO
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
