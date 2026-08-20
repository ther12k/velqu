---
task_id: M28-001-Z
parent_task: M28-001
milestone: M28
priority: P0
mode: EVIDENCE
status: TODO
context_card: context/milestones/M28.md
commit_required: true
---

# M28-001-Z — Package evidence for Accept fetch, TLS, redirect, and SSRF security ADR

## Atomic goal

Create source-backed evidence and handoff for parent task M28-001; update status only if verification passed.

## Parent intent

Freeze the public subset, trust boundaries, defaults, and non-goals.

## Dependencies

- `M28-001-V` — `tasks/05_m28_native_fetch/M28-001-V-verify-accept-fetch-tls-redirect-and-ssrf-security-adr.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M28.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`

### Source files

- `AGENTS.md`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-http/src/lib.rs`
- `Cargo.toml`
- `conformance/security/security.conformance.test.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-http/tests/fuzz_parsers.rs`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `.github/workflows/verify.yml`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- Security defaults fail closed.
- Private/link-local/metadata behavior is explicit.
- Redirect revalidation is required.
- Direct TLS policy is documented.

## Targeted commands

```bash
cargo test -p q-pack
```
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

## Required evidence for this microtask

- ADR.
- Threat model.
- Security test matrix.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m28-001-z: package evidence for accept fetch tls redirect and ssrf secu
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
