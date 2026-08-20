---
task_id: M28-001-V
parent_task: M28-001
milestone: M28
priority: P0
mode: VERIFY
status: TODO
context_card: context/milestones/M28.md
commit_required: true
---

# M28-001-V — Verify Accept fetch, TLS, redirect, and SSRF security ADR

## Atomic goal

Prove every acceptance criterion for parent task M28-001 without broadening scope.

## Parent intent

Freeze the public subset, trust boundaries, defaults, and non-goals.

## Dependencies

- `M28-001-A` — `tasks/05_m28_native_fetch/M28-001-A-define-url-schemes-redirect-policy-dns-rebinding-controls-proxy-behavior-tls-roo.md`
- `M28-001-B` — `tasks/05_m28_native_fetch/M28-001-B-specify-reverse-proxy-and-outbound-trust.md`
- `M28-001-C` — `tasks/05_m28_native_fetch/M28-001-C-define-unsupported-web-features.md`
- `M28-001-D` — `tasks/05_m28_native_fetch/M28-001-D-document-same-process-trusted-code-assumption.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M28.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/schema-codecs.md`

### Source files

- `AGENTS.md`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-http/src/lib.rs`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `crates/q-engine-quickjs/src/convert.rs`
- `packages/schema/src/index.ts`
- `conformance/schema/schema.conformance.test.ts`
- `Cargo.toml`
- `conformance/security/security.conformance.test.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Security defaults fail closed.
- Private/link-local/metadata behavior is explicit.
- Redirect revalidation is required.
- Direct TLS policy is documented.

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

- ADR.
- Threat model.
- Security test matrix.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m28-001-v: verify accept fetch tls redirect and ssrf security adr
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
