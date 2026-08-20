---
task_id: BETA-015-Z
parent_task: BETA-015
milestone: BETA
priority: P0
mode: EVIDENCE
status: TODO
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-015-Z — Package evidence for Generate beta release evidence, SBOM, and checksums

## Atomic goal

Create source-backed evidence and handoff for parent task BETA-015; update status only if verification passed.

## Parent intent

Create a self-verifying public-beta packet.

## Dependencies

- `BETA-015-V` — `tasks/08_public_beta/BETA-015-V-verify-generate-beta-release-evidence-sbom-and-checksums.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/qpack-router.md`
- `context/components/ingress-bridge.md`
- `context/components/schema-codecs.md`

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
- `conformance/security/security.conformance.test.ts`
- `crates/q-http/tests/fuzz_parsers.rs`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- Checksums verify from release directory.
- Artifacts map to one source commit.
- SBOM identifies dependencies/licenses.
- No stale historical metadata is current.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-http
```
```bash
cargo test -p q-schema-runtime
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

- Release packet.
- Verification transcript.
- Artifact inventory.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
beta-015-z: package evidence for generate beta release evidence sbom and
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
