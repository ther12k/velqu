---
task_id: BETA-015-D
parent_task: BETA-015
milestone: BETA
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-015-D — npm package tarballs

## Atomic goal

npm package tarballs.

## Parent intent

Create a self-verifying public-beta packet.

## Dependencies

- `BETA-015-C` — `tasks/08_public_beta/BETA-015-C-linux-binaries.md`

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

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: npm package tarballs.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

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

## Required evidence for this microtask

- Release packet.
- Verification transcript.
- Artifact inventory.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
beta-015-d: npm package tarballs
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-015-D) — PASS (2026-09-05)

- Branch/PR: beta-015-d (squash-merged; see git log for final hash)
- Closes: #600

### Behavior implemented

npm package tarballs deliverable of the beta release evidence:
- Added `scripts/npm-package-tarballs.sh`: packs all 9 `@velqu/*` workspace packages with `bun pm pack` into `release/npm-tarballs/`, writes and verifies `SHA256SUMS.txt` from inside the directory, fails closed on missing packages.
- Rehearsed: 9/9 tarballs packed (velqu-core … velqu-treaty, all 0.1.0), checksums all OK, ending `NPM-TARBALLS-OK`.
- Documented the artifact inventory (sizes + sha256 prefixes) in `docs/reports/beta-015-d-npm-package-tarballs.md`.
- Packages remain `private`; publication stays Owner-gated (BETA-010-C/BETA-011 posture), and the `0.1.0-beta.1` label is applied at publication under Owner authority, not silently here.

### Changed files

- `scripts/npm-package-tarballs.sh` (new)
- `docs/reports/beta-015-d-npm-package-tarballs.md`
- `docs/codex-spark-beta/tasks/08_public_beta/BETA-015-D-npm-package-tarballs.md`
- `docs/codex-spark-beta/STATUS.md`
- `docs/codex-spark-beta/indexes/TASK_INDEX.md`

### Gates

- `cargo test -p q-pack` — pass (100+2)
- `cargo test -p q-http` — pass (15)
- `cargo test -p q-schema-runtime` — pass (58)
- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `cargo fmt --all --check` / `cargo clippy -D warnings` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

### Disclosures

- Generated evidence only; registry publication is Owner-gated.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
