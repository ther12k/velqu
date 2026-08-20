---
task_id: BETA-015-V
parent_task: BETA-015
milestone: BETA
priority: P0
mode: VERIFY
status: TODO
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-015-V — Verify Generate beta release evidence, SBOM, and checksums

## Atomic goal

Prove every acceptance criterion for parent task BETA-015 without broadening scope.

## Parent intent

Create a self-verifying public-beta packet.

## Dependencies

- `BETA-015-A` — `tasks/08_public_beta/BETA-015-A-source-zip.md`
- `BETA-015-B` — `tasks/08_public_beta/BETA-015-B-git-bundle.md`
- `BETA-015-C` — `tasks/08_public_beta/BETA-015-C-linux-binaries.md`
- `BETA-015-D` — `tasks/08_public_beta/BETA-015-D-npm-package-tarballs.md`
- `BETA-015-E` — `tasks/08_public_beta/BETA-015-E-qpack-tools.md`
- `BETA-015-F` — `tasks/08_public_beta/BETA-015-F-sbom.md`
- `BETA-015-G` — `tasks/08_public_beta/BETA-015-G-checksums.md`
- `BETA-015-H` — `tasks/08_public_beta/BETA-015-H-review-evidence-indexes.md`
- `BETA-015-I` — `tasks/08_public_beta/BETA-015-I-known-limitations.md`

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

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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

## Required evidence for this microtask

- Release packet.
- Verification transcript.
- Artifact inventory.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
beta-015-v: verify generate beta release evidence sbom and checksums
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
