---
task_id: BETA-GATE
parent_task: BETA-GATE
milestone: BETA
priority: P0
mode: GATE_REVIEW
status: TODO
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-GATE — Public Beta Readiness and Release exit gate

## Atomic goal

Review and decide the BETA exit gate from source, tests, and evidence.

## Parent intent

Public Beta 0.1.0-beta.1

## Dependencies

- `BETA-001-Z` — `tasks/08_public_beta/BETA-001-Z-package-evidence-for-make-the-real-world-benchmark-harness-executable.md`
- `BETA-002-Z` — `tasks/08_public_beta/BETA-002-Z-package-evidence-for-implement-matched-competitor-candidates.md`
- `BETA-003-Z` — `tasks/08_public_beta/BETA-003-Z-package-evidence-for-run-controlled-i-o-and-cpu-jit-crossover-suites.md`
- `BETA-004-Z` — `tasks/08_public_beta/BETA-004-Z-package-evidence-for-implement-optional-first-party-postgres-capability.md`
- `BETA-005-Z` — `tasks/08_public_beta/BETA-005-Z-package-evidence-for-implement-jwt-auth-reference-package.md`
- `BETA-006-Z` — `tasks/08_public_beta/BETA-006-Z-package-evidence-for-implement-beta-observability-baseline.md`
- `BETA-007-Z` — `tasks/08_public_beta/BETA-007-Z-package-evidence-for-implement-configuration-and-secret-handling.md`
- `BETA-008-Z` — `tasks/08_public_beta/BETA-008-Z-package-evidence-for-implement-reverse-proxy-drain-and-deployment-semantics.md`
- `BETA-009-Z` — `tasks/08_public_beta/BETA-009-Z-package-evidence-for-run-beta-security-and-reliability-baseline.md`
- `BETA-010-Z` — `tasks/08_public_beta/BETA-010-Z-package-evidence-for-create-supported-beta-platform-and-packaging-matrix.md`
- `BETA-011-Z` — `tasks/08_public_beta/BETA-011-Z-package-evidence-for-automate-beta-publishing-and-versioning.md`
- `BETA-012-Z` — `tasks/08_public_beta/BETA-012-Z-package-evidence-for-complete-beta-documentation-and-limitations.md`
- `BETA-013-Z` — `tasks/08_public_beta/BETA-013-Z-package-evidence-for-run-beta-soak-and-leak-qualification.md`
- `BETA-014-Z` — `tasks/08_public_beta/BETA-014-Z-package-evidence-for-publish-canonical-beta-benchmark-report.md`
- `BETA-015-Z` — `tasks/08_public_beta/BETA-015-Z-package-evidence-for-generate-beta-release-evidence-sbom-and-checksums.md`
- `BETA-016-Z` — `tasks/08_public_beta/BETA-016-Z-package-evidence-for-run-external-clean-install-and-tutorial-verification.md`
- `BETA-017-Z` — `tasks/08_public_beta/BETA-017-Z-package-evidence-for-resolve-beta-owner-decisions.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `docs/beta/`
- `scripts/verify`
- `package.json`
- `.github/workflows/verify.yml`
- `scripts/package`
- `scripts/release-packet`
- `packages/cli/package.json`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Freeze the candidate commit and confirm a clean working tree.
2. Review every dependency evidence packet against source and test reality.
3. Run the full verification commands from the exact candidate commit.
4. Check raw-to-report parity, index commit hashes, artifact hashes, and unresolved P0/P1 findings.
5. If any criterion fails, keep the gate IN_PROGRESS and list the exact blocking task; do not patch silently inside the gate review.
6. If all criteria pass, update the gate status and produce the milestone review packet, source archive, Git bundle, and checksum manifest.

## Parent acceptance guardrails

- Every parent task for BETA has a passing verification and evidence packet.
- Full project verification passes from a clean tree.
- Evidence indexes identify the exact commit and artifacts.
- No unresolved P0/P1 criterion is hidden or waived without owner/reviewer approval.

## Targeted commands

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

- Milestone report.
- Review index.
- Evidence index.
- Commit-named source archive and Git bundle.
- SHA-256 manifest.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Implementing missing milestone work inside the gate task.
- Waiving P0/P1 without explicit owner/reviewer approval.
- Calling a single benchmark run canonical when repeated evidence is required.

## Commit guidance

Suggested subject:

```text
beta-gate: public beta readiness and release exit gate
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
